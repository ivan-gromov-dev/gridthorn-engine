use super::*;

#[test]
fn disabled_collection_and_independent_phase_limits() {
    let mut performance = WindowPerformance {
        enabled: false,
        preparation: Vec::new(),
        redraw: Vec::new(),
    };
    assert!(performance.start().is_none());
    performance.preparation(None);
    assert_eq!(performance.preparation, []);
    for _ in 0..SAMPLE_LIMIT + 10 {
        performance.preparation(Some(Instant::now()));
    }
    performance.redraw(Some(Instant::now()));
    assert_eq!(performance.preparation.len(), SAMPLE_LIMIT);
    assert_eq!(performance.redraw.len(), 1);
}
