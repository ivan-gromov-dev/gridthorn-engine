mod errors;
mod exit;
mod headless;
mod performance;

use std::time::Duration;

use gridthorn_simulation::{
    FixedStepClock, FixedStepConfig, FixedTime, FrameTiming, SimulationControl,
};
use gridthorn_world::{ScheduleRuntime, WorldAccess};
use tracing::warn;

use crate::{GameStateStack, SceneController};

pub use errors::LifecycleError;
pub use exit::ExitRequest;
pub use headless::{HeadlessProgress, HeadlessSimulation};

/// Drives Gridthorn schedules through one explicit application lifecycle.
pub struct ApplicationRuntime {
    schedules: ScheduleRuntime,
    fixed_clock: FixedStepClock,
    shutdown: bool,
    performance: performance::RuntimePerformance,
}

impl ApplicationRuntime {
    /// Create an application runtime from a completed schedule set.
    #[must_use]
    pub fn new(schedules: ScheduleRuntime) -> Self {
        Self::with_fixed_step(schedules, FixedStepConfig::default())
    }

    /// Create an application runtime with explicit fixed-step configuration.
    #[must_use]
    pub fn with_fixed_step(mut schedules: ScheduleRuntime, fixed_step: FixedStepConfig) -> Self {
        schedules.world().insert_resource(ExitRequest::default());
        if schedules
            .world()
            .read_resource(|control: &SimulationControl| *control)
            .is_none()
        {
            schedules
                .world()
                .insert_resource(SimulationControl::default());
        }
        Self {
            schedules,
            fixed_clock: FixedStepClock::new(fixed_step),
            shutdown: false,
            performance: performance::RuntimePerformance::new(),
        }
    }

    /// Borrow world state between lifecycle operations.
    pub fn world(&mut self) -> WorldAccess<'_> {
        self.schedules.world()
    }

    pub(crate) fn fixed_step_interval(&self) -> Duration {
        self.fixed_clock.fixed_step()
    }

    /// Run `Startup` once without advancing a host frame.
    ///
    /// # Errors
    ///
    /// Returns [`LifecycleError::AlreadyShutdown`] after shutdown begins.
    pub fn startup(&mut self) -> Result<(), LifecycleError> {
        self.ensure_running()?;
        self.schedules.run_startup();
        Ok(())
    }

    /// Run one host frame with an explicit number of fixed simulation steps.
    /// Explicit work ignores simulation pause, speed, and catch-up limits.
    ///
    /// `Startup` runs once before the first frame. Every frame then executes
    /// `PollEvents`, `Input`, zero or more `FixedUpdate` steps, `Update`,
    /// `PostUpdate`, and `Render` in that order.
    ///
    /// # Errors
    ///
    /// Returns [`LifecycleError::AlreadyShutdown`] after shutdown begins.
    pub fn run_frame(&mut self, fixed_steps: u32) -> Result<(), LifecycleError> {
        self.ensure_running()?;
        self.prepare_frame();
        let timing = self.fixed_clock.advance_steps(fixed_steps)?;
        self.execute_frame(timing);
        Ok(())
    }

    /// Run one host frame using scaled elapsed time.
    /// Controls are sampled after Input; paused frames still run presentation.
    /// Startup and input may have run when time arithmetic returns an error.
    ///
    /// # Errors
    ///
    /// Returns an error after shutdown or if the fixed clock exceeds its
    /// supported elapsed-time or tick-index range.
    pub fn run_timed_frame(&mut self, elapsed: Duration) -> Result<FrameTiming, LifecycleError> {
        self.ensure_running()?;
        self.prepare_frame();
        let control = self
            .schedules
            .world()
            .read_resource(|control: &SimulationControl| *control)
            .unwrap_or_default();
        let timing = self.fixed_clock.advance_controlled(elapsed, control)?;
        if timing.overloaded() {
            warn!(
                component = "app",
                event = "fixed_step_overload",
                fixed_steps = timing.fixed_steps(),
                accumulated_lag = ?timing.accumulated_lag(),
                "fixed-step catch-up limit reached; backlog preserved"
            );
        }
        self.execute_frame(timing);
        Ok(timing)
    }

    fn ensure_running(&self) -> Result<(), LifecycleError> {
        if self.shutdown {
            return Err(LifecycleError::AlreadyShutdown);
        }
        Ok(())
    }

    fn prepare_frame(&mut self) {
        self.schedules.run_startup();
        let start = self.performance.start(0);
        self.schedules.run_poll_events();
        self.performance.record(0, start);
        let start = self.performance.start(1);
        self.prepare_simulation();
        self.performance.record(1, start);
    }

    fn prepare_simulation(&mut self) {
        self.schedules.run_input();
        self.schedules
            .world()
            .update_resource(GameStateStack::apply_pending);
        let scene_change = self
            .schedules
            .world()
            .update_resource_with(SceneController::apply_pending)
            .flatten();
        if let Some(change) = scene_change {
            if let Some(exited) = change.exited() {
                self.schedules.world().despawn_scene(exited);
            }
            self.schedules.run_scene_transition();
        }
    }

    fn execute_frame(&mut self, timing: FrameTiming) {
        self.schedules.world().insert_resource(timing);
        let start = self.performance.start(2);
        self.execute_fixed(timing);
        self.performance.record(2, start);
        let start = self.performance.start(3);
        self.schedules.run_update();
        self.performance.record(3, start);
        let start = self.performance.start(4);
        self.schedules.run_post_update();
        self.performance.record(4, start);
        let start = self.performance.start(5);
        self.schedules.run_render();
        self.performance.record(5, start);
    }

    fn execute_fixed(&mut self, timing: FrameTiming) {
        for offset in 0..timing.fixed_steps() {
            let tick_index = timing.first_tick_index() + u64::from(offset);
            self.schedules
                .world()
                .insert_resource(FixedTime::new(tick_index, self.fixed_clock.fixed_step()));
            self.schedules.run_fixed_update();
        }
    }

    /// Run `Shutdown` once and stop accepting frames.
    pub fn shutdown(&mut self) {
        if self.shutdown {
            return;
        }
        self.schedules.run_shutdown();
        self.shutdown = true;
    }
}

#[cfg(test)]
mod test;
