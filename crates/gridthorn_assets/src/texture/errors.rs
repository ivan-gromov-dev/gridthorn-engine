use std::path::PathBuf;

use thiserror::Error;

/// Failure to load or decode a texture asset.
#[derive(Debug, Error)]
pub enum TextureAssetError {
    /// The source file could not be opened or inspected.
    #[error("could not open texture asset '{}': {source}", path.display())]
    Open {
        /// Requested source path.
        path: PathBuf,
        /// Underlying filesystem error.
        #[source]
        source: std::io::Error,
    },
    /// The source bytes are not a supported image.
    #[error("could not decode texture asset '{}': {source}", path.display())]
    Decode {
        /// Requested source path.
        path: PathBuf,
        /// Underlying decoder error.
        #[source]
        source: image::ImageError,
    },
}
