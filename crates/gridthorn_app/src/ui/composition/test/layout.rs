use super::*;

#[test]
fn column_constraints_preserve_overflow_and_scroll_recovers_after_resize() {
    let mut root = panel(0, [UiLength::Fill; 2], UiFlow::Column);
    root.style.scroll = true;
    root.scroll_offset = [0.0, 100.0];
    let mut fixed = panel(1, [UiLength::Fill, UiLength::Pixels(80.0)], UiFlow::Overlay);
    fixed.style.min_size[1] = 90.0;
    root.children = vec![
        fixed,
        panel(2, [UiLength::Fill, UiLength::Pixels(50.0)], UiFlow::Overlay),
    ];
    let tree = UiTree::new(root, UiTheme::default()).unwrap();
    let small = tree.layout([100.0; 2], 1.0, None).unwrap();
    assert_eq!(
        small.placement(UiNodeId(0)).unwrap().scroll_offset,
        [0.0, 40.0]
    );
    assert_eq!(small.placement(UiNodeId(1)).unwrap().bounds.size[1], 90.0);
    let large = tree.layout([100.0, 200.0], 1.0, None).unwrap();
    assert_eq!(
        large.placement(UiNodeId(0)).unwrap().scroll_offset,
        [0.0; 2]
    );
}

#[test]
fn row_distributes_fill_after_fixed_children_and_padding() {
    let mut root = panel(0, [UiLength::Fill; 2], UiFlow::Row);
    root.style.padding = [10.0; 4];
    root.style.gap = 5.0;
    root.children = vec![
        panel(1, [UiLength::Pixels(50.0), UiLength::Fill], UiFlow::Overlay),
        panel(2, [UiLength::Fill; 2], UiFlow::Overlay),
        panel(3, [UiLength::Fill; 2], UiFlow::Overlay),
    ];
    let tree = UiTree::new(root, UiTheme::default()).unwrap();
    let layout = tree.layout([300.0, 100.0], 1.0, None).unwrap();
    assert_eq!(
        layout.placement(UiNodeId(1)).unwrap().bounds,
        UiBounds {
            position: [10.0, 10.0],
            size: [50.0, 80.0]
        }
    );
    assert_eq!(
        layout.placement(UiNodeId(2)).unwrap().bounds,
        UiBounds {
            position: [65.0, 10.0],
            size: [110.0, 80.0]
        }
    );
    assert_eq!(
        layout.placement(UiNodeId(3)).unwrap().bounds.position,
        [180.0, 10.0]
    );
}

#[test]
fn anchor_fraction_constraints_and_resize() {
    let mut root = panel(0, [UiLength::Fill; 2], UiFlow::Overlay);
    let mut child = panel(
        1,
        [UiLength::Fraction(0.5), UiLength::Pixels(20.0)],
        UiFlow::Overlay,
    );
    child.style.max_size[0] = 120.0;
    child.style.anchor = [UiAnchor::End, UiAnchor::Center];
    child.style.offset = [-5.0, 3.0];
    root.children.push(child);
    let tree = UiTree::new(root, UiTheme::default()).unwrap();
    for (width, expected) in [(200.0, 100.0), (400.0, 120.0)] {
        let layout = tree.layout([width, 100.0], 2.0, None).unwrap();
        let bounds = layout.placement(UiNodeId(1)).unwrap().bounds;
        assert_eq!(bounds.size, [expected, 20.0]);
        assert_eq!(bounds.position, [width - expected - 5.0, 43.0]);
    }
}

#[test]
fn nested_scroll_clamps_and_propagates_clip() {
    let mut root = panel(0, [UiLength::Pixels(100.0); 2], UiFlow::Column);
    root.style.scroll = true;
    root.style.padding = [10.0; 4];
    root.scroll_offset = [0.0, 500.0];
    let mut child = panel(
        1,
        [UiLength::Pixels(80.0), UiLength::Pixels(200.0)],
        UiFlow::Overlay,
    );
    child.style.clip = true;
    child
        .children
        .push(panel(2, [UiLength::Pixels(100.0); 2], UiFlow::Overlay));
    root.children.push(child);
    let layout = UiTree::new(root, UiTheme::default())
        .unwrap()
        .layout([100.0; 2], 1.0, None)
        .unwrap();
    assert_eq!(
        layout.placement(UiNodeId(0)).unwrap().scroll_offset,
        [0.0, 120.0]
    );
    assert_eq!(
        layout.placement(UiNodeId(1)).unwrap().bounds.position,
        [10.0, -110.0]
    );
    assert_eq!(
        layout.placement(UiNodeId(2)).unwrap().clip,
        UiBounds {
            position: [10.0; 2],
            size: [80.0; 2]
        }
    );
}

#[test]
fn intrinsic_column_and_empty_viewport() {
    let mut root = panel(0, [UiLength::Auto; 2], UiFlow::Column);
    root.style.gap = 4.0;
    root.children = vec![
        UiNode::new(UiNodeId(1), UiControl::Label("ABC".into())),
        UiNode::new(UiNodeId(2), UiControl::Label("A\nB".into())),
    ];
    let tree = UiTree::new(root, UiTheme::default()).unwrap();
    assert_eq!(
        tree.layout([200.0; 2], 1.0, None)
            .unwrap()
            .placement(UiNodeId(0))
            .unwrap()
            .bounds
            .size,
        [36.0, 52.0]
    );
    assert_eq!(tree.layout([0.0; 2], 1.0, None).unwrap().primitives(), []);
}
