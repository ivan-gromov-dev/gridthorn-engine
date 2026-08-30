mod error;

use std::time::Duration;

use gridthorn_simulation::{FixedStepClock, FixedStepConfig, FixedTime, FrameTiming};
use gridthorn_world::{ScheduleRuntime, WorldAccess};
use tracing::warn;

pub use error::LifecycleError;

/// Drives Gridthorn schedules through one explicit application lifecycle.
pub struct ApplicationRuntime {
    schedules: ScheduleRuntime,
    fixed_clock: FixedStepClock,
    shutdown: bool,
}

impl ApplicationRuntime {
    /// Create an application runtime from a completed schedule set.
    #[must_use]
    pub fn new(schedules: ScheduleRuntime) -> Self {
        Self::with_fixed_step(schedules, FixedStepConfig::default())
    }

    /// Create an application runtime with explicit fixed-step configuration.
    #[must_use]
    pub fn with_fixed_step(schedules: ScheduleRuntime, fixed_step: FixedStepConfig) -> Self {
        Self {
            schedules,
            fixed_clock: FixedStepClock::new(fixed_step),
            shutdown: false,
        }
    }

    /// Borrow world state between lifecycle operations.
    pub fn world(&mut self) -> WorldAccess<'_> {
        self.schedules.world()
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
        let timing = self.fixed_clock.advance_steps(fixed_steps)?;
        self.execute_frame(timing);
        Ok(())
    }

    /// Run one host frame using accumulated elapsed time.
    ///
    /// # Errors
    ///
    /// Returns an error after shutdown or if the fixed clock exceeds its
    /// supported elapsed-time or tick-index range.
    pub fn run_timed_frame(&mut self, elapsed: Duration) -> Result<FrameTiming, LifecycleError> {
        self.ensure_running()?;
        let timing = self.fixed_clock.advance(elapsed)?;
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

    fn execute_frame(&mut self, timing: FrameTiming) {
        self.schedules.run_startup();
        self.schedules.run_poll_events();
        self.schedules.run_input();
        self.schedules.world().insert_resource(timing);
        for offset in 0..timing.fixed_steps() {
            let tick_index = timing.first_tick_index() + u64::from(offset);
            self.schedules
                .world()
                .insert_resource(FixedTime::new(tick_index, self.fixed_clock.fixed_step()));
            self.schedules.run_fixed_update();
        }
        self.schedules.run_update();
        self.schedules.run_post_update();
        self.schedules.run_render();
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
