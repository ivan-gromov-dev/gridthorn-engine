use super::{PresentationConfig, PresentationError, PresentationOperation, PresentationState};

/// Game-facing coalesced mailbox. Native settings remain unavailable in headless runs.
#[derive(Clone, Debug, Default)]
pub struct PresentationSettings {
    state: PresentationState,
    feedback: Option<PresentationOperation>,
    next_id: u64,
    request: Option<(u64, PresentationConfig)>,
}

#[cfg(test)]
#[path = "test/request_ids.rs"]
mod test;
impl PresentationSettings {
    /// Submit a complete configuration; the last queued request wins.
    /// # Errors
    /// Returns `RequestIdsExhausted` rather than reusing a correlation identifier.
    pub fn request(&mut self, config: PresentationConfig) -> Result<u64, PresentationError> {
        let id = self
            .next_id
            .checked_add(1)
            .ok_or(PresentationError::RequestIdsExhausted)?;
        self.next_id = id;
        self.request = Some((id, config));
        self.feedback = Some(PresentationOperation::Pending { id });
        Ok(id)
    }
    /// Latest capabilities and observed configuration, separate from pending requests.
    #[must_use]
    pub fn state(&self) -> &PresentationState {
        &self.state
    }
    /// Latest correlated feedback.
    #[must_use]
    pub fn feedback(&self) -> Option<&PresentationOperation> {
        self.feedback.as_ref()
    }
    pub(crate) fn take_request(&mut self) -> Option<(u64, PresentationConfig)> {
        self.request.take()
    }
    pub(crate) fn publish_state(&mut self, state: PresentationState) {
        self.state = state;
    }
    pub(crate) fn publish_feedback(&mut self, feedback: PresentationOperation) {
        if self
            .feedback
            .as_ref()
            .is_none_or(|previous| previous.id() <= feedback.id())
        {
            self.feedback = Some(feedback);
        }
    }
}
