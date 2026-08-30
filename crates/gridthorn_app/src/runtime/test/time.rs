use std::time::Duration;

use gridthorn_simulation::{FixedStepConfig, FixedTime};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use super::ApplicationRuntime;

#[derive(Default)]
struct TickTrace(Vec<(u64, Duration)>);

#[test]
fn timed_frames_expose_integer_tick_time_to_fixed_updates() {
    let mut schedules = ScheduleBuilder::new();
    schedules
        .add_system(ScheduleStage::Startup, |world| {
            world.insert_resource(TickTrace::default());
        })
        .add_system(ScheduleStage::FixedUpdate, |world| {
            let fixed_time = world
                .read_resource(|time: &FixedTime| *time)
                .expect("fixed time should exist during FixedUpdate");
            world.update_resource(|trace: &mut TickTrace| {
                trace
                    .0
                    .push((fixed_time.tick_index(), fixed_time.fixed_step()));
            });
        });
    let config = FixedStepConfig::new(Duration::from_millis(10), 4)
        .expect("test configuration should be valid");
    let mut runtime = ApplicationRuntime::with_fixed_step(schedules.build(), config);

    let partial = runtime
        .run_timed_frame(Duration::from_millis(6))
        .expect("partial frame should run");
    let advanced = runtime
        .run_timed_frame(Duration::from_millis(15))
        .expect("accumulated frame should run");

    assert_eq!(partial.fixed_steps(), 0);
    assert_eq!(advanced.fixed_steps(), 2);
    assert_eq!(advanced.completed_ticks(), 2);
    assert_eq!(
        runtime
            .world()
            .read_resource(|trace: &TickTrace| trace.0.clone()),
        Some(vec![
            (0, Duration::from_millis(10)),
            (1, Duration::from_millis(10)),
        ])
    );
}
