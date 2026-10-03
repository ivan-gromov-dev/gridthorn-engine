use std::path::PathBuf;

/// Font loading and validation failures.
#[derive(Debug, thiserror::Error)]
pub enum FontAssetError {
    /// File access failed.
    #[error("cannot read font {path}: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    /// Input did not contain a usable font face.
    #[error("font data contains no usable OpenType or TrueType faces")]
    InvalidFont,
    /// Input exceeded the memory budget.
    #[error("font asset exceeds the 32 MiB limit")]
    TooLarge,
}
