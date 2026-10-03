use super::*;

#[test]
fn operation_limits_are_independent_and_totals_survive_truncation() {
    let mut performance = TextPerformance::default();
    for _ in 0..=LIMIT {
        let start = performance.start(0);
        performance.record(0, start, 12);
    }
    assert_eq!(performance.samples[0].len(), LIMIT);
    assert_eq!(performance.totals[0], LIMIT + 1);
    assert!(performance.start(0).is_none());
    let start = performance.start(1);
    assert!(start.is_some());
    performance.record(1, start, 31);
    assert_eq!(performance.samples[1].len(), 1);
    assert_eq!(performance.samples[1][0].units, 31);
}
