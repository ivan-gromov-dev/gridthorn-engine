use gridthorn_input::{ButtonState, MouseButton as EngineMouseButton};
use winit::event::{ElementState, MouseButton};

use super::{map_button_state, map_mouse_button};

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
mod coverage;
mod logical;
mod pointer;
