use super::{MonitorId, MonitorSelection, MonitorSelectionError};
use std::time::{Duration, Instant};

pub(super) struct PendingSelection {
    monitor: MonitorId,
    started: Instant,
}

impl PendingSelection {
    pub(super) fn new(monitor: MonitorId, started: Instant) -> Self {
        Self { monitor, started }
    }
    pub(super) fn observe(
        &self,
        actual: Option<MonitorId>,
        now: Instant,
    ) -> Option<MonitorSelection> {
        if actual == Some(self.monitor) {
            Some(MonitorSelection::Applied {
                monitor: self.monitor,
            })
        } else if now.saturating_duration_since(self.started) >= Duration::from_secs(2) {
            Some(MonitorSelection::Failed {
                monitor: self.monitor,
                error: MonitorSelectionError::NotApplied { actual },
            })
        } else {
            None
        }
    }
}

pub(super) fn centered_origin(
    origin: (i32, i32),
    extent: (u32, u32),
    window: (u32, u32),
) -> (i32, i32) {
    fn coordinate(origin: i32, extent: u32, window: u32) -> i32 {
        let centered = i64::from(origin) + (i64::from(extent) - i64::from(window)) / 2;
        i32::try_from(centered.clamp(i64::from(i32::MIN), i64::from(i32::MAX))).unwrap_or(origin)
    }
    (
        coordinate(origin.0, extent.0, window.0),
        coordinate(origin.1, extent.1, window.1),
    )
}
