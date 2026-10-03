use super::NativeTextInput;
use gridthorn_input::{ImeCursorArea, InputEvent, Modifiers, TextInputError, TextInputEvent};
use winit::event::{Ime, WindowEvent};

fn active() -> NativeTextInput {
    let mut text = NativeTextInput::default();
    text.request(Some(ImeCursorArea::new(0.0, 0.0, 1.0, 18.0).unwrap()), true);
    text
}

#[test]
fn ime_lifecycle_clears_without_duplicate_commits() {
    let mut text = active();
    assert_eq!(
        text.translate(&WindowEvent::Ime(Ime::Enabled)),
        vec![InputEvent::Text(TextInputEvent::ImeEnabled)]
    );
    let composition = text.translate(&WindowEvent::Ime(Ime::Preedit("日本".into(), Some((3, 6)))));
    assert_eq!(
        composition,
        vec![InputEvent::Text(TextInputEvent::Composition {
            text: "日本".into(),
            cursor: Some((3, 6))
        })]
    );
    assert_eq!(
        text.translate(&WindowEvent::Ime(Ime::Preedit(String::new(), None))),
        vec![InputEvent::Text(TextInputEvent::CompositionCancelled)]
    );
    assert_eq!(
        text.translate(&WindowEvent::Ime(Ime::Commit("日本".into()))),
        vec![InputEvent::Text(TextInputEvent::Commit("日本".into()))]
    );
    assert_eq!(
        text.translate(&WindowEvent::Ime(Ime::Disabled)),
        vec![InputEvent::Text(TextInputEvent::ImeDisabled)]
    );
}

#[test]
fn focus_and_explicit_stop_cancel_and_ignore_late_ime() {
    for focused in [true, false] {
        let mut text = active();
        text.translate(&WindowEvent::Ime(Ime::Preedit("你".into(), None)));
        let events = if focused {
            text.request(None, true)
        } else {
            text.translate(&WindowEvent::Focused(false))
        };
        assert_eq!(
            events[0],
            InputEvent::Text(TextInputEvent::CompositionCancelled)
        );
        assert!(!text.active());
        assert_eq!(
            text.translate(&WindowEvent::Ime(Ime::Commit("你".into()))),
            []
        );
        text.translate(&WindowEvent::Focused(true));
        assert!(!text.active());
    }
}

#[test]
fn unfocused_and_invalid_requests_preserve_state() {
    let mut text = NativeTextInput::default();
    let area = ImeCursorArea::new(1.0, 2.0, 0.0, 18.0).unwrap();
    assert_eq!(
        text.request(Some(area), false),
        vec![InputEvent::TextInputChanged {
            active: false,
            error: Some(TextInputError::Unfocused)
        }]
    );
    text.request(Some(area), true);
    assert_eq!(
        text.request(
            Some(ImeCursorArea {
                width: -1.0,
                ..area
            }),
            true
        ),
        vec![InputEvent::TextInputChanged {
            active: true,
            error: Some(TextInputError::InvalidCursorArea)
        }]
    );
}

#[test]
fn unicode_keyboard_text_preserves_sequences_but_excludes_shortcuts_and_controls() {
    assert_eq!(
        NativeTextInput::keyboard_text(Some("Йe\u{301}👩‍💻"), Modifiers::default()),
        Some("Йe\u{301}👩‍💻".into())
    );
    assert_eq!(
        NativeTextInput::keyboard_text(Some("\r\t\u{7f}"), Modifiers::default()),
        None
    );
    assert_eq!(
        NativeTextInput::keyboard_text(
            Some("c"),
            Modifiers {
                control: true,
                ..Modifiers::default()
            }
        ),
        None
    );
    assert_eq!(
        NativeTextInput::keyboard_text(
            Some("v"),
            Modifiers {
                super_key: true,
                ..Modifiers::default()
            }
        ),
        None
    );
    assert_eq!(
        NativeTextInput::keyboard_text(
            Some("€"),
            Modifiers {
                control: true,
                alt: true,
                ..Modifiers::default()
            }
        ),
        Some("€".into())
    );
}
#[test]
fn keyboard_delivery_repeats_but_never_duplicates_ime_or_synthetic_input() {
    use winit::event::ElementState;
    let mut text = active();
    for _ in 0..2 {
        assert_eq!(
            text.keyboard_commit(Some("й"), ElementState::Pressed, false),
            Some("й".into())
        );
    }
    assert_eq!(
        text.keyboard_commit(Some("й"), ElementState::Released, false),
        None
    );
    assert_eq!(
        text.keyboard_commit(Some("й"), ElementState::Pressed, true),
        None
    );
    text.translate(&WindowEvent::Ime(Ime::Enabled));
    assert_eq!(
        text.keyboard_commit(Some("你"), ElementState::Pressed, false),
        None
    );
    text.translate(&WindowEvent::Ime(Ime::Preedit("你".into(), None)));
    text.translate(&WindowEvent::Ime(Ime::Preedit(String::new(), None)));
    assert_eq!(
        text.keyboard_commit(Some("你"), ElementState::Pressed, false),
        None
    );
    text.translate(&WindowEvent::Ime(Ime::Commit("你".into())));
    assert_eq!(
        text.keyboard_commit(Some("你"), ElementState::Pressed, false),
        None
    );
    text.translate(&WindowEvent::Ime(Ime::Disabled));
    assert_eq!(
        text.keyboard_commit(Some("й"), ElementState::Pressed, false),
        Some("й".into())
    );
    text.request(None, true);
    assert_eq!(
        text.keyboard_commit(Some("й"), ElementState::Pressed, false),
        None
    );
}
