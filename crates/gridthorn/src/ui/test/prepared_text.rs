use super::super::*;
use crate::{Color, FontAsset, RasterText, TextStyle, TextSystem, UiPrimitive, UiRect};

fn fonts() -> TextSystem {
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
    .map(|bytes| FontAsset::from_bytes(bytes.to_vec()).unwrap());
    TextSystem::new("en-US", &assets).unwrap()
}

fn collect(primitives: &[UiPrimitive], output: &mut Vec<RasterText>) {
    for primitive in primitives {
        match primitive {
            UiPrimitive::Clipped { children, .. } => collect(children, output),
            UiPrimitive::ShapedText(text) => output.push(text.clone()),
            _ => {}
        }
    }
}

#[test]
fn expanded_pass_matches_independent_rasters_after_edits_width_and_dpi_changes() {
    let mut root = UiNode::new(UiNodeId(0), UiControl::Panel);
    root.style.size = [UiLength::Fill; 2];
    root.style.flow = UiFlow::Column;
    root.style.scroll = true;
    for id in 1..=128 {
        let value = format!(
            "{} {id}",
            ["office e\u{301}", "Привет", "مرحبا 123", "文章を確認する"]
                [usize::try_from(id % 4).unwrap()]
        );
        let mut field = UiNode::new(
            UiNodeId(id),
            UiControl::TextField {
                value,
                placeholder: String::new(),
            },
        );
        field.style.size = [UiLength::Fill, UiLength::Pixels(36.0)];
        root.children.push(field);
    }
    let mut tree = UiTree::new(
        root,
        UiTheme {
            text: Some(TextStyle::new("Noto Sans", 20.0)),
            ..UiTheme::default()
        },
    )
    .unwrap();
    let mut service = fonts();
    let router = UiRouter::new(0);
    let old = router
        .layout(&tree, [600.0, 6000.0], 1.0, Some(&mut service))
        .unwrap();
    tree.command(UiNodeId(1), UiCommand::SetText("changed 日本語".into()))
        .unwrap();
    for (width, dpi) in [(600.0, 1.0), (120.0, 2.0)] {
        let layout = router
            .layout(&tree, [width, 6000.0], dpi, Some(&mut service))
            .unwrap();
        let mut actual = Vec::new();
        collect(layout.primitives(), &mut actual);
        assert_eq!(actual.len(), 128);
        let mut reference = fonts();
        for (index, raster) in actual.iter().enumerate() {
            let node = tree
                .node(UiNodeId(u64::try_from(index + 1).unwrap()))
                .unwrap();
            let UiControl::TextField { value, .. } = &node.control else {
                panic!("field")
            };
            let placement = layout.placement(node.id).unwrap();
            let mut style = TextStyle::new("Noto Sans", 20.0);
            style.width = Some(placement.content.size[0]);
            let shaped = reference.layout(value, &style).unwrap();
            let expected = reference
                .rasterize_clipped(
                    &shaped,
                    dpi,
                    UiTheme::default().foreground,
                    UiRect::new(
                        placement
                            .content
                            .position
                            .map(|coordinate| -coordinate * dpi),
                        [width * dpi, 6000.0 * dpi],
                        Color::default(),
                    )
                    .unwrap(),
                )
                .unwrap()
                .at(placement.content.position)
                .unwrap();
            assert_eq!(*raster, expected);
        }
    }
    drop(service);
    assert_ne!(old.primitives(), []);
    assert!(tree.layout([600.0, 6000.0], 1.0, None).is_err());
}
