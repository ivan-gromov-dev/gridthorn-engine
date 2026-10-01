use std::{fmt, io, path::PathBuf};

/// Filesystem failure while preparing a complete source snapshot.
#[derive(Debug)]
pub(crate) struct WatchError {
    pub(crate) path: PathBuf,
    pub(crate) source: io::Error,
}

impl fmt::Display for WatchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "cannot scan watch input {}: {}",
            self.path.display(),
            self.source
        )
    }
}

impl std::error::Error for WatchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
