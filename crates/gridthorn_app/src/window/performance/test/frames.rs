use super::*;

#[test]
fn pairs_accumulated_preparations_once_and_ignores_unprepared_redraws() {
    let mut frames = FrameSamples::default();
    frames.redraw(Duration::from_micros(100));
    frames.preparation(Duration::from_micros(2));
    frames.preparation(Duration::from_micros(3));
    frames.redraw(Duration::from_micros(7));
    frames.redraw(Duration::from_micros(100));
    frames.preparation(Duration::from_micros(4));
    assert_eq!(frames.samples, [(2, Duration::from_micros(12))]);
    frames.redraw(Duration::from_micros(5));
    assert_eq!(frames.samples[1], (1, Duration::from_micros(9)));
}

#[test]
fn bounds_retention_and_pending_work_after_limit() {
    let mut frames = FrameSamples::default();
    for _ in 0..super::super::SAMPLE_LIMIT + 10 {
        frames.preparation(Duration::from_micros(1));
        frames.redraw(Duration::from_micros(2));
    }
    assert_eq!(frames.samples.len(), super::super::SAMPLE_LIMIT);
    assert_eq!(frames.pending, Duration::ZERO);
    assert_eq!(frames.preparations, 0);
}
