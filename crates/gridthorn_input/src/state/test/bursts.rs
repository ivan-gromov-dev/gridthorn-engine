use crate::{ButtonState, InputEvent, KeyCode, TextInputEvent};

use super::super::InputBuffer;

#[test]
fn large_ordered_burst_snapshots_remain_independent_across_focus_cancellation() {
    let mut buffer = InputBuffer::new();
    let events = (0..4096)
        .map(|index| match index % 3 {
            0 => InputEvent::Text(TextInputEvent::Commit(format!(
                "{index} Привет 日本語 e\u{301}"
            ))),
            1 => InputEvent::PointerMotion {
                x: f64::from(index),
                y: -1.0,
            },
            _ => InputEvent::Keyboard {
                key: KeyCode::KeyD,
                state: ButtonState::Pressed,
            },
        })
        .collect::<Vec<_>>();
    for event in &events {
        buffer.push(event.clone());
    }
    buffer.push(InputEvent::Text(TextInputEvent::Composition {
        text: "日本語".into(),
        cursor: Some((0, 9)),
    }));
    let first = buffer.snapshot();
    assert_eq!(&first.events()[..events.len()], events);
    assert!(first.key_down(KeyCode::KeyD));
    assert_eq!(first.composition(), Some(("日本語", Some((0, 9)))));
    let held = buffer.snapshot();
    assert_eq!(held.events(), []);
    assert!(held.key_down(KeyCode::KeyD));
    assert!(!held.key_just_pressed(KeyCode::KeyD));
    assert_eq!(held.composition(), first.composition());
    buffer.push(InputEvent::FocusLost);
    buffer.push(InputEvent::Text(TextInputEvent::Commit(
        "after cancellation".into(),
    )));
    let cancelled = buffer.snapshot();
    assert_eq!(
        cancelled.events(),
        [
            InputEvent::FocusLost,
            InputEvent::Text(TextInputEvent::CompositionCancelled),
            InputEvent::Text(TextInputEvent::Commit("after cancellation".into())),
        ]
    );
    assert!(!cancelled.key_down(KeyCode::KeyD));
    assert!(cancelled.key_just_released(KeyCode::KeyD));
    assert_eq!(cancelled.composition(), None);
    assert_eq!(first.events().len(), events.len() + 1);
    assert!(first.key_down(KeyCode::KeyD));
    assert_eq!(first.composition(), Some(("日本語", Some((0, 9)))));
    assert_eq!(held.composition(), first.composition());
    assert_eq!(buffer.snapshot().events(), []);
}
