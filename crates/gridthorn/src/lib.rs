//! Public SDK facade for Gridthorn.
//!
//! Runtime APIs remain provisional while the Milestone 1 vertical slice is
//! under construction.

mod runtime;
mod version;

pub use runtime::{
    ApplicationError, ApplicationRuntime, ButtonState, CursorPosition, EntityId, ExitRequest,
    FixedStepConfig, FixedStepConfigError, FixedTime, FrameTiming, GameCommandQueue, InputBuffer,
    InputEvent, InputState, KeyCode, LifecycleError, MouseButton, ScheduleBuilder, ScheduleRuntime,
    ScheduleStage, TimeError, WindowConfig, WindowedApplication, WorldAccess,
};
pub use version::version;

/// Commonly used Gridthorn APIs.
///
pub mod prelude {
    pub use crate::{
        ApplicationRuntime, ExitRequest, FixedStepConfig, FixedTime, FrameTiming, GameCommandQueue,
        InputState, KeyCode, MouseButton, ScheduleBuilder, ScheduleStage, WindowConfig,
        WindowedApplication,
    };
}
