use gridthorn_simulation::{FixedStepConfig, SimulationControl};
use gridthorn_world::{ScheduleRuntime, WorldAccess};

use super::{Scenario, ScenarioError, ScenarioState, SimulationSnapshot};
use crate::{ExitRequest, HeadlessProgress, HeadlessSimulation};

/// Provisional headless scenario orchestration with explicit in-memory reset boundaries.
/// Schedules must mutate only `ScenarioState` for authoritative game data.
pub struct ScenarioRuntime<S, C> {
    simulation: HeadlessSimulation,
    scenario: Scenario<S, C>,
    config: FixedStepConfig,
}

impl<S, C> ScenarioRuntime<S, C>
where
    S: Clone + Send + Sync + 'static,
    C: Clone + Send + Sync + 'static,
{
    /// Install an independent initial root before Startup, then run Startup once.
    /// Startup may initialize the root but must not keep authoritative closure-local state.
    ///
    /// # Errors
    /// Returns lifecycle errors or `MissingState` if Startup removes the root.
    pub fn new(
        schedules: ScheduleRuntime,
        config: FixedStepConfig,
        scenario: Scenario<S, C>,
    ) -> Result<Self, ScenarioError> {
        let mut simulation = HeadlessSimulation::new(schedules, config);
        simulation.world().insert_resource(scenario.initial.clone());
        simulation.run_ticks(0)?;
        if simulation
            .world()
            .read_resource(|_: &ScenarioState<S, C>| ())
            .is_none()
        {
            return Err(ScenarioError::MissingState);
        }
        Ok(Self {
            simulation,
            scenario,
            config,
        })
    }

    /// Access the root for command injection and explicit inspection between ticks.
    pub fn world(&mut self) -> WorldAccess<'_> {
        self.simulation.world()
    }

    /// Execute exact headless work with the existing lifecycle and exit semantics.
    ///
    /// # Errors
    /// Returns lifecycle errors or `MissingState` before work if the root was removed.
    pub fn run_ticks(&mut self, ticks: u64) -> Result<HeadlessProgress, ScenarioError> {
        if self
            .world()
            .read_resource(|_: &ScenarioState<S, C>| ())
            .is_none()
        {
            return Err(ScenarioError::MissingState);
        }
        let progress = self.simulation.run_ticks(ticks)?;
        Ok(progress)
    }

    /// Clone the authoritative root, controls, exit state, and next tick index between requests.
    ///
    /// # Errors
    /// Returns `MissingState` if the required root was removed.
    pub fn snapshot(&mut self) -> Result<SimulationSnapshot<S, C>, ScenarioError> {
        let state = self
            .world()
            .read_resource(|state: &ScenarioState<S, C>| state.clone())
            .ok_or(ScenarioError::MissingState)?;
        let control = self
            .world()
            .read_resource(|control: &SimulationControl| *control)
            .unwrap_or_default();
        let exit = self
            .world()
            .read_resource(|exit: &ExitRequest| *exit)
            .unwrap_or_default();
        Ok(SimulationSnapshot {
            scenario: self.scenario.name.clone(),
            revision: self.scenario.revision,
            engine: env!("CARGO_PKG_VERSION"),
            config: self.config,
            completed_ticks: self.simulation.completed_ticks(),
            state,
            control,
            exit,
        })
    }

    /// Validate compatibility, clone all data, then restore at an explicit reset boundary.
    /// No schedules run; Startup is not repeated. Untracked world data remains untouched.
    ///
    /// # Errors
    /// Rejects mismatched identity, revision, exact engine version, configuration, or shutdown.
    /// Recoverable failures leave the live state and clock unchanged.
    pub fn restore(&mut self, snapshot: &SimulationSnapshot<S, C>) -> Result<(), ScenarioError> {
        self.validate_snapshot(snapshot)?;
        self.restore_owned(snapshot.clone())
    }

    pub(super) fn save_identity(&mut self) -> Result<(&str, u32, FixedStepConfig), ScenarioError> {
        self.world()
            .read_resource(|_: &ScenarioState<S, C>| ())
            .ok_or(ScenarioError::MissingState)?;
        Ok((&self.scenario.name, self.scenario.revision, self.config))
    }

    fn validate_snapshot(&self, snapshot: &SimulationSnapshot<S, C>) -> Result<(), ScenarioError> {
        if snapshot.scenario != self.scenario.name || snapshot.revision != self.scenario.revision {
            return Err(ScenarioError::Incompatible("scenario identity or revision"));
        }
        if snapshot.engine != env!("CARGO_PKG_VERSION") {
            return Err(ScenarioError::Incompatible("engine release"));
        }
        if snapshot.config != self.config {
            return Err(ScenarioError::Incompatible("fixed-step configuration"));
        }
        Ok(())
    }

    pub(super) fn restore_owned(
        &mut self,
        snapshot: SimulationSnapshot<S, C>,
    ) -> Result<(), ScenarioError> {
        self.validate_snapshot(&snapshot)?;
        self.simulation
            .reset_tick(self.config, snapshot.completed_ticks)?;
        self.world().insert_resource(snapshot.state);
        self.world().insert_resource(snapshot.control);
        self.world().insert_resource(snapshot.exit);
        Ok(())
    }

    /// Run Shutdown once and reject further ticks or restores.
    pub fn shutdown(&mut self) {
        self.simulation.shutdown();
    }
}
