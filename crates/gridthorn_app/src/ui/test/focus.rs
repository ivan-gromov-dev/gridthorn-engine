use super::*;

#[test]
fn focus_loss_cannot_activate_a_button_when_cursor_returns_in_the_same_frame() {
    let mut button = UiButton::new([0.0, 0.0], [30.0, 30.0]).unwrap();
    let mut input = InputBuffer::new();
    input.push(InputEvent::CursorMoved(CursorPosition { x: 10.0, y: 10.0 }));
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
    });
    assert!(button.update(&input.snapshot()).pressed());
    input.push(InputEvent::FocusLost);
    input.push(InputEvent::FocusGained);
    input.push(InputEvent::CursorMoved(CursorPosition { x: 10.0, y: 10.0 }));
    let frame = input.snapshot();
    assert!(frame.mouse_button_just_released(MouseButton::Left));
    assert!(!button.update(&frame).activated());
    assert!(!button.update(&input.snapshot()).pressed());
}
