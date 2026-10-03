use super::*;

#[test]
fn text_width_errors_and_capacity_control_measurement_reuse() {
    let mut cache = MeasurementCache::default();
    assert_eq!(
        cache
            .measure("caption", 500.0, || Ok([100.0, 20.0]))
            .unwrap(),
        [100.0, 20.0]
    );
    assert_eq!(
        cache
            .measure("caption", 500.0, || panic!("repeat must reuse measurement"))
            .unwrap(),
        [100.0, 20.0]
    );
    assert_eq!(
        cache.measure("caption", 90.0, || Ok([80.0, 40.0])).unwrap(),
        [80.0, 40.0]
    );
    cache
        .measure("changed caption", 500.0, || Ok([200.0, 20.0]))
        .unwrap();
    assert_eq!(cache.entries, 3);
    assert!(
        cache
            .measure("failed", 500.0, || Err(UiCompositionError::InvalidMetrics(
                "font"
            )))
            .is_err()
    );
    assert_eq!(cache.entries, 3);
    cache.measure("failed", 500.0, || Ok([10.0, 20.0])).unwrap();
    cache.entries = LIMIT;
    cache
        .measure("uncached", 500.0, || Ok([10.0, 20.0]))
        .unwrap();
    assert!(!cache.sizes.contains_key("uncached"));
    cache.entries = 4;
    cache.text_bytes = TEXT_BYTES_LIMIT;
    cache
        .measure("large key", 500.0, || Ok([10.0, 20.0]))
        .unwrap();
    assert!(!cache.sizes.contains_key("large key"));
    let mut fresh = MeasurementCache::default();
    assert_eq!(
        fresh
            .measure("caption", 500.0, || Ok([300.0, 30.0]))
            .unwrap(),
        [300.0, 30.0]
    );
}
