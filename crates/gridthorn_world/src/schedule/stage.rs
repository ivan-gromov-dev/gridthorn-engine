/// Gridthorn-owned lifecycle schedule selected for system registration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleStage {
    /// Runs once before the first frame.
    Startup,
    /// Collects platform events without mutating authoritative state.
    PollEvents,
    /// Converts collected device state into commands for future fixed ticks.
    Input,
    /// Rebuilds scene-owned world state after an atomic scene switch.
    SceneTransition,
    /// Runs once for each requested authoritative fixed step.
    FixedUpdate,
    /// Runs once for each presentation frame.
    Update,
    /// Finalizes presentation state after frame-based updates.
    PostUpdate,
    /// Consumes presentation state without mutating authoritative state.
    Render,
    /// Runs once while application services are shutting down.
    Shutdown,
}
