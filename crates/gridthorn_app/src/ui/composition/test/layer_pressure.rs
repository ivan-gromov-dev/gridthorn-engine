use std::{hint::black_box, time::Instant};

use gridthorn_input::{CursorPosition, InputEvent};

use super::*;

/// Prepared bitmap-field routing and layout across many overlapping open layers.
#[test]
#[ignore = "manual expanded layers probe; run alone in release mode without diagnostics"]
fn measure_expanded_layer_pressure() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    assert_eq!(std::env::var_os("GRIDTHORN_UI_PERFORMANCE"), None);
    println!("layer_pressure,layers,fields_per_layer,modal,operation,sample,elapsed_ns");
    for layers in [3, 16, 64] {
        for fields in [1, 16] {
            for modal in [false, true] {
                measure(layers, fields, modal);
            }
        }
    }
}

fn authored_tree(layers: u16, fields: u16) -> UiTree {
    let mut root = panel(0, [UiLength::Fill; 2], UiFlow::Overlay);
    for layer in 1..=layers {
        let mut node = panel(
            u64::from(layer) * 100,
            [UiLength::Pixels(600.0), UiLength::Pixels(600.0)],
            UiFlow::Column,
        );
        node.style.offset = [f32::from(layer), f32::from(layer)];
        node.style.clip = true;
        for field in 1..=fields {
            let mut child = UiNode::new(
                UiNodeId(u64::from(layer) * 100 + u64::from(field)),
                UiControl::TextField {
                    value: "Review 123".into(),
                    placeholder: String::new(),
                },
            );
            child.style.size = [UiLength::Pixels(500.0), UiLength::Pixels(30.0)];
            node.children.push(child);
        }
        root.children.push(node);
    }
    UiTree::new(root, UiTheme::default()).unwrap()
}

fn measure(layers: u16, fields: u16, modal: bool) {
    let mut tree = authored_tree(layers, fields);
    let raw = tree.layout([1000.0, 800.0], 1.0, None).unwrap();
    let mut router = UiRouter::new(0);
    for layer in 1..=layers {
        router
            .open_layer(
                &mut tree,
                &raw,
                UiNodeId(u64::from(layer) * 100),
                UiLayer {
                    modal: modal && layer == layers,
                    ..UiLayer::default()
                },
            )
            .unwrap();
    }
    let mut layout = router.layout(&tree, [1000.0, 800.0], 1.0, None).unwrap();
    let top = UiNodeId(u64::from(layers) * 100 + 1);
    let bounds = layout.placement(top).unwrap().bounds;
    let events: Vec<_> = (0..32)
        .map(|index| {
            InputEvent::CursorMoved(CursorPosition {
                x: f64::from(bounds.position[0]) + 10.0 + f64::from(index) * 0.01,
                y: f64::from(bounds.position[1]) + 10.0,
            })
        })
        .collect();
    for sample in 0..60 {
        let start = Instant::now();
        let route = black_box(router.route_events(&mut tree, &layout, &events).unwrap());
        let routing = start.elapsed().as_nanos();
        assert_eq!(route.consumed, (0..32).collect::<Vec<_>>());
        assert_eq!(route.world_events, []);
        let start = Instant::now();
        layout = black_box(router.layout(&tree, [1000.0, 800.0], 1.0, None).unwrap());
        let painting = start.elapsed().as_nanos();
        assert_eq!(
            router.hit_test_layers(
                &tree,
                &layout,
                [bounds.position[0] + 10.0, bounds.position[1] + 10.0]
            ),
            Some(top)
        );
        if sample >= 10 {
            for (operation, elapsed) in [("pointer32", routing), ("layout", painting)] {
                println!(
                    "layer_pressure,{layers},{fields},{modal},{operation},{},{elapsed}",
                    sample - 10
                );
            }
        }
    }
    assert_eq!(router.open_layers().len(), usize::from(layers));
}
