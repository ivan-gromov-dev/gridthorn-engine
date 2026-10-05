use super::*;

#[test]
fn reports_primary_switch_without_monitor_property_changes() {
    fn assert_owned_thread_safe<T: Send + Sync>() {}
    let mut catalog = DisplayCatalog::default();
    let initial = catalog.sample(vec![observation(1), observation(2)], Some(&1));
    let next = catalog.sample(vec![observation(1), observation(2)], Some(&2));
    assert_eq!(
        next.changes(),
        &[DisplayChange::PrimaryChanged(Some(
            initial.monitors()[1].id
        ))]
    );
    assert_owned_thread_safe::<Displays>();
}

#[test]
fn tracks_changes_disconnects_reconnects_and_foreign_ids() {
    let mut catalog = DisplayCatalog::default();
    let initial = catalog.sample(vec![observation(1), observation(2)], Some(&1));
    let id = initial.monitors()[0].id;
    let other = initial.monitors()[1].id;
    assert_ne!(id, other);
    assert_eq!(initial.primary(), Some(id));
    assert_eq!(
        initial.changes(),
        &[
            DisplayChange::Connected(id),
            DisplayChange::Connected(other),
            DisplayChange::PrimaryChanged(Some(id))
        ]
    );
    let unchanged = catalog.sample(vec![observation(2), observation(1)], Some(&1));
    assert_eq!(unchanged.changes(), []);
    assert_eq!(unchanged.revision(), initial.revision() + 1);
    assert_eq!(unchanged.monitors(), initial.monitors());
    let mut changed = observation(1);
    changed.1.scale_factor = 2.0;
    changed.1.resolution.width = 2560;
    changed.1.refresh_rate_millihertz = None;
    let snapshot = catalog.sample(vec![changed], None);
    assert_eq!(
        snapshot.changes(),
        &[
            DisplayChange::Changed(id),
            DisplayChange::Disconnected(other),
            DisplayChange::PrimaryChanged(None)
        ]
    );
    assert!(snapshot.monitor(other).is_none());
    assert!(catalog.key(other).is_none());
    assert_eq!(catalog.id(&1), Some(id));
    assert_eq!(catalog.key(id), Some(&1));
    let empty = catalog.sample(vec![], None);
    assert_eq!(empty.availability(), DisplayAvailability::Available);
    assert_eq!(empty.changes(), &[DisplayChange::Disconnected(id)]);
    let reconnect = catalog.sample(vec![observation(1)], Some(&1));
    assert_ne!(reconnect.monitors()[0].id, id);
    let foreign = DisplayCatalog::default().sample(vec![observation(1)], Some(&1));
    assert!(reconnect.monitor(foreign.monitors()[0].id).is_none());
}
