use super::{Camera2d, Sprite};

/// Immutable presentation snapshot consumed by the renderer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderFrame {
    camera: Camera2d,
    sprites: Vec<Sprite>,
}

impl RenderFrame {
    /// Create a frame from one camera and an ordered sprite list.
    #[must_use]
    pub fn new(camera: Camera2d, sprites: Vec<Sprite>) -> Self {
        Self { camera, sprites }
    }

    /// Camera used for this frame.
    #[must_use]
    pub fn camera(&self) -> Camera2d {
        self.camera
    }

    /// Sprites in back-to-front submission order.
    #[must_use]
    pub fn sprites(&self) -> &[Sprite] {
        &self.sprites
    }
}
