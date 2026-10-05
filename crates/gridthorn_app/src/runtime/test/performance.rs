use super::{RuntimePerformance, SAMPLE_LIMIT};

#[test]
fn disabled_collection_never_starts_or_stores_samples() {
    let mut diagnostics = RuntimePerformance::with_enabled(false);
    for phase in 0..6 {
        let start = diagnostics.start(phase);
        assert_eq!(start, None);
        diagnostics.record(phase, start);
        assert_eq!(diagnostics.samples[phase], []);
    }
}

#[test]
fn phases_stop_timing_at_independent_limits() {
    let mut diagnostics = RuntimePerformance::with_enabled(true);
    for _ in 0..=SAMPLE_LIMIT {
        let start = diagnostics.start(0);
        diagnostics.record(0, start);
    }
    assert_eq!(diagnostics.samples[0].len(), SAMPLE_LIMIT);
    assert_eq!(diagnostics.start(0), None);
    assert_ne!(diagnostics.start(1), None);
    assert_eq!(diagnostics.samples[1], []);
}
