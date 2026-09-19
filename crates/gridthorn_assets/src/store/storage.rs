use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::{AssetId, AssetStoreError};
use crate::TextureAsset;

#[derive(Clone, Debug)]
struct Entry {
    bytes: Arc<[u8]>,
    texture: Option<TextureAsset>,
    dependencies: BTreeSet<AssetId>,
}

/// Provisional synchronous asset dependency and hot-reload service.
///
/// Register raw files or textures, then declare dependency edges. Call
/// `reload_changed` once at a chosen frame boundary before extracting render
/// input. Reads compare contents, so equal-sized edits and coarse filesystem
/// timestamps are supported. A failed read or decode commits nothing and is
/// retried on the next call. Unchanged assets keep their shared allocations.
///
/// Existing texture clones and byte snapshots remain immutable; resolve IDs
/// again for new frames. The service is `Send + Sync`, needs exclusive mutable
/// access to reload, and starts no threads. Root paths should be absolute if
/// the process may change its working directory. Polling reads every registered
/// file and is intended for small development projects, not large asset sets.
#[derive(Debug)]
pub struct AssetStore {
    root: PathBuf,
    entries: BTreeMap<AssetId, Entry>,
}

impl AssetStore {
    pub(crate) fn snapshot(&self) -> Self {
        Self {
            root: self.root.clone(),
            entries: self.entries.clone(),
        }
    }

    /// Create an empty store rooted at the supplied asset directory.
    #[must_use]
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self {
            root: root.into(),
            entries: BTreeMap::new(),
        }
    }

    /// Register a raw file, such as metadata consumed by game code.
    ///
    /// # Errors
    /// Returns a duplicate-ID or contextual read error without changing storage.
    pub fn load_source(&mut self, id: AssetId) -> Result<(), AssetStoreError> {
        self.load(id, false)
    }

    /// Register and decode a texture using the same ID across future reloads.
    ///
    /// # Errors
    /// Returns a duplicate-ID, read, or decode error without changing storage.
    pub fn load_texture(&mut self, id: AssetId) -> Result<(), AssetStoreError> {
        self.load(id, true)
    }

    /// Replace direct dependency edges; all dependencies must already exist.
    ///
    /// Duplicate edges are collapsed. Edges propagate invalidation, but do not
    /// interpret raw metadata or invoke custom loaders. Set edges during setup;
    /// changing the graph does not itself reload any data.
    ///
    /// # Errors
    /// Unknown IDs and cycles are rejected without changing the previous graph.
    pub fn set_dependencies(
        &mut self,
        id: &AssetId,
        dependencies: &[AssetId],
    ) -> Result<(), AssetStoreError> {
        self.entry(id)?;
        for dependency in dependencies {
            self.entry(dependency)?;
            if self.reaches(dependency, id) {
                return Err(AssetStoreError::DependencyCycle { id: id.clone() });
            }
        }
        if let Some(entry) = self.entries.get_mut(id) {
            entry.dependencies = dependencies.iter().cloned().collect();
        }
        Ok(())
    }

    /// Resolve the current decoded texture, or `None` for raw/unregistered IDs.
    #[must_use]
    pub fn texture(&self, id: &AssetId) -> Option<&TextureAsset> {
        self.entries.get(id)?.texture.as_ref()
    }

    /// Resolve the last committed source bytes for either kind of asset.
    #[must_use]
    pub fn source(&self, id: &AssetId) -> Option<Arc<[u8]>> {
        self.entries.get(id).map(|entry| Arc::clone(&entry.bytes))
    }

    /// Poll files and atomically replace changed assets and transitive dependents.
    ///
    /// Returns affected IDs in dependency-first order, breaking available ties
    /// lexicographically. Dependents are re-decoded even if their own bytes did
    /// not change. Raw dependents appear in the result so callers can rebuild
    /// their derived presentation data. An empty result means no content changed.
    /// Files are read once per call; external writes during polling are not a
    /// filesystem transaction. Authoritative data requires a separate validated
    /// load/reset boundary rather than adopting these presentation snapshots.
    ///
    /// # Errors
    /// Any read or decode failure preserves all previous bytes and textures,
    /// including the comparison baseline, so a later call retries the change.
    pub fn reload_changed(&mut self) -> Result<Vec<AssetId>, AssetStoreError> {
        let mut snapshots = BTreeMap::new();
        let mut affected = BTreeSet::new();
        for (id, entry) in &self.entries {
            let bytes = self.read(id)?;
            if bytes.as_slice() != entry.bytes.as_ref() {
                affected.insert(id.clone());
            }
            snapshots.insert(id.clone(), bytes);
        }
        loop {
            let previous_len = affected.len();
            for (id, entry) in &self.entries {
                if entry
                    .dependencies
                    .iter()
                    .any(|dependency| affected.contains(dependency))
                {
                    affected.insert(id.clone());
                }
            }
            if previous_len == affected.len() {
                break;
            }
        }
        let order = self.dependency_order(affected);
        let mut prepared = BTreeMap::new();
        for id in &order {
            let bytes = &snapshots[id];
            let texture = if self.entries[id].texture.is_some() {
                Some(self.decode(id, bytes)?)
            } else {
                None
            };
            prepared.insert(id.clone(), (Arc::from(bytes.as_slice()), texture));
        }
        for (id, (bytes, texture)) in prepared {
            if let Some(entry) = self.entries.get_mut(&id) {
                entry.bytes = bytes;
                entry.texture = texture;
            }
        }
        Ok(order)
    }

    fn load(&mut self, id: AssetId, is_texture: bool) -> Result<(), AssetStoreError> {
        if self.entries.contains_key(&id) {
            return Err(AssetStoreError::Duplicate { id });
        }
        let bytes = self.read(&id)?;
        let texture = if is_texture {
            Some(self.decode(&id, &bytes)?)
        } else {
            None
        };
        self.entries.insert(
            id,
            Entry {
                bytes: Arc::from(bytes),
                texture,
                dependencies: BTreeSet::new(),
            },
        );
        Ok(())
    }

    fn read(&self, id: &AssetId) -> Result<Vec<u8>, AssetStoreError> {
        let path = self.root.join(Path::new(id.as_str()));
        std::fs::read(&path).map_err(|source| AssetStoreError::Read {
            id: id.clone(),
            path,
            source,
        })
    }

    fn decode(&self, id: &AssetId, bytes: &[u8]) -> Result<TextureAsset, AssetStoreError> {
        TextureAsset::decode(bytes, &self.root.join(id.as_str())).map_err(|source| {
            AssetStoreError::Texture {
                id: id.clone(),
                source,
            }
        })
    }

    fn entry(&self, id: &AssetId) -> Result<&Entry, AssetStoreError> {
        self.entries
            .get(id)
            .ok_or_else(|| AssetStoreError::Unknown { id: id.clone() })
    }

    fn reaches(&self, start: &AssetId, target: &AssetId) -> bool {
        let mut pending = vec![start];
        let mut visited = BTreeSet::new();
        while let Some(id) = pending.pop() {
            if id == target {
                return true;
            }
            if visited.insert(id) {
                pending.extend(&self.entries[id].dependencies);
            }
        }
        false
    }

    fn dependency_order(&self, mut pending: BTreeSet<AssetId>) -> Vec<AssetId> {
        let mut ordered = Vec::with_capacity(pending.len());
        while let Some(id) = pending
            .iter()
            .find(|id| {
                self.entries[*id]
                    .dependencies
                    .iter()
                    .all(|dependency| !pending.contains(dependency))
            })
            .cloned()
        {
            pending.remove(&id);
            ordered.push(id);
        }
        ordered
    }
}
