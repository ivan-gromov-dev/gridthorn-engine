use super::*;
use gridthorn_input::TextInputEvent;
fn commit(text: &str) -> InputEvent {
    InputEvent::Text(TextInputEvent::Commit(text.into()))
}

#[test]
fn selection_replaces_graphemes_and_backspace_preserves_emoji_and_combining_sequences() {
    let mut tree = field("e\u{301}👨‍👩‍👧‍👦Я");
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    router
        .route_events(&mut tree, &layout, &[press(KeyCode::Backspace)])
        .unwrap();
    assert_eq!(value(&tree), "e\u{301}👨‍👩‍👧‍👦");
    router
        .route_events(&mut tree, &layout, &[press(KeyCode::Backspace)])
        .unwrap();
    assert_eq!(value(&tree), "e\u{301}");
    assert!(
        router
            .select(
                &tree,
                UiSelection {
                    anchor: 1,
                    caret: 3
                }
            )
            .is_err()
    );
    router
        .select(
            &tree,
            UiSelection {
                anchor: 3,
                caret: 0,
            },
        )
        .unwrap();
    let result = router
        .route_events(&mut tree, &layout, &[commit("日本語")])
        .unwrap();
    assert_eq!(value(&tree), "日本語");
    assert_eq!(result.effects, [(UiNodeId(1), UiEffect::Changed)]);
    assert_eq!(router.selection(&tree).unwrap().caret, "日本語".len());
}

#[test]
fn grapheme_merging_insertion_never_leaves_caret_inside_a_cluster() {
    let mut tree = field("e");
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    router
        .route_events(
            &mut tree,
            &layout,
            &[commit("\u{301}"), press(KeyCode::ArrowLeft)],
        )
        .unwrap();
    assert_eq!(router.selection(&tree).unwrap().caret, 0);
    router
        .route_events(&mut tree, &layout, &[press(KeyCode::Delete)])
        .unwrap();
    assert_eq!(value(&tree), "");
}

#[test]
fn composition_does_not_edit_value_or_interpret_editing_keys_before_commit() {
    let mut tree = field("hello");
    let layout = tree.layout([200.0; 2], 2.0, None).unwrap();
    let mut router = UiRouter::new(100);
    let focus = router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    assert!(focus.platform.iter().any(|request| matches!(request, UiPlatformRequest::Text(gridthorn_input::TextInputRequest::Start(area)) if (area.height - 32.0).abs() < f64::EPSILON)));
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                InputEvent::Text(TextInputEvent::Composition {
                    text: "にほん".into(),
                    cursor: Some((9, 9)),
                }),
                press(KeyCode::Backspace),
            ],
        )
        .unwrap();
    assert_eq!(value(&tree), "hello");
    assert_eq!(router.preedit(), "にほん");
    let decorated = router.layout(&tree, [200.0; 2], 2.0, None).unwrap();
    assert_ne!(decorated.primitives(), layout.primitives());
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                InputEvent::Text(TextInputEvent::CompositionCancelled),
                commit("日本"),
            ],
        )
        .unwrap();
    assert_eq!(value(&tree), "hello日本");
    assert_eq!(router.preedit(), "");
    router
        .route_events(
            &mut tree,
            &layout,
            &[InputEvent::FocusLost, commit("ignored")],
        )
        .unwrap();
    assert_eq!(value(&tree), "hello日本");
}

#[test]
fn rejected_batch_preserves_values_focus_selection_and_platform_operations() {
    let mut tree = field("x");
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    let events = [
        press(KeyCode::Tab),
        commit("valid"),
        commit(&"z".repeat(65536)),
    ];
    assert!(router.route_events(&mut tree, &layout, &events).is_err());
    assert_eq!(value(&tree), "x");
    assert_eq!(router.focused(), None);
}

#[test]
fn pointer_drag_selects_and_external_commands_reset_stale_selection() {
    let mut tree = field("abcd");
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                pointer(12.0, 1.0),
                mouse(ButtonState::Pressed),
                pointer(36.0, 1.0),
                mouse(ButtonState::Released),
            ],
        )
        .unwrap();
    assert_eq!(
        router.selection(&tree),
        Some(UiSelection {
            anchor: 1,
            caret: 3
        })
    );
    tree.command(UiNodeId(1), UiCommand::SetText("я".into()))
        .unwrap();
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    router.route_events(&mut tree, &layout, &[]).unwrap();
    assert_eq!(
        router.selection(&tree),
        Some(UiSelection {
            anchor: 2,
            caret: 2
        })
    );
}

