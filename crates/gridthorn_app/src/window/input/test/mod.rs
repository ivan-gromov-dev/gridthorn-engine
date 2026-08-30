use gridthorn_input::{ButtonState, KeyCode as EngineKeyCode, MouseButton as EngineMouseButton};
use winit::event::{ElementState, MouseButton};
use winit::keyboard::KeyCode;

use super::{map_button_state, map_key_code, map_mouse_button};

#[test]
fn maps_supported_physical_keys_without_exposing_winit_types() {
    assert_eq!(map_key_code(KeyCode::KeyW), Some(EngineKeyCode::KeyW));
    assert_eq!(
        map_key_code(KeyCode::ArrowRight),
        Some(EngineKeyCode::ArrowRight)
    );
    assert_eq!(map_key_code(KeyCode::F12), None);
}

#[test]
fn maps_mouse_buttons_and_digital_state() {
    assert_eq!(
        map_mouse_button(MouseButton::Other(7)),
        EngineMouseButton::Other(7)
    );
    assert_eq!(
        map_button_state(ElementState::Pressed),
        ButtonState::Pressed
    );
    assert_eq!(
        map_button_state(ElementState::Released),
        ButtonState::Released
    );
}
