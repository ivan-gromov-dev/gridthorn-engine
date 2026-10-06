/// Explicit controller discovery/input polling requests. No automatic polling is enabled.
#[derive(Clone, Debug, Default)]
pub struct ControllerPolling {
    requested: bool,
}
impl ControllerPolling {
    /// Request discovery and collection of pending controller input once after this frame.
    /// Multiple requests before dispatch coalesce. Results arrive in the next snapshot.
    pub fn request_poll(&mut self) {
        self.requested = true;
    }
    /// Drain the pending request once; custom adapters perform the actual polling.
    pub fn take_request(&mut self) -> bool {
        std::mem::take(&mut self.requested)
    }
}
