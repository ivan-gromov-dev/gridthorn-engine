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
use super::errors::ApplicationError;
use super::input::map_window_input;
use super::lifecycle::WindowLifecycle;

/// Application runner that owns the platform window lifecycle.
pub struct WindowApplication<L> {
    graphics_selection: Option<gridthorn_render::GraphicsSelection>,
    graphics_adapter: Option<gridthorn_render::GraphicsAdapterKey>,
    rendering_enabled: bool,
    config: WindowConfig,
    lifecycle: L,
}

impl<L> WindowApplication<L>
where
    L: WindowLifecycle,
{
    /// Create an application with engine-owned configuration and lifecycle hooks.
    pub fn new(config: WindowConfig, lifecycle: L) -> Self {
        Self {
            graphics_selection: None,
            graphics_adapter: None,
            config,
            lifecycle,
            rendering_enabled: true,
        }
    }

    /// Disable GPU initialization and presentation for an input-only native window.
    #[must_use]
    pub fn without_renderer(mut self) -> Self {
        self.rendering_enabled = false;
        self
    }

    /// Select a graphics adapter at initialization; a different selection requires a new run.
    /// Missing, ambiguous and incompatible preferences fail rather than selecting a fallback.
    #[must_use]
    pub fn with_graphics_adapter(mut self, adapter: gridthorn_render::GraphicsAdapterKey) -> Self {
        self.graphics_selection = None;
        self.graphics_adapter = Some(adapter);
        self
    }

    /// Select the device and rendering API independently at initialization.
    /// A new run is required to apply a different selection; the last builder call wins.
    #[must_use]
    pub fn with_graphics_selection(
        mut self,
        selection: gridthorn_render::GraphicsSelection,
    ) -> Self {
        self.graphics_adapter = None;
        self.graphics_selection = Some(selection);
        self
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
        state.rendering_enabled = self.rendering_enabled;
        state.graphics_adapter = self.graphics_adapter;
        state.graphics_selection = self.graphics_selection;
        let event_result = event_loop
            .run_app(&mut state)
            .map_err(ApplicationError::event_loop);
        state.finish(event_result)
    }
}

