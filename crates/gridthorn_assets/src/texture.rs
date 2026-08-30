use std::path::{Path, PathBuf};
use std::sync::Arc;

use image::ImageReader;
use thiserror::Error;

/// Decoded RGBA8 texture data independent of renderer resources.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextureAsset {
    dimensions: [u32; 2],
    pixels: Arc<[u8]>,
}

impl TextureAsset {
    /// Load and decode an image file as RGBA8 texture data.
    ///
    /// # Errors
    ///
    /// Returns a contextual error when the file cannot be opened, its format
    /// cannot be determined, or its image data cannot be decoded.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, TextureAssetError> {
        let path = path.as_ref();
        let reader = ImageReader::open(path).map_err(|source| TextureAssetError::Open {
            path: path.to_path_buf(),
            source,
        })?;
        let reader = reader
            .with_guessed_format()
            .map_err(|source| TextureAssetError::Open {
                path: path.to_path_buf(),
                source,
            })?;
        let image = reader
            .decode()
            .map_err(|source| TextureAssetError::Decode {
                path: path.to_path_buf(),
                source,
            })?;
        let rgba = image.into_rgba8();
        Ok(Self {
            dimensions: [rgba.width(), rgba.height()],
            pixels: Arc::from(rgba.into_raw()),
        })
    }

    /// Width and height in texels.
    #[must_use]
    pub fn dimensions(&self) -> [u32; 2] {
        self.dimensions
    }

    /// Decoded pixels in row-major RGBA8 order.
    #[must_use]
    pub fn rgba8(&self) -> &[u8] {
        &self.pixels
    }
}

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

#[cfg(test)]
mod test;
