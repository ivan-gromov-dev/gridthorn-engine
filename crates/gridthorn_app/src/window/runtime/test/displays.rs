use super::*;
use crate::display::{DisplayAvailability, DisplayChange, Displays, MonitorId, MonitorSelection};

#[test]
fn does_not_request_inventory_until_game_asks_and_dispatches_only_once() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        assert_eq!(
            world.read_resource(|displays: &Displays| displays.availability()),
            Some(DisplayAvailability::Unavailable)
        );
    });
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
    let mut control = WindowControl::default();
    lifecycle.started(&mut control).unwrap();
    assert!(!control.refresh_displays);
    for _ in 0..10 {
        let mut control = WindowControl::default();
        lifecycle.idle(&mut control).unwrap();
        assert!(!control.refresh_displays);
        assert!(control.selected_monitor.is_none());
    }
    lifecycle
        .runtime
        .world()
        .update_resource(|displays: &mut Displays| {
            displays.request_refresh();
            displays.request_refresh();
        });
    let mut control = WindowControl::default();
    lifecycle.idle(&mut control).unwrap();
    assert!(control.refresh_displays);
    let mut control = WindowControl::default();
    lifecycle.idle(&mut control).unwrap();
    assert!(!control.refresh_displays);
}

#[test]
fn dispatches_startup_request_and_preserves_frame_changes_and_selection() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.update_resource(Displays::request_refresh);
    });
    schedules.add_system(ScheduleStage::Update, |world| {
        let changes = world
            .read_resource(|displays: &Displays| displays.changes().len())
            .unwrap();
        world.insert_resource(changes);
    });
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
    let mut control = WindowControl::default();
    lifecycle.started(&mut control).unwrap();
    assert!(control.refresh_displays);
    let id = MonitorId(42, 1);
    let mut displays = Displays {
        availability: DisplayAvailability::Available,
        ..Displays::default()
    };
    displays.changes.push(DisplayChange::Connected(id));
    lifecycle.displays_changed(displays);
    lifecycle
        .runtime
        .world()
        .update_resource(|displays: &mut Displays| displays.select_monitor(id));
    let mut control = WindowControl::default();
    lifecycle.idle(&mut control).unwrap();
    assert_eq!(control.selected_monitor, Some(id));
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(|count: &usize| *count),
        Some(1)
    );
    lifecycle.monitor_selection_changed(MonitorSelection::Applied { monitor: id });
    lifecycle.displays_changed(Displays {
        active: Some(id),
        availability: DisplayAvailability::Available,
        ..Displays::default()
    });
    let mut control = WindowControl::default();
    lifecycle.idle(&mut control).unwrap();
    assert!(!control.refresh_displays);
    assert!(control.selected_monitor.is_none());
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(|count: &usize| *count),
        Some(0)
    );
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(|displays: &Displays| displays.selection().cloned()),
        Some(Some(MonitorSelection::Applied { monitor: id }))
    );
}
