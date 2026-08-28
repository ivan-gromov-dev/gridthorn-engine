/// Gridthorn-owned lifecycle schedule selected for system registration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScheduleStage {
    /// Runs once before the first frame.
    Startup,
    /// Runs once for each requested authoritative fixed step.
    FixedUpdate,
    /// Runs once for each presentation frame.
    Update,
}
