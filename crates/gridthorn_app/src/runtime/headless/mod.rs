use gridthorn_simulation::FixedStepConfig;
use gridthorn_world::{ScheduleRuntime, WorldAccess};

use crate::{ApplicationRuntime, ExitRequest, LifecycleError};

/// Exact work completed by one bounded headless request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HeadlessProgress {
    /// Ticks executed by this request.
    pub executed_ticks: u64,
    /// Total ticks executed since construction.
    pub completed_ticks: u64,
    /// Whether an orderly exit prevented further work.
    pub exit_requested: bool,
}

/// Provisional synchronous simulation driver with no platform or presentation work.
/// Callbacks run on the caller thread. Shutdown is explicit, never implicit on drop.
pub struct HeadlessSimulation {
    runtime: ApplicationRuntime,
    completed_ticks: u64,
}

impl HeadlessSimulation {
    /// Construct a runner with explicit fixed duration; catch-up limits are ignored.
    #[must_use]
    pub fn new(schedules: ScheduleRuntime, config: FixedStepConfig) -> Self {
        Self {
            runtime: ApplicationRuntime::with_fixed_step(schedules, config),
            completed_ticks: 0,
        }
    }

    /// Access state between tick requests, including command injection and explicit loads.
    pub fn world(&mut self) -> WorldAccess<'_> {
        self.runtime.world()
    }

    /// Run Startup once, then up to the requested number of exact ticks.
    /// Each tick runs Input and state/scene transitions before `FixedUpdate`.
    /// Exit is checked after Startup, after Input, and after each complete tick.
    /// Pause and speed do not affect explicit work. Zero ticks runs only Startup.
    /// `PollEvents`, `Update`, `PostUpdate`, `Render`, `Suspend`, and `Resume` never run.
    ///
    /// # Errors
    /// Returns a lifecycle error after shutdown or on tick-index overflow.
    /// Input and transitions may already have executed before an arithmetic error.
    pub fn run_ticks(&mut self, ticks: u64) -> Result<HeadlessProgress, LifecycleError> {
        self.runtime.startup()?;
        let mut executed_ticks = 0;
        while executed_ticks < ticks && !self.exit_requested() {
            self.runtime.prepare_simulation();
            if self.exit_requested() {
                break;
            }
            let timing = self.runtime.fixed_clock.advance_steps(1)?;
            self.runtime.schedules.world().insert_resource(timing);
            self.runtime.execute_fixed(timing);
            self.completed_ticks = timing.completed_ticks();
            executed_ticks += 1;
        }
        Ok(HeadlessProgress {
            executed_ticks,
            completed_ticks: self.completed_ticks,
            exit_requested: self.exit_requested(),
        })
    }

    fn exit_requested(&mut self) -> bool {
        self.runtime
            .world()
            .read_resource(|exit: &ExitRequest| exit.is_requested())
            .unwrap_or(false)
    }

    /// Run Shutdown once, including when no ticks were requested.
    pub fn shutdown(&mut self) {
        self.runtime.shutdown();
    }
}

#[cfg(test)]
mod test;
