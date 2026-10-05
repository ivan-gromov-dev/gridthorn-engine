use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use gridthorn_simulation::{FixedStepConfig, GameCommandQueue, SimulationControl};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use super::ApplicationRuntime;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Agent {
    position: i64,
    stock: [u64; 8],
}

fn populated(count: usize) -> ApplicationRuntime {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        let sum = world
            .update_resource_with(|queue: &mut GameCommandQueue<u64>| queue.drain().sum::<u64>())
            .unwrap();
        world.for_each_component_mut::<Agent>(|_, agent| {
            agent.position += 1;
            agent.stock[0] += sum;
        });
    });
    let mut runtime = ApplicationRuntime::with_fixed_step(
        schedules.build(),
        FixedStepConfig::new(Duration::from_millis(10), 8).unwrap(),
    );
    runtime
        .world()
        .insert_resource(GameCommandQueue::<u64>::new());
    for index in 0..count {
        runtime.world().spawn(Agent {
            position: i64::try_from(index).unwrap(),
            stock: [0; 8],
        });
    }
    runtime
        .world()
        .update_resource(|queue: &mut GameCommandQueue<u64>| {
            for command in 1..=65_536 {
                queue.push(command);
            }
        });
    runtime
}

fn catch_up(runtime: &mut ApplicationRuntime) -> usize {
    runtime.world().update_resource(SimulationControl::pause);
    assert_eq!(
        runtime
            .run_timed_frame(Duration::from_secs(60))
            .unwrap()
            .fixed_steps(),
        0
    );
    assert_eq!(
        runtime.world().read_resource(GameCommandQueue::<u64>::len),
        Some(65_536)
    );
    runtime.world().update_resource(SimulationControl::resume);
    let first = runtime
        .run_timed_frame(Duration::from_millis(1280))
        .unwrap();
    assert!(first.overloaded());
    assert_eq!(first.fixed_steps(), 8);
    let mut frames = 1;
    let mut report = first;
    while report.accumulated_lag() >= Duration::from_millis(10) {
        report = runtime.run_timed_frame(Duration::ZERO).unwrap();
        frames += 1;
    }
    assert_eq!(report.completed_ticks(), 128);
    assert_eq!(report.accumulated_lag(), Duration::ZERO);
    assert_eq!(frames, 16);
    frames
}

fn agents(runtime: &mut ApplicationRuntime) -> Vec<Agent> {
    let mut agents = Vec::new();
    runtime
        .world()
        .for_each_component_mut::<Agent>(|_, agent| agents.push(agent.clone()));
    agents
}

#[test]
fn ecs_catch_up_preserves_ticks_and_consumes_paused_burst_once() {
    let mut timed = populated(256);
    let mut exact = populated(256);
    catch_up(&mut timed);
    exact.run_frame(128).unwrap();
    assert_eq!(agents(&mut timed), agents(&mut exact));
    assert!(
        agents(&mut timed)
            .iter()
            .all(|agent| agent.stock[0] == 2_147_516_416)
    );
    assert_eq!(
        timed.world().read_resource(GameCommandQueue::<u64>::len),
        Some(0)
    );
}

/// Caller-owned queued burst and sixteen capped frames of actual ECS mutations.
#[test]
#[ignore = "manual ECS catch-up CPU/heap probe; run alone with --release"]
fn measure_ecs_catch_up() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let heap = std::env::var_os("GRIDTHORN_DOMAIN_HEAP").is_some();
    for count in [1024, 16_384, 65_536] {
        for batch in 0..if heap { 1 } else { 22 } {
            let profile = heap.then(|| {
                dhat::Profiler::builder()
                    .testing()
                    .trim_backtraces(Some(4))
                    .build()
            });
            let mut runtime = populated(count);
            let start = Instant::now();
            catch_up(&mut runtime);
            let elapsed = start.elapsed().as_nanos();
            let stats = profile.as_ref().map(|_| dhat::HeapStats::get());
            drop(runtime);
            let released = profile.as_ref().map(|_| dhat::HeapStats::get().curr_bytes);
            drop(profile);
            if batch >= 2 || heap {
                println!("ecs_catch_up,{count},{batch},{elapsed}");
            }
            if let Some(stats) = stats {
                println!(
                    "ecs_catch_up_heap,{count},{},{},{},{},{}",
                    stats.total_blocks,
                    stats.total_bytes,
                    stats.max_bytes,
                    stats.curr_bytes,
                    released.unwrap()
                );
            }
        }
    }
}
