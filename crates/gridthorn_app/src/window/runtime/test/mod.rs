use std::time::{Duration, Instant};

use gridthorn_simulation::FixedStepConfig;
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use crate::{ApplicationRuntime, WindowControl, WindowLifecycle};

use super::{FrameTimer, RuntimeWindowLifecycle};

mod exit;
mod input;
mod presentation;

#[derive(Default)]
struct StageCounts {
    startup: u32,
    fixed_update: u32,
    update: u32,
    shutdown: u32,
}

#[test]
fn drives_timed_runtime_and_shutdown_without_platform_types() {
    let mut schedules = ScheduleBuilder::new();
    schedules
        .add_system(ScheduleStage::Startup, |world| {
            world.insert_resource(StageCounts {
                startup: 1,
                ..StageCounts::default()
            });
        })
        .add_system(ScheduleStage::FixedUpdate, |world| {
            world.update_resource(|counts: &mut StageCounts| counts.fixed_update += 1);
        })
        .add_system(ScheduleStage::Update, |world| {
            world.update_resource(|counts: &mut StageCounts| counts.update += 1);
        })
        .add_system(ScheduleStage::Shutdown, |world| {
            world.update_resource(|counts: &mut StageCounts| counts.shutdown += 1);
        });
    let config = FixedStepConfig::new(Duration::from_millis(10), 4)
        .expect("test configuration should be valid");
    let runtime = ApplicationRuntime::with_fixed_step(schedules.build(), config);
    let mut lifecycle = RuntimeWindowLifecycle::new(runtime);

    lifecycle
        .started(&mut WindowControl::default())
        .expect("startup should run");
    lifecycle
        .run_elapsed_frame(Duration::from_millis(25))
        .expect("timed frame should run");
    lifecycle.shutdown();

    let counts = lifecycle
        .runtime
        .world()
        .read_resource(|counts: &StageCounts| {
            (
                counts.startup,
                counts.fixed_update,
                counts.update,
                counts.shutdown,
            )
        });
    assert_eq!(counts, Some((1, 2, 1, 1)));
}

#[test]
fn excludes_suspended_time_from_the_next_frame() {
    let start = Instant::now();
    let mut timer = FrameTimer::default();
    timer.start(start);
    assert_eq!(
        timer.advance(start + Duration::from_millis(7)),
        Duration::from_millis(7)
    );

    timer.suspend();
    assert_eq!(
        timer.advance(start + Duration::from_secs(5)),
        Duration::ZERO
    );
}
