use std::hint::black_box;
use std::time::Instant;

use gridthorn_input::{CursorPosition, InputEvent};

use super::*;

const CALLS: u32 = 8;
const BATCHES: u32 = 32;

/// Manual release probe of flat bitmap-button layout and atomic pointer routing.
#[test]
#[ignore = "manual UI scaling probe; run alone in release mode without UI diagnostics"]
fn measure_control_and_pointer_routing_scaling() {
    assert_eq!(std::env::var_os("GRIDTHORN_UI_PERFORMANCE"), None);
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("ui_scaling,operation,controls,events,batch,calls,elapsed_ns");
    for count in [16, 128, 1024] {
        for operation in [
            "construction",
            "layout",
            "route_empty",
            "route_first",
            "route_last",
            "route_miss",
        ] {
            measure(count, operation);
        }
    }
}

fn measure(count: u16, operation: &str) {
    let mut root = panel(0, [UiLength::Fill; 2], UiFlow::Column);
    for index in 1..=count {
        let mut node = UiNode::new(
            UiNodeId(u64::from(index)),
            UiControl::Button("Apply".into()),
        );
        node.style.size = [UiLength::Pixels(100.0), UiLength::Pixels(30.0)];
        root.children.push(node);
    }
    let mut tree = UiTree::new(root, UiTheme::default()).unwrap();
    let viewport = [200.0, f32::from(count) * 30.0 + 100.0];
    let layout = tree.layout(viewport, 1.0, None).unwrap();
    assert_eq!(layout.placements().len(), usize::from(count) + 1);
    let mut router = UiRouter::new(100);
    let target = if operation == "route_last" { count } else { 1 };
    let bounds = layout
        .placement(UiNodeId(u64::from(target)))
        .unwrap()
        .bounds;
    let point = if operation == "route_miss" {
        [-5.0, -5.0]
    } else {
        [
            f64::from(bounds.position[0]) + 10.0,
            f64::from(bounds.position[1]) + 10.0,
        ]
    };
    let events = if matches!(operation, "construction" | "layout" | "route_empty") {
        Vec::new()
    } else {
        (0..32)
            .map(|index| {
                InputEvent::CursorMoved(CursorPosition {
                    x: point[0] + f64::from(index) * 0.01,
                    y: point[1],
                })
            })
            .collect::<Vec<_>>()
    };
    run_batch(
        &mut tree,
        &mut router,
        &layout,
        viewport,
        &events,
        operation,
    );
    for batch in 0..BATCHES {
        let start = Instant::now();
        run_batch(
            &mut tree,
            &mut router,
            &layout,
            viewport,
            &events,
            operation,
        );
        let elapsed_ns = start.elapsed().as_nanos();
        println!(
            "ui_scaling,{operation},{count},{},{batch},{CALLS},{elapsed_ns}",
            events.len()
        );
    }
    if matches!(operation, "construction" | "layout") {
        assert_eq!(
            tree.layout(viewport, 1.0, None).unwrap().placements().len(),
            usize::from(count) + 1
        );
    } else {
        let route = router.route_events(&mut tree, &layout, &events).unwrap();
        assert_eq!(route.effects, []);
        if matches!(operation, "route_first" | "route_last") {
            assert_eq!(route.consumed, (0..events.len()).collect::<Vec<_>>());
            assert_eq!(route.world_events, []);
            assert_eq!(
                UiRouter::hit_test(
                    &tree,
                    &layout,
                    [bounds.position[0] + 10.0, bounds.position[1] + 10.0]
                ),
                Some(UiNodeId(u64::from(target)))
            );
        } else {
            assert_eq!(route.consumed, []);
            assert_eq!(route.world_events, events);
        }
    }
}

fn run_batch(
    tree: &mut UiTree,
    router: &mut UiRouter,
    layout: &UiLayout,
    viewport: [f32; 2],
    events: &[InputEvent],
    operation: &str,
) {
    for _ in 0..CALLS {
        if operation == "construction" {
            black_box(UiTree::new(tree.root().clone(), UiTheme::default()).unwrap());
        } else if operation == "layout" {
            black_box(tree.layout(viewport, 1.0, None).unwrap());
        } else {
            black_box(
                router
                    .route_events(tree, layout, black_box(events))
                    .unwrap(),
            );
        }
    }
}
