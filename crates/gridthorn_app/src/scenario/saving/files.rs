use std::fs::{self, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{WorldSaveCodec, WorldSaveError};
use crate::ScenarioRuntime;

static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);
const MAX_FILE_BYTES: u64 = 16 * 1024 * 1024;

fn io_error(operation: &'static str, path: &Path, source: std::io::Error) -> WorldSaveError {
    WorldSaveError::Io {
        operation,
        path: path.to_owned(),
        source,
    }
}

/// Create a unique sibling without truncating existing files; rename stays on one filesystem.
pub(super) fn replace(path: &Path, contents: &[u8]) -> Result<(), WorldSaveError> {
    let name = path
        .file_name()
        .ok_or_else(|| WorldSaveError::Document("save path requires a filename".to_owned()))?;
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let (temporary, mut file) = loop {
        let id = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let mut temporary_name = name.to_os_string();
        temporary_name.push(format!(".{}.{}.tmp", std::process::id(), id));
        let temporary = parent.join(temporary_name);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
        {
            Ok(file) => break (temporary, file),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(io_error("create temporary file", &temporary, error)),
        }
    };
    let prepared = file.write_all(contents).and_then(|()| file.sync_all());
    drop(file);
    let result = prepared
        .map_err(|error| io_error("write/sync temporary file", &temporary, error))
        .and_then(|()| {
            fs::rename(&temporary, path).map_err(|error| io_error("replace", path, error))
        });
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result
}

impl<S, C> ScenarioRuntime<S, C>
where
    S: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Encode completely, sync a unique sibling file, then atomically replace the destination.
    /// Does not create parent directories. Parent-directory power-loss durability is not promised.
    ///
    /// # Errors
    /// Returns capture/codec errors, the 16 MiB file limit, or contextual filesystem errors.
    /// A failure before replacement leaves the previous destination intact.
    pub fn save_file(
        &mut self,
        path: impl AsRef<Path>,
        codec: &impl WorldSaveCodec<S, C>,
    ) -> Result<(), WorldSaveError> {
        let document = self.save_document(codec)?;
        if document.len() as u64 > MAX_FILE_BYTES {
            return Err(WorldSaveError::Document(
                "save exceeds 16 MiB file limit".to_owned(),
            ));
        }
        replace(path.as_ref(), document.as_bytes())
    }

    /// Read at most 16 MiB and validate before changing live state at the load boundary.
    ///
    /// # Errors
    /// Returns contextual read errors, oversized/invalid UTF-8 documents, or load validation errors.
    pub fn load_file(
        &mut self,
        path: impl AsRef<Path>,
        codec: &impl WorldSaveCodec<S, C>,
    ) -> Result<(), WorldSaveError> {
        let path = path.as_ref();
        let file = fs::File::open(path).map_err(|error| io_error("open", path, error))?;
        let mut bytes = Vec::new();
        file.take(MAX_FILE_BYTES + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| io_error("read", path, error))?;
        if bytes.len() as u64 > MAX_FILE_BYTES {
            return Err(WorldSaveError::Document(
                "save exceeds 16 MiB file limit".to_owned(),
            ));
        }
        let document = std::str::from_utf8(&bytes)
            .map_err(|error| WorldSaveError::Document(error.to_string()))?;
        self.load_document(document, codec)
    }
}
