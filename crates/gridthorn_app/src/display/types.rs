/// Opaque identity valid only within one native runner and observed connection.
/// Never persist this value; an observed reconnect receives a new identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonitorId(pub(crate) u64, pub(crate) u64);

/// Physical pixel dimensions of a display or fullscreen mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayResolution {
    /// Physical width in pixels.
    pub width: u32,
    /// Physical height in pixels.
    pub height: u32,
}

/// Backend-advertised fullscreen mode, without a promise of successful activation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayMode {
    /// Physical resolution.
    pub resolution: DisplayResolution,
    /// Refresh rate in millihertz; zero means the backend did not report a rate.
    pub refresh_rate_millihertz: u32,
    /// Backend-reported color bit depth; some platforms supply a fixed value.
    pub bit_depth: u16,
}

/// Latest observed properties of a connected monitor.
#[derive(Clone, Debug, PartialEq)]
pub struct MonitorInfo {
    /// Runner-local connection identity.
    pub id: MonitorId,
    /// Optional display name; neither unique nor persistent.
    pub name: Option<String>,
    /// Current desktop physical dimensions, independent of advertised modes.
    pub resolution: DisplayResolution,
    /// Desktop origin in physical pixels; negative coordinates are valid.
    pub position: (i32, i32),
    /// Current desktop refresh rate; unavailable or zero rates become `None`.
    pub refresh_rate_millihertz: Option<u32>,
    /// OS scaling in physical pixels per logical pixel, not measured physical DPI.
    /// May differ from the window scale, particularly on Wayland.
    pub scale_factor: f64,
    /// Sorted, deduplicated advertised modes; empty means none were reported.
    pub modes: Vec<DisplayMode>,
}

/// Whether a native desktop enumeration has been performed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DisplayAvailability {
    /// No native enumeration (for example, headless execution).
    #[default]
    Unavailable,
    /// A native request completed; an empty list can still be valid.
    Available,
}

/// Difference between consecutive sampled display inventories.
#[derive(Clone, Debug, PartialEq)]
pub enum DisplayChange {
    /// A newly observed connection, including initial enumeration.
    Connected(MonitorId),
    /// Properties or advertised modes changed for an existing connection.
    Changed(MonitorId),
    /// A connection disappeared; its identity is retired.
    Disconnected(MonitorId),
    /// The reported primary monitor changed (or is unavailable).
    PrimaryChanged(Option<MonitorId>),
}

/// Persistent feedback for the latest explicit monitor-selection request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MonitorSelection {
    /// Placement is queued or awaiting native confirmation.
    Pending {
        /// Requested monitor.
        monitor: MonitorId,
    },
    /// The OS reported this monitor as the window's current monitor.
    Applied {
        /// Confirmed monitor.
        monitor: MonitorId,
    },
    /// The request failed; see the typed diagnostic.
    Failed {
        /// Requested monitor.
        monitor: MonitorId,
        /// Failure reason.
        error: super::MonitorSelectionError,
    },
}
