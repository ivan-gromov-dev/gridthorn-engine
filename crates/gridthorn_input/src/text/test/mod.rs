use crate::{ImeCursorArea, InputBuffer, InputEvent, TextInput, TextInputError, TextInputEvent};

#[test]
fn unicode_composition_survives_frames_and_commit_is_one_shot() {
    let mut buffer = InputBuffer::new();
    buffer.push(InputEvent::Text(TextInputEvent::Composition {
        text: "日本".into(),
        cursor: Some((3, 6)),
    }));
    assert_eq!(
        buffer.snapshot().composition(),
        Some(("日本", Some((3, 6))))
    );
    assert_eq!(
        buffer.snapshot().composition(),
        Some(("日本", Some((3, 6))))
    );
    buffer.push(InputEvent::Text(TextInputEvent::Commit(
        "Й日本e\u{301}👩‍💻".into(),
    )));
    let state = buffer.snapshot();
    assert!(state.composition().is_none());
    assert_eq!(state.events().len(), 1);
    assert_eq!(buffer.snapshot().events(), []);
}

#[test]
fn focus_loss_cancels_preedit_and_session_without_commit() {
    let mut buffer = InputBuffer::new();
    buffer.push(InputEvent::TextInputChanged {
        active: true,
        error: None,
    });
    buffer.push(InputEvent::Text(TextInputEvent::Composition {
        text: "你".into(),
        cursor: None,
    }));
    let _ = buffer.snapshot();
    buffer.push(InputEvent::FocusLost);
    let frame = buffer.snapshot();
    assert!(!frame.text_input_active());
    assert!(frame.composition().is_none());
    assert_eq!(
        frame.events(),
        &[
            InputEvent::FocusLost,
            InputEvent::Text(TextInputEvent::CompositionCancelled)
        ]
    );
}

#[test]
fn invalid_cursor_offsets_are_hidden_and_disabled_clears_preedit() {
    let mut buffer = InputBuffer::new();
    buffer.push(InputEvent::Text(TextInputEvent::Composition {
        text: "é".into(),
        cursor: Some((1, usize::MAX)),
    }));
    assert_eq!(buffer.snapshot().composition(), Some(("é", None)));
    buffer.push(InputEvent::Text(TextInputEvent::ImeDisabled));
    assert!(buffer.snapshot().composition().is_none());
}

#[test]
fn text_requests_validate_and_last_request_wins() {
    let area = ImeCursorArea::new(10.0, 20.0, 0.0, 18.0).unwrap();
    let mut input = TextInput::default();
    input.start(area).unwrap();
    assert_eq!(
        input.start(ImeCursorArea {
            x: f64::NAN,
            ..area
        }),
        Err(TextInputError::InvalidCursorArea)
    );
    assert_eq!(
        input.take_request(),
        Some(crate::TextInputRequest::Start(area))
    );
    input.start(area).unwrap();
    input.stop();
    assert_eq!(input.take_request(), Some(crate::TextInputRequest::Stop));
    assert_eq!(input.take_request(), None);
    assert!(ImeCursorArea::new(0.0, 0.0, -1.0, 1.0).is_err());
}
