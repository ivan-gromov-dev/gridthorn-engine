//! Public SDK facade for Gridthorn.
//!
//! Runtime APIs remain provisional while the Milestone 1 vertical slice is
//! under construction.

mod runtime;
mod version;

pub use runtime::{
    ApplicationError, ApplicationRuntime, ButtonState, CursorPosition, EntityId, FixedStepConfig,
    FixedStepConfigError, FixedTime, FrameTiming, InputEvent, InputState, KeyCode, LifecycleError,
    MouseButton, ScheduleBuilder, ScheduleRuntime, ScheduleStage, TimeError, WindowConfig,
    WindowedApplication, WorldAccess,
};
pub use version::version;

/// Commonly used Gridthorn APIs.
///
pub mod prelude {
    pub use crate::{
        ApplicationRuntime, FixedStepConfig, FixedTime, FrameTiming, InputState, KeyCode,
        MouseButton, ScheduleBuilder, ScheduleStage, WindowConfig, WindowedApplication,
    };
}
