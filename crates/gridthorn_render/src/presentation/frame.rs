use super::{Camera2d, Sprite, TexturedSprite, TimingOverlay};

/// Immutable presentation snapshot consumed by the renderer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderFrame {
    camera: Camera2d,
    sprites: Vec<Sprite>,
    textured_sprites: Vec<TexturedSprite>,
    timing_overlay: Option<TimingOverlay>,
}

impl RenderFrame {
    /// Create a frame from one camera and an ordered sprite list.
    #[must_use]
    pub fn new(camera: Camera2d, sprites: Vec<Sprite>) -> Self {
        Self {
            camera,
            sprites,
            textured_sprites: Vec::new(),
            timing_overlay: None,
        }
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

    /// Add ordered textured sprites to this snapshot.
    #[must_use]
    pub fn with_textured_sprites(mut self, sprites: Vec<TexturedSprite>) -> Self {
        self.textured_sprites = sprites;
        self
    }

    /// Ordered textured sprites in this snapshot.
    #[must_use]
    pub fn textured_sprites(&self) -> &[TexturedSprite] {
        &self.textured_sprites
    }

    /// Attach a screen-space diagnostic timing overlay.
    #[must_use]
    pub fn with_timing_overlay(mut self, overlay: TimingOverlay) -> Self {
        self.timing_overlay = Some(overlay);
        self
    }

    /// Timing overlay requested for this snapshot.
    #[must_use]
    pub fn timing_overlay(&self) -> Option<TimingOverlay> {
        self.timing_overlay
    }
}
