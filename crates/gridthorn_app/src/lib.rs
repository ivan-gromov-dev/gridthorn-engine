//! Provisional application lifecycle services for Gridthorn.

mod runtime;
mod window;

pub use runtime::{ApplicationRuntime, LifecycleError};
pub use window::{
    ApplicationError, WindowApplication, WindowConfig, WindowControl, WindowLifecycle,
    WindowedApplication,
};
