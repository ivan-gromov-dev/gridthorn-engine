use super::{Camera2d, Sprite, TexturedSprite, TimingOverlay, UiPrimitive};

/// Immutable presentation snapshot consumed by the renderer.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RenderFrame {
    camera: Camera2d,
    sprites: Vec<Sprite>,
    textured_sprites: Vec<TexturedSprite>,
    timing_overlay: Option<TimingOverlay>,
    ui: Vec<UiPrimitive>,
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
            ui: Vec::new(),
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

    /// Attach ordered screen-space UI rendered after world presentation.
    #[must_use]
    pub fn with_ui(mut self, ui: Vec<UiPrimitive>) -> Self {
        self.ui = ui;
        self
    }

    /// Ordered screen-space UI primitives.
    #[must_use]
    pub fn ui(&self) -> &[UiPrimitive] {
        &self.ui
    }
}
