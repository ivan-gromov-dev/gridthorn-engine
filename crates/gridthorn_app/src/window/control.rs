use std::time::Instant;

/// Window operations available to lifecycle hooks.
#[derive(Debug, Default)]
pub struct WindowControl {
    pub(super) exit_requested: bool,
    pub(super) minimized: Option<bool>,
    pub(super) requested_size: Option<(u32, u32)>,
    pub(super) wake_at: Option<Instant>,
}

impl WindowControl {
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
