use super::*;

#[test]
fn advertised_mode_changes_preserve_identity_and_ignore_mode_order() {
    let mut catalog = DisplayCatalog::default();
    let initial = catalog.sample(vec![observation(1)], Some(&1));
    let id = initial.monitors()[0].id;
    let mut monitor = observation(1);
    let mut mode = monitor.1.modes[0];
    mode.refresh_rate_millihertz = 144_000;
    monitor.1.modes.insert(0, mode);
    let changed = catalog.sample(vec![monitor.clone()], Some(&1));
    assert_eq!(changed.changes(), &[DisplayChange::Changed(id)]);
    assert_eq!(
        changed.monitors()[0].modes[0].refresh_rate_millihertz,
        59940
    );
    monitor.1.modes.reverse();
    assert_eq!(catalog.sample(vec![monitor], Some(&1)).changes(), []);
}

#[test]
fn normalizes_modes_unknown_refresh_and_duplicate_observations() {
    let mut catalog = DisplayCatalog::default();
    let mut monitor = observation(1);
    monitor.1.refresh_rate_millihertz = Some(0);
    monitor.1.modes.push(monitor.1.modes[0]);
    let snapshot = catalog.sample(vec![monitor, observation(1)], Some(&99));
    assert_eq!(snapshot.monitors().len(), 1);
    assert_eq!(snapshot.monitors()[0].modes.len(), 1);
    assert_eq!(snapshot.monitors()[0].refresh_rate_millihertz, None);
    assert_eq!(snapshot.primary(), None);
    assert_eq!(
        Displays::default().availability(),
        DisplayAvailability::Unavailable
    );
}
