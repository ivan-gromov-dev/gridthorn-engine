//! Public SDK facade for Gridthorn.
//!
//! Runtime APIs remain provisional while the Milestone 1 vertical slice is
//! under construction.

mod runtime;
mod version;

pub use runtime::{
    ApplicationRuntime, EntityId, FixedStepConfig, FixedStepConfigError, FixedTime, FrameTiming,
    LifecycleError, ScheduleBuilder, ScheduleRuntime, ScheduleStage, TimeError, WorldAccess,
};
pub use version::version;

/// Commonly used Gridthorn APIs.
///
pub mod prelude {
    pub use crate::{
        ApplicationRuntime, FixedStepConfig, FixedTime, FrameTiming, ScheduleBuilder, ScheduleStage,
    };
}