#[test]
fn changed_control_kind_closes_text_session_and_discards_pending_editor_owner() {
    let mut tree = field("");
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    let mut root = tree.root().clone();
    root.children[0].control = UiControl::Button("replacement".into());
    tree.replace(root).unwrap();
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let result = router.route_events(&mut tree, &layout, &[]).unwrap();
    assert_eq!(router.focused(), None);
    assert!(result.platform.contains(&UiPlatformRequest::Text(
        gridthorn_input::TextInputRequest::Stop
    )));
}

#[test]
fn multiline_hit_testing_uses_the_line_box_and_shift_arrows_select_graphemes() {
    let mut tree = field("ab\ncd");
    let mut root = tree.root().clone();
    root.children[0].style.size[1] = UiLength::Pixels(80.0);
    tree.replace(root).unwrap();
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                pointer(12.0, 15.0),
                mouse(ButtonState::Pressed),
                mouse(ButtonState::Released),
            ],
        )
        .unwrap();
    assert_eq!(router.selection(&tree).unwrap().caret, 1);
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                InputEvent::ModifiersChanged(gridthorn_input::Modifiers {
                    shift: true,
                    ..Default::default()
                }),
                press(KeyCode::ArrowDown),
            ],
        )
        .unwrap();
    assert_eq!(
        router.selection(&tree),
        Some(UiSelection {
            anchor: 1,
            caret: 4
        })
    );
}

#[test]
fn native_preedit_cursor_is_validated_and_cancelled_with_composition() {
    let mut tree = field("");
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    let preedit = |cursor| {
        InputEvent::Text(TextInputEvent::Composition {
            text: "日本".into(),
            cursor,
        })
    };
    router
        .route_events(&mut tree, &layout, &[preedit(Some((6, 3)))])
        .unwrap();
    assert_eq!(router.composition_cursor(), Some((6, 3)));
    let selected = router.layout(&tree, [200.0; 2], 1.0, None).unwrap();
    router
        .route_events(&mut tree, &layout, &[preedit(Some((1, 30)))])
        .unwrap();
    assert_eq!(router.composition_cursor(), None);
    let no_cursor = router.layout(&tree, [200.0; 2], 1.0, None).unwrap();
    assert_ne!(selected.primitives(), no_cursor.primitives());
    router
        .route_events(&mut tree, &layout, &[press(KeyCode::Escape)])
        .unwrap();
    assert_eq!(router.preedit(), "");
    assert_eq!(router.composition_cursor(), None);
    assert_eq!(router.focused(), Some(UiNodeId(1)));
}

#[test]
fn field_focus_transfer_waits_for_native_stop_before_accepting_commits() {
    let mut tree = tree(vec![
        UiControl::TextField {
            value: "first".into(),
            placeholder: String::new(),
        },
        UiControl::TextField {
            value: "second".into(),
            placeholder: String::new(),
        },
    ]);
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    router
        .route_events(
            &mut tree,
            &layout,
            &[InputEvent::Text(TextInputEvent::Composition {
                text: "old".into(),
                cursor: None,
            })],
        )
        .unwrap();
    let transfer = router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(2)));
    assert!(transfer.platform.contains(&UiPlatformRequest::Text(
        gridthorn_input::TextInputRequest::Stop
    )));
    assert!(!transfer.platform.iter().any(|request| matches!(
        request,
        UiPlatformRequest::Text(gridthorn_input::TextInputRequest::Start(_))
    )));
    router
        .route_events(&mut tree, &layout, &[commit("late")])
        .unwrap();
    assert!(
        matches!(&tree.node(UiNodeId(2)).unwrap().control, UiControl::TextField { value, .. } if value == "second")
    );
    let reopened = router
        .route_events(
            &mut tree,
            &layout,
            &[InputEvent::TextInputChanged {
                active: false,
                error: None,
            }],
        )
        .unwrap();
    assert!(reopened.platform.iter().any(|request| matches!(
        request,
        UiPlatformRequest::Text(gridthorn_input::TextInputRequest::Start(_))
    )));
    router
        .route_events(&mut tree, &layout, &[commit("new")])
        .unwrap();
    assert!(
        matches!(&tree.node(UiNodeId(2)).unwrap().control, UiControl::TextField { value, .. } if value == "secondnew")
    );
}
