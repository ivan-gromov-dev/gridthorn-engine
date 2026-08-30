use super::Color;
use gridthorn_assets::TextureAsset;

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
        }
    }

    /// Set the multiplicative texture tint.
    #[must_use]
    pub fn with_tint(mut self, tint: Color) -> Self {
        self.tint = tint;
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
}
