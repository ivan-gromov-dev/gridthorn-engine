use super::*;

fn composition() -> UiNode {
    let mut root = panel(0, [UiLength::Fill; 2], UiFlow::Column);
    let mut nested = panel(7, [UiLength::Fill; 2], UiFlow::Column);
    nested.children.push(UiNode::new(
        UiNodeId(u64::MAX),
        UiControl::Button("nested".into()),
    ));
    root.children = vec![
        nested,
        UiNode::new(
            UiNodeId(90),
            UiControl::Toggle {
                label: "other".into(),
                checked: false,
            },
        ),
    ];
    root
}

#[test]
fn replacement_reorders_and_moves_ids_without_affecting_an_old_clone() {
    let mut tree = UiTree::new(composition(), UiTheme::default()).unwrap();
    let mut old = tree.clone();
    let mut next = composition();
    let mut nested = next.children.remove(0);
    let leaf = nested.children.remove(0);
    next.children.push(leaf);
    tree.replace(next).unwrap();
    assert!(tree.node(UiNodeId(7)).is_none());
    assert!(old.node(UiNodeId(7)).is_some());
    tree.command(UiNodeId(90), UiCommand::SetChecked(true))
        .unwrap();
    old.command(
        UiNodeId(u64::MAX),
        UiCommand::Visual(UiVisualState::Disabled),
    )
    .unwrap();
    assert_eq!(
        tree.node(UiNodeId(u64::MAX)).unwrap().visual,
        UiVisualState::Normal
    );
    assert_eq!(
        old.node(UiNodeId(u64::MAX)).unwrap().visual,
        UiVisualState::Disabled
    );
    assert!(matches!(
        old.node(UiNodeId(90)).unwrap().control,
        UiControl::Toggle { checked: false, .. }
    ));
    assert!(matches!(
        tree.node(UiNodeId(90)).unwrap().control,
        UiControl::Toggle { checked: true, .. }
    ));
    let style = UiStyle {
        gap: 3.0,
        ..UiStyle::default()
    };
    tree.set_style(UiNodeId(u64::MAX), style).unwrap();
    assert_eq!(tree.node(UiNodeId(u64::MAX)).unwrap().style.gap, 3.0);
    assert_eq!(old.node(UiNodeId(u64::MAX)).unwrap().style.gap, 0.0);
}

#[test]
fn rejected_topology_and_unknown_commands_preserve_existing_lookup() {
    let mut tree = UiTree::new(composition(), UiTheme::default()).unwrap();
    let mut duplicate = composition();
    duplicate.children.reverse();
    duplicate.children.push(UiNode::new(
        UiNodeId(90),
        UiControl::Button("duplicate".into()),
    ));
    assert!(matches!(
        tree.replace(duplicate),
        Err(UiCompositionError::DuplicateNode(UiNodeId(90)))
    ));
    assert!(tree.command(UiNodeId(123), UiCommand::Activate).is_err());
    assert!(tree.node(UiNodeId(123)).is_none());
    tree.command(UiNodeId(90), UiCommand::SetChecked(true))
        .unwrap();
    assert_eq!(
        tree.node(UiNodeId(u64::MAX)).unwrap().control,
        UiControl::Button("nested".into())
    );
    assert!(matches!(
        tree.node(UiNodeId(90)).unwrap().control,
        UiControl::Toggle { checked: true, .. }
    ));
}

#[test]
fn deepest_valid_node_supports_reads_and_mutation() {
    let mut root = UiNode::new(UiNodeId(u64::MAX), UiControl::Button("leaf".into()));
    for id in 0..63 {
        let mut parent = panel(id, [UiLength::Auto; 2], UiFlow::Column);
        parent.children.push(root);
        root = parent;
    }
    let mut tree = UiTree::new(root, UiTheme::default()).unwrap();
    assert_eq!(
        tree.node(UiNodeId(u64::MAX)).unwrap().control,
        UiControl::Button("leaf".into())
    );
    assert_eq!(
        tree.command(UiNodeId(u64::MAX), UiCommand::Activate)
            .unwrap(),
        UiEffect::Activated
    );
}

#[test]
fn widest_valid_tree_reorders_last_node_and_preserves_identity() {
    let mut root = panel(u64::MAX, [UiLength::Fill; 2], UiFlow::Column);
    for id in 0..4095 {
        root.children.push(UiNode::new(
            UiNodeId(id),
            UiControl::Button(format!("button-{id}")),
        ));
    }
    let mut tree = UiTree::new(root, UiTheme::default()).unwrap();
    assert_eq!(
        tree.node(UiNodeId(4094)).unwrap().control,
        UiControl::Button("button-4094".into())
    );
    let mut replacement = tree.root().clone();
    replacement.children.reverse();
    tree.replace(replacement).unwrap();
    tree.command(UiNodeId(4094), UiCommand::Visual(UiVisualState::Pressed))
        .unwrap();
    assert_eq!(tree.root().children[0].id, UiNodeId(4094));
    assert_eq!(
        tree.node(UiNodeId(4094)).unwrap().visual,
        UiVisualState::Pressed
    );
    assert_eq!(
        tree.node(UiNodeId(0)).unwrap().visual,
        UiVisualState::Normal
    );
}
