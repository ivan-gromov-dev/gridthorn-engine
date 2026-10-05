use super::*;

#[test]
fn diagnostics_bound_storage_and_skip_busy_collection() {
    let profile = LayoutPerformance {
        samples: Mutex::new(Vec::new()),
        skipped: AtomicUsize::new(0),
    };
    let disabled = LayoutSample::new(false);
    assert!(disabled.start().is_none());
    for _ in 0..=LIMIT {
        profile.record(&disabled);
    }
    let held = profile.samples.lock().unwrap();
    assert_eq!(held.len(), LIMIT);
    profile.record(&disabled);
    assert_eq!(profile.skipped.load(Ordering::Relaxed), 1);
}
