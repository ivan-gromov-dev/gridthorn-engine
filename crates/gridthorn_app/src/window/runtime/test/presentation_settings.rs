use super::*;
use crate::presentation::*;

#[test]
fn startup_collects_presentation_requests_without_changing_fixed_ticks() {
    let config = PresentationConfig {
        present_mode: PresentMode::Immediate,
        frame_rate_limit: Some(FrameRateLimit::new(30).unwrap()),
    };
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, move |world| {
        world.update_resource(|settings: &mut PresentationSettings| {
            settings.request(config).unwrap();
        });
        world.insert_resource(0_u32);
    });
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        world.update_resource(|ticks: &mut u32| *ticks += 1);
    });
    let fixed = FixedStepConfig::new(Duration::from_millis(10), 20).unwrap();
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::with_fixed_step(
        schedules.build(),
        fixed,
    ));
    let mut control = WindowControl::default();
    lifecycle.started(&mut control).unwrap();
    assert_eq!(control.presentation_request, Some((1, config)));
    lifecycle
        .run_elapsed_frame(Duration::from_millis(100))
        .unwrap();
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(|ticks: &u32| *ticks),
        Some(10)
    );
    let mut control = WindowControl::default();
    lifecycle.collect_display_requests(&mut control);
    assert!(control.presentation_request.is_none());
    let before = Instant::now();
    lifecycle.idle(&mut control).unwrap();
    assert!(control.wake_at.unwrap() <= before + Duration::from_millis(20));
    lifecycle.presentation_operation_changed(PresentationOperation::Failed {
        id: 1,
        error: PresentationError::Unavailable,
    });
    assert!(
        lifecycle
            .runtime
            .world()
            .read_resource(|settings: &PresentationSettings| matches!(
                settings.feedback(),
                Some(PresentationOperation::Failed { .. })
            ))
            .unwrap()
    );
}
