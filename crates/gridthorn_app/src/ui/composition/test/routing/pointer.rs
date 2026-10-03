use super::*;

#[test]
fn short_click_routes_once_and_world_receives_only_outside_events() {
    let mut tree = tree(vec![UiControl::Button("apply".into())]);
    let layout = tree.layout([200.0; 2], 2.0, None).unwrap();
    let mut router = UiRouter::new(100);
    let events = [
        pointer(20.0, 20.0),
        mouse(ButtonState::Pressed),
        mouse(ButtonState::Released),
        pointer(300.0, 100.0),
        mouse(ButtonState::Pressed),
        mouse(ButtonState::Released),
    ];
    let route = router.route_events(&mut tree, &layout, &events).unwrap();
    assert_eq!(route.effects, [(UiNodeId(1), UiEffect::Activated)]);
    assert_eq!(route.consumed, [0, 1, 2]);
    assert_eq!(route.world_events, events[3..]);
    assert_eq!(router.focused(), None);
    assert_eq!(router.captured(), None);
    assert!(!route.pointer_blocked);
}

#[test]
fn drag_updates_slider_outside_and_release_is_still_consumed() {
    let mut tree = tree(vec![UiControl::Slider {
        min: 0.0,
        max: 10.0,
        value: 0.0,
    }]);
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    let held = router
        .route_events(
            &mut tree,
            &layout,
            &[
                pointer(50.0, 10.0),
                mouse(ButtonState::Pressed),
                pointer(300.0, 10.0),
                InputEvent::MouseWheel {
                    delta: gridthorn_input::WheelDelta::Lines { x: 0.0, y: 1.0 },
                    phase: gridthorn_input::ScrollPhase::Moved,
                },
            ],
        )
        .unwrap();
    assert!(held.pointer_blocked);
    assert_eq!(held.world_events, []);
    assert_eq!(router.captured(), Some(UiNodeId(1)));
    assert!(matches!(
        tree.node(UiNodeId(1)).unwrap().control,
        UiControl::Slider { value: 10.0, .. }
    ));
    assert_eq!(
        router
            .route_events(&mut tree, &layout, &[mouse(ButtonState::Released)])
            .unwrap()
            .world_events,
        []
    );
    assert_eq!(router.captured(), None);
}

#[test]
fn focus_loss_cancels_click_and_preserves_world_cancellation_event() {
    let mut tree = tree(vec![UiControl::Button("apply".into())]);
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    let route = router
        .route_events(
            &mut tree,
            &layout,
            &[
                pointer(10.0, 10.0),
                mouse(ButtonState::Pressed),
                InputEvent::FocusLost,
                mouse(ButtonState::Released),
            ],
        )
        .unwrap();
    assert_eq!(route.effects, []);
    assert!(route.world_events.contains(&InputEvent::FocusLost));
    assert_eq!(router.focused(), None);
    assert_eq!(router.captured(), None);
}

#[test]
fn painter_order_disabled_blocking_and_ancestor_clip_define_hit_testing() {
    let mut tree = tree(vec![
        UiControl::Button("behind".into()),
        UiControl::Button("front".into()),
    ]);
    let mut root = tree.root().clone();
    root.style.flow = UiFlow::Overlay;
    root.style.clip = true;
    root.style.size = [UiLength::Pixels(50.0); 2];
    tree.replace(root).unwrap();
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    assert_eq!(
        UiRouter::hit_test(&tree, &layout, [20.0; 2]),
        Some(UiNodeId(2))
    );
    assert_eq!(UiRouter::hit_test(&tree, &layout, [70.0, 20.0]), None);
    tree.command(UiNodeId(2), UiCommand::Visual(UiVisualState::Disabled))
        .unwrap();
    assert_eq!(UiRouter::hit_test(&tree, &layout, [20.0; 2]), None);
    let route = UiRouter::new(100)
        .route_events(
            &mut tree,
            &layout,
            &[
                pointer(20.0, 20.0),
                mouse(ButtonState::Pressed),
                mouse(ButtonState::Released),
            ],
        )
        .unwrap();
    assert_eq!(route.world_events, []);
    assert_eq!(route.effects, []);
}

