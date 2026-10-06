use super::{ControllerError, ControllerId};
/// One bounded dual-motor rumble request. Zero duration stops existing rumble.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RumbleRequest {
    /// Caller-owned correlation identity.
    pub request: u64,
    /// Current connection key.
    pub id: ControllerId,
    /// Low-frequency motor magnitude in `[0, 1]`.
    pub strong: f32,
    /// High-frequency motor magnitude in `[0, 1]`.
    pub weak: f32,
    /// Duration in milliseconds, at most 60 seconds.
    pub duration_ms: u32,
}
impl RumbleRequest {
    /// Validate before changing any native effect.
    /// # Errors
    /// Rejects invalid magnitudes or excessive duration.
    pub fn validate(self) -> Result<(), ControllerError> {
        if [self.strong, self.weak]
            .iter()
            .all(|v| v.is_finite() && (0.0..=1.0).contains(v))
            && self.duration_ms <= 60_000
        {
            Ok(())
        } else {
            Err(ControllerError::InvalidParameters)
        }
    }
}
/// Ordered one-shot feedback queue; custom adapters can drain it without hardware.
#[derive(Clone, Debug, Default)]
pub struct ControllerFeedback {
    requests: Vec<RumbleRequest>,
}
impl ControllerFeedback {
    /// Queue a request; native validation produces correlated feedback.
    pub fn rumble(&mut self, request: RumbleRequest) {
        self.requests.push(request);
    }
    /// Drain queued requests once, in submission order.
    pub fn take_requests(&mut self) -> Vec<RumbleRequest> {
        std::mem::take(&mut self.requests)
    }
}
