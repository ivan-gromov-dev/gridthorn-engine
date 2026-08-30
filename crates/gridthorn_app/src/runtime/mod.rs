mod error;

use gridthorn_world::{ScheduleRuntime, WorldAccess};

pub use error::LifecycleError;

/// Drives Gridthorn schedules through one explicit application lifecycle.
pub struct ApplicationRuntime {
    schedules: ScheduleRuntime,
    shutdown: bool,
}

impl ApplicationRuntime {
    /// Create an application runtime from a completed schedule set.
    #[must_use]
    pub fn new(schedules: ScheduleRuntime) -> Self {
        Self {
            schedules,
            shutdown: false,
        }
    }

    /// Borrow world state between lifecycle operations.
    pub fn world(&mut self) -> WorldAccess<'_> {
        self.schedules.world()
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
        if self.shutdown {
            return Err(LifecycleError::AlreadyShutdown);
        }

        self.schedules.run_startup();
        self.schedules.run_poll_events();
        self.schedules.run_input();
        for _ in 0..fixed_steps {
            self.schedules.run_fixed_update();
        }
        self.schedules.run_update();
        self.schedules.run_post_update();
        self.schedules.run_render();
        Ok(())
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
