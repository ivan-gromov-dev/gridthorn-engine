use super::*;
use crate::{ExitRequest, LifecycleError};
use gridthorn_simulation::{FixedStepConfig, FixedTime, GameCommandQueue, RandomStreams};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

fn scenario(name: &str, revision: u32) -> Scenario<Vec<u64>, u64> {
    let mut random = RandomStreams::new(42);
    random.register("economy").unwrap();
    Scenario::new(
        name,
        revision,
        ScenarioState {
            data: Vec::new(),
            commands: GameCommandQueue::new(),
            random,
        },
    )
    .unwrap()
}

fn runtime(name: &str, revision: u32, config: FixedStepConfig) -> ScenarioRuntime<Vec<u64>, u64> {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        let tick = world
            .read_resource(|time: &FixedTime| time.tick_index())
            .unwrap();
        world.update_resource(|state: &mut ScenarioState<Vec<u64>, u64>| {
            let command_sum: u64 = state.commands.drain().sum();
            state
                .data
                .push(state.random.next_u64("economy").unwrap() ^ tick ^ command_sum);
        });
    });
    ScenarioRuntime::new(schedules.build(), config, scenario(name, revision)).unwrap()
}

mod clock;
mod continuation;
mod validation;
