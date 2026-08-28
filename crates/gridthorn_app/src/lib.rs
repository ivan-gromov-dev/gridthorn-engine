//! Provisional application lifecycle services for Gridthorn.

mod window;

pub use window::{
    ApplicationError, WindowApplication, WindowConfig, WindowControl, WindowLifecycle,
};
