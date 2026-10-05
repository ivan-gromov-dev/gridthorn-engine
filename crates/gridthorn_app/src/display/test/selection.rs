use super::*;
use crate::display::selection::{PendingSelection, centered_origin};
use std::time::{Duration, Instant};

#[test]
fn confirms_actual_monitor_and_reports_timeout_without_success_guessing() {
    let now = Instant::now();
    let id = MonitorId(1, 1);
    let other = MonitorId(1, 2);
    let pending = PendingSelection::new(id, now);
    assert_eq!(pending.observe(None, now), None);
    assert_eq!(
        pending.observe(Some(other), now + Duration::from_millis(1999)),
        None
    );
    assert_eq!(
        pending.observe(Some(id), now),
        Some(MonitorSelection::Applied { monitor: id })
    );
    assert_eq!(
        pending.observe(Some(other), now + Duration::from_secs(2)),
        Some(MonitorSelection::Failed {
            monitor: id,
            error: MonitorSelectionError::NotApplied {
                actual: Some(other)
            },
        })
    );
}

#[test]
fn centers_on_negative_origins_and_bounds_coordinate_overflow() {
    assert_eq!(
        centered_origin((-1920, 0), (1920, 1080), (800, 600)),
        (-1360, 240)
    );
    assert_eq!(
        centered_origin((0, 0), (800, 600), (1000, 800)),
        (-100, -100)
    );
    assert_eq!(
        centered_origin((i32::MAX, i32::MIN), (u32::MAX, 1), (1, u32::MAX)),
        (i32::MAX, i32::MIN)
    );
}
