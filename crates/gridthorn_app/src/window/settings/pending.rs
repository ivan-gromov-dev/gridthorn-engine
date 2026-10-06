use super::{WindowModeKind, WindowOperation, WindowOperationError, WindowState};
use crate::display::{DisplayMode, DisplayResolution, MonitorId};
use std::time::{Duration, Instant};

#[derive(Default)]
pub(super) struct ExpectedWindow {
    pub mode: Option<WindowModeKind>,
    pub monitor: Option<MonitorId>,
    pub display_mode: Option<DisplayMode>,
    pub size: Option<DisplayResolution>,
    pub position: Option<(i32, i32)>,
    pub resizable: Option<bool>,
}
impl ExpectedWindow {
    pub fn matches(&self, state: &WindowState) -> bool {
        self.mode.is_none_or(|mode| mode == state.mode)
            && self
                .monitor
                .is_none_or(|monitor| Some(monitor) == state.monitor)
            && self.display_mode.is_none_or(|mode| {
                state
                    .display_mode
                    .is_some_and(|actual| mode_matches(mode, actual))
            })
            && self.size.is_none_or(|size| size == state.size)
            && self
                .resizable
                .is_none_or(|value| Some(value) == state.resizable)
            && self.position.is_none_or(|position| {
                state.position.is_some_and(|actual| {
                    (i64::from(position.0) - i64::from(actual.0)).abs() <= 2
                        && (i64::from(position.1) - i64::from(actual.1)).abs() <= 2
                })
            })
    }
}

fn mode_matches(expected: DisplayMode, actual: DisplayMode) -> bool {
    expected.resolution == actual.resolution
        && expected.bit_depth == actual.bit_depth
        && (expected.refresh_rate_millihertz == actual.refresh_rate_millihertz
            || cfg!(target_os = "windows")
                && matches!(
                    (
                        expected.refresh_rate_millihertz,
                        actual.refresh_rate_millihertz
                    ),
                    (59_000, 60_000) | (60_000, 59_000)
                ))
}

pub(super) struct PendingWindow {
    pub id: u64,
    pub expected: ExpectedWindow,
    pub started: Instant,
}
impl PendingWindow {
    pub fn observe(&self, state: WindowState, now: Instant) -> Option<WindowOperation> {
        if self.expected.matches(&state) {
            Some(WindowOperation::Applied { id: self.id, state })
        } else if now.saturating_duration_since(self.started) >= Duration::from_secs(5) {
            Some(WindowOperation::Failed {
                id: self.id,
                error: WindowOperationError::NotApplied,
                state,
            })
        } else {
            None
        }
    }
}
