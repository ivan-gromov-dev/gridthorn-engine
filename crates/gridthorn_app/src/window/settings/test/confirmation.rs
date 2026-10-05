use super::super::{
    pending::{ExpectedWindow, PendingWindow},
    *,
};
use crate::display::{DisplayResolution, MonitorId};
use std::time::{Duration, Instant};

fn state() -> WindowState {
    WindowState {
        size: DisplayResolution {
            width: 800,
            height: 600,
        },
        position: Some((-1200, 40)),
        mode: WindowModeKind::Windowed,
        monitor: Some(MonitorId(1, 1)),
        display_mode: None,
        resize_policy: WindowResizePolicy::Fixed,
        resizable: Some(false),
    }
}

#[test]
fn confirmation_requires_requested_properties_and_reports_actual_state_on_timeout() {
    let now = Instant::now();
    let mut actual = state();
    let pending = PendingWindow {
        id: 1,
        expected: ExpectedWindow {
            size: Some(actual.size),
            position: actual.position,
            monitor: actual.monitor,
            resizable: Some(false),
            mode: Some(WindowModeKind::Windowed),
            ..ExpectedWindow::default()
        },
        started: now,
    };
    assert!(matches!(
        pending.observe(actual, now),
        Some(WindowOperation::Applied { id: 1, .. })
    ));
    actual.size.width = 900;
    assert_eq!(pending.observe(actual, now), None);
    assert_eq!(
        pending.observe(actual, now + Duration::from_secs(5)),
        Some(WindowOperation::Failed {
            id: 1,
            error: WindowOperationError::NotApplied,
            state: actual
        })
    );
}

#[test]
fn policy_readback_can_be_unavailable_and_fullscreen_size_alone_is_not_success() {
    let mut actual = state();
    actual.resizable = None;
    assert!(ExpectedWindow::default().matches(&actual));
    let expected = ExpectedWindow {
        mode: Some(WindowModeKind::Borderless),
        size: Some(actual.size),
        ..ExpectedWindow::default()
    };
    assert!(!expected.matches(&actual));
    actual.mode = WindowModeKind::Borderless;
    assert!(expected.matches(&actual));
}

#[test]
fn exclusive_confirmation_reports_refresh_aliases_without_hiding_wrong_modes() {
    let mut actual = state();
    actual.mode = WindowModeKind::Exclusive;
    let requested = crate::display::DisplayMode {
        resolution: actual.size,
        refresh_rate_millihertz: 60_000,
        bit_depth: 32,
    };
    let expected = ExpectedWindow {
        mode: Some(WindowModeKind::Exclusive),
        display_mode: Some(requested),
        ..ExpectedWindow::default()
    };
    actual.display_mode = Some(requested);
    assert!(expected.matches(&actual));
    actual.display_mode = Some(crate::display::DisplayMode {
        refresh_rate_millihertz: 59_000,
        ..requested
    });
    assert_eq!(expected.matches(&actual), cfg!(target_os = "windows"));
    for wrong in [
        crate::display::DisplayMode {
            refresh_rate_millihertz: 58_000,
            ..requested
        },
        crate::display::DisplayMode {
            refresh_rate_millihertz: 0,
            ..requested
        },
        crate::display::DisplayMode {
            bit_depth: 16,
            ..requested
        },
        crate::display::DisplayMode {
            resolution: DisplayResolution {
                width: 640,
                height: 480,
            },
            ..requested
        },
    ] {
        actual.display_mode = Some(wrong);
        assert!(!expected.matches(&actual));
    }
    let high_rate = crate::display::DisplayMode {
        refresh_rate_millihertz: 144_000,
        ..requested
    };
    actual.display_mode = Some(crate::display::DisplayMode {
        refresh_rate_millihertz: 143_000,
        ..high_rate
    });
    assert!(
        !ExpectedWindow {
            display_mode: Some(high_rate),
            ..ExpectedWindow::default()
        }
        .matches(&actual)
    );
}
