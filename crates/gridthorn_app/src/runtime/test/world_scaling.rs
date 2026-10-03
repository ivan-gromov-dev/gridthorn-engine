use std::hint::black_box;
use std::time::{Duration, Instant};

use gridthorn_simulation::FixedStepConfig;
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use super::ApplicationRuntime;

const FRAMES: u32 = 32;
const BATCHES: u32 = 50;

struct Counter(u64);

/// Manual release probe of resident entities and a homogeneous fixed-tick traversal.
#[test]
#[ignore = "manual scaling probe; run alone in release mode without runtime diagnostics"]
fn measure_populated_world_scaling() {
    assert_eq!(std::env::var_os("GRIDTHORN_RUNTIME_PERFORMANCE"), None);
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("world_runtime,mode,entities,batch,frames,elapsed_ns");
    for entities in [0, 1000, 10_000, 100_000] {
        for scan in [false, true] {
            measure(entities, scan);
        }
    }
}

fn measure(entities: usize, scan: bool) {
    let mut schedules = ScheduleBuilder::new();
    if scan {
        schedules.add_system(ScheduleStage::FixedUpdate, |world| {
            world.for_each_component_mut(|_, counter: &mut Counter| counter.0 += 1);
        });
    }
    let mut schedules = schedules.build();
    for _ in 0..entities {
        schedules.world().spawn(Counter(0));
    }
    let mut runtime = ApplicationRuntime::new(schedules);
    runtime.startup().unwrap();
    let elapsed = if scan {
        FixedStepConfig::default().fixed_step()
    } else {
        Duration::ZERO
    };
    run_batch(&mut runtime, elapsed);
    let mode = if scan { "fixed_scan" } else { "resident_idle" };
    for batch in 0..BATCHES {
        let start = Instant::now();
        run_batch(&mut runtime, elapsed);
        let elapsed_ns = start.elapsed().as_nanos();
        println!("world_runtime,{mode},{entities},{batch},{FRAMES},{elapsed_ns}");
    }
    let expected = if scan {
        u64::from(FRAMES * (BATCHES + 1))
    } else {
        0
    };
    let mut visited = 0;
    runtime
        .world()
        .for_each_component_mut(|_, counter: &mut Counter| {
            assert_eq!(counter.0, expected);
            visited += 1;
        });
    assert_eq!(visited, entities);
    runtime.shutdown();
}

fn run_batch(runtime: &mut ApplicationRuntime, elapsed: Duration) {
    for _ in 0..FRAMES {
        black_box(runtime.run_timed_frame(black_box(elapsed)).unwrap());
    }
}
