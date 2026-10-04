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

/// Phase attribution excludes fixture generation, assertions and CSV output.
#[test]
#[ignore = "manual input phase probe; run alone in release mode"]
fn measure_input_burst_phases() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    assert_eq!(std::env::var_os("GRIDTHORN_RUNTIME_PERFORMANCE"), None);
    println!("input_phase,kind,events,operation,sample,elapsed_ns");
    for kind in ["pointer", "keyboard", "commit", "composition", "mixed"] {
        for count in [1024, 16_384, 65_536] {
            let events = (0..count)
                .map(|index| phase_event(kind, index))
                .collect::<Vec<_>>();
            let payload_bytes = events.iter().map(payload_bytes).sum::<usize>();
            println!(
                "input_storage,{kind},{count},{payload_bytes},{}",
                std::mem::size_of::<InputEvent>()
            );
            let mut input = gridthorn_input::InputBuffer::new();
            let mut runtime = ApplicationRuntime::new(ScheduleBuilder::new().build());
            runtime.startup().unwrap();
            for sample in 0..51 {
                let start = Instant::now();
                let owned = events.clone();
                let clone = start.elapsed().as_nanos();
                let start = Instant::now();
                for event in owned {
                    input.push(black_box(event));
                }
                let ingest = start.elapsed().as_nanos();
                let start = Instant::now();
                let state = input.snapshot();
                let snapshot = start.elapsed().as_nanos();
                assert_eq!(state.events(), events);
                let start = Instant::now();
                runtime.world().insert_resource(state);
                let publish = start.elapsed().as_nanos();
                let start = Instant::now();
                runtime.run_timed_frame(Duration::ZERO).unwrap();
                let dispatch = start.elapsed().as_nanos();
                for (operation, elapsed) in [
                    ("fixture_clone", clone),
                    ("ingest", ingest),
                    ("snapshot", snapshot),
                    ("publish_drop_previous", publish),
                    ("dispatch", dispatch),
                ] {
                    println!("input_phase,{kind},{count},{operation},{sample},{elapsed}");
                }
            }
            assert_eq!(input.snapshot().events(), []);
            runtime.shutdown();
        }
    }
}

fn phase_event(kind: &str, index: usize) -> InputEvent {
    match kind {
        "mixed" => phase_event(
            ["pointer", "keyboard", "commit", "composition"][index % 4],
            index,
        ),
        "composition" => {
            let text = format!("preedit-{index} 日本語 e\u{301}");
            let end = text.len();
            InputEvent::Text(TextInputEvent::Composition {
                text,
                cursor: Some((0, end)),
            })
        }
        _ => event(kind, index),
    }
}

fn payload_bytes(event: &InputEvent) -> usize {
    match event {
        InputEvent::Text(
            TextInputEvent::Commit(text) | TextInputEvent::Composition { text, .. },
        )
        | InputEvent::Key(KeyboardEvent {
            logical_key: LogicalKey::Character(text),
            ..
        }) => text.len(),
        _ => 0,
    }
}
