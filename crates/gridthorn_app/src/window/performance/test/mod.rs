use super::*;

#[test]
fn disabled_collection_and_independent_phase_limits() {
    let mut performance = WindowPerformance {
        enabled: false,
        frames: frames::FrameSamples::default(),
        preparation: Vec::new(),
        redraw: Vec::new(),
        extraction: Vec::new(),
    };
    assert!(performance.start().is_none());
    assert_eq!(performance.extraction_start(), None);
    performance.extraction(None);
    assert_eq!(performance.extraction, []);
    performance.preparation(None);
    assert_eq!(performance.preparation, []);
    for _ in 0..SAMPLE_LIMIT + 10 {
        performance.preparation(Some(Instant::now()));
    }
    performance.redraw(Some(Instant::now()));
    assert_eq!(performance.preparation.len(), SAMPLE_LIMIT);
    assert_eq!(performance.redraw.len(), 1);
}

#[test]
fn extraction_has_its_own_cap_and_stops_timing_when_full() {
    let mut performance = WindowPerformance {
        enabled: true,
        frames: frames::FrameSamples::default(),
        preparation: Vec::new(),
        redraw: Vec::new(),
        extraction: Vec::new(),
    };
    for _ in 0..=SAMPLE_LIMIT {
        let start = performance.extraction_start();
        performance.extraction(start);
    }
    assert_eq!(performance.extraction.len(), SAMPLE_LIMIT);
    assert_eq!(performance.extraction_start(), None);
    assert_eq!(performance.preparation, []);
    assert_eq!(performance.redraw, []);
}