#[test]
fn resize_and_dpi_reconcile_stationary_pointer_without_motion() {
    let mut tree = tree(vec![UiControl::Button("apply".into())]);
    let mut router = UiRouter::new(100);
    let layout = tree.layout([200.0; 2], 2.0, None).unwrap();
    router
        .route_events(&mut tree, &layout, &[pointer(150.0, 10.0)])
        .unwrap();
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let result = router
        .route_events(&mut tree, &layout, &[mouse(ButtonState::Pressed)])
        .unwrap();
    assert_eq!(result.world_events.len(), 1);
    assert_eq!(router.captured(), None);
}

#[test]
fn wheel_scrolls_in_explicit_units_and_picks_scrolled_list_rows() {
    let mut tree = tree(vec![UiControl::List {
        items: vec!["a".into(), "b".into(), "c".into()],
        selected: None,
    }]);
    let mut root = tree.root().clone();
    root.children[0].style.scroll = true;
    tree.replace(root).unwrap();
    let layout = tree.layout([200.0; 2], 2.0, None).unwrap();
    let mut router = UiRouter::new(100);
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                pointer(20.0, 10.0),
                InputEvent::MouseWheel {
                    delta: gridthorn_input::WheelDelta::Pixels { x: 0.0, y: -56.0 },
                    phase: gridthorn_input::ScrollPhase::Moved,
                },
            ],
        )
        .unwrap();
    let layout = tree.layout([200.0; 2], 2.0, None).unwrap();
    assert_eq!(
        layout.placement(UiNodeId(1)).unwrap().scroll_offset[1],
        28.0
    );
    router
        .route_events(&mut tree, &layout, &[mouse(ButtonState::Pressed)])
        .unwrap();
    assert!(matches!(
        tree.node(UiNodeId(1)).unwrap().control,
        UiControl::List {
            selected: Some(1),
            ..
        }
    ));
}

#[test]
fn leaving_window_cancels_activation_and_secondary_release_keeps_ownership() {
    let mut tree = tree(vec![UiControl::Button("a".into())]);
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut router = UiRouter::new(100);
    let result = router
        .route_events(
            &mut tree,
            &layout,
            &[
                pointer(10.0, 10.0),
                mouse(ButtonState::Pressed),
                InputEvent::CursorLeft,
                mouse(ButtonState::Released),
            ],
        )
        .unwrap();
    assert_eq!(result.effects, []);
    let right = |state| InputEvent::MouseButton {
        button: MouseButton::Right,
        state,
    };
    router
        .route_events(
            &mut tree,
            &layout,
            &[
                pointer(10.0, 10.0),
                right(ButtonState::Pressed),
                pointer(150.0, 150.0),
            ],
        )
        .unwrap();
    let result = router
        .route_events(&mut tree, &layout, &[right(ButtonState::Released)])
        .unwrap();
    assert_eq!(result.consumed, [0]);
    assert!(!result.pointer_blocked);
}

#[test]
fn repeated_wheel_events_accumulate_without_scrolling_under_an_overlay_control() {
    let mut tree = tree(vec![
        UiControl::List {
            items: vec!["a".into(); 10],
            selected: None,
        },
        UiControl::Button("overlay".into()),
    ]);
    let mut root = tree.root().clone();
    root.style.flow = UiFlow::Overlay;
    root.children[0].style.scroll = true;
    root.children[1].style.offset = [0.0, 40.0];
    tree.replace(root).unwrap();
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let wheel = InputEvent::MouseWheel {
        delta: gridthorn_input::WheelDelta::Lines { x: 0.0, y: -1.0 },
        phase: gridthorn_input::ScrollPhase::Moved,
    };
    let mut router = UiRouter::new(100);
    router
        .route_events(
            &mut tree,
            &layout,
            &[pointer(10.0, 10.0), wheel.clone(), wheel.clone()],
        )
        .unwrap();
    assert_eq!(tree.node(UiNodeId(1)).unwrap().scroll_offset, [0.0, 56.0]);
    let mut root = tree.root().clone();
    root.children[1].style.offset = [0.0; 2];
    tree.replace(root).unwrap();
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    router.route_events(&mut tree, &layout, &[wheel]).unwrap();
    assert_eq!(tree.node(UiNodeId(1)).unwrap().scroll_offset, [0.0, 56.0]);
}
