use crate::prelude::*;

mod command;
mod input;
mod time;

#[derive(Default)]
struct TickCount(u32);

#[test]
fn prelude_builds_and_runs_a_game_lifecycle() {
    let mut schedules = ScheduleBuilder::new();
    schedules
        .add_system(ScheduleStage::Startup, |world| {
            world.insert_resource(TickCount::default());
        })
        .add_system(ScheduleStage::FixedUpdate, |world| {
            world.update_resource(|ticks: &mut TickCount| ticks.0 += 1);
        });
    let mut application = ApplicationRuntime::new(schedules.build());

    application.run_frame(3).expect("frame should run");

    assert_eq!(
        application
            .world()
            .read_resource(|ticks: &TickCount| ticks.0),
        Some(3)
    );
}
