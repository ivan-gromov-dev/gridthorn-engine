use super::Color;
use gridthorn_assets::TextureAsset;

mod errors;

pub use errors::SpriteRegionError;

/// Normalized rectangular region of a sprite-sheet texture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpriteRegion {
    min: [f32; 2],
    max: [f32; 2],
}

impl SpriteRegion {
    /// Construct a normalized region from its top-left and bottom-right UV corners.
    ///
    /// # Errors
    ///
    /// Returns [`SpriteRegionError::InvalidBounds`] unless every coordinate is
    /// finite and inside `0.0..=1.0`, with `min` strictly before `max`.
    pub fn new(min: [f32; 2], max: [f32; 2]) -> Result<Self, SpriteRegionError> {
        let valid = min
            .into_iter()
            .chain(max)
            .all(|coordinate| coordinate.is_finite() && (0.0..=1.0).contains(&coordinate))
            && min[0] < max[0]
            && min[1] < max[1];
        if !valid {
            return Err(SpriteRegionError::InvalidBounds);
        }
        Ok(Self { min, max })
    }

    /// Region covering the complete texture.
    #[must_use]
    pub const fn full() -> Self {
        Self {
            min: [0.0, 0.0],
            max: [1.0, 1.0],
        }
    }

    /// Top-left normalized UV coordinate.
    #[must_use]
    pub const fn min(self) -> [f32; 2] {
        self.min
    }

    /// Bottom-right normalized UV coordinate.
    #[must_use]
    pub const fn max(self) -> [f32; 2] {
        self.max
    }
}

/// One colored axis-aligned sprite in world space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sprite {
    position: [f32; 2],
    size: [f32; 2],
    color: Color,
}

/// One textured axis-aligned sprite in world space.
#[derive(Clone, Debug, PartialEq)]
pub struct TexturedSprite {
    position: [f32; 2],
    size: [f32; 2],
    texture: TextureAsset,
    tint: Color,
    region: SpriteRegion,
}

impl Sprite {
    /// Create a colored sprite centered on a world position.
    #[must_use]
    pub fn new(position: [f32; 2], size: [f32; 2], color: Color) -> Self {
        Self {
            position,
            size,
            color,
        }
    }

    /// Center position in world units.
    #[must_use]
    pub fn position(self) -> [f32; 2] {
        self.position
    }

    /// Width and height in world units.
    #[must_use]
    pub fn size(self) -> [f32; 2] {
        self.size
    }

    /// Sprite tint color.
    #[must_use]
    pub fn color(self) -> Color {
        self.color
    }
}

impl TexturedSprite {
    /// Create a textured sprite centered on a world position.
    #[must_use]
    pub fn new(position: [f32; 2], size: [f32; 2], texture: TextureAsset) -> Self {
        Self {
            position,
            size,
            texture,
            tint: Color::rgb(1.0, 1.0, 1.0),
            region: SpriteRegion::full(),
        }
    }

    /// Set the multiplicative texture tint.
    #[must_use]
    pub fn with_tint(mut self, tint: Color) -> Self {
        self.tint = tint;
        self
    }

    /// Select the normalized sprite-sheet region sampled by this sprite.
    #[must_use]
    pub fn with_region(mut self, region: SpriteRegion) -> Self {
        self.region = region;
        self
    }

    /// Center position in world units.
    #[must_use]
    pub fn position(&self) -> [f32; 2] {
        self.position
    }

    /// Width and height in world units.
    #[must_use]
    pub fn size(&self) -> [f32; 2] {
        self.size
    }

    /// Decoded source texture.
    #[must_use]
    pub fn texture(&self) -> &TextureAsset {
        &self.texture
    }

    /// Multiplicative texture tint.
    #[must_use]
    pub fn tint(&self) -> Color {
        self.tint
    }

    /// Normalized sprite-sheet region sampled by this sprite.
    #[must_use]
    pub fn region(&self) -> SpriteRegion {
        self.region
    }
}

#[cfg(test)]
mod test;
