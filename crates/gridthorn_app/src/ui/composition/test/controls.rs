use super::*;

#[test]
fn text_limit_rejects_append_without_partial_mutation() {
    let node = UiNode::new(
        UiNodeId(1),
        UiControl::TextField {
            value: "a".repeat(65535),
            placeholder: String::new(),
        },
    );
    let mut tree = UiTree::new(node, UiTheme::default()).unwrap();
    let before = tree.root().control.clone();
    assert!(
        tree.command(UiNodeId(1), UiCommand::AppendText("Ж".into()))
            .is_err()
    );
    assert_eq!(tree.root().control, before);
    assert!(tree.command(UiNodeId(1), UiCommand::Select(None)).is_err());
    assert_eq!(tree.root().control, before);
}

#[test]
fn theme_visual_state_and_node_override_change_paint() {
    let node = UiNode::new(UiNodeId(1), UiControl::Button("Go".into()));
    let mut tree = UiTree::new(node, UiTheme::default()).unwrap();
    let normal = tree.layout([100.0; 2], 1.0, None).unwrap();
    tree.command(UiNodeId(1), UiCommand::Visual(UiVisualState::Hovered))
        .unwrap();
    let hovered = tree.layout([100.0; 2], 1.0, None).unwrap();
    assert_eq!(normal.placements(), hovered.placements());
    assert_ne!(normal.primitives(), hovered.primitives());
    let mut node = tree.root().clone();
    node.style.background = Some(gridthorn_render::Color::rgb(1.0, 0.0, 0.0));
    tree.replace(node).unwrap();
    let overridden = tree.layout([100.0; 2], 1.0, None).unwrap();
    tree.command(UiNodeId(1), UiCommand::Visual(UiVisualState::Pressed))
        .unwrap();
    assert_eq!(
        overridden.primitives(),
        tree.layout([100.0; 2], 1.0, None).unwrap().primitives()
    );
    tree.set_theme(UiTheme {
        foreground: gridthorn_render::Color::rgb(0.0, 1.0, 0.0),
        ..UiTheme::default()
    })
    .unwrap();
    assert_ne!(
        overridden.primitives(),
        tree.layout([100.0; 2], 1.0, None).unwrap().primitives()
    );
}

#[test]
fn explicit_control_commands_validate_and_report_changes() {
    let mut root = panel(0, [UiLength::Fill; 2], UiFlow::Column);
    root.children = vec![
        UiNode::new(UiNodeId(1), UiControl::Button("Apply".into())),
        UiNode::new(
            UiNodeId(2),
            UiControl::Toggle {
                label: "On".into(),
                checked: false,
            },
        ),
        UiNode::new(
            UiNodeId(3),
            UiControl::Slider {
                min: -10.0,
                max: 10.0,
                value: 0.0,
            },
        ),
        UiNode::new(
            UiNodeId(4),
            UiControl::List {
                items: vec!["A".into(), "B".into()],
                selected: None,
            },
        ),
        UiNode::new(
            UiNodeId(5),
            UiControl::TextField {
                value: "Ж".into(),
                placeholder: "Name".into(),
            },
        ),
    ];
    let mut tree = UiTree::new(root, UiTheme::default()).unwrap();
    assert_eq!(
        tree.command(UiNodeId(1), UiCommand::Activate).unwrap(),
        UiEffect::Activated
    );
    assert_eq!(
        tree.command(UiNodeId(2), UiCommand::Activate).unwrap(),
        UiEffect::Changed
    );
    tree.command(UiNodeId(3), UiCommand::SetValue(99.0))
        .unwrap();
    assert!(matches!(
        tree.node(UiNodeId(3)).unwrap().control,
        UiControl::Slider { value: 10.0, .. }
    ));
    let before = tree.node(UiNodeId(3)).unwrap().control.clone();
    assert!(
        tree.command(UiNodeId(3), UiCommand::SetValue(f32::NAN))
            .is_err()
    );
    assert_eq!(tree.node(UiNodeId(3)).unwrap().control, before);
    tree.command(UiNodeId(4), UiCommand::Select(Some(1)))
        .unwrap();
    assert!(
        tree.command(UiNodeId(4), UiCommand::Select(Some(2)))
            .is_err()
    );
    tree.command(UiNodeId(5), UiCommand::AppendText("日本語".into()))
        .unwrap();
    tree.command(UiNodeId(5), UiCommand::PopText).unwrap();
    assert!(
        matches!(&tree.node(UiNodeId(5)).unwrap().control, UiControl::TextField { value, .. } if value == "Ж日本")
    );
    tree.command(UiNodeId(2), UiCommand::Visual(UiVisualState::Disabled))
        .unwrap();
    assert_eq!(
        tree.command(UiNodeId(2), UiCommand::Activate).unwrap(),
        UiEffect::None
    );
    assert_ne!(tree.layout([300.0; 2], 1.5, None).unwrap().primitives(), []);
}

#[test]
fn scroll_list_exposes_extent_and_snapshot_survives_edits() {
    let mut node = UiNode::new(
        UiNodeId(1),
        UiControl::List {
            items: vec!["One".into(), "Two".into(), "Three".into()],
            selected: None,
        },
    );
    node.style.size = [UiLength::Pixels(100.0), UiLength::Pixels(28.0)];
    node.style.scroll = true;
    let mut tree = UiTree::new(node, UiTheme::default()).unwrap();
    tree.command(UiNodeId(1), UiCommand::ScrollTo([0.0, 100.0]))
        .unwrap();
    let snapshot = tree.layout([100.0; 2], 1.0, None).unwrap();
    assert_eq!(
        snapshot.placement(UiNodeId(1)).unwrap().scroll_offset,
        [0.0, 56.0]
    );
    tree.command(UiNodeId(1), UiCommand::Select(Some(2)))
        .unwrap();
    assert_ne!(
        snapshot.primitives(),
        tree.layout([100.0; 2], 1.0, None).unwrap().primitives()
    );
}
