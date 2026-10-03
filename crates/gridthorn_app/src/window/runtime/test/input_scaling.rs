use std::hint::black_box;
use std::time::{Duration, Instant};

use gridthorn_input::{
    ButtonState, InputEvent, InputState, KeyCode, KeyLocation, KeyboardEvent, LogicalKey,
    PhysicalKey, TextInputEvent,
};
use gridthorn_world::ScheduleBuilder;

use crate::{ApplicationRuntime, WindowLifecycle};

use super::RuntimeWindowLifecycle;

const FRAMES: u32 = 8;
const BATCHES: u32 = 32;

/// Manual release probe of event ingestion, snapshots and runtime publication.
#[test]
#[ignore = "manual input scaling probe; run alone in release mode without runtime diagnostics"]
fn measure_input_publication_scaling() {
    assert_eq!(std::env::var_os("GRIDTHORN_RUNTIME_PERFORMANCE"), None);
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("input_runtime,kind,events,text_bytes,batch,frames,elapsed_ns");
    for kind in ["pointer", "keyboard", "commit"] {
        for count in [0, 32, 1024, 16_384] {
            measure(kind, count);
        }
    }
}

fn measure(kind: &str, count: usize) {
    let events = (0..count)
        .map(|index| event(kind, index))
        .collect::<Vec<_>>();
    let text_bytes = events
        .iter()
        .map(|event| match event {
            InputEvent::Text(TextInputEvent::Commit(text)) => text.len(),
            InputEvent::Key(key) => match &key.logical_key {
                LogicalKey::Character(text) => text.len(),
                _ => 0,
            },
            _ => 0,
        })
        .sum::<usize>();
    let runtime = ApplicationRuntime::new(ScheduleBuilder::new().build());
    let mut lifecycle = RuntimeWindowLifecycle::new(runtime);
    lifecycle.runtime.startup().unwrap();
    run_batch(&mut lifecycle, &events);
    for batch in 0..BATCHES {
        let start = Instant::now();
        run_batch(&mut lifecycle, &events);
        let elapsed_ns = start.elapsed().as_nanos();
        println!("input_runtime,{kind},{count},{text_bytes},{batch},{FRAMES},{elapsed_ns}");
        assert_eq!(
            lifecycle
                .runtime
                .world()
                .read_resource(|state: &InputState| state.events() == events),
            Some(true),
        );
    }
    assert_eq!(lifecycle.input.snapshot().events(), []);
    lifecycle.shutdown();
}

fn run_batch(lifecycle: &mut RuntimeWindowLifecycle, events: &[InputEvent]) {
    for _ in 0..FRAMES {
        for event in events {
            lifecycle.input(black_box(event.clone())).unwrap();
        }
        lifecycle.run_elapsed_frame(Duration::ZERO).unwrap();
    }
}

fn event(kind: &str, index: usize) -> InputEvent {
    match kind {
        "pointer" => InputEvent::PointerMotion {
            x: f64::from(u32::try_from(index).unwrap()),
            y: -1.0,
        },
        "keyboard" => InputEvent::Key(KeyboardEvent {
            physical_key: PhysicalKey::Code(KeyCode::KeyD),
            logical_key: LogicalKey::Character("d".into()),
            location: KeyLocation::Standard,
            state: if index.is_multiple_of(2) {
                ButtonState::Pressed
            } else {
                ButtonState::Released
            },
            repeat: false,
            synthetic: false,
        }),
        "commit" => InputEvent::Text(TextInputEvent::Commit(format!(
            "edit-{index} Привет العربية 日本語 e\u{301}"
        ))),
        _ => unreachable!("probe kind is fixed by the test"),
    }
}
