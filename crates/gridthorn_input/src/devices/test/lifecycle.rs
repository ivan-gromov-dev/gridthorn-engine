use crate::{ButtonState, InputBuffer, InputEvent, KeyCode, PhysicalKey, devices::*};

#[test]
fn separate_connections_retain_observed_capabilities_and_cancel_held_input() {
    let mut input = InputBuffer::new();
    input.push(InputEvent::RawInputChanged {
        enabled: true,
        error: None,
    });
    let a = InputDeviceId(1);
    let b = InputDeviceId(2);
    let key = PhysicalKey::Code(KeyCode::KeyA);
    for id in [a, b] {
        input.push(InputEvent::Device(DeviceEvent::Connected(id)));
    }
    input.push(InputEvent::Device(DeviceEvent::Key {
        id: a,
        key: key.clone(),
        state: ButtonState::Pressed,
    }));
    input.push(InputEvent::Device(DeviceEvent::Button {
        id: b,
        button: 42,
        state: ButtonState::Pressed,
    }));
    input.push(InputEvent::Device(DeviceEvent::Motion {
        id: b,
        x: 2.0,
        y: -3.0,
    }));
    let frame = input.snapshot();
    assert!(frame.device(a).unwrap().key_down(&key));
    assert!(!frame.device(b).unwrap().key_down(&key));
    assert!(frame.device(b).unwrap().button_down(42));
    assert!(!frame.key_down(KeyCode::KeyA));
    assert!(input.snapshot().device(a).unwrap().key_down(&key));
    input.push(InputEvent::FocusLost);
    let cancelled = input.snapshot();
    assert!(!cancelled.device(a).unwrap().key_down(&key));
    assert!(!cancelled.device(b).unwrap().button_down(42));
    assert!(cancelled.device(a).unwrap().keyboard_observed);
    assert!(cancelled.device(b).unwrap().motion_observed);
    input.push(InputEvent::Device(DeviceEvent::Disconnected(a)));
    input.push(InputEvent::Device(DeviceEvent::Key {
        id: a,
        key: key.clone(),
        state: ButtonState::Pressed,
    }));
    let removed = input.snapshot();
    assert!(removed.device(a).is_none());
    assert_eq!(removed.devices().count(), 1);
    assert!(frame.device(a).unwrap().key_down(&key));
}

#[test]
fn duplicate_arrival_preserves_state_and_new_connection_is_empty() {
    let mut input = InputBuffer::new();
    input.push(InputEvent::RawInputChanged {
        enabled: true,
        error: None,
    });
    let id = InputDeviceId(7);
    input.push(InputEvent::Device(DeviceEvent::Connected(id)));
    input.push(InputEvent::Device(DeviceEvent::Button {
        id,
        button: 0,
        state: ButtonState::Pressed,
    }));
    input.push(InputEvent::Device(DeviceEvent::Connected(id)));
    assert!(input.snapshot().device(id).unwrap().button_down(0));
    input.push(InputEvent::Device(DeviceEvent::Disconnected(id)));
    input.push(InputEvent::Device(DeviceEvent::Connected(InputDeviceId(8))));
    let frame = input.snapshot();
    assert!(frame.device(id).is_none());
    assert!(!frame.device(InputDeviceId(8)).unwrap().buttons_observed);
}
