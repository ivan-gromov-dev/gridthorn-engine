use super::FrameRateLimit;
use std::time::Instant;

/// Minimum spacing between redraw attempts; late frames never create catch-up bursts.
#[derive(Default)]
pub(in crate::window) struct FramePacer {
    pub(in crate::window) limit: Option<FrameRateLimit>,
    previous: Option<Instant>,
}
impl FramePacer {
    pub(in crate::window) fn deadline(&self) -> Option<Instant> {
        self.previous
            .zip(self.limit)
            .map(|(previous, limit)| previous + limit.interval())
    }
    pub(in crate::window) fn ready(&self, now: Instant) -> bool {
        self.deadline().is_none_or(|deadline| now >= deadline)
    }
    pub(in crate::window) fn record(&mut self, now: Instant) {
        self.previous = Some(now);
    }
    pub(in crate::window) fn reset(&mut self) {
        self.previous = None;
    }
}
