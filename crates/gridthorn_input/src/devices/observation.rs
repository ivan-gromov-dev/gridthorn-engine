/// Explicit raw-device observation requests. Disabled by default; installed before startup.
/// Applied state and unsupported-request feedback arrive through `InputEvent::RawInputChanged`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RawInput {
    pending: Option<bool>,
}

impl RawInput {
    /// Request continuous observation or stop it. The last request before dispatch wins.
    /// Stopping cancels raw actions and clears the observed inventory; aggregate input
    /// and captured aggregate pointer motion remain independent.
    pub fn request(&mut self, enabled: bool) {
        self.pending = Some(enabled);
    }

    /// Drain a one-shot request for a custom platform adapter.
    pub fn take_request(&mut self) -> Option<bool> {
        self.pending.take()
    }
}
