use super::*;
use crate::controller::{ControllerButton, ControllerEvent, ControllerId, ControllerInfo};
use crate::devices::{DeviceEvent, InputDeviceId};

#[test]
fn disconnect_does_not_cancel_when_another_alternative_holds_the_action() {
    let (mut input, mut bindings, mut mapper) = map();
    input.push(InputEvent::RawInputChanged {
        enabled: true,
        error: None,
    });
    let id = InputDeviceId(9);
    bindings.set(
        "jump",
        vec![
            key_binding(KeyCode::Space),
            ActionBinding::new(BindingInput::DeviceButton { id, button: 1 }),
        ],
    );
    input.push(InputEvent::Device(DeviceEvent::Connected(id)));
    input.push(InputEvent::Device(DeviceEvent::Button {
        id,
        button: 1,
        state: ButtonState::Pressed,
    }));
    key(&mut input, KeyCode::Space, ButtonState::Pressed);
    let _ = mapper.evaluate(&bindings, &input.snapshot());
    input.push(InputEvent::Device(DeviceEvent::Disconnected(id)));
    let held = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(held.down && !held.cancelled && !held.just_released && !held.just_pressed);
    key(&mut input, KeyCode::Space, ButtonState::Released);
    assert!(
        mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"jump")
            .just_released
    );
}

#[test]
fn selected_raw_connection_cannot_be_triggered_by_another_or_the_aggregate() {
    let (mut input, mut bindings, mut mapper) = map();
    input.push(InputEvent::RawInputChanged {
        enabled: true,
        error: None,
    });
    let selected = InputDeviceId(1);
    let other = InputDeviceId(2);
    bindings.set(
        "jump",
        vec![ActionBinding::new(BindingInput::DeviceKey {
            id: selected,
            key: PhysicalKey::Code(KeyCode::Space),
        })],
    );
    for id in [selected, other] {
        input.push(InputEvent::Device(DeviceEvent::Connected(id)));
    }
    key(&mut input, KeyCode::Space, ButtonState::Pressed);
    input.push(InputEvent::Device(DeviceEvent::Key {
        id: other,
        key: PhysicalKey::Code(KeyCode::Space),
        state: ButtonState::Pressed,
    }));
    assert!(
        !mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"jump")
            .down
    );
    input.push(InputEvent::Device(DeviceEvent::Key {
        id: selected,
        key: PhysicalKey::Code(KeyCode::Space),
        state: ButtonState::Pressed,
    }));
    assert!(
        mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"jump")
            .just_pressed
    );
    input.push(InputEvent::Device(DeviceEvent::Disconnected(selected)));
    let cancelled = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(cancelled.cancelled && !cancelled.just_released && !cancelled.down);
    assert!(
        bindings.get(&"jump")[0]
            .input
            .source()
            .validate(&input.snapshot())
            .is_err()
    );
}

#[test]
fn raw_button_tap_and_controller_disconnect_use_the_same_action_contract() {
    let (mut input, mut bindings, mut mapper) = map();
    input.push(InputEvent::RawInputChanged {
        enabled: true,
        error: None,
    });
    let raw = InputDeviceId(3);
    let controller = ControllerId(1);
    bindings.set(
        "jump",
        vec![
            ActionBinding::new(BindingInput::DeviceButton { id: raw, button: 8 }),
            ActionBinding::new(BindingInput::ControllerButton {
                id: controller,
                button: ControllerButton::South,
            }),
        ],
    );
    input.push(InputEvent::Device(DeviceEvent::Connected(raw)));
    for state in [ButtonState::Pressed, ButtonState::Released] {
        input.push(InputEvent::Device(DeviceEvent::Button {
            id: raw,
            button: 8,
            state,
        }));
    }
    let tap = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(tap.just_pressed && tap.just_released && !tap.down);
    input.push(InputEvent::Controller(ControllerEvent::Connected(
        ControllerInfo {
            id: controller,
            name: "test".into(),
            model_uuid: [0; 16],
            vendor_id: None,
            product_id: None,
            buttons: vec![ControllerButton::South],
            axes: vec![],
            rumble_supported: false,
        },
    )));
    input.push(InputEvent::Controller(ControllerEvent::Button {
        id: controller,
        button: ControllerButton::South,
        state: ButtonState::Pressed,
    }));
    assert!(
        mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"jump")
            .down
    );
    assert!(
        bindings.get(&"jump")[1]
            .input
            .source()
            .validate(&input.snapshot())
            .is_ok()
    );
    input.push(InputEvent::Controller(ControllerEvent::Disconnected(
        controller,
    )));
    let cancelled = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(cancelled.cancelled && !cancelled.just_released);
}
