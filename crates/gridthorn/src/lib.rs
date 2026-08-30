//! Public SDK facade for Gridthorn.
//!
//! Runtime APIs remain provisional while the Milestone 1 vertical slice is
//! under construction.

mod runtime;
mod version;

pub use runtime::{
    ApplicationError, ApplicationRuntime, ButtonState, Camera2d, Color, CursorPosition, EntityId,
    ExitRequest, FixedStepConfig, FixedStepConfigError, FixedTime, FrameTiming, GameCommandQueue,
    GameStateChange, GameStateError, GameStateId, GameStateStack, InputBuffer, InputEvent,
    InputState, KeyCode, LifecycleError, MouseButton, RenderFrame, SceneChange, SceneController,
    SceneId, SceneIdError, ScheduleBuilder, ScheduleRuntime, ScheduleStage, Sprite, TextureAsset,
    TextureAssetError, TexturedSprite, TimeError, TimingOverlay, WindowConfig, WindowedApplication,
    WorldAccess,
};
pub use version::version;

/// Commonly used Gridthorn APIs.
///
pub mod prelude {
    pub use crate::{
        ApplicationRuntime, Camera2d, Color, ExitRequest, FixedStepConfig, FixedTime, FrameTiming,
        GameCommandQueue, GameStateId, GameStateStack, InputState, KeyCode, MouseButton,
        RenderFrame, SceneController, SceneId, ScheduleBuilder, ScheduleStage, Sprite,
        TextureAsset, TexturedSprite, TimingOverlay, WindowConfig, WindowedApplication,
    };
}
