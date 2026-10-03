//! Provisional application lifecycle services for Gridthorn.

mod runtime;
mod scenario;
mod scene;
mod state;
mod ui;
mod window;

pub use runtime::{
    ApplicationRuntime, ExitRequest, HeadlessProgress, HeadlessSimulation, LifecycleError,
};
pub use scenario::{
    Scenario, ScenarioError, ScenarioRuntime, ScenarioState, SimulationSnapshot, WorldSaveCodec,
    WorldSaveError,
};
pub use scene::{SceneChange, SceneController};
pub use state::{GameStateChange, GameStateError, GameStateId, GameStateStack};
pub use ui::composition;
pub use ui::{UiButton, UiButtonError, UiButtonInteraction};
pub use window::{
    ApplicationError, WindowApplication, WindowConfig, WindowControl, WindowLifecycle,
    WindowScaleFactor, WindowViewport, WindowedApplication,
};
