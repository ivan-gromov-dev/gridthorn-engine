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

#[test]
fn input_controls_affect_current_frame_and_presentation_keeps_running() {
    use gridthorn_simulation::{SimulationControl, SimulationSpeed};
    #[derive(Default)]
    struct Counts {
        input: u32,
        fixed: u32,
        presentation: u32,
    }
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.insert_resource(Counts::default());
    });
    schedules.add_system(ScheduleStage::Input, |world| {
        let frame = world
            .update_resource_with(|counts: &mut Counts| {
                counts.input += 1;
                counts.input
            })
            .unwrap();
        world.update_resource(|control: &mut SimulationControl| {
            if frame == 1 {
                control.pause();
            }
            if frame == 2 {
                control.resume();
                control.set_speed(SimulationSpeed::new(2, 1).unwrap());
            }
        });
    });
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        world.update_resource(|counts: &mut Counts| counts.fixed += 1);
    });
    for stage in [
        ScheduleStage::Update,
        ScheduleStage::PostUpdate,
        ScheduleStage::Render,
    ] {
        schedules.add_system(stage, |world| {
            world.update_resource(|counts: &mut Counts| counts.presentation += 1);
        });
    }
    let config = FixedStepConfig::new(Duration::from_millis(10), 4).unwrap();
    let mut runtime = ApplicationRuntime::with_fixed_step(schedules.build(), config);
    assert_eq!(
        runtime
            .run_timed_frame(Duration::from_secs(1))
            .unwrap()
            .fixed_steps(),
        0
    );
    assert_eq!(
        runtime
            .run_timed_frame(Duration::from_millis(10))
            .unwrap()
            .fixed_steps(),
        2
    );
    runtime.world().update_resource(SimulationControl::pause);
    runtime.run_frame(3).unwrap();
    assert_eq!(
        runtime
            .world()
            .read_resource(|c: &Counts| (c.fixed, c.presentation)),
        Some((5, 9))
    );
}
