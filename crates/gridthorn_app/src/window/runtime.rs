use std::time::{Duration, Instant};

use crate::{ApplicationRuntime, ExitRequest};
use gridthorn_input::{InputBuffer, InputEvent};
use gridthorn_render::RenderFrame;

use super::{ApplicationError, WindowApplication, WindowConfig, WindowControl, WindowLifecycle};

/// Windowed runner for an application runtime and its timed frame schedules.
pub struct WindowedApplication {
    config: WindowConfig,
    runtime: ApplicationRuntime,
}

impl WindowedApplication {
    /// Create a windowed application from platform and runtime configuration.
    #[must_use]
    pub fn new(config: WindowConfig, runtime: ApplicationRuntime) -> Self {
        Self { config, runtime }
    }

    /// Run timed frames until the platform event loop exits.
    ///
    /// # Errors
    ///
    /// Returns contextual platform, renderer, or runtime failures.
    pub fn run(self) -> Result<(), ApplicationError> {
        WindowApplication::new(self.config, RuntimeWindowLifecycle::new(self.runtime)).run()
    }
}

struct RuntimeWindowLifecycle {
    runtime: ApplicationRuntime,
    frame_timer: FrameTimer,
    input: InputBuffer,
}

impl RuntimeWindowLifecycle {
    fn new(runtime: ApplicationRuntime) -> Self {
        Self {
            runtime,
            frame_timer: FrameTimer::default(),
            input: InputBuffer::new(),
        }
    }

    fn run_elapsed_frame(&mut self, elapsed: Duration) -> Result<(), ApplicationError> {
        self.runtime.world().insert_resource(self.input.snapshot());
        self.runtime.run_timed_frame(elapsed)?;
        Ok(())
    }
}

impl WindowLifecycle for RuntimeWindowLifecycle {
    fn started(&mut self, _control: &mut WindowControl) -> Result<(), ApplicationError> {
        self.runtime.startup()?;
        self.frame_timer.start(Instant::now());
        Ok(())
    }

    fn idle(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        let elapsed = self.frame_timer.advance(Instant::now());
        self.run_elapsed_frame(elapsed)?;
        if self
            .runtime
            .world()
            .read_resource(|exit: &ExitRequest| exit.is_requested())
            .unwrap_or(false)
        {
            control.exit();
        }
        Ok(())
    }

    fn input(&mut self, event: InputEvent) -> Result<(), ApplicationError> {
        self.input.push(event);
        Ok(())
    }

    fn render_frame(&mut self) -> RenderFrame {
        self.runtime
            .world()
            .read_resource(Clone::clone)
            .unwrap_or_default()
    }

    fn suspended(&mut self) {
        self.frame_timer.suspend();
    }

    fn resumed(&mut self) {
        self.frame_timer.start(Instant::now());
    }

    fn shutdown(&mut self) {
        self.runtime.shutdown();
    }
}

#[derive(Default)]
struct FrameTimer {
    previous: Option<Instant>,
}

impl FrameTimer {
    fn start(&mut self, now: Instant) {
        self.previous = Some(now);
    }

    fn advance(&mut self, now: Instant) -> Duration {
        let elapsed = self.previous.map_or(Duration::ZERO, |previous| {
            now.saturating_duration_since(previous)
        });
        self.previous = Some(now);
        elapsed
    }

    fn suspend(&mut self) {
        self.previous = None;
    }
}

#[cfg(test)]
mod test;
