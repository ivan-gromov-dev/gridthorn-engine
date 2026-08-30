//! Public SDK facade for Gridthorn.
//!
//! Runtime APIs remain provisional while the Milestone 1 vertical slice is
//! under construction.

mod runtime;
mod version;

pub use runtime::{
    AnimationClip, AnimationClipError, AnimationPlayback, AnimationPlayer, ApplicationError,
    ApplicationRuntime, AudioClip, AudioClipError, AudioCommand, AudioCommandQueue,
    AudioQueueError, AudioSettingsError, AudioVoiceId, ButtonState, Camera2d, Color,
    CursorPosition, EntityId, ExitRequest, FixedStepConfig, FixedStepConfigError, FixedTime,
    FrameTiming, GameCommandQueue, GameStateChange, GameStateError, GameStateId, GameStateStack,
    InputBuffer, InputEvent, InputState, KeyCode, LifecycleError, MouseButton, PlaybackSettings,
    RenderFrame, SceneChange, SceneController, SceneId, SceneIdError, ScheduleBuilder,
    ScheduleRuntime, ScheduleStage, Sprite, SpriteRegion, SpriteRegionError, TextLabel,
    TextureAsset, TextureAssetError, TexturedSprite, TimeError, TimingOverlay, UiButton,
    UiButtonError, UiButtonInteraction, UiError, UiPrimitive, UiRect, WavDecodeError, WindowConfig,
    WindowedApplication, WorldAccess,
};
pub use version::version;

/// Commonly used Gridthorn APIs.
///
pub mod prelude {
    pub use crate::{
        AnimationClip, AnimationPlayback, AnimationPlayer, ApplicationRuntime, AudioClip,
        AudioCommand, AudioCommandQueue, AudioVoiceId, Camera2d, Color, ExitRequest,
        FixedStepConfig, FixedTime, FrameTiming, GameCommandQueue, GameStateId, GameStateStack,
        InputState, KeyCode, MouseButton, PlaybackSettings, RenderFrame, SceneController, SceneId,
        ScheduleBuilder, ScheduleStage, Sprite, SpriteRegion, TextLabel, TextureAsset,
        TexturedSprite, TimingOverlay, UiButton, UiPrimitive, UiRect, WindowConfig,
        WindowedApplication,
    };
}
