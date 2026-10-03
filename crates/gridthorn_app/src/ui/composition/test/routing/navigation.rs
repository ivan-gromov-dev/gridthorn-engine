use super::*;

#[test]
fn tab_shift_tab_hooks_and_activation_share_enabled_visible_focus_order() {
    let mut tree = tree(vec![
        UiControl::Button("a".into()),
        UiControl::Toggle {
            label: "b".into(),
            checked: false,
        },
        UiControl::Button("disabled".into()),
    ]);
    tree.command(UiNodeId(3), UiCommand::Visual(UiVisualState::Disabled))
        .unwrap();
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    assert!(
        router
            .navigate(&mut tree, &layout, UiNavigation::Next)
            .unwrap()
            .keyboard_blocked
    );
    assert_eq!(router.focused(), Some(UiNodeId(1)));
    router
        .route_events(&mut tree, &layout, &[press(KeyCode::Tab)])
        .unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(2)));
    assert_eq!(
        router
            .navigate(&mut tree, &layout, UiNavigation::Activate)
            .unwrap()
            .effects,
        [(UiNodeId(2), UiEffect::Changed)]
    );
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                InputEvent::ModifiersChanged(gridthorn_input::Modifiers {
                    shift: true,
                    ..Default::default()
                }),
                press(KeyCode::Tab),
            ],
        )
        .unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(1)));
    router
        .navigate(&mut tree, &layout, UiNavigation::Previous)
        .unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(2)));
}

#[test]
fn key_release_remains_consumed_after_escape_until_held_owner_releases() {
    let mut tree = tree(vec![UiControl::Button("a".into())]);
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    let result = router
        .route_events(
            &mut tree,
            &layout,
            &[press(KeyCode::Tab), press(KeyCode::Escape)],
        )
        .unwrap();
    assert_eq!(router.focused(), None);
    assert!(result.keyboard_blocked);
    let result = router
        .route_events(
            &mut tree,
            &layout,
            &[
                key(KeyCode::Tab, ButtonState::Released),
                key(KeyCode::Escape, ButtonState::Released),
                press(KeyCode::KeyW),
            ],
        )
        .unwrap();
    assert_eq!(result.consumed, [0, 1]);
    assert_eq!(result.world_events, [press(KeyCode::KeyW)]);
    assert!(!result.keyboard_blocked);
}

#[test]
fn removed_or_disabled_focus_and_capture_are_cancelled() {
    let mut tree = tree(vec![UiControl::Button("a".into())]);
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .route_events(
            &mut tree,
            &layout,
            &[pointer(10.0, 10.0), mouse(ButtonState::Pressed)],
        )
        .unwrap();
    tree.command(UiNodeId(1), UiCommand::Visual(UiVisualState::Disabled))
        .unwrap();
    let result = router
        .route_events(&mut tree, &layout, &[mouse(ButtonState::Released)])
        .unwrap();
    assert_eq!(router.focused(), None);
    assert_eq!(router.captured(), None);
    assert_eq!(result.effects, []);
    assert_eq!(result.consumed, [0]);
}

#[test]
fn directional_hooks_adjust_values_and_choose_spatial_focus() {
    let mut tree = tree(vec![
        UiControl::Button("a".into()),
        UiControl::Slider {
            min: 0.0,
            max: 100.0,
            value: 50.0,
        },
        UiControl::List {
            items: vec!["a".into(), "b".into()],
            selected: None,
        },
    ]);
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    router
        .navigate(&mut tree, &layout, UiNavigation::Down)
        .unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(2)));
    router
        .navigate(&mut tree, &layout, UiNavigation::Right)
        .unwrap();
    assert!(matches!(
        tree.node(UiNodeId(2)).unwrap().control,
        UiControl::Slider { value: 51.0, .. }
    ));
    router
        .navigate(&mut tree, &layout, UiNavigation::Down)
        .unwrap();
    assert_eq!(router.focused(), Some(UiNodeId(3)));
    router
        .navigate(&mut tree, &layout, UiNavigation::Down)
        .unwrap();
    assert!(matches!(
        tree.node(UiNodeId(3)).unwrap().control,
        UiControl::List {
            selected: Some(0),
            ..
        }
    ));
}
