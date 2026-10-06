use std::time::Instant;

/// Window operations available to lifecycle hooks.
#[derive(Debug, Default)]
pub struct WindowControl {
    pub(super) poll_controllers: bool,
    pub(super) controller_feedback: Vec<gridthorn_input::controller::RumbleRequest>,
    pub(super) presentation_request: Option<(u64, super::presentation::PresentationConfig)>,
    pub(super) window_request: Option<(u64, super::settings::WindowRequest)>,
    pub(super) refresh_displays: bool,
    pub(super) selected_monitor: Option<crate::display::MonitorId>,
    pub(super) text_input: Option<gridthorn_input::TextInputRequest>,
    pub(super) clipboard: Vec<gridthorn_input::ClipboardRequest>,
    pub(super) capture: Option<gridthorn_input::PointerCaptureMode>,
    pub(super) exit_requested: bool,
    pub(super) minimized: Option<bool>,
    pub(super) requested_size: Option<(u32, u32)>,
    pub(super) wake_at: Option<Instant>,
}

impl WindowControl {
    /// Request controller discovery and pending input once; results arrive as input events.
    pub fn poll_controllers(&mut self) {
        self.poll_controllers = true;
    }
    /// Queue a controller rumble request; results arrive as input events.
    pub fn rumble(&mut self, request: gridthorn_input::controller::RumbleRequest) {
        self.controller_feedback.push(request);
    }
    /// Request a correlated surface policy and software cap; the last call wins.
    pub fn configure_presentation(
        &mut self,
        id: u64,
        config: super::presentation::PresentationConfig,
    ) {
        self.presentation_request = Some((id, config));
    }
    /// Submit one correlated explicit window operation; the last call wins.
    /// Feedback arrives through `window_operation_changed`. Invalid data is
    /// rejected by the native adapter before changing OS state.
    pub fn configure_window(&mut self, id: u64, request: super::settings::WindowRequest) {
        self.window_request = Some((id, request));
    }
    /// Request one inventory snapshot; feedback arrives through `displays_changed`.
    pub fn refresh_displays(&mut self) {
        self.refresh_displays = true;
    }

    /// Request windowed placement on a monitor, with typed applied-state feedback.
    /// Revalidates the inventory once; the last request before dispatch wins.
    pub fn select_monitor(&mut self, monitor: crate::display::MonitorId) {
        self.selected_monitor = Some(monitor);
    }

    /// Open/update a text session or close it with `None`; feedback arrives as input events.
    pub fn set_text_input(&mut self, area: Option<gridthorn_input::ImeCursorArea>) {
        self.text_input = Some(area.map_or(
            gridthorn_input::TextInputRequest::Stop,
            gridthorn_input::TextInputRequest::Start,
        ));
    }

    /// Queue a correlated clipboard operation on the native event-loop thread.
    pub fn clipboard(&mut self, request: gridthorn_input::ClipboardRequest) {
        self.clipboard.push(request);
    }
    /// Request native pointer capture; feedback arrives through input events.
    pub fn set_pointer_capture(&mut self, mode: gridthorn_input::PointerCaptureMode) {
        self.capture = Some(mode);
    }

    /// Request application shutdown after the current hook returns.
    pub fn exit(&mut self) {
        self.exit_requested = true;
    }

    /// Request a minimized or restored platform window.
    pub fn set_minimized(&mut self, minimized: bool) {
        self.minimized = Some(minimized);
    }

    /// Request a new physical window size.
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.requested_size = Some((width, height));
    }

    /// Request the event loop to wake at a monotonic deadline.
    pub fn wake_at(&mut self, deadline: Instant) {
        self.wake_at = Some(deadline);
    }
}

#[cfg(test)]
mod test;
