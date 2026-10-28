use crate::actions::{ActionBinding, ActionBindings, ActionMapper, BindingInput};
use crate::devices::{DeviceEvent, InputDeviceId, RawInput};
use crate::{ButtonState, InputBuffer, InputEvent, KeyCode, PhysicalKey};

#[test]
fn requests_are_explicit_coalesced_and_drained_once() {
    let mut raw = RawInput::default();
    assert_eq!(raw.take_request(), None);
    raw.request(true);
    raw.request(false);
    assert_eq!(raw.take_request(), Some(false));
    assert_eq!(raw.take_request(), None);
    raw.request(true);
    assert_eq!(raw.take_request(), Some(true));
}

#[test]
fn stopping_clears_raw_state_cancels_actions_and_preserves_aggregate_input() {
    let mut input = InputBuffer::new();
    let id = InputDeviceId(1);
    input.push(InputEvent::Device(DeviceEvent::Connected(id)));
    assert!(!input.snapshot().raw_input_enabled());
    assert_eq!(input.snapshot().devices().count(), 0);
    input.push(InputEvent::RawInputChanged {
        enabled: true,
        error: None,
    });
    input.push(InputEvent::Device(DeviceEvent::Connected(id)));
    input.push(InputEvent::Device(DeviceEvent::Button {
        id,
        button: 0,
        state: ButtonState::Pressed,
    }));
    input.push(InputEvent::Keyboard {
        key: KeyCode::Space,
        state: ButtonState::Pressed,
    });
    input.push(InputEvent::PointerCaptureChanged(
        crate::PointerCaptureStatus {
            requested: crate::PointerCaptureMode::Locked,
            effective: crate::PointerCaptureMode::Locked,
            error: None,
        },
    ));
    let mut bindings = ActionBindings::default();
    bindings.set(
        "raw",
        vec![ActionBinding::new(BindingInput::DeviceButton {
            id,
            button: 0,
        })],
    );
    bindings.set(
        "aggregate",
        vec![ActionBinding::new(BindingInput::Key(PhysicalKey::Code(
            KeyCode::Space,
        )))],
    );
    let mut mapper = ActionMapper::new();
    assert!(
        mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"raw")
            .down
    );
    assert!(input.snapshot().raw_input_enabled());
    input.push(InputEvent::RawInputChanged {
        enabled: false,
        error: None,
    });
    let stopped = input.snapshot();
    let actions = mapper.evaluate(&bindings, &stopped);
    assert!(actions.get(&"raw").cancelled);
    assert!(!actions.get(&"raw").just_released && !actions.get(&"raw").down);
    assert!(actions.get(&"aggregate").down && !actions.get(&"aggregate").cancelled);
    assert_eq!(stopped.devices().count(), 0);
    assert!(stopped.key_down(KeyCode::Space));
    assert_eq!(stopped.pointer_capture(), crate::PointerCaptureMode::Locked);
    input.push(InputEvent::Device(DeviceEvent::Button {
        id,
        button: 0,
        state: ButtonState::Pressed,
    }));
    assert_eq!(input.snapshot().devices().count(), 0);
    input.push(InputEvent::RawInputChanged {
        enabled: true,
        error: None,
    });
    assert_eq!(input.snapshot().devices().count(), 0);
    input.push(InputEvent::Device(DeviceEvent::Connected(InputDeviceId(2))));
    assert!(
        !mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"raw")
            .down
    );
}
