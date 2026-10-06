use std::time::{Duration, Instant};

use crate::{ApplicationRuntime, ExitRequest};
use gridthorn_input::{InputBuffer, InputEvent};
use gridthorn_render::RenderFrame;

use super::{ApplicationError, WindowApplication, WindowConfig, WindowControl, WindowLifecycle};

/// Windowed runner for an application runtime and its timed frame schedules.
pub struct WindowedApplication {
    graphics_selection: Option<gridthorn_render::GraphicsSelection>,
    graphics_adapter: Option<gridthorn_render::GraphicsAdapterKey>,
    rendering_enabled: bool,
    config: WindowConfig,
    runtime: ApplicationRuntime,
}

impl WindowedApplication {
    /// Create a windowed application from platform and runtime configuration.
    #[must_use]
    pub fn new(config: WindowConfig, runtime: ApplicationRuntime) -> Self {
        Self {
            graphics_selection: None,
            graphics_adapter: None,
            config,
            runtime,
            rendering_enabled: true,
        }
    }

    /// Run a native input window without initializing or presenting GPU resources.
    #[must_use]
    pub fn without_renderer(mut self) -> Self {
        self.rendering_enabled = false;
        self
    }

    /// Choose an adapter for this run. Restart the application to change GPU resources.
    /// Explicit preferences are revalidated and never silently replaced.
    #[must_use]
    pub fn with_graphics_adapter(mut self, adapter: gridthorn_render::GraphicsAdapterKey) -> Self {
        self.graphics_selection = None;
        self.graphics_adapter = Some(adapter);
        self
    }

    /// Select device and rendering API independently for this run.
    /// The last selection builder call wins; changing GPU resources requires a new run.
    #[must_use]
    pub fn with_graphics_selection(
        mut self,
        selection: gridthorn_render::GraphicsSelection,
    ) -> Self {
        self.graphics_adapter = None;
        self.graphics_selection = Some(selection);
        self
    }

    /// Run timed frames until the platform event loop exits.
    ///
    /// # Errors
    ///
    /// Returns contextual platform, renderer, or runtime failures.
    pub fn run(self) -> Result<(), ApplicationError> {
        let mut application =
            WindowApplication::new(self.config, RuntimeWindowLifecycle::new(self.runtime));
        if let Some(adapter) = self.graphics_adapter {
            application = application.with_graphics_adapter(adapter);
        }
        if let Some(selection) = self.graphics_selection {
            application = application.with_graphics_selection(selection);
        }
        if self.rendering_enabled {
            application.run()
        } else {
            application.without_renderer().run()
        }
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

    fn collect_display_requests(&mut self, control: &mut WindowControl) {
        if let Some(Some((id, config))) = self
            .runtime
            .world()
            .update_resource_with(super::presentation::PresentationSettings::take_request)
        {
            control.configure_presentation(id, config);
        }
        if let Some(Some((id, request))) = self
            .runtime
            .world()
            .update_resource_with(super::settings::WindowSettings::take_request)
        {
            control.configure_window(id, request);
        }
        if let Some((refresh, monitor)) =
            self.runtime
                .world()
                .update_resource_with(|displays: &mut crate::display::Displays| {
                    displays.clear_changes();
                    displays.take_requests()
                })
        {
            if refresh {
                control.refresh_displays();
            }
            if let Some(monitor) = monitor {
                control.select_monitor(monitor);
            }
        }
    }
}

impl WindowLifecycle for RuntimeWindowLifecycle {
    fn presentation_state_changed(&mut self, state: super::presentation::PresentationState) {
        if self
            .runtime
            .world()
            .read_resource(|_: &super::presentation::PresentationSettings| ())
            .is_none()
        {
            self.runtime
                .world()
                .insert_resource(super::presentation::PresentationSettings::default());
        }
        self.runtime.world().update_resource(
            |settings: &mut super::presentation::PresentationSettings| {
                settings.publish_state(state);
            },
        );
    }
    fn presentation_operation_changed(
        &mut self,
        operation: super::presentation::PresentationOperation,
    ) {
        self.runtime.world().update_resource(
            |settings: &mut super::presentation::PresentationSettings| {
                settings.publish_feedback(operation);
            },
        );
    }
    fn graphics_adapters_initialized(&mut self, adapters: gridthorn_render::GraphicsAdapters) {
        self.runtime.world().insert_resource(adapters);
    }

