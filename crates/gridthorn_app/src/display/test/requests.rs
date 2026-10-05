use super::*;

#[test]
fn coalesces_queries_and_latest_selection_without_implicit_followup() {
    let mut displays = Displays::default();
    assert_eq!(displays.take_requests(), (false, None));
    displays.request_refresh();
    displays.request_refresh();
    displays.select_monitor(MonitorId(1, 1));
    displays.select_monitor(MonitorId(1, 2));
    assert_eq!(displays.take_requests(), (true, Some(MonitorId(1, 2))));
    assert_eq!(displays.take_requests(), (false, None));
    assert_eq!(
        displays.selection(),
        Some(&MonitorSelection::Pending {
            monitor: MonitorId(1, 2)
        })
    );
}

#[test]
fn inventory_response_preserves_queued_requests_and_selection_feedback() {
    let mut displays = Displays::default();
    displays.request_refresh();
    displays.select_monitor(MonitorId(1, 1));
    displays.publish_inventory(Displays {
        availability: DisplayAvailability::Available,
        ..Displays::default()
    });
    assert_eq!(displays.take_requests(), (true, Some(MonitorId(1, 1))));
    displays.publish_selection(MonitorSelection::Applied {
        monitor: MonitorId(1, 1),
    });
    assert_eq!(displays.active(), Some(MonitorId(1, 1)));
    let failed = MonitorSelection::Failed {
        monitor: MonitorId(1, 2),
        error: MonitorSelectionError::Unavailable {
            monitor: MonitorId(1, 2),
        },
    };
    displays.publish_selection(failed.clone());
    assert_eq!(displays.selection(), Some(&failed));
    assert_eq!(displays.active(), Some(MonitorId(1, 1)));
}
