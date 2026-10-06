use super::super::{FramePacer, FrameRateLimit, PresentationError};
use std::time::{Duration, Instant};

#[test]
fn validates_caps_and_rounds_intervals_up() {
    for fps in [0, 1_000_000_001, u32::MAX] {
        assert_eq!(
            FrameRateLimit::new(fps),
            Err(PresentationError::InvalidFrameRate(fps))
        );
    }
    assert_eq!(
        FrameRateLimit::new(60).unwrap().interval(),
        Duration::from_nanos(16_666_667)
    );
    assert_eq!(
        FrameRateLimit::new(1).unwrap().interval(),
        Duration::from_secs(1)
    );
    assert_eq!(
        FrameRateLimit::new(1_000_000_000).unwrap().interval(),
        Duration::from_nanos(1)
    );
}

#[test]
fn caps_early_events_and_late_frames_without_catchup_bursts() {
    let start = Instant::now();
    let mut pacer = FramePacer::default();
    pacer.limit = Some(FrameRateLimit::new(100).unwrap());
    assert!(pacer.ready(start));
    pacer.record(start);
    assert!(!pacer.ready(start + Duration::from_millis(9)));
    assert!(pacer.ready(start + Duration::from_millis(10)));
    let late = start + Duration::from_millis(65);
    pacer.record(late);
    assert_eq!(pacer.deadline(), Some(start + Duration::from_millis(75)));
    pacer.limit = Some(FrameRateLimit::new(50).unwrap());
    assert_eq!(pacer.deadline(), Some(start + Duration::from_millis(85)));
    pacer.limit = None;
    assert!(pacer.ready(late));
    pacer.limit = Some(FrameRateLimit::new(1).unwrap());
    pacer.reset();
    assert!(pacer.ready(late));
}
