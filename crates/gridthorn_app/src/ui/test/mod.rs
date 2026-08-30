use gridthorn_input::{ButtonState, CursorPosition, InputBuffer, InputEvent, MouseButton};

use super::{UiButton, UiButtonError};

#[test]
fn activates_once_after_an_inside_press_and_release() {
    let mut button = UiButton::new([10.0, 20.0], [100.0, 40.0]).expect("button should be valid");
    let mut input = InputBuffer::new();
    input.push(InputEvent::CursorMoved(CursorPosition { x: 30.0, y: 40.0 }));
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
    });

    let pressed = button.update(&input.snapshot());
    assert!(pressed.hovered());
    assert!(pressed.pressed());
    assert!(!pressed.activated());

    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Released,
    });
    let released = button.update(&input.snapshot());
    assert!(released.activated());
    assert!(!released.pressed());

    assert!(!button.update(&input.snapshot()).activated());
}

#[test]
fn cancels_activation_when_press_begins_or_ends_outside() {
    let mut button = UiButton::new([10.0, 10.0], [20.0, 20.0]).expect("button should be valid");
    let mut input = InputBuffer::new();
    input.push(InputEvent::CursorMoved(CursorPosition { x: 0.0, y: 0.0 }));
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
    });
    button.update(&input.snapshot());
    input.push(InputEvent::CursorMoved(CursorPosition { x: 15.0, y: 15.0 }));
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Released,
    });
    assert!(!button.update(&input.snapshot()).activated());

    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
    });
    button.update(&input.snapshot());
    input.push(InputEvent::CursorLeft);
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Released,
    });
    assert!(!button.update(&input.snapshot()).activated());
}

#[test]
fn rejects_invalid_bounds() {
    assert_eq!(
        UiButton::new([0.0, 0.0], [-1.0, 10.0]).err(),
        Some(UiButtonError::InvalidSize)
    );
}
