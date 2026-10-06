use super::{PresentMode, PresentationError};
use std::time::Duration;

/// Validated software frame cap, independent of fixed ticks and display refresh.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameRateLimit(u32);
impl FrameRateLimit {
    /// Validate a positive FPS cap with at least one nanosecond per frame.
    /// # Errors
    /// Returns `InvalidFrameRate` outside 1 through 1,000,000,000 FPS.
    pub fn new(fps: u32) -> Result<Self, PresentationError> {
        if !(1..=1_000_000_000).contains(&fps) {
            return Err(PresentationError::InvalidFrameRate(fps));
        }
        Ok(Self(fps))
    }
    /// Requested maximum frames per second.
    #[must_use]
    pub fn fps(self) -> u32 {
        self.0
    }
    /// Rounded-up monotonic minimum interval; rounding never exceeds the cap.
    #[must_use]
    pub fn interval(self) -> Duration {
        Duration::from_nanos(1_000_000_000_u64.div_ceil(u64::from(self.0)))
    }
}

/// Complete presentation request; no cap by default, with FIFO `VSync`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PresentationConfig {
    /// Explicit GPU queue policy, validated against surface capabilities.
    pub present_mode: PresentMode,
    /// Software cap; `None` removes the software limit.
    pub frame_rate_limit: Option<FrameRateLimit>,
}

/// Configured state and current surface-specific capabilities.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PresentationState {
    /// Empty for renderer-free windows; only explicit backend-supported policies.
    pub supported_modes: Vec<PresentMode>,
    /// Last successfully configured mode; absent before acquisition or at zero size.
    pub applied_mode: Option<PresentMode>,
    /// Active software cap, applied on the event-loop thread.
    pub frame_rate_limit: Option<FrameRateLimit>,
}

/// Correlated feedback; pending mode changes wait for a renderable surface.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PresentationOperation {
    /// Queued or accepted, awaiting configuration; inspect state for the active cap.
    Pending { id: u64 },
    /// GPU policy has been configured and software cap accepted.
    Applied { id: u64, config: PresentationConfig },
    /// Rejected without changing the previous requested policy or cap.
    Failed { id: u64, error: PresentationError },
}
impl PresentationOperation {
    pub(super) fn id(&self) -> u64 {
        match self {
            Self::Pending { id } | Self::Applied { id, .. } | Self::Failed { id, .. } => *id,
        }
    }
}
