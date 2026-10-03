use super::*;

fn composition() -> UiTree {
    let mut tree = tree(vec![UiControl::Button("base".into())]);
    let mut root = tree.root().clone();
    for index in [10, 20] {
        let mut panel = UiNode::new(UiNodeId(index), UiControl::Panel);
        panel.style.size = [UiLength::Pixels(100.0); 2];
        panel.style.offset = [120.0, 0.0];
        let mut button = UiNode::new(UiNodeId(index + 1), UiControl::Button("layer".into()));
        button.style.size = [UiLength::Pixels(80.0), UiLength::Pixels(30.0)];
        panel.children.push(button);
        root.children.push(panel);
    }
    tree.replace(root).unwrap();
    tree
}

#[test]
fn nested_modal_scopes_restore_focus_and_painter_order() {
    let mut tree = composition();
    let mut router = UiRouter::new(0);
    let layout = tree.layout([400.0; 2], 1.0, None).unwrap();
    router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    let policy = UiLayer {
        modal: true,
        dismiss_escape: true,
        dismiss_outside: false,
    };
    router
        .open_layer(&mut tree, &layout, UiNodeId(20), policy)
        .unwrap();
    router
        .open_layer(&mut tree, &layout, UiNodeId(10), policy)
        .unwrap();
    let painted = router.layout(&tree, [400.0; 2], 1.0, None).unwrap();
    assert_eq!(painted.placements().last().unwrap().id, UiNodeId(11));
    for _ in 0..4 {
        router
            .navigate(&mut tree, &painted, UiNavigation::Next)
            .unwrap();
        assert_eq!(router.focused(), Some(UiNodeId(11)));
    }
    let route = router
        .route_events(&mut tree, &painted, &[press(KeyCode::Escape)])
        .unwrap();
    assert_eq!(route.dismissed, [UiNodeId(10)]);
    assert_eq!(router.focused(), Some(UiNodeId(21)));
    router.close_layer(&tree, &layout);
    assert_eq!(router.focused(), Some(UiNodeId(1)));
    let closed = router.layout(&tree, [400.0; 2], 1.0, None).unwrap();
    assert!(closed.placement(UiNodeId(10)).is_none());
    assert!(closed.placement(UiNodeId(20)).is_none());
}

#[test]
fn modal_empty_space_and_dismissal_do_not_click_through() {
    let mut tree = composition();
    let mut router = UiRouter::new(0);
    let layout = tree.layout([400.0; 2], 1.0, None).unwrap();
    router
        .open_layer(
            &mut tree,
            &layout,
            UiNodeId(10),
            UiLayer {
                modal: true,
                ..UiLayer::default()
            },
        )
        .unwrap();
    let route = router
        .route_events(
            &mut tree,
            &layout,
            &[
                pointer(5.0, 5.0),
                mouse(ButtonState::Pressed),
                press(KeyCode::Escape),
            ],
        )
        .unwrap();
    assert_eq!(route.world_events, []);
    assert!(route.keyboard_blocked && route.pointer_blocked);
    assert_eq!(route.dismissed, []);
    router.close_layer(&tree, &layout);
    let release = router
        .route_events(
            &mut tree,
            &layout,
            &[
                mouse(ButtonState::Released),
                key(KeyCode::Escape, ButtonState::Released),
            ],
        )
        .unwrap();
    assert_eq!(release.world_events, []);
    assert_eq!(release.effects, []);
    router
        .open_layer(
            &mut tree,
            &layout,
            UiNodeId(10),
            UiLayer {
                dismiss_outside: true,
                ..UiLayer::default()
            },
        )
        .unwrap();
    let route = router
        .route_events(
            &mut tree,
            &layout,
            &[
                pointer(5.0, 5.0),
                mouse(ButtonState::Pressed),
                mouse(ButtonState::Released),
            ],
        )
        .unwrap();
    assert_eq!(route.dismissed, [UiNodeId(10)]);
    assert_eq!(route.effects, []);
    assert_eq!(route.consumed, [0, 1, 2]);
}

#[test]
fn invalid_open_is_atomic_and_removed_restore_target_is_skipped() {
    let mut tree = composition();
    let mut router = UiRouter::new(0);
    let layout = tree.layout([400.0; 2], 1.0, None).unwrap();
    router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    assert!(
        router
            .open_layer(&mut tree, &layout, UiNodeId(1), UiLayer::default())
            .is_err()
    );
    assert_eq!(router.open_layers(), []);
    assert_eq!(router.focused(), Some(UiNodeId(1)));
    router
        .open_layer(&mut tree, &layout, UiNodeId(10), UiLayer::default())
        .unwrap();
    tree.command(UiNodeId(1), UiCommand::Visual(UiVisualState::Disabled))
        .unwrap();
    router.close_layer(&tree, &layout);
    assert_eq!(router.focused(), None);
}

