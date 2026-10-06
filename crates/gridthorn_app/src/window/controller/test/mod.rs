use super::*;
#[test]
fn rumble_validation_precedes_device_lookup_and_disconnected_feedback_is_correlated() {
    let mut native = NativeControllers::default();
    let request = RumbleRequest {
        request: 42,
        id: ControllerId(8),
        strong: 0.5,
        weak: 0.5,
        duration_ms: 100,
    };
    assert_eq!(
        native.feedback(request),
        InputEvent::Controller(ControllerEvent::Feedback {
            request: 42,
            id: ControllerId(8),
            result: Err(ControllerError::Disconnected)
        })
    );
    assert_eq!(
        native.feedback(RumbleRequest {
            strong: f32::NAN,
            ..request
        }),
        InputEvent::Controller(ControllerEvent::Feedback {
            request: 42,
            id: ControllerId(8),
            result: Err(ControllerError::InvalidParameters)
        })
    );
    native.stop();
    assert!(native.effects.is_empty());
    assert_eq!(
        native.feedback_focused(request, false),
        InputEvent::Controller(ControllerEvent::Feedback {
            request: 42,
            id: ControllerId(8),
            result: Err(ControllerError::Unfocused)
        })
    );
}
#[test]
#[ignore = "requires native controller services; run explicitly"]
fn native_controller_inventory_initializes_once() {
    let mut native = NativeControllers::default();
    let events = native.poll();
    println!("controller discovery: {events:?}");
    assert!(events.contains(&InputEvent::Controller(ControllerEvent::Ready)));
    assert!(
        !native
            .poll()
            .contains(&InputEvent::Controller(ControllerEvent::Ready))
    );
    native.stop();
}
