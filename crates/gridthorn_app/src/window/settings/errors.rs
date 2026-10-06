use crate::display::{DisplayMode, MonitorId};
use thiserror::Error;

/// Recoverable explicit window-operation error.
#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum WindowOperationError {
    /// Reserved for future recovery; upstream fullscreen rejection currently may panic.
    #[error("Windows rejected fullscreen mode activation/restoration (display status {code})")]
    NativeRejected {
        /// Signed `DISP_CHANGE` status returned by Windows.
        code: i32,
    },
    /// No property was requested.
    #[error("window request contains no operation")]
    EmptyRequest,
    /// Dimensions, min/max constraints or requested size are inconsistent.
    #[error("invalid window dimensions or constraints: {reason}")]
    InvalidSize {
        /// Validation context.
        reason: &'static str,
    },
    /// Windowed-only properties were combined with fullscreen.
    #[error("window size, placement and resize policy require windowed mode")]
    WindowedOnly,
    /// Backend explicitly lacks the requested capability.
    #[error("window capability is unavailable: {capability}")]
    Unsupported {
        /// Missing native capability.
        capability: &'static str,
    },
    /// Requested connection identity is foreign, retired or disconnected.
    #[error("monitor {monitor:?} is no longer available for window operations")]
    MonitorUnavailable {
        /// Requested monitor.
        monitor: MonitorId,
    },
    /// An exclusive video mode is not in the monitor's current advertised inventory.
    #[error("exclusive mode {mode:?} is unavailable on monitor {monitor:?}")]
    ModeUnavailable {
        /// Target monitor.
        monitor: MonitorId,
        /// Requested advertised mode.
        mode: DisplayMode,
    },
    /// Requested properties were not observed within the confirmation deadline.
    #[error("window request was not confirmed before its deadline")]
    NotApplied,
    /// A newer operation owns window control.
    #[error("window operation was superseded by a newer request")]
    Superseded,
}
