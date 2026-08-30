//! Provisional application lifecycle services for Gridthorn.

mod runtime;
mod scene;
mod state;
mod window;

pub use runtime::{ApplicationRuntime, ExitRequest, LifecycleError};
pub use scene::{SceneChange, SceneController};
pub use state::{GameStateChange, GameStateError, GameStateId, GameStateStack};
pub use window::{
    ApplicationError, WindowApplication, WindowConfig, WindowControl, WindowLifecycle,
    WindowedApplication,
};
