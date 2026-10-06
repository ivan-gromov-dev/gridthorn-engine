use super::{
    WindowCapabilities, WindowOperation, WindowOperationError, WindowRequest, WindowState,
};

/// Explicit window request mailbox and retained observation/feedback.
/// Contains only owned `Send + Sync` data. Native operations execute on the
/// event-loop thread; requests are presentation operations, not simulation commands.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WindowSettings {
    capabilities: WindowCapabilities,
    state: Option<WindowState>,
    feedback: Option<WindowOperation>,
    next_id: u64,
    request: Option<(u64, WindowRequest)>,
}
impl WindowSettings {
    /// Queue a validated operation; the last queued request wins.
    /// Older feedback is ignored after a new request is queued. No native work
    /// occurs until dispatch after Startup or the current frame.
    ///
    /// # Errors
    /// Rejects empty requests, invalid sizes/constraints and inconsistent modes.
    pub fn request(&mut self, request: WindowRequest) -> Result<u64, WindowOperationError> {
        request.validate()?;
        self.next_id += 1;
        self.request = Some((self.next_id, request));
        self.feedback = Some(WindowOperation::Pending { id: self.next_id });
        Ok(self.next_id)
    }
    /// Platform capabilities, unavailable for the default/headless service.
    #[must_use]
    pub fn capabilities(&self) -> WindowCapabilities {
        self.capabilities
    }
    /// Latest observed native state, updated on relevant window events and requests.
    #[must_use]
    pub fn state(&self) -> Option<WindowState> {
        self.state
    }
    /// Persistent result of the most recently queued request.
    #[must_use]
    pub fn feedback(&self) -> Option<&WindowOperation> {
        self.feedback.as_ref()
    }
    pub(crate) fn take_request(&mut self) -> Option<(u64, WindowRequest)> {
        self.request.take()
    }
    pub(crate) fn publish_state(&mut self, state: WindowState, capabilities: WindowCapabilities) {
        self.state = Some(state);
        self.capabilities = capabilities;
    }
    pub(crate) fn publish_feedback(&mut self, feedback: WindowOperation) {
        let id = match &feedback {
            WindowOperation::Pending { id }
            | WindowOperation::Applied { id, .. }
            | WindowOperation::Failed { id, .. } => *id,
        };
        if id != self.next_id {
            return;
        }
        match &feedback {
            WindowOperation::Applied { state, .. } | WindowOperation::Failed { state, .. } => {
                self.state = Some(*state);
            }
            WindowOperation::Pending { .. } => {}
        }
        self.feedback = Some(feedback);
    }
}
