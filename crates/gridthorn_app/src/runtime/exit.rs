/// Engine-owned request to stop the application after the current frame.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ExitRequest {
    requested: bool,
}

impl ExitRequest {
    /// Mark the application for orderly shutdown.
    pub fn request(&mut self) {
        self.requested = true;
    }

    /// Whether orderly shutdown was requested.
    #[must_use]
    pub fn is_requested(self) -> bool {
        self.requested
    }
}
