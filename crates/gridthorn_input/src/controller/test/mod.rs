use crate::controller::*;
use crate::{ButtonState, InputBuffer, InputEvent};
fn info(id: ControllerId) -> ControllerInfo {
    ControllerInfo {
        id,
        name: "same model".into(),
        model_uuid: [7; 16],
        vendor_id: None,
        product_id: None,
        buttons: vec![ControllerButton::South],
        axes: vec![ControllerAxis::LeftStickX],
        rumble_supported: false,
    }
}
fn push(buffer: &mut InputBuffer, event: ControllerEvent) {
    buffer.push(InputEvent::Controller(event));
}
#[test]
fn independent_connections_ordered_taps_and_frame_edges() {
    let mut buffer = InputBuffer::new();
    let a = ControllerId(1);
    let b = ControllerId(2);
    push(&mut buffer, ControllerEvent::Connected(info(a)));
    push(&mut buffer, ControllerEvent::Connected(info(b)));
    for state in [
        ButtonState::Pressed,
        ButtonState::Pressed,
        ButtonState::Released,
    ] {
        push(
            &mut buffer,
            ControllerEvent::Button {
                id: a,
                button: ControllerButton::South,
                state,
            },
        );
    }
    push(
        &mut buffer,
        ControllerEvent::Axis {
            id: b,
            axis: ControllerAxis::LeftStickX,
            value: 0.75,
        },
    );
    let frame = buffer.snapshot();
    assert_eq!(frame.events().len(), 6);
    let state = frame.controller(a).unwrap();
    assert!(state.button_just_pressed(ControllerButton::South));
    assert!(state.button_just_released(ControllerButton::South));
    assert!(!state.button_down(ControllerButton::South));
    let next = buffer.snapshot();
    assert!(
        !next
            .controller(a)
            .unwrap()
            .button_just_pressed(ControllerButton::South)
    );
    assert_eq!(
        next.controller(b).unwrap().axis(ControllerAxis::LeftStickX),
        0.75
    );
    assert!(
        frame
            .controller(a)
            .unwrap()
            .button_just_pressed(ControllerButton::South)
    );
    push(&mut buffer, ControllerEvent::Disconnected(a));
    push(
        &mut buffer,
        ControllerEvent::Button {
            id: a,
            button: ControllerButton::South,
            state: ButtonState::Pressed,
        },
    );
    let frame = buffer.snapshot();
    assert!(frame.controller(a).is_none());
    assert_eq!(frame.controllers().count(), 1);
}
#[test]
fn focus_cancels_controls_preserving_inventory_and_invalid_samples_are_ignored() {
    let mut buffer = InputBuffer::new();
    let id = ControllerId(4);
    push(&mut buffer, ControllerEvent::Connected(info(id)));
    push(
        &mut buffer,
        ControllerEvent::Button {
            id,
            button: ControllerButton::South,
            state: ButtonState::Pressed,
        },
    );
    push(
        &mut buffer,
        ControllerEvent::Axis {
            id,
            axis: ControllerAxis::LeftStickX,
            value: 0.5,
        },
    );
    push(
        &mut buffer,
        ControllerEvent::Axis {
            id,
            axis: ControllerAxis::LeftStickX,
            value: f32::NAN,
        },
    );
    assert_eq!(
        buffer
            .snapshot()
            .controller(id)
            .unwrap()
            .axis(ControllerAxis::LeftStickX),
        0.5
    );
    buffer.push(InputEvent::FocusLost);
    let frame = buffer.snapshot();
    let state = frame.controller(id).unwrap();
    assert!(state.button_just_released(ControllerButton::South));
    assert!(!state.button_down(ControllerButton::South));
    assert_eq!(state.axis(ControllerAxis::LeftStickX), 0.0);
}
#[test]
fn dead_zones_validate_rescale_and_preserve_direction() {
    for invalid in [f32::NAN, f32::INFINITY, -0.1, 1.0] {
        assert!(DeadZone::new(invalid).is_err());
    }
    let zone = DeadZone::new(0.2).unwrap();
    assert_eq!(zone.axial(0.1), 0.0);
    assert!((zone.axial(-0.6) + 0.5).abs() < 0.0001);
    assert_eq!(zone.axial(2.0), 1.0);
    assert_eq!(zone.axial(f32::NAN), 0.0);
    assert_eq!(zone.radial([0.1, 0.1]), [0.0; 2]);
    let radial = zone.radial([1.0, 1.0]);
    assert!((radial[0].hypot(radial[1]) - 1.0).abs() < 0.0001);
    assert_eq!(radial[0], radial[1]);
    let extreme = zone.radial([f32::MAX, f32::MAX]);
    assert!((extreme[0].hypot(extreme[1]) - 1.0).abs() < 0.0001);
}
#[test]
fn feedback_validation_and_one_shot_order() {
    let valid = RumbleRequest {
        request: 10,
        id: ControllerId(1),
        strong: 0.5,
        weak: 1.0,
        duration_ms: 120,
    };
    assert!(valid.validate().is_ok());
    for bad in [
        RumbleRequest {
            strong: f32::NAN,
            ..valid
        },
        RumbleRequest {
            weak: -0.1,
            ..valid
        },
        RumbleRequest {
            duration_ms: 60_001,
            ..valid
        },
    ] {
        assert_eq!(bad.validate(), Err(ControllerError::InvalidParameters));
    }
    let mut queue = ControllerFeedback::default();
    queue.rumble(valid);
    queue.rumble(RumbleRequest {
        request: 11,
        duration_ms: 0,
        ..valid
    });
    assert_eq!(
        queue
            .take_requests()
            .iter()
            .map(|r| r.request)
            .collect::<Vec<_>>(),
        [10, 11]
    );
    assert_eq!(queue.take_requests(), []);
}

#[test]
fn polling_requests_are_explicit_coalesced_and_drained_once() {
    let mut polling = ControllerPolling::default();
    assert!(!polling.take_request());
    polling.request_poll();
    polling.request_poll();
    assert!(polling.take_request());
    assert!(!polling.take_request());
}
