use super::super::*;
use crate::{
    ApplicationRuntime, ButtonState, InputEvent, KeyCode, ScheduleBuilder, ScheduleStage,
    TextInputEvent,
};

#[test]
fn input_schedule_routes_ui_before_world_mapping_and_presentation() {
    struct Interface {
        tree: UiTree,
        router: UiRouter,
        layout: UiLayout,
    }
    let mut root = UiNode::new(UiNodeId(0), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    root.children.push(UiNode::new(
        UiNodeId(1),
        UiControl::TextField {
            value: String::new(),
            placeholder: "Type".into(),
        },
    ));
    root.children[0].style.size = [UiLength::Pixels(100.0), UiLength::Pixels(30.0)];
    let tree = UiTree::new(root, UiTheme::default()).unwrap();
    let layout = tree.layout([200.0; 2], 1.0, None).unwrap();
    let mut interface = Some(Interface {
        tree,
        router: UiRouter::new(100),
        layout,
    });
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, move |world| {
        world.insert_resource(interface.take().unwrap());
    });
    schedules.add_system(ScheduleStage::Input, |world| {
        let route = world
            .update_resource_with(|state: &mut Interface| {
                state
                    .router
                    .route_events(
                        &mut state.tree,
                        &state.layout,
                        &[
                            InputEvent::Keyboard {
                                key: KeyCode::Tab,
                                state: ButtonState::Pressed,
                            },
                            InputEvent::Text(TextInputEvent::Commit("Привет 日本語".into())),
                        ],
                    )
                    .unwrap()
            })
            .unwrap();
        world.insert_resource(route);
    });
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        assert!(
            world
                .read_resource(
                    |route: &UiRoute| route.keyboard_blocked && route.world_events.is_empty()
                )
                .unwrap()
        );
    });
    schedules.add_system(ScheduleStage::Render, |world| {
        let primitives = world
            .update_resource_with(|state: &mut Interface| {
                state
                    .router
                    .layout(&state.tree, [200.0; 2], 1.0, None)
                    .unwrap()
                    .into_primitives()
            })
            .unwrap();
        world.insert_resource(crate::RenderFrame::default().with_ui(primitives));
    });
    let mut application = ApplicationRuntime::new(schedules.build());
    application.run_frame(1).unwrap();
    assert_eq!(
        application
            .world()
            .read_resource(|state: &Interface| state.router.focused()),
        Some(Some(UiNodeId(1)))
    );
    assert_eq!(
        application
            .world()
            .read_resource(|route: &UiRoute| route.effects.clone()),
        Some(vec![(UiNodeId(1), UiEffect::Changed)])
    );
    assert!(
        application
            .world()
            .read_resource(|frame: &crate::RenderFrame| !frame.ui().is_empty())
            .unwrap()
    );
}

#[test]
fn shaped_multilingual_fields_route_pointer_selection_and_render_caret_at_dpi() {
    let assets = [
        include_bytes!("../../../../gridthorn_render/src/text/test/fonts/NotoSans-Regular.ttf")
            .as_slice(),
        include_bytes!(
            "../../../../gridthorn_render/src/text/test/fonts/NotoSansArabic-Regular.ttf"
        )
        .as_slice(),
        include_bytes!("../../../../gridthorn_render/src/text/test/fonts/NotoSansJP-Regular.otf")
            .as_slice(),
    ]
    .map(|bytes| crate::FontAsset::from_bytes(bytes.to_vec()).unwrap());
    let mut fonts = crate::TextSystem::new("en-US", &assets).unwrap();
    for value in [
        "Привет",
        "العربية",
        "日本語",
        "e\u{301}",
        "office",
        "one\ntwo",
        "one\rtwo",
        "one\r\ntwo",
        "one\n\rtwo",
        "one\r",
        "",
    ] {
        let mut node = UiNode::new(
            UiNodeId(1),
            UiControl::TextField {
                value: value.into(),
                placeholder: String::new(),
            },
        );
        node.style.size = [UiLength::Pixels(160.0), UiLength::Pixels(100.0)];
        let mut tree = UiTree::new(
            node,
            UiTheme {
                text: Some(crate::TextStyle::new("Noto Sans", 20.0)),
                ..UiTheme::default()
            },
        )
        .unwrap();
        let plain = tree.layout([200.0; 2], 2.0, Some(&mut fonts)).unwrap();
        let mut router = UiRouter::new(100);
        router
            .route_events(
                &mut tree,
                &plain,
                &[
                    InputEvent::CursorMoved(crate::CursorPosition { x: 30.0, y: 2.0 }),
                    InputEvent::MouseButton {
                        button: crate::MouseButton::Left,
                        state: ButtonState::Pressed,
                    },
                    InputEvent::MouseButton {
                        button: crate::MouseButton::Left,
                        state: ButtonState::Released,
                    },
                ],
            )
            .unwrap();
        router.selection(&tree).unwrap().validate(value).unwrap();
        router
            .select(
                &tree,
                UiSelection {
                    anchor: 0,
                    caret: value.len(),
                },
            )
            .unwrap();
        let painted = router
            .layout(&tree, [200.0; 2], 2.0, Some(&mut fonts))
            .unwrap();
        assert_eq!(plain.placements(), painted.placements());
        assert_ne!(plain.primitives(), painted.primitives());
        router
            .route_events(
                &mut tree,
                &painted,
                &[InputEvent::Text(TextInputEvent::Commit("Я".into()))],
            )
            .unwrap();
        assert!(
            matches!(&tree.node(UiNodeId(1)).unwrap().control, UiControl::TextField { value, .. } if value == "Я")
        );
    }
}
