use crate::display::{DisplayMode, DisplayResolution, MonitorId};

/// Requested presentation mode; monitor IDs come from an explicit display query.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowMode {
    /// Decorated window; restores the last windowed geometry unless overridden.
    Windowed,
    /// Monitor-sized borderless fullscreen without changing the desktop video mode.
    Borderless {
        /// Target connected monitor.
        monitor: MonitorId,
    },
    /// Exclusive fullscreen using one of the target monitor's advertised modes.
    ///
    /// Currently supports the happy path: Windows native mode-switch rejection
    /// may panic in upstream winit. Recovery is deferred to Milestone 5.
    Exclusive {
        /// Target connected monitor.
        monitor: MonitorId,
        /// Advertised mode to revalidate before applying.
        mode: DisplayMode,
    },
}

/// Backend-reported presentation mode classification.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum WindowModeKind {
    /// Ordinary windowed presentation.
    #[default]
    Windowed,
    /// Borderless fullscreen.
    Borderless,
    /// Exclusive fullscreen.
    Exclusive,
}

/// Windowed desktop placement in physical pixels.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowPlacement {
    /// Physical top-left of the outer frame; negative coordinates are valid.
    Position {
        /// Desktop x coordinate.
        x: i32,
        /// Desktop y coordinate.
        y: i32,
    },
    /// Center the outer frame within a monitor's full desktop bounds.
    Centered {
        /// Target connected monitor.
        monitor: MonitorId,
    },
}

/// User resizing policy and physical client-area constraints.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WindowResizePolicy {
    /// Disable user resizing; programmatic size changes remain allowed.
    Fixed,
    /// Allow user resizing, optionally constrained on each physical axis.
    Resizable {
        /// Minimum physical client area; `None` clears the constraint.
        min: Option<DisplayResolution>,
        /// Maximum physical client area; `None` clears the constraint.
        max: Option<DisplayResolution>,
    },
}
impl Default for WindowResizePolicy {
    fn default() -> Self {
        Self::Resizable {
            min: None,
            max: None,
        }
    }
}

/// One explicit operation. Absent fields preserve state, except windowed mode restoration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WindowRequest {
    /// Presentation mode change.
    pub mode: Option<WindowMode>,
    /// Desired physical client size, valid only in windowed mode.
    pub size: Option<DisplayResolution>,
    /// User resizing policy, valid only in windowed mode.
    pub resize_policy: Option<WindowResizePolicy>,
    /// Desktop placement, valid only in windowed mode.
    pub placement: Option<WindowPlacement>,
}

/// Explicit backend support; headless/default values are unavailable.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "independent backend capability flags are not lifecycle states"
)]
pub struct WindowCapabilities {
    /// Native window service is available.
    pub available: bool,
    /// Programmatic physical client resizing.
    pub size: bool,
    /// User resizing policy and client-size constraints.
    pub resize_policy: bool,
    /// Native resizable flag can be read back (unavailable on X11).
    pub resize_policy_feedback: bool,
    /// Desktop placement is supported (unavailable on Wayland).
    pub placement: bool,
    /// Borderless fullscreen is supported.
    pub borderless: bool,
    /// Exclusive fullscreen is supported (unavailable on Wayland).
    pub exclusive: bool,
}

/// Current window observation; configured constraints are distinguished from native readback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowState {
    /// Actual physical client size; zero is possible while minimized.
    pub size: DisplayResolution,
    /// Actual outer frame origin, when the platform reports it.
    pub position: Option<(i32, i32)>,
    /// Current presentation mode as reported by the backend.
    pub mode: WindowModeKind,
    /// Current window monitor when it resolves in the last explicit inventory.
    pub monitor: Option<MonitorId>,
    /// Exclusive mode readback, when applicable. Windows uses the OS-reported
    /// current refresh; its integer precision can report 59 Hz for a 60 Hz request.
    pub display_mode: Option<DisplayMode>,
    /// Last configured policy; OS constraint enforcement has no portable readback.
    pub resize_policy: WindowResizePolicy,
    /// Native resizable flag, or `None` where unsupported.
    pub resizable: Option<bool>,
}

/// Correlated latest-operation feedback; failures do not terminate the application.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WindowOperation {
    /// Queued or waiting for requested native properties.
    Pending {
        /// Runner-local request sequence number.
        id: u64,
    },
    /// Requested observable properties were confirmed by the backend.
    Applied {
        /// Completed request number.
        id: u64,
        /// Applied observation, including the configured resize policy.
        state: WindowState,
    },
    /// Validation, capability checks or native confirmation failed.
    Failed {
        /// Failed request number.
        id: u64,
        /// Recoverable contextual diagnostic.
        error: super::WindowOperationError,
        /// Actual state at failure; OS operations are not rolled back automatically.
        state: WindowState,
    },
}
