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
    for operation in 1..6 {
        let start = performance.start(operation);
        assert!(start.is_some());
        performance.record(operation, start, 31);
        assert_eq!(performance.samples[operation].len(), 1);
        assert_eq!(performance.samples[operation][0].units, 31);
    }
}

#[test]
fn oversized_layout_does_not_publish_partial_shape_timings() {
    let mut service = crate::text::test::system();
    service.performance = Some(TextPerformance::default());
    let mut style = crate::TextStyle::new("Noto Sans", 20.0);
    style.line_height = 1000.0;
    assert!(matches!(
        service.layout(&"\n".repeat(100), &style),
        Err(crate::TextError::TooLarge)
    ));
    let performance = service.performance.as_ref().unwrap();
    assert_eq!(performance.totals, [0; 6]);
    assert!(performance.samples.iter().all(Vec::is_empty));
}
