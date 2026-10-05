use super::MonitorId;
use thiserror::Error;

/// Recoverable monitor-selection failure; the application keeps running.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum MonitorSelectionError {
    /// An explicit window operation superseded legacy monitor-only placement.
    #[error("monitor selection was superseded by window configuration")]
    Superseded,
    /// The identity is foreign, retired or absent from the latest native query.
    #[error("monitor {monitor:?} is no longer available in this runner")]
    Unavailable {
        /// Requested connection identity.
        monitor: MonitorId,
    },
    /// This backend cannot position the window in desktop coordinates.
    #[error("monitor placement is unavailable: {reason}")]
    PositionUnavailable {
        /// Backend diagnostic.
        reason: String,
    },
    /// Fullscreen switching requires a separate mode-selection contract.
    #[error("monitor selection currently supports windowed windows only")]
    FullscreenUnsupported,
    /// The OS did not confirm the requested monitor within the placement deadline.
    #[error("monitor placement was not confirmed; observed monitor: {actual:?}")]
    NotApplied {
        /// Last observed window monitor, when it resolves in the latest inventory.
        actual: Option<MonitorId>,
    },
}
