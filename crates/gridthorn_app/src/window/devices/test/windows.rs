use super::NativeInputDevices;
use gridthorn_input::{
    ButtonState, InputEvent, KeyCode, PhysicalKey,
    devices::{DeviceEvent, InputDeviceId},
};
use winit::event::{DeviceEvent as NativeEvent, DeviceId, ElementState, RawKeyEvent};

#[test]
fn observation_is_off_by_default_and_reactivation_invalidates_old_selections() {
    let mut devices = NativeInputDevices::default();
    let id = DeviceId::dummy();
    assert_eq!(devices.map(id, &NativeEvent::Added, true), []);
    assert_eq!(
        devices.map(
            id,
            &NativeEvent::Button {
                button: 0,
                state: ElementState::Pressed
            },
            true
        ),
        []
    );
    let _ = devices.set_enabled(true);
    assert_eq!(
        devices.map(id, &NativeEvent::Added, true),
        vec![InputEvent::Device(DeviceEvent::Connected(InputDeviceId(1)))]
    );
    let _ = devices.set_enabled(true);
    assert_eq!(devices.map(id, &NativeEvent::Added, true), []);
    assert_eq!(
        devices.set_enabled(false),
        InputEvent::RawInputChanged {
            enabled: false,
            error: None
        }
    );
    assert_eq!(devices.map(id, &NativeEvent::Added, true), []);
    assert_eq!(devices.map(id, &NativeEvent::Removed, true), []);
    let _ = devices.set_enabled(true);
    assert_eq!(
        devices.map(id, &NativeEvent::Added, true),
        vec![InputEvent::Device(DeviceEvent::Connected(InputDeviceId(2)))]
    );
}

#[test]
fn first_input_discovers_before_delivery_and_reconnect_never_reuses_identity() {
    let mut devices = NativeInputDevices::default();
    assert_eq!(
        devices.set_enabled(true),
        InputEvent::RawInputChanged {
            enabled: true,
            error: None
        }
    );
    let native = DeviceId::dummy();
    let key = NativeEvent::Key(RawKeyEvent {
        physical_key: winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyW),
        state: ElementState::Pressed,
    });
    let events = devices.map(native, &key, true);
    assert_eq!(
        events,
        vec![
            InputEvent::Device(DeviceEvent::Connected(InputDeviceId(1))),
            InputEvent::Device(DeviceEvent::Key {
                id: InputDeviceId(1),
                key: PhysicalKey::Code(KeyCode::KeyW),
                state: ButtonState::Pressed
            })
        ]
    );
    assert_eq!(devices.map(native, &NativeEvent::Added, true), vec![]);
    assert_eq!(devices.map(native, &key, true).len(), 1);
    assert_eq!(
        devices.map(native, &NativeEvent::Removed, false),
        vec![InputEvent::Device(DeviceEvent::Disconnected(
            InputDeviceId(1)
        ))]
    );
    assert_eq!(devices.map(native, &NativeEvent::Removed, false), []);
    assert_eq!(
        devices.map(native, &NativeEvent::Added, false),
        vec![InputEvent::Device(DeviceEvent::Connected(InputDeviceId(2)))]
    );
}

#[test]
fn unfocused_raw_input_is_ignored_but_connection_changes_are_not() {
    let mut devices = NativeInputDevices::default();
    let _ = devices.set_enabled(true);
    let id = DeviceId::dummy();
    let button = NativeEvent::Button {
        button: 5,
        state: ElementState::Pressed,
    };
    assert_eq!(devices.map(id, &button, false), []);
    assert_eq!(
        devices.map(id, &button, true),
        vec![
            InputEvent::Device(DeviceEvent::Connected(InputDeviceId(1))),
            InputEvent::Device(DeviceEvent::Button {
                id: InputDeviceId(1),
                button: 5,
                state: ButtonState::Pressed
            })
        ]
    );
    assert_eq!(
        devices.map(id, &NativeEvent::MouseMotion { delta: (1.0, 2.0) }, false),
        []
    );
    assert_eq!(
        devices.map(id, &NativeEvent::MouseMotion { delta: (1.0, 2.0) }, true),
        vec![InputEvent::Device(DeviceEvent::Motion {
            id: InputDeviceId(1),
            x: 1.0,
            y: 2.0
        })]
    );
    assert_eq!(
        devices.map(
            id,
            &NativeEvent::Motion {
                axis: 0,
                value: 1.0
            },
            true
        ),
        []
    );
    assert_eq!(
        devices.map(
            id,
            &NativeEvent::MouseWheel {
                delta: winit::event::MouseScrollDelta::LineDelta(0.0, 1.0)
            },
            true
        ),
        []
    );
}
