use std::sync::Arc;

use super::*;

/// Cloned layouts keep detached geometry alive without coupling later tree edits.
#[test]
fn cloned_text_geometry_survives_new_layout_and_releases_with_last_snapshot() {
    let mut root = panel(0, [UiLength::Fill; 2], UiFlow::Column);
    let mut layer = panel(1, [UiLength::Fill; 2], UiFlow::Column);
    layer.children.push(UiNode::new(
        UiNodeId(2),
        UiControl::TextField {
            value: "Review".into(),
            placeholder: String::new(),
        },
    ));
    root.children.push(layer);
    let mut tree = UiTree::new(root, UiTheme::default()).unwrap();
    let old = tree.layout([400.0; 2], 1.0, None).unwrap();
    let copy = old.clone();
    let retained = Arc::downgrade(&old.text_geometry);
    tree.command(UiNodeId(2), UiCommand::AppendText(" 123".into()))
        .unwrap();
    let mut router = UiRouter::new(0);
    router.register_layer(&tree, UiNodeId(1)).unwrap();
    router
        .open_layer(&mut tree, &old, UiNodeId(1), UiLayer::default())
        .unwrap();
    let fresh = router.layout(&tree, [400.0; 2], 2.0, None).unwrap();
    assert_eq!(copy.text_geometry[&UiNodeId(2)].value, "Review");
    assert_eq!(fresh.text_geometry[&UiNodeId(2)].value, "Review 123");
    router.route_events(&mut tree, &old, &[]).unwrap();
    drop(old);
    assert!(retained.upgrade().is_some());
    let paint = copy.into_primitives();
    assert!(retained.upgrade().is_none());
    assert_ne!(paint, []);
    assert_eq!(fresh.text_geometry[&UiNodeId(2)].value, "Review 123");
    let current = Arc::downgrade(&fresh.text_geometry);
    drop(fresh);
    assert!(current.upgrade().is_none());
    assert_eq!(router.open_layers(), [UiNodeId(1)]);
}
