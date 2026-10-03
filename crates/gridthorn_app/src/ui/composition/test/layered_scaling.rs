use std::{hint::black_box, time::Instant};

use gridthorn_input::{CursorPosition, InputEvent};

use super::*;

/// Manual probe of registered-layer routing with prepared bitmap text fields.
#[test]
#[ignore = "manual layered routing probe; run alone in release mode without UI diagnostics"]
fn measure_layered_text_routing_scaling() {
    assert_eq!(std::env::var_os("GRIDTHORN_UI_PERFORMANCE"), None);
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("layered_routing,state,fields,events,batch,calls,elapsed_ns");
    for count in [16, 128, 1024] {
        for state in ["closed", "open", "modal"] {
            for event_count in [0, 32] {
                measure(count, state, event_count);
            }
        }
    }
}

fn measure(count: u16, state: &str, event_count: u16) {
    let viewport = [300.0, f32::from(count) * 30.0 + 100.0];
    let mut root = panel(0, [UiLength::Fill; 2], UiFlow::Column);
    let mut layer = panel(1, [UiLength::Fill; 2], UiFlow::Column);
    for index in 0..count {
        let mut field = UiNode::new(
            UiNodeId(u64::from(index) + 2),
            UiControl::TextField {
                value: "Review 123".into(),
                placeholder: String::new(),
            },
        );
        field.style.size = [UiLength::Pixels(200.0), UiLength::Pixels(30.0)];
        layer.children.push(field);
    }
    root.children.push(layer);
    let mut tree = UiTree::new(root, UiTheme::default()).unwrap();
    let layout = tree.layout(viewport, 1.0, None).unwrap();
    assert_eq!(layout.text_geometry.len(), usize::from(count));
    let original = layout.clone();
    let mut router = UiRouter::new(0);
    router.register_layer(&tree, UiNodeId(1)).unwrap();
    if state != "closed" {
        router
            .open_layer(
                &mut tree,
                &layout,
                UiNodeId(1),
                UiLayer {
                    modal: state == "modal",
                    ..UiLayer::default()
                },
            )
            .unwrap();
    }
    let bounds = layout.placement(UiNodeId(2)).unwrap().bounds;
    let events: Vec<_> = (0..event_count)
        .map(|index| {
            InputEvent::CursorMoved(CursorPosition {
                x: f64::from(bounds.position[0]) + 10.0 + f64::from(index) * 0.01,
                y: f64::from(bounds.position[1]) + 10.0,
            })
        })
        .collect();
    batch(&mut tree, &mut router, &layout, &events);
    for index in 0..32 {
        let start = Instant::now();
        batch(&mut tree, &mut router, &layout, &events);
        let elapsed_ns = start.elapsed().as_nanos();
        println!("layered_routing,{state},{count},{event_count},{index},8,{elapsed_ns}");
    }
    let route = router.route_events(&mut tree, &layout, &events).unwrap();
    assert_eq!(route.effects, []);
    if state == "closed" {
        assert_eq!(route.consumed, []);
        assert_eq!(route.world_events, events);
    } else {
        assert_eq!(route.consumed, (0..events.len()).collect::<Vec<_>>());
        assert_eq!(route.world_events, []);
    }
    assert_eq!(layout.placements(), original.placements());
    assert_eq!(layout.primitives(), original.primitives());
    assert_eq!(layout.text_geometry[&UiNodeId(2)].value, "Review 123");
}

fn batch(tree: &mut UiTree, router: &mut UiRouter, layout: &UiLayout, events: &[InputEvent]) {
    for _ in 0..8 {
        black_box(
            router
                .route_events(tree, layout, black_box(events))
                .unwrap(),
        );
    }
}
