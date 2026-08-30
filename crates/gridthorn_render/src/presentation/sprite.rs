use super::Color;

/// One colored axis-aligned sprite in world space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sprite {
    position: [f32; 2],
    size: [f32; 2],
    color: Color,
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
