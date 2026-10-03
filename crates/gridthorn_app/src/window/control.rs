use std::time::Instant;

/// Window operations available to lifecycle hooks.
#[derive(Debug, Default)]
pub struct WindowControl {
    pub(super) text_input: Option<gridthorn_input::TextInputRequest>,
    pub(super) clipboard: Vec<gridthorn_input::ClipboardRequest>,
    pub(super) capture: Option<gridthorn_input::PointerCaptureMode>,
    pub(super) exit_requested: bool,
    pub(super) minimized: Option<bool>,
    pub(super) requested_size: Option<(u32, u32)>,
    pub(super) wake_at: Option<Instant>,
}

impl WindowControl {
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
