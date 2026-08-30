use std::path::Path;
use std::sync::Arc;

use image::ImageReader;

mod errors;

pub use errors::TextureAssetError;

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

#[cfg(test)]
mod test;
