//! Public SDK facade for Gridthorn.
//!
//! Runtime APIs remain provisional while the Milestone 1 vertical slice is
//! under construction.

mod runtime;
mod version;

pub use runtime::{
    Aabb2d, AnimationClip, AnimationClipError, AnimationPlayback, AnimationPlayer,
    ApplicationError, ApplicationRuntime, AssetId, AssetReloadError, AssetReloader, AssetStore,
    AssetStoreError, AudioClip, AudioClipError, AudioCommand, AudioCommandQueue, AudioQueueError,
    AudioSettingsError, AudioVoiceId, ButtonState, Camera2d, Circle2d, Collider2d, ColliderError,
    Color, Contact2d, CursorPosition, EntityId, ExitRequest, FixedStepConfig, FixedStepConfigError,
    FixedTime, FrameTiming, GameCommandQueue, GameStateChange, GameStateError, GameStateId,
    GameStateStack, InputBuffer, InputEvent, InputState, KeyCode, LifecycleError, MouseButton,
    PlaybackSettings, RenderFrame, SceneChange, SceneController, SceneId, SceneIdError,
    ScheduleBuilder, ScheduleRuntime, ScheduleStage, Sprite, SpriteRegion, SpriteRegionError,
    TextLabel, TextureAsset, TextureAssetError, TexturedSprite, TimeError, TimingOverlay, UiButton,
    UiButtonError, UiButtonInteraction, UiError, UiPrimitive, UiRect, Vec2, WavDecodeError,
    WindowConfig, WindowedApplication, WorldAccess, contact, overlaps,
};
pub use version::version;

/// Commonly used Gridthorn APIs.
///
pub mod prelude {
    pub use crate::{
        Aabb2d, AnimationClip, AnimationPlayback, AnimationPlayer, ApplicationRuntime, AudioClip,
        AudioCommand, AudioCommandQueue, AudioVoiceId, Camera2d, Circle2d, Collider2d, Color,
        ExitRequest, FixedStepConfig, FixedTime, FrameTiming, GameCommandQueue, GameStateId,
        GameStateStack, InputState, KeyCode, MouseButton, PlaybackSettings, RenderFrame,
        SceneController, SceneId, ScheduleBuilder, ScheduleStage, Sprite, SpriteRegion, TextLabel,
        TextureAsset, TexturedSprite, TimingOverlay, UiButton, UiPrimitive, UiRect, Vec2,
        WindowConfig, WindowedApplication,
    };
}
