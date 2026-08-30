use std::sync::Arc;

use gridthorn_render::{SurfaceRenderer, WindowSurfaceTarget};
use tracing::{error, info};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

use super::config::WindowConfig;
use super::control::WindowControl;
use super::error::ApplicationError;
use super::lifecycle::WindowLifecycle;

/// Application runner that owns the platform window lifecycle.
pub struct WindowApplication<L> {
    config: WindowConfig,
    lifecycle: L,
}

impl<L> WindowApplication<L>
where
    L: WindowLifecycle,
{
    /// Create an application with engine-owned configuration and lifecycle hooks.
    pub fn new(config: WindowConfig, lifecycle: L) -> Self {
        Self { config, lifecycle }
    }

    /// Run the platform event loop until shutdown.
    ///
    /// # Errors
    ///
    /// Returns an error when window creation, renderer initialization, event
    /// processing, or frame presentation fails.
    pub fn run(self) -> Result<(), ApplicationError> {
        let event_loop = EventLoop::new().map_err(ApplicationError::event_loop)?;
        event_loop.set_control_flow(ControlFlow::Wait);

        let mut state = WinitApplication::new(self.config, self.lifecycle);
        let event_result = event_loop
            .run_app(&mut state)
            .map_err(ApplicationError::event_loop);
        state.finish(event_result)
    }
}

struct WinitApplication<L> {
    config: WindowConfig,
    error: Option<ApplicationError>,
    lifecycle: L,
    renderer: Option<SurfaceRenderer>,
    window: Option<Arc<Window>>,
}

impl<L> WinitApplication<L>
where
    L: WindowLifecycle,
{
    fn new(config: WindowConfig, lifecycle: L) -> Self {
        Self {
            config,
            error: None,
            lifecycle,
            renderer: None,
            window: None,
        }
    }

    fn finish(
        mut self,
        event_result: Result<(), ApplicationError>,
    ) -> Result<(), ApplicationError> {
        self.lifecycle.shutdown();
        event_result?;
        self.error.map_or(Ok(()), Err)
    }

    fn initialize(&mut self, event_loop: &ActiveEventLoop) -> Result<(), ApplicationError> {
        let attributes = Window::default_attributes()
            .with_title(self.config.title.clone())
            .with_inner_size(PhysicalSize::new(self.config.width, self.config.height));
        let window = Arc::new(
            event_loop
                .create_window(attributes)
                .map_err(ApplicationError::window_creation)?,
        );
        let size = window.inner_size();
        let target = WindowSurfaceTarget::new(window.clone());
        let renderer = SurfaceRenderer::new(target, size.width, size.height)?;
        self.window = Some(window);
        self.renderer = Some(renderer);

        let mut control = WindowControl::default();
        self.lifecycle.started(&mut control)?;
        self.apply_control(event_loop, &control);
        info!(
            component = "app",
            event = "initialized",
            "window and GPU surface initialized"
        );
        Ok(())
    }

    fn apply_control(&self, event_loop: &ActiveEventLoop, control: &WindowControl) {
        let Some(window) = self.window.as_ref() else {
            return;
        };

        if let Some((width, height)) = control.requested_size {
            let _ignored = window.request_inner_size(PhysicalSize::new(width, height));
        }
        if let Some(minimized) = control.minimized {
            window.set_minimized(minimized);
        }
        if let Some(deadline) = control.wake_at {
            event_loop.set_control_flow(ControlFlow::WaitUntil(deadline));
        }
        if control.exit_requested {
            event_loop.exit();
        }
    }

    fn fail(&mut self, event_loop: &ActiveEventLoop, error: ApplicationError) {
        error!(
            component = "app",
            event = "failure",
            %error,
            "window application failed"
        );
        self.error = Some(error);
        event_loop.exit();
    }

    fn matches_window(&self, window_id: WindowId) -> bool {
        self.window
            .as_ref()
            .is_some_and(|window| window.id() == window_id)
    }
}

impl<L> ApplicationHandler for WinitApplication<L>
where
    L: WindowLifecycle,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            if let Err(error) = self.initialize(event_loop) {
                self.fail(event_loop, error);
            }
        } else if let Some(renderer) = self.renderer.as_mut() {
            renderer.set_occluded(false);
            self.lifecycle.resumed();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        self.lifecycle.suspended();
        if let Some(renderer) = self.renderer.as_mut() {
            renderer.set_occluded(true);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if !self.matches_window(window_id) {
            return;
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(renderer) = self.renderer.as_mut()
                    && let Err(error) = renderer.resize(size.width, size.height)
                {
                    self.fail(event_loop, error.into());
                }
            }
            WindowEvent::Occluded(occluded) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.set_occluded(occluded);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = self.renderer.as_mut()
                    && let Err(error) = renderer.render()
                {
                    self.fail(event_loop, error.into());
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.error.is_some() {
            return;
        }
        let mut control = WindowControl::default();
        if let Err(error) = self.lifecycle.idle(&mut control) {
            self.fail(event_loop, error);
            return;
        }
        self.apply_control(event_loop, &control);
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }
}

#[cfg(test)]
mod test;
