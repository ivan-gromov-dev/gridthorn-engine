use std::{io::Read, path::Path, sync::Arc};

mod errors;
pub use errors::FontAssetError;

/// Validated in-memory OpenType/TrueType font or font collection, without GPU state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FontAsset {
    bytes: Arc<[u8]>,
    families: Vec<String>,
}

impl FontAsset {
    /// Validate font bytes and retain their family names. Maximum size is 32 MiB.
    ///
    /// # Errors
    /// Returns an error for oversized data or data with no usable font faces.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, FontAssetError> {
        if bytes.len() > 32 * 1024 * 1024 {
            return Err(FontAssetError::TooLarge);
        }
        let mut database = fontdb::Database::new();
        database.load_font_data(bytes.clone());
        let mut families: Vec<_> = database
            .faces()
            .flat_map(|face| face.families.iter().map(|(name, _)| name.clone()))
            .collect();
        families.sort();
        families.dedup();
        if families.is_empty() {
            return Err(FontAssetError::InvalidFont);
        }
        Ok(Self {
            bytes: bytes.into(),
            families,
        })
    }

    /// Read and validate a font file independently of platform font discovery.
    ///
    /// # Errors
    /// Reports the path for I/O failures and rejects invalid or oversized fonts.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, FontAssetError> {
        let path = path.as_ref();
        let file = std::fs::File::open(path).map_err(|source| FontAssetError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        let mut bytes = Vec::new();
        file.take(32 * 1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|source| FontAssetError::Read {
                path: path.to_path_buf(),
                source,
            })?;
        Self::from_bytes(bytes)
    }

    /// Original font data; cloned assets share its allocation.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Sorted, unique family names available in the asset.
    #[must_use]
    pub fn families(&self) -> &[String] {
        &self.families
    }
}

#[cfg(test)]
mod test;
