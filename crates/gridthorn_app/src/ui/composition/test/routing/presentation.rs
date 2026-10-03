use super::*;

#[test]
fn undecorated_router_preserves_tree_layout_and_clipped_paint() {
    let mut tree = field("Привет مرحبًا 日本語 e\u{301}");
    let mut root = tree.root().clone();
    root.style.clip = true;
    root.style.scroll = true;
    root.scroll_offset = [0.0, 15.0];
    tree.replace(root).unwrap();
    let router = UiRouter::new(0);
    for viewport in [[200.0; 2], [50.0, 20.0], [0.0; 2]] {
        for dpi in [1.0, 2.0] {
            let raw = tree.layout(viewport, dpi, None).unwrap();
            let prepared = router.layout(&tree, viewport, dpi, None).unwrap();
            assert_eq!(prepared.placements(), raw.placements());
            assert_eq!(prepared.primitives(), raw.primitives());
            let raw_geometry = &raw.text_geometry[&UiNodeId(1)];
            let prepared_geometry = &prepared.text_geometry[&UiNodeId(1)];
            assert_eq!(prepared_geometry.stops, raw_geometry.stops);
            assert_eq!(prepared_geometry.segments, raw_geometry.segments);
        }
    }
}

#[test]
fn closed_layer_excludes_field_geometry_and_reopening_restores_text_input() {
    let mut tree = tree(vec![UiControl::Button("base".into())]);
    let mut root = tree.root().clone();
    let mut layer = UiNode::new(UiNodeId(10), UiControl::Panel);
    layer.style.size = [UiLength::Pixels(150.0); 2];
    let mut editor = UiNode::new(
        UiNodeId(11),
        UiControl::TextField {
            value: "e\u{301} 日本語".into(),
            placeholder: String::new(),
        },
    );
    editor.style.size = [UiLength::Fill, UiLength::Pixels(30.0)];
    layer.children.push(editor);
    root.children.push(layer);
    tree.replace(root).unwrap();
    let mut router = UiRouter::new(0);
    router.register_layer(&tree, UiNodeId(10)).unwrap();
    for _ in 0..2 {
        let closed = router.layout(&tree, [400.0; 2], 2.0, None).unwrap();
        assert!(!closed.text_geometry.contains_key(&UiNodeId(11)));
        router
            .open_layer(
                &mut tree,
                &closed,
                UiNodeId(10),
                UiLayer {
                    modal: true,
                    ..UiLayer::default()
                },
            )
            .unwrap();
        let opened = router.layout(&tree, [400.0; 2], 2.0, None).unwrap();
        assert!(opened.text_geometry.contains_key(&UiNodeId(11)));
        router
            .navigate(&mut tree, &opened, UiNavigation::Next)
            .unwrap();
        assert_eq!(router.focused(), Some(UiNodeId(11)));
        let route = router.route_events(&mut tree, &opened, &[]).unwrap();
        assert!(route.platform.iter().any(|request| matches!(
            request,
            UiPlatformRequest::Text(gridthorn_input::TextInputRequest::Start(_))
        )));
        router.close_layer(&tree, &opened);
        let closed = router.layout(&tree, [400.0; 2], 2.0, None).unwrap();
        let route = router.route_events(&mut tree, &closed, &[]).unwrap();
        assert!(!route.platform.iter().any(|request| matches!(
            request,
            UiPlatformRequest::Text(gridthorn_input::TextInputRequest::Start(_))
        )));
        router
            .route_events(
                &mut tree,
                &closed,
                &[InputEvent::TextInputChanged {
                    active: false,
                    error: None,
                }],
            )
            .unwrap();
    }
}
