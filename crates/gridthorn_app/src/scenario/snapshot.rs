use gridthorn_simulation::{FixedStepConfig, SimulationControl};

use super::ScenarioState;
use crate::ExitRequest;

/// Opaque, cloneable in-memory snapshot for the same typed scenario and SDK release.
/// This is neither a wire format nor a capture of arbitrary ECS entities/resources.
#[derive(Clone, Debug)]
pub struct SimulationSnapshot<S, C> {
    pub(super) scenario: String,
    pub(super) revision: u32,
    pub(super) engine: &'static str,
    pub(super) config: FixedStepConfig,
    pub(super) completed_ticks: u64,
    pub(super) state: ScenarioState<S, C>,
    pub(super) control: SimulationControl,
    pub(super) exit: ExitRequest,
}

impl<S, C> SimulationSnapshot<S, C> {
    /// Next fixed tick index when restored.
    #[must_use]
    pub fn completed_ticks(&self) -> u64 {
        self.completed_ticks
    }

    /// Scenario identity and project-owned data revision.
    #[must_use]
    pub fn scenario(&self) -> (&str, u32) {
        (&self.scenario, self.revision)
    }

    /// Exact SDK release required for restoration.
    #[must_use]
    pub fn engine_version(&self) -> &str {
        self.engine
    }

    /// Inspect captured game state, pending commands, and RNG without changing the snapshot.
    #[must_use]
    pub fn state(&self) -> &ScenarioState<S, C> {
        &self.state
    }
}
