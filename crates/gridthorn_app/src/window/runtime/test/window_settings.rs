use super::*;
use crate::settings::{WindowOperation, WindowRequest, WindowResizePolicy, WindowSettings};

#[test]
fn startup_requests_are_coalesced_and_idle_does_not_repeat_them() {
    let mut schedules = ScheduleBuilder::new();
    let request = WindowRequest {
        resize_policy: Some(WindowResizePolicy::Fixed),
        ..WindowRequest::default()
    };
    schedules.add_system(ScheduleStage::Startup, move |world| {
        assert_eq!(
            world.read_resource(|settings: &WindowSettings| settings.capabilities().available),
            Some(false)
        );
        world.update_resource(|settings: &mut WindowSettings| {
            settings.request(request).unwrap();
            settings.request(request).unwrap();
        });
    });
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
    let mut control = WindowControl::default();
    lifecycle.started(&mut control).unwrap();
    assert_eq!(control.window_request, Some((2, request)));
    lifecycle.window_operation_changed(WindowOperation::Pending { id: 1 });
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(|settings: &WindowSettings| settings.feedback().cloned()),
        Some(Some(WindowOperation::Pending { id: 2 }))
    );
    for _ in 0..10 {
        let mut control = WindowControl::default();
        lifecycle.idle(&mut control).unwrap();
        assert!(control.window_request.is_none());
        assert!(!control.refresh_displays);
    }
}
