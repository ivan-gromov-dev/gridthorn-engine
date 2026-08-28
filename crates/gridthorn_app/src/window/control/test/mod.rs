use std::time::{Duration, Instant};

use super::WindowControl;

#[test]
fn records_requested_window_operations() {
    let mut control = WindowControl::default();

    control.set_size(800, 600);
    control.set_minimized(true);
    let deadline = Instant::now() + Duration::from_secs(1);
    control.wake_at(deadline);
    control.exit();

    assert_eq!(control.requested_size, Some((800, 600)));
    assert_eq!(control.minimized, Some(true));
    assert_eq!(control.wake_at, Some(deadline));
    assert!(control.exit_requested);
}
