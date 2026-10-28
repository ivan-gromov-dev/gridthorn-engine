use super::*;
use crate::{Modifiers, MouseButton};

#[test]
fn short_tap_retains_both_edges_and_repeat_is_not_a_press() {
    let (mut input, bindings, mut mapper) = map();
    key(&mut input, KeyCode::Space, ButtonState::Pressed);
    let first = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(first.down && first.just_pressed && !first.just_released);
    key(&mut input, KeyCode::Space, ButtonState::Pressed);
    let repeat = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(repeat.down && !repeat.just_pressed);
    key(&mut input, KeyCode::Space, ButtonState::Released);
    let released = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(!released.down && released.just_released);
    key(&mut input, KeyCode::Space, ButtonState::Pressed);
    key(&mut input, KeyCode::Space, ButtonState::Released);
    let tap = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(!tap.down && tap.just_pressed && tap.just_released && !tap.cancelled);
    assert_eq!(
        mapper.evaluate(&bindings, &input.snapshot()).get(&"jump"),
        crate::actions::ActionState::default()
    );
}

#[test]
fn alternatives_hold_until_last_release_and_modifier_changes_are_ordered() {
    let (mut input, mut bindings, mut mapper) = map();
    bindings.set(
        "jump",
        vec![
            key_binding(KeyCode::Space),
            ActionBinding::new(BindingInput::MouseButton(MouseButton::Left)),
        ],
    );
    key(&mut input, KeyCode::Space, ButtonState::Pressed);
    input.push(InputEvent::MouseButton {
        button: MouseButton::Left,
        state: ButtonState::Pressed,
    });
    let _ = mapper.evaluate(&bindings, &input.snapshot());
    key(&mut input, KeyCode::Space, ButtonState::Released);
    let held = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(held.down && !held.just_released && !held.just_pressed);
    bindings.set(
        "jump",
        vec![
            ActionBinding::new(BindingInput::MouseButton(MouseButton::Left)).with_modifiers(
                Modifiers {
                    control: true,
                    ..Modifiers::default()
                },
            ),
        ],
    );
    let rebound = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(!rebound.down && rebound.just_released);
    input.push(InputEvent::ModifiersChanged(Modifiers {
        control: true,
        ..Modifiers::default()
    }));
    input.push(InputEvent::ModifiersChanged(Modifiers::default()));
    let chord = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(!chord.down && chord.just_pressed && chord.just_released);
    input.push(InputEvent::ModifiersChanged(Modifiers {
        control: true,
        shift: true,
        ..Modifiers::default()
    }));
    assert!(
        !mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"jump")
            .down
    );
}

#[test]
fn rebind_and_remove_apply_against_held_input_without_stuck_actions() {
    let (mut input, mut bindings, mut mapper) = map();
    key(&mut input, KeyCode::Space, ButtonState::Pressed);
    let _ = mapper.evaluate(&bindings, &input.snapshot());
    bindings.set("jump", vec![key_binding(KeyCode::KeyJ)]);
    assert!(
        mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"jump")
            .just_released
    );
    key(&mut input, KeyCode::KeyJ, ButtonState::Pressed);
    assert!(
        mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"jump")
            .just_pressed
    );
    assert_eq!(bindings.get(&"jump").len(), 1);
    bindings.remove(&"jump");
    assert_eq!(bindings.get(&"jump"), []);
    assert!(
        mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"jump")
            .just_released
    );
    assert_eq!(
        mapper.evaluate(&bindings, &input.snapshot()).iter().count(),
        0
    );
}

#[test]
fn focus_loss_cancels_instead_of_completing_click_and_new_mapper_recovers_held_state() {
    let (mut input, bindings, mut mapper) = map();
    key(&mut input, KeyCode::Space, ButtonState::Pressed);
    let _ = input.snapshot();
    assert!(
        mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"jump")
            .down
    );
    input.push(InputEvent::FocusLost);
    let cancelled = mapper.evaluate(&bindings, &input.snapshot()).get(&"jump");
    assert!(cancelled.cancelled && !cancelled.down && !cancelled.just_released);
    input.push(InputEvent::FocusGained);
    assert!(
        !mapper
            .evaluate(&bindings, &input.snapshot())
            .get(&"jump")
            .down
    );
}
