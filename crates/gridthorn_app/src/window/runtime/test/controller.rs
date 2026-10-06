use super::*;
use gridthorn_input::{InputEvent, InputState, controller::*};
#[test]
fn controller_inventory_precedes_startup_and_feedback_drains_once() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        assert_eq!(
            world.read_resource(|input: &InputState| input.controller_availability().cloned()),
            Some(Some(Ok(())))
        );
        world.update_resource(|feedback: &mut ControllerFeedback| {
            feedback.rumble(RumbleRequest {
                request: 7,
                id: ControllerId(99),
                strong: 0.2,
                weak: 0.0,
                duration_ms: 10,
            });
        });
    });
    let mut lifecycle = RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
    lifecycle
        .input(InputEvent::Controller(ControllerEvent::Ready))
        .unwrap();
    let mut control = WindowControl::default();
    lifecycle.started(&mut control).unwrap();
    assert_eq!(control.controller_feedback.len(), 1);
    lifecycle.idle(&mut WindowControl::default()).unwrap();
    lifecycle
        .input(InputEvent::Controller(ControllerEvent::Feedback {
            request: 7,
            id: ControllerId(99),
            result: Err(ControllerError::Disconnected),
        }))
        .unwrap();
    lifecycle.run_elapsed_frame(Duration::ZERO).unwrap();
    assert!(
        lifecycle
            .runtime
            .world()
            .read_resource(|input: &InputState| matches!(
                input.events(),
                [InputEvent::Controller(ControllerEvent::Feedback {
                    request: 7,
                    ..
                })]
            ))
            .unwrap()
    );
    let mut control = WindowControl::default();
    lifecycle.idle(&mut control).unwrap();
    assert_eq!(control.controller_feedback, []);
}

#[test]
fn idle_does_not_request_controllers_without_explicit_game_request() {
    let mut lifecycle =
        RuntimeWindowLifecycle::new(ApplicationRuntime::new(ScheduleBuilder::new().build()));
    let mut control = WindowControl::default();
    lifecycle.started(&mut control).unwrap();
    assert!(!control.poll_controllers);
    assert_eq!(
        lifecycle
            .runtime
            .world()
            .read_resource(|input: &InputState| input.controller_availability().cloned()),
        Some(None)
    );
    lifecycle.idle(&mut control).unwrap();
    assert!(!control.poll_controllers);
    lifecycle
        .runtime
        .world()
        .update_resource(ControllerPolling::request_poll);
    let mut requested = WindowControl::default();
    lifecycle.idle(&mut requested).unwrap();
    assert!(requested.poll_controllers);
    let mut next = WindowControl::default();
    lifecycle.idle(&mut next).unwrap();
    assert!(!next.poll_controllers);
}