struct WinitApplication<L> {
    controllers: super::controller::NativeControllers,
    suspended: bool,
    presentation: super::presentation::native::NativePresentation,
    graphics_selection: Option<gridthorn_render::GraphicsSelection>,
    graphics_adapter: Option<gridthorn_render::GraphicsAdapterKey>,
    window_settings: super::settings::native::NativeWindowSettings,
    displays: crate::display::native::NativeDisplays,
    performance: super::performance::WindowPerformance,
    text: super::text::NativeTextInput,
    clipboard: super::clipboard::NativeClipboard,
    rendering_enabled: bool,
    capture: super::capture::NativeCapture,
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
            controllers: super::controller::NativeControllers::default(),
            suspended: false,
            presentation: super::presentation::native::NativePresentation::default(),
            graphics_selection: None,
            graphics_adapter: None,
            config,
            error: None,
            window_settings: super::settings::native::NativeWindowSettings::default(),
            displays: crate::display::native::NativeDisplays::default(),
            performance: super::performance::WindowPerformance::new(),
            text: super::text::NativeTextInput::default(),
            clipboard: super::clipboard::NativeClipboard::default(),
            rendering_enabled: true,
            capture: super::capture::NativeCapture::default(),
            lifecycle,
            renderer: None,
            window: None,
        }
    }

    fn finish(
        mut self,
        event_result: Result<(), ApplicationError>,
    ) -> Result<(), ApplicationError> {
        self.controllers.stop();
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
        self.performance.report_configuration(&window);
        if self.rendering_enabled {
            let target = WindowSurfaceTarget::new(window.clone());
            let renderer = if let Some(selection) = &self.graphics_selection {
                SurfaceRenderer::with_graphics_selection(
                    target,
                    size.width,
                    size.height,
                    selection,
                )?
            } else {
                SurfaceRenderer::with_adapter(
                    target,
                    size.width,
                    size.height,
                    self.graphics_adapter.as_ref(),
                )?
            };
            self.lifecycle
                .graphics_adapters_initialized(renderer.graphics_adapters().clone());
            self.renderer = Some(renderer);
        }
        self.lifecycle.scale_factor_changed(window.scale_factor());
        self.publish_presentation();
        self.window_settings.initialize(event_loop, &window);
        self.lifecycle.window_state_changed(
            self.window_settings.state(&window, &self.displays),
            self.window_settings.capabilities(),
        );
        self.window = Some(window);
        self.lifecycle.resized(size.width, size.height);

        let mut control = WindowControl::default();

        self.lifecycle.started(&mut control)?;
        self.apply_control(event_loop, &control);
        info!(
            component = "app",
            event = "initialized",
            "window initialized"
        );
        Ok(())
    }

    fn apply_window_settings(&mut self, event_loop: &ActiveEventLoop, control: &WindowControl) {
        let Some(window) = self.window.as_ref() else {
            return;
        };
        if control.refresh_displays
            || control.selected_monitor.is_some()
            || control
                .window_request
                .is_some_and(|(_, request)| request.monitor().is_some())
        {
            let inventory = self.displays.refresh(event_loop, window);
            self.lifecycle.displays_changed(inventory);
            self.lifecycle.window_state_changed(
                self.window_settings.state(window, &self.displays),
                self.window_settings.capabilities(),
            );
        }
        if let Some((id, request)) = control.window_request {
            if let Some(operation) = self.window_settings.cancel(window, &self.displays) {
                self.lifecycle.window_operation_changed(operation);
            }
            if let Some(selection) = self.displays.cancel() {
                self.lifecycle.monitor_selection_changed(selection);
            }
            if let Some(monitor) = control.selected_monitor {
                self.lifecycle.monitor_selection_changed(
                    crate::display::MonitorSelection::Failed {
                        monitor,
                        error: crate::display::MonitorSelectionError::Superseded,
                    },
                );
            }
            let operation = self
                .window_settings
                .submit(window, &self.displays, id, request);
            self.lifecycle.window_operation_changed(operation);
        } else if let Some(monitor) = control.selected_monitor {
            if let Some(operation) = self.window_settings.cancel(window, &self.displays) {
                self.lifecycle.window_operation_changed(operation);
            }
            let selection = self.displays.select(window, monitor);
            self.lifecycle.monitor_selection_changed(selection);
        }
    }

    fn poll_controllers(&mut self, event_loop: &ActiveEventLoop) {
        for event in self.controllers.poll() {
            if matches!(
                event,
                gridthorn_input::InputEvent::Controller(
                    gridthorn_input::controller::ControllerEvent::Button { .. }
                        | gridthorn_input::controller::ControllerEvent::Axis { .. }
                )
            ) && !self
                .window
                .as_ref()
                .is_some_and(|window| window.has_focus())
            {
                continue;
            }
            if let Err(error) = self.lifecycle.input(event) {
                self.fail(event_loop, error);
                return;
            }
        }
    }

    fn apply_controller_control(&mut self, event_loop: &ActiveEventLoop, control: &WindowControl) {
        if control.poll_controllers {
            self.poll_controllers(event_loop);
        }
        for request in &control.controller_feedback {
            let event = self.controllers.feedback_focused(
                *request,
                self.window
                    .as_ref()
                    .is_some_and(|window| window.has_focus()),
            );
            if let Err(error) = self.lifecycle.input(event) {
                self.fail(event_loop, error);
                return;
            }
        }
    }

    fn apply_control(&mut self, event_loop: &ActiveEventLoop, control: &WindowControl) {
        self.apply_controller_control(event_loop, control);
        if let Some((id, config)) = control.presentation_request {
            self.configure_presentation(id, config);
        }
        self.apply_window_settings(event_loop, control);
        let Some(window) = self.window.as_ref() else {
            return;
        };

        if let Some(area) = control.text_input {
            let area = match area {
                gridthorn_input::TextInputRequest::Start(area) => Some(area),
                gridthorn_input::TextInputRequest::Stop => None,
            };
            let events = self.text.request(area, window.has_focus());
            if events.iter().all(|event| {
                !matches!(
                    event,
                    gridthorn_input::InputEvent::TextInputChanged { error: Some(_), .. }
                )
            }) {
                window.set_ime_allowed(self.text.active());
                if let Some(area) = area {
                    window.set_ime_cursor_area(
                        winit::dpi::PhysicalPosition::new(area.x, area.y),
                        PhysicalSize::new(area.width, area.height),
                    );
                }
            }
            for event in events {
                if let Err(error) = self.lifecycle.input(event) {
                    self.fail(event_loop, error);
                    return;
                }
            }
        }
        for request in &control.clipboard {
            let response = self.clipboard.execute(request, window.has_focus());
            if let Err(error) = self
                .lifecycle
                .input(gridthorn_input::InputEvent::Clipboard(response))
            {
                self.fail(event_loop, error);
                return;
            }
        }

        if let Some(mode) = control.capture {
            let status = self.capture.apply(mode, window.has_focus(), |mode| {
                window
                    .set_cursor_grab(super::capture::grab_mode(mode))
                    .map_err(|error| error.to_string())
            });
            if let Err(error) = self
                .lifecycle
                .input(gridthorn_input::InputEvent::PointerCaptureChanged(status))
            {
                self.fail(event_loop, error);
                return;
            }
        }
        if control.window_request.is_none()
            && let Some((width, height)) = control.requested_size
        {
            let request = super::settings::WindowRequest {
                size: Some(crate::display::DisplayResolution { width, height }),
                ..super::settings::WindowRequest::default()
            };
            let operation = self
                .window_settings
                .submit(window, &self.displays, 0, request);
            self.lifecycle.window_operation_changed(operation);
        }
        if let Some(minimized) = control.minimized {
            window.set_minimized(minimized);
        }
        let frame_deadline = self
            .presentation
            .pacer
            .deadline()
            .filter(|deadline| *deadline > std::time::Instant::now());
        let deadline = match (control.wake_at, frame_deadline) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        };
        event_loop.set_control_flow(deadline.map_or(ControlFlow::Wait, ControlFlow::WaitUntil));
        if control.exit_requested {
            event_loop.exit();
        }
    }

    fn configure_presentation(&mut self, id: u64, config: super::presentation::PresentationConfig) {
        let operation = self
            .presentation
            .configure(self.renderer.as_mut(), id, config);
        self.lifecycle.presentation_operation_changed(operation);
        self.publish_presentation();
    }

    fn publish_presentation(&mut self) {
        let (state, operation) = self.presentation.observe(self.renderer.as_ref());
        if let Some(state) = state {
            self.lifecycle.presentation_state_changed(state);
        }
        if let Some(operation) = operation {
            self.lifecycle.presentation_operation_changed(operation);
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

    fn cancel_capture(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            let status =
                self.capture
                    .apply(gridthorn_input::PointerCaptureMode::None, false, |mode| {
                        window
                            .set_cursor_grab(super::capture::grab_mode(mode))
                            .map_err(|error| error.to_string())
                    });
            if let Err(error) = self
                .lifecycle
                .input(gridthorn_input::InputEvent::PointerCaptureChanged(status))
            {
                self.fail(event_loop, error);
            }
        }
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
        self.controllers.cancel_input();
        self.suspended = false;
        self.presentation.pacer.reset();
        if self.window.is_none() {
            if let Err(error) = self.initialize(event_loop) {
                self.fail(event_loop, error);
            }
        } else {
            if let Some(renderer) = self.renderer.as_mut() {
                renderer.set_occluded(false);
            }
            self.lifecycle.resumed();
        }
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        self.controllers.stop();
        self.suspended = true;
        event_loop.set_control_flow(ControlFlow::Wait);
        self.cancel_capture(event_loop);
        let events = self.text.translate(&WindowEvent::Focused(false));
        if let Some(window) = &self.window {
            window.set_ime_allowed(false);
        }
        for event in events {
            if let Err(error) = self.lifecycle.input(event) {
                self.fail(event_loop, error);
                return;
            }
        }
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

        if matches!(
            event,
            WindowEvent::Moved(_)
                | WindowEvent::Resized(_)
                | WindowEvent::ScaleFactorChanged { .. }
        ) && let Some(window) = self.window.as_ref()
        {
            self.lifecycle.window_state_changed(
                self.window_settings.state(window, &self.displays),
                self.window_settings.capabilities(),
            );
        }

        if matches!(event, WindowEvent::Focused(_)) {
            self.controllers.cancel_input();
        }
        if matches!(event, WindowEvent::Focused(false)) {
            self.cancel_capture(event_loop);
            if let Some(window) = &self.window {
                window.set_ime_allowed(false);
            }
        }
        let mut inputs = self.text.translate(&event);
        if let Some(input) = map_window_input(&event) {
            if matches!(event, WindowEvent::Focused(false)) {
                inputs.push(input);
            } else {
                inputs.insert(0, input);
            }
        }
        for input in inputs {
            if let Err(error) = self.lifecycle.input(input) {
                self.fail(event_loop, error);
                return;
            }
        }

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                self.lifecycle.scale_factor_changed(scale_factor);
            }
            WindowEvent::Resized(size) => {
                self.lifecycle.resized(size.width, size.height);
                if let Some(renderer) = self.renderer.as_mut()
                    && let Err(error) = renderer.resize(size.width, size.height)
                {
                    self.fail(event_loop, error.into());
                }
                self.publish_presentation();
            }
            WindowEvent::Occluded(occluded) => {
                if let Some(renderer) = self.renderer.as_mut() {
                    renderer.set_occluded(occluded);
                }
            }
            WindowEvent::RedrawRequested => {
                if self.suspended {
                    return;
                }
                let now = std::time::Instant::now();
                if !self.presentation.pacer.ready(now) {
                    return;
                }
                self.presentation.pacer.record(now);
                let start = self.performance.start();
                if let Some(renderer) = self.renderer.as_mut()
                    && let Err(error) = renderer.render()
                {
                    self.fail(event_loop, error.into());
                }
                self.performance.redraw(start);
                self.publish_presentation();
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        if self.capture.effective() != gridthorn_input::PointerCaptureMode::None
            && self
                .window
                .as_ref()
                .is_some_and(|window| window.has_focus())
            && let winit::event::DeviceEvent::MouseMotion { delta } = event
            && let Err(error) = self
                .lifecycle
                .input(gridthorn_input::InputEvent::PointerMotion {
                    x: delta.0,
                    y: delta.1,
                })
        {
            self.fail(event_loop, error);
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if self.error.is_some() || self.suspended {
            return;
        }
        let start = self.performance.start();
        let mut control = WindowControl::default();
        if let Some(window) = self.window.as_ref()
            && let Some(operation) = self.window_settings.feedback(window, &self.displays)
        {
            self.lifecycle.window_operation_changed(operation);
        }
        if let Some(window) = self.window.as_ref()
            && let Some(selection) = self.displays.selection_feedback(window)
        {
            self.lifecycle.monitor_selection_changed(selection);
        }
        if let Err(error) = self.lifecycle.idle(&mut control) {
            self.fail(event_loop, error);
            return;
        }
        if let Some(renderer) = self.renderer.as_mut() {
            let extraction_start = self.performance.extraction_start();
            let frame = self.lifecycle.render_frame();
            self.performance.extraction(extraction_start);
            renderer.set_frame(frame);
        }
        self.apply_control(event_loop, &control);
        if self.presentation.pacer.ready(std::time::Instant::now())
            && let Some(window) = self.window.as_ref()
        {
            window.request_redraw();
        }
        self.performance.preparation(start);
    }
}

#[cfg(test)]
mod test;
