/// Orthographic 2D camera measured in world units.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera2d {
    center: [f32; 2],
    viewport_height: f32,
}

impl Camera2d {
    /// Create a camera centered on a world position.
    #[must_use]
    pub fn new(center: [f32; 2], viewport_height: f32) -> Self {
        Self {
            center,
            viewport_height,
        }
    }

    /// World position shown at the center of the viewport.
    #[must_use]
    pub fn center(self) -> [f32; 2] {
        self.center
    }

    /// Vertical extent visible in world units.
    #[must_use]
    pub fn viewport_height(self) -> f32 {
        self.viewport_height
    }
}

impl Default for Camera2d {
    fn default() -> Self {
        Self::new([0.0, 0.0], 600.0)
    }
}