#[test]
fn nonmodal_panel_background_blocks_underlying_controls_and_closed_roots() {
    let mut tree = composition();
    let mut root = tree.root().clone();
    root.style.flow = UiFlow::Overlay;
    root.children[0].style.size = [UiLength::Fill; 2];
    tree.replace(root).unwrap();
    let mut router = UiRouter::new(0);
    router.register_layer(&tree, UiNodeId(10)).unwrap();
    router.register_layer(&tree, UiNodeId(20)).unwrap();
    let hidden = router.layout(&tree, [400.0; 2], 1.0, None).unwrap();
    assert!(hidden.placement(UiNodeId(11)).is_none());
    let raw = tree.layout([400.0; 2], 1.0, None).unwrap();
    router
        .open_layer(&mut tree, &raw, UiNodeId(10), UiLayer::default())
        .unwrap();
    assert_eq!(router.hit_test_layers(&tree, &raw, [130.0, 80.0]), None);
    assert_eq!(
        router.hit_test_layers(&tree, &raw, [5.0, 5.0]),
        Some(UiNodeId(1))
    );
    let route = router
        .route_events(
            &mut tree,
            &raw,
            &[
                pointer(130.0, 80.0),
                mouse(ButtonState::Pressed),
                mouse(ButtonState::Released),
            ],
        )
        .unwrap();
    assert_eq!(route.world_events, []);
    assert_eq!(route.effects, []);
    assert!(route.pointer_blocked);
}

#[test]
fn ime_escape_precedes_layer_dismissal_and_failures_restore_stack() {
    let mut tree = composition();
    let mut root = tree.root().clone();
    root.children[1].children[0].control = UiControl::TextField {
        value: String::new(),
        placeholder: String::new(),
    };
    tree.replace(root).unwrap();
    let mut router = UiRouter::new(0);
    let layout = tree.layout([400.0; 2], 1.0, None).unwrap();
    router
        .open_layer(
            &mut tree,
            &layout,
            UiNodeId(10),
            UiLayer {
                modal: true,
                dismiss_escape: true,
                ..UiLayer::default()
            },
        )
        .unwrap();
    router
        .route_events(
            &mut tree,
            &layout,
            &[InputEvent::Text(
                gridthorn_input::TextInputEvent::Composition {
                    text: "に".into(),
                    cursor: Some((3, 3)),
                },
            )],
        )
        .unwrap();
    let route = router
        .route_events(&mut tree, &layout, &[press(KeyCode::Escape)])
        .unwrap();
    assert_eq!(route.dismissed, []);
    assert_eq!(router.preedit(), "");
    assert_eq!(router.open_layers(), [UiNodeId(10)]);
    router
        .route_events(
            &mut tree,
            &layout,
            &[InputEvent::TextInputChanged {
                active: false,
                error: None,
            }],
        )
        .unwrap();
    let huge = "x".repeat(65537);
    assert!(
        router
            .route_events(
                &mut tree,
                &layout,
                &[InputEvent::Text(gridthorn_input::TextInputEvent::Commit(
                    huge
                ))]
            )
            .is_err()
    );
    assert_eq!(router.open_layers(), [UiNodeId(10)]);
    let route = router
        .route_events(&mut tree, &layout, &[press(KeyCode::Escape)])
        .unwrap();
    assert_eq!(route.dismissed, [UiNodeId(10)]);
}

#[test]
fn repeated_escape_preserves_next_layer_and_focus_loss_keeps_stack() {
    let mut tree = composition();
    let layout = tree.layout([400.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(0);
    let options = UiLayer {
        modal: true,
        dismiss_escape: true,
        ..UiLayer::default()
    };
    router
        .open_layer(&mut tree, &layout, UiNodeId(10), options)
        .unwrap();
    router
        .open_layer(&mut tree, &layout, UiNodeId(20), options)
        .unwrap();
    let repeated = InputEvent::Key(gridthorn_input::KeyboardEvent {
        physical_key: gridthorn_input::PhysicalKey::Code(KeyCode::Escape),
        logical_key: gridthorn_input::LogicalKey::Named(gridthorn_input::NamedKey::Escape),
        location: gridthorn_input::KeyLocation::Standard,
        state: ButtonState::Pressed,
        repeat: true,
        synthetic: false,
    });
    let route = router
        .route_events(
            &mut tree,
            &layout,
            &[press(KeyCode::Escape), repeated, InputEvent::FocusLost],
        )
        .unwrap();
    assert_eq!(route.dismissed, [UiNodeId(20)]);
    assert_eq!(router.open_layers(), [UiNodeId(10)]);
    assert_eq!(route.world_events, [InputEvent::FocusLost]);
    assert_eq!(router.focused(), None);
    assert!(route.keyboard_blocked && route.pointer_blocked);
}

#[test]
fn empty_modal_owns_keys_until_release_and_removed_roots_restore_focus() {
    let mut tree = composition();
    let mut root = tree.root().clone();
    root.children[1].children.clear();
    tree.replace(root).unwrap();
    let layout = tree.layout([400.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(0);
    router
        .navigate(&mut tree, &layout, UiNavigation::Next)
        .unwrap();
    router
        .open_layer(
            &mut tree,
            &layout,
            UiNodeId(10),
            UiLayer {
                modal: true,
                ..UiLayer::default()
            },
        )
        .unwrap();
    let route = router
        .route_events(&mut tree, &layout, &[press(KeyCode::KeyW)])
        .unwrap();
    assert_eq!(route.world_events, []);
    assert_eq!(router.focused(), None);
    let mut root = tree.root().clone();
    root.children.retain(|node| node.id != UiNodeId(10));
    tree.replace(root).unwrap();
    let route = router
        .route_events(
            &mut tree,
            &layout,
            &[key(KeyCode::KeyW, ButtonState::Released)],
        )
        .unwrap();
    assert_eq!(route.dismissed, [UiNodeId(10)]);
    assert_eq!(route.world_events, []);
    assert_eq!(router.focused(), Some(UiNodeId(1)));
}
