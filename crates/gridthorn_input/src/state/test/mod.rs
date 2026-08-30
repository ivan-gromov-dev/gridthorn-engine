use crate::{ButtonState, CursorPosition, InputEvent, KeyCode, MouseButton};

use super::InputBuffer;

#[test]
fn preserves_held_state_and_clears_frame_edges() {
    let mut input = InputBuffer::new();
    input.push(InputEvent::Keyboard {
        key: KeyCode::KeyW,
        state: ButtonState::Pressed,
    });
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
    });
    input.push(InputEvent::CursorMoved(CursorPosition { x: 12.0, y: 34.0 }));

    let first = input.snapshot();
    let second = input.snapshot();

    assert!(first.key_down(KeyCode::KeyW));
    assert!(first.key_just_pressed(KeyCode::KeyW));
    assert!(first.mouse_button_down(MouseButton::Left));
    assert!(first.mouse_button_just_pressed(MouseButton::Left));
    assert_eq!(
        first.cursor_position(),
        Some(CursorPosition { x: 12.0, y: 34.0 })
    );
    assert!(second.key_down(KeyCode::KeyW));
    assert!(!second.key_just_pressed(KeyCode::KeyW));
    assert!(second.mouse_button_down(MouseButton::Left));
    assert!(!second.mouse_button_just_pressed(MouseButton::Left));
}

#[test]
fn focus_loss_releases_every_held_button() {
    let mut input = InputBuffer::new();
    input.push(InputEvent::Keyboard {
        key: KeyCode::ArrowLeft,
        state: ButtonState::Pressed,
    });
    input.push(InputEvent::MouseButton {
        button: MouseButton::Right,
        state: ButtonState::Pressed,
    });
    let _pressed = input.snapshot();

    input.push(InputEvent::FocusLost);
    let released = input.snapshot();

    assert!(!released.key_down(KeyCode::ArrowLeft));
    assert!(released.key_just_released(KeyCode::ArrowLeft));
    assert!(!released.mouse_button_down(MouseButton::Right));
    assert!(released.mouse_button_just_released(MouseButton::Right));
}
