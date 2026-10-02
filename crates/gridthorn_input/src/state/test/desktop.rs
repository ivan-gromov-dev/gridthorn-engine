use crate::*;

fn key(state: ButtonState, repeat: bool) -> InputEvent {
    InputEvent::Key(KeyboardEvent {
        physical_key: PhysicalKey::Code(KeyCode::KeyQ),
        logical_key: LogicalKey::Character("й".into()),
        location: KeyLocation::Standard,
        state,
        repeat,
        synthetic: false,
    })
}

#[test]
fn ordered_events_preserve_repeat_layout_and_short_taps() {
    let mut buffer = InputBuffer::new();
    let events = vec![
        key(ButtonState::Pressed, false),
        key(ButtonState::Pressed, true),
        key(ButtonState::Released, false),
    ];
    for event in &events {
        buffer.push(event.clone());
    }
    let frame = buffer.snapshot();
    assert_eq!(frame.events(), events);
    assert!(frame.key_just_pressed(KeyCode::KeyQ));
    assert!(frame.key_just_released(KeyCode::KeyQ));
    assert!(!frame.key_down(KeyCode::KeyQ));
    assert_eq!(buffer.snapshot().events(), []);
}

#[test]
fn repeat_does_not_retrigger_edges() {
    let mut buffer = InputBuffer::new();
    buffer.push(key(ButtonState::Pressed, false));
    let _frame = buffer.snapshot();
    buffer.push(key(ButtonState::Pressed, true));
    let frame = buffer.snapshot();
    assert!(frame.key_down(KeyCode::KeyQ));
    assert!(!frame.key_just_pressed(KeyCode::KeyQ));
    assert_eq!(frame.events(), [key(ButtonState::Pressed, true)]);
}

#[test]
fn focus_cancels_native_keys_modifiers_cursor_and_capture() {
    let mut buffer = InputBuffer::new();
    let physical = PhysicalKey::Unidentified(NativeKey::Windows(777));
    buffer.push(InputEvent::FocusGained);
    buffer.push(InputEvent::Key(KeyboardEvent {
        physical_key: physical.clone(),
        logical_key: LogicalKey::Dead(Some('^')),
        location: KeyLocation::Right,
        state: ButtonState::Pressed,
        repeat: false,
        synthetic: true,
    }));
    buffer.push(InputEvent::ModifiersChanged(Modifiers {
        control: true,
        ..Modifiers::default()
    }));
    buffer.push(InputEvent::CursorMoved(CursorPosition { x: 10.0, y: 20.0 }));
    buffer.push(InputEvent::PointerCaptureChanged(PointerCaptureStatus {
        requested: PointerCaptureMode::Confined,
        effective: PointerCaptureMode::Confined,
        error: None,
    }));
    buffer.push(InputEvent::CursorLeft);
    let held = buffer.snapshot();
    assert!(held.physical_key_down(&physical));
    assert!(held.modifiers().control);
    assert!(held.cursor_position().is_some());
    buffer.push(InputEvent::FocusLost);
    let cancelled = buffer.snapshot();
    assert!(!cancelled.physical_key_down(&physical));
    assert_eq!(cancelled.modifiers(), Modifiers::default());
    assert_eq!(cancelled.pointer_capture(), PointerCaptureMode::None);
    assert!(!cancelled.focused());
    assert!(cancelled.cursor_position().is_none());
}

#[test]
fn wheel_units_phases_and_relative_motion_remain_ordered() {
    let mut buffer = InputBuffer::new();
    let events = [
        InputEvent::MouseWheel {
            delta: WheelDelta::Lines { x: -1.0, y: 2.0 },
            phase: ScrollPhase::Started,
        },
        InputEvent::MouseWheel {
            delta: WheelDelta::Pixels { x: 0.5, y: -2.75 },
            phase: ScrollPhase::Ended,
        },
        InputEvent::PointerMotion { x: -7.0, y: 3.0 },
    ];
    for event in &events {
        buffer.push(event.clone());
    }
    assert_eq!(buffer.snapshot().events(), events);
    assert_eq!(buffer.snapshot().events(), []);
}
