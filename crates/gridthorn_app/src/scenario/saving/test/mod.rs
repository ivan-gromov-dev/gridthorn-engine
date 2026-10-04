use super::*;
use crate::{Scenario, ScenarioRuntime, ScenarioState};
use gridthorn_simulation::{FixedStepConfig, FixedTime, GameCommandQueue, RandomStreams};
use gridthorn_world::{ScheduleBuilder, ScheduleStage};

struct Codec;

impl WorldSaveCodec<Vec<u64>, u64> for Codec {
    fn encode(&self, data: &Vec<u64>, commands: &GameCommandQueue<u64>) -> Result<String, String> {
        let mut commands = commands.clone();
        Ok(format!(
            "{};{}",
            values(data.iter().copied()),
            values(commands.drain())
        ))
    }

    fn decode(&self, payload: &str) -> Result<(Vec<u64>, GameCommandQueue<u64>), String> {
        let (data, commands) = payload.split_once(';').ok_or("missing command separator")?;
        let mut queue = GameCommandQueue::new();
        for command in parse(commands)? {
            queue.push(command);
        }
        Ok((parse(data)?, queue))
    }
}

fn values(values: impl Iterator<Item = u64>) -> String {
    values
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn parse(source: &str) -> Result<Vec<u64>, String> {
    if source.is_empty() {
        return Ok(Vec::new());
    }
    source
        .split(',')
        .map(|value| value.parse().map_err(|_| "invalid game integer".to_owned()))
        .collect()
}

fn runtime() -> ScenarioRuntime<Vec<u64>, u64> {
    let mut random = RandomStreams::new(u64::MAX);
    random.register("economy").unwrap();
    let scenario = Scenario::new(
        "world",
        1,
        ScenarioState {
            data: Vec::new(),
            commands: GameCommandQueue::new(),
            random,
        },
    )
    .unwrap();
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::FixedUpdate, |world| {
        let tick = world
            .read_resource(|time: &FixedTime| time.tick_index())
            .unwrap();
        world.update_resource(|state: &mut ScenarioState<Vec<u64>, u64>| {
            let sum: u64 = state.commands.drain().sum();
            state
                .data
                .push(state.random.next_u64("economy").unwrap() ^ sum ^ tick);
        });
    });
    ScenarioRuntime::new(schedules.build(), FixedStepConfig::default(), scenario).unwrap()
}

mod documents;
mod files;
mod scaling;

mod attribution;
mod heap;
mod ownership;
