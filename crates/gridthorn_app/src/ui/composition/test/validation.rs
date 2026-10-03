use super::*;

#[test]
fn rejected_replacement_preserves_tree() {
    let mut tree = UiTree::new(
        panel(0, [UiLength::Fill; 2], UiFlow::Overlay),
        UiTheme::default(),
    )
    .unwrap();
    let mut duplicate = tree.root().clone();
    duplicate.children.push(duplicate.clone());
    assert!(matches!(
        tree.replace(duplicate),
        Err(UiCompositionError::DuplicateNode(_))
    ));
    assert!(tree.root().children.is_empty());
    let mut invalid = tree.root().clone();
    invalid.style.padding[1] = f32::NAN;
    assert!(tree.replace(invalid).is_err());
    assert!(tree.layout([f32::INFINITY, 10.0], 1.0, None).is_err());
    assert!(tree.layout([100.0; 2], 0.0, None).is_err());
    assert!(tree.command(UiNodeId(99), UiCommand::Activate).is_err());
}

#[test]
fn depth_and_control_contracts_are_bounded() {
    let mut node = panel(0, [UiLength::Auto; 2], UiFlow::Overlay);
    for id in 1..65 {
        let mut parent = panel(id, [UiLength::Auto; 2], UiFlow::Overlay);
        parent.children.push(node);
        node = parent;
    }
    assert!(matches!(
        UiTree::new(node, UiTheme::default()),
        Err(UiCompositionError::TooLarge)
    ));
    let node = UiNode::new(
        UiNodeId(0),
        UiControl::Slider {
            min: -f32::MAX,
            max: f32::MAX,
            value: 0.0,
        },
    );
    assert!(UiTree::new(node, UiTheme::default()).is_err());
}
