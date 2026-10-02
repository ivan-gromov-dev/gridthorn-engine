use super::*;
use gridthorn_world::{ScheduleBuilder, ScheduleStage};
use std::time::Duration;

fn runner(stop: u64) -> HeadlessSimulation {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.insert_resource(Vec::<u64>::new());
    });
    for stage in [
        ScheduleStage::PollEvents,
        ScheduleStage::Update,
        ScheduleStage::PostUpdate,
        ScheduleStage::Render,
    ] {
        schedules.add_system(stage, |_| panic!("presentation must not run"));
    }
    schedules.add_system(ScheduleStage::FixedUpdate, move |world| {
        let tick = world
            .read_resource(|time: &gridthorn_simulation::FixedTime| time.tick_index())
            .unwrap();
        world.update_resource(|trace: &mut Vec<u64>| trace.push(tick));
        if tick + 1 == stop {
            world.update_resource(ExitRequest::request);
        }
    });
    HeadlessSimulation::new(
        schedules.build(),
        FixedStepConfig::new(Duration::from_millis(10), 1).unwrap(),
    )
}

#[test]
fn exact_work_is_repeatable_and_partition_independent() {
    let mut first = runner(u64::MAX);
    let mut second = runner(u64::MAX);
    assert_eq!(first.run_ticks(0).unwrap().completed_ticks, 0);
    first
        .world()
        .update_resource(gridthorn_simulation::SimulationControl::pause);
    assert_eq!(first.run_ticks(10).unwrap().executed_ticks, 10);
    second.run_ticks(3).unwrap();
    assert_eq!(second.run_ticks(7).unwrap().completed_ticks, 10);
    assert_eq!(
        first.world().read_resource(|v: &Vec<u64>| v.clone()),
        second.world().read_resource(|v: &Vec<u64>| v.clone())
    );
    assert_eq!(
        first.world().read_resource(|v: &Vec<u64>| v.clone()),
        Some((0..10).collect())
    );
    first.shutdown();
    first.shutdown();
    assert_eq!(first.run_ticks(1), Err(LifecycleError::AlreadyShutdown));
}

#[test]
fn exit_stops_at_complete_tick_and_remains_observable() {
    let mut simulation = runner(3);
    assert_eq!(
        simulation.run_ticks(100).unwrap(),
        HeadlessProgress {
            executed_ticks: 3,
            completed_ticks: 3,
            exit_requested: true
        }
    );
    assert_eq!(simulation.run_ticks(1).unwrap().executed_ticks, 0);
}

#[test]
fn startup_input_and_shutdown_have_exact_boundaries() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.insert_resource(Vec::<&'static str>::from(["startup"]));
    });
    schedules.add_system(ScheduleStage::Input, |world| {
        world.update_resource(|trace: &mut Vec<&'static str>| trace.push("input"));
        world.update_resource(ExitRequest::request);
    });
    schedules.add_system(ScheduleStage::FixedUpdate, |_| {
        panic!("input exit must prevent tick")
    });
    schedules.add_system(ScheduleStage::Shutdown, |world| {
        world.update_resource(|trace: &mut Vec<&'static str>| trace.push("shutdown"));
    });
    let mut simulation = HeadlessSimulation::new(schedules.build(), FixedStepConfig::default());
    simulation.run_ticks(0).unwrap();
    assert_eq!(simulation.run_ticks(5).unwrap().completed_ticks, 0);
    simulation.shutdown();
    simulation.shutdown();
    assert_eq!(
        simulation
            .world()
            .read_resource(|trace: &Vec<&'static str>| trace.clone()),
        Some(vec!["startup", "input", "shutdown"])
    );
}

#[test]
fn startup_exit_prevents_input_and_ticks() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Startup, |world| {
        world.update_resource(ExitRequest::request);
    });
    schedules.add_system(ScheduleStage::Input, |_| {
        panic!("startup exit must prevent input")
    });
    let mut simulation = HeadlessSimulation::new(schedules.build(), FixedStepConfig::default());
    assert_eq!(
        simulation.run_ticks(5).unwrap(),
        HeadlessProgress {
            executed_ticks: 0,
            completed_ticks: 0,
            exit_requested: true
        }
    );
}
