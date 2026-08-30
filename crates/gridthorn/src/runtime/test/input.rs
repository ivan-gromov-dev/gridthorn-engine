use crate::prelude::*;

#[test]
fn prelude_exposes_engine_owned_input_without_backend_types() {
    let input = InputState::default();

    assert!(!input.key_down(KeyCode::KeyW));
    assert!(!input.mouse_button_down(MouseButton::Left));
}
