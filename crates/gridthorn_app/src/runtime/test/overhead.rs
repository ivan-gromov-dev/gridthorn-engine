use std::hint::black_box;
use std::time::{Duration, Instant};

use gridthorn_simulation::FixedStepConfig;
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use super::ApplicationRuntime;

const BATCH_FRAMES: u32 = 1000;
const BATCHES: u32 = 100;

/// Manual release probe of runtime orchestration and empty system dispatch.
#[test]
#[ignore = "manual performance probe; run alone in release mode without runtime diagnostics"]
fn measure_empty_runtime_and_schedule_dispatch() {
    assert_eq!(std::env::var_os("GRIDTHORN_RUNTIME_PERFORMANCE"), None);
    assert!(
        !black_box(cfg!(debug_assertions)),
        "run this probe with --release"
    );
    println!("runtime_overhead,systems_per_stage,fixed_steps,batch,frames,elapsed_ns");
    for systems in [0, 1, 32] {
        for fixed_steps in [0, 1, 8] {
            measure(systems, fixed_steps);
        }
    }
}

fn measure(systems: u32, fixed_steps: u32) {
    let mut schedules = ScheduleBuilder::new();
    for stage in [
        ScheduleStage::PollEvents,
        ScheduleStage::Input,
        ScheduleStage::FixedUpdate,
        ScheduleStage::Update,
        ScheduleStage::PostUpdate,
        ScheduleStage::Render,
    ] {
        for _ in 0..systems {
            schedules.add_system(stage, |_| black_box(()));
        }
    }
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime.startup().unwrap();
    let elapsed = FixedStepConfig::default().fixed_step() * fixed_steps;
    run_batch(&mut runtime, elapsed, fixed_steps);
    for batch in 0..BATCHES {
        let start = Instant::now();
        run_batch(&mut runtime, elapsed, fixed_steps);
        let elapsed_ns = start.elapsed().as_nanos();
        println!("runtime_overhead,{systems},{fixed_steps},{batch},{BATCH_FRAMES},{elapsed_ns}");
    }
    runtime.shutdown();
}

fn run_batch(runtime: &mut ApplicationRuntime, elapsed: Duration, fixed_steps: u32) {
    for _ in 0..BATCH_FRAMES {
        let timing = black_box(runtime.run_timed_frame(black_box(elapsed)).unwrap());
        assert_eq!(timing.fixed_steps(), fixed_steps);
    }
}
