//! Provisional renderer services for Gridthorn.

mod graphics;
mod presentation;
mod surface;
mod text;

pub use text::{
    RasterText, TextAlignment, TextError, TextGlyph, TextLayout, TextLine, TextMeasurement,
    TextStyle, TextSystem, TextWrap,
};

pub use graphics::{
    GraphicsAdapter, GraphicsAdapterCompatibility, GraphicsAdapterKey, GraphicsAdapters,
    GraphicsBackend, enumerate_graphics_adapters,
};
pub use graphics::{GraphicsDevice, GraphicsDeviceKey, GraphicsSelection, graphics_devices};
pub use presentation::{
    AnimationClip, AnimationClipError, AnimationPlayback, AnimationPlayer, Camera2d, Color,
    RenderFrame, Sprite, SpriteRegion, SpriteRegionError, TextLabel, TexturedSprite, TimingOverlay,
    UiError, UiPrimitive, UiRect,
};
pub use surface::{PresentMode, RenderSurfaceError, SurfaceRenderer, WindowSurfaceTarget};
