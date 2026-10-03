//! Provisional renderer services for Gridthorn.

mod presentation;
mod surface;
mod text;

pub use text::{
    RasterText, TextAlignment, TextError, TextGlyph, TextLayout, TextLine, TextMeasurement,
    TextStyle, TextSystem, TextWrap,
};

pub use presentation::{
    AnimationClip, AnimationClipError, AnimationPlayback, AnimationPlayer, Camera2d, Color,
    RenderFrame, Sprite, SpriteRegion, SpriteRegionError, TextLabel, TexturedSprite, TimingOverlay,
    UiError, UiPrimitive, UiRect,
};
pub use surface::{RenderSurfaceError, SurfaceRenderer, WindowSurfaceTarget};