    fn window_state_changed(
        &mut self,
        state: super::settings::WindowState,
        capabilities: super::settings::WindowCapabilities,
    ) {
        if self
            .runtime
            .world()
            .read_resource(|_: &super::settings::WindowSettings| ())
            .is_none()
        {
            self.runtime
                .world()
                .insert_resource(super::settings::WindowSettings::default());
        }
        self.runtime
            .world()
            .update_resource(|settings: &mut super::settings::WindowSettings| {
                settings.publish_state(state, capabilities);
            });
    }
    fn window_operation_changed(&mut self, operation: super::settings::WindowOperation) {
        self.runtime
            .world()
            .update_resource(|settings: &mut super::settings::WindowSettings| {
                settings.publish_feedback(operation);
            });
    }
    fn displays_changed(&mut self, displays: crate::display::Displays) {
        self.runtime
            .world()
            .update_resource(|resource: &mut crate::display::Displays| {
                resource.publish_inventory(displays);
            });
    }

    fn monitor_selection_changed(&mut self, selection: crate::display::MonitorSelection) {
        self.runtime
            .world()
            .update_resource(|displays: &mut crate::display::Displays| {
                displays.publish_selection(selection);
            });
    }

    fn scale_factor_changed(&mut self, scale_factor: f64) {
        self.runtime
            .world()
            .insert_resource(super::WindowScaleFactor(scale_factor));
    }

    fn resized(&mut self, width: u32, height: u32) {
        self.runtime
            .world()
            .insert_resource(super::WindowViewport { width, height });
    }

    fn started(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        if self
            .runtime
            .world()
            .read_resource(|_: &super::presentation::PresentationSettings| ())
            .is_none()
        {
            self.runtime
                .world()
                .insert_resource(super::presentation::PresentationSettings::default());
        }
        if self
            .runtime
            .world()
            .read_resource(|_: &super::settings::WindowSettings| ())
            .is_none()
        {
            self.runtime
                .world()
                .insert_resource(super::settings::WindowSettings::default());
        }
        self.runtime
            .world()
            .insert_resource(crate::display::Displays::default());
        self.runtime
            .world()
            .insert_resource(gridthorn_input::TextInput::default());
        self.runtime
            .world()
            .insert_resource(gridthorn_input::Clipboard::default());
        self.runtime
            .world()
            .insert_resource(gridthorn_input::PointerCapture::default());
        self.runtime.startup()?;
        self.collect_display_requests(control);
        self.frame_timer.start(Instant::now());
        Ok(())
    }

    fn idle(&mut self, control: &mut WindowControl) -> Result<(), ApplicationError> {
        let now = Instant::now();
        let elapsed = self.frame_timer.advance(now);
        self.run_elapsed_frame(elapsed)?;
        if let Some(deadline) = now.checked_add(self.runtime.fixed_step_interval()) {
            control.wake_at(deadline);
        }
        self.collect_display_requests(control);
        if let Some(Some(area)) = self
            .runtime
            .world()
            .update_resource_with(gridthorn_input::TextInput::take_request)
        {
            control.text_input = Some(area);
        }
        if let Some(requests) = self
            .runtime
            .world()
            .update_resource_with(gridthorn_input::Clipboard::take_requests)
        {
            for request in requests {
                control.clipboard(request);
            }
        }
        if let Some(Some(mode)) = self
            .runtime
            .world()
            .update_resource_with(gridthorn_input::PointerCapture::take_request)
        {
            control.set_pointer_capture(mode);
        }
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
        self.input.push(InputEvent::FocusLost);
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
