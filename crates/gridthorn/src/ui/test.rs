use super::*;

#[test]
fn multilingual_controls_wrap_and_repaint_at_dpi_without_changing_layout() {
    let assets = [
        include_bytes!("../../../gridthorn_render/src/text/test/fonts/NotoSans-Regular.ttf")
            .as_slice(),
        include_bytes!("../../../gridthorn_render/src/text/test/fonts/NotoSansArabic-Regular.ttf")
            .as_slice(),
        include_bytes!("../../../gridthorn_render/src/text/test/fonts/NotoSansJP-Regular.otf")
            .as_slice(),
    ]
    .map(|bytes| crate::FontAsset::from_bytes(bytes.to_vec()).unwrap());
    let mut service = crate::TextSystem::new("en-US", &assets).unwrap();
    let theme = UiTheme {
        text: Some(crate::TextStyle::new("Noto Sans", 20.0)),
        ..UiTheme::default()
    };
    let mut root = UiNode::new(UiNodeId(0), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    root.style.flow = UiFlow::Column;
    let mut label = UiNode::new(
        UiNodeId(1),
        UiControl::Label("Привет 日本語 العربية long wrapping text with combining e\u{301}".into()),
    );
    label.style.size[0] = UiLength::Fill;
    root.children.push(label);
    let tree = UiTree::new(root, theme).unwrap();
    assert!(tree.layout([160.0, 300.0], 1.0, None).is_err());
    let one = tree
        .layout([160.0, 300.0], 1.0, Some(&mut service))
        .unwrap();
    let two = tree
        .layout([160.0, 300.0], 2.0, Some(&mut service))
        .unwrap();
    assert_eq!(one.placements(), two.placements());
    assert!(one.placement(UiNodeId(1)).unwrap().bounds.size[1] > 28.0);
    assert_ne!(one.primitives(), two.primitives());
}

#[test]
fn facade_composes_controls_into_render_frame() {
    let mut root = UiNode::new(UiNodeId(0), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    root.style.flow = UiFlow::Column;
    root.children
        .push(UiNode::new(UiNodeId(1), UiControl::Button("Apply".into())));
    let mut tree = UiTree::new(root, UiTheme::default()).unwrap();
    assert_eq!(
        tree.command(UiNodeId(1), UiCommand::Activate).unwrap(),
        UiEffect::Activated
    );
    let layout = tree.layout([200.0; 2], 2.0, None).unwrap();
    let frame = crate::RenderFrame::default().with_ui(layout.into_primitives());
    assert_ne!(frame.ui(), []);
}
mod routing;
