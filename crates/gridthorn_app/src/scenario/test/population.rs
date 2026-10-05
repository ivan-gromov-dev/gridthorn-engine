use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Agent {
    position: i64,
    inventory: Vec<u64>,
    route: Vec<(i32, i32)>,
    name: String,
}

pub(super) type Population = Vec<Agent>;
pub(super) type Command = Vec<u64>;

pub(super) fn populated(count: usize) -> ScenarioRuntime<Population, Command> {
    let mut random = RandomStreams::new(42);
    for index in 0..64 {
        random
            .register(&format!("economy/sector/{index:04}"))
            .unwrap();
    }
    let initial = ScenarioState {
        data: (0..count)
            .map(|index| Agent {
                position: i64::try_from(index).unwrap(),
                inventory: vec![7; 16],
                route: vec![(-3, 9); 32],
                name: format!("agent-{index:08}"),
            })
            .collect(),
        commands: GameCommandQueue::new(),
        random,
    };
    let names: Vec<_> = (0..64).map(|i| format!("economy/sector/{i:04}")).collect();
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::FixedUpdate, move |world| {
        world.update_resource(|state: &mut ScenarioState<Population, Command>| {
            let commands = state.commands.drain().fold(0_u64, |sum, command| {
                command.into_iter().fold(sum, u64::wrapping_add)
            });
            for (index, agent) in state.data.iter_mut().enumerate() {
                let draw = state.random.next_u64(&names[index % names.len()]).unwrap();
                agent.position = agent.position.wrapping_add(1);
                agent.inventory[0] = agent.inventory[0].wrapping_add(draw ^ commands);
                agent.route.rotate_left(1);
            }
        });
    });
    ScenarioRuntime::new(
        schedules.build(),
        FixedStepConfig::default(),
        Scenario::new("population", 1, initial).unwrap(),
    )
    .unwrap()
}

pub(super) fn enqueue(runner: &mut ScenarioRuntime<Population, Command>, count: usize) {
    runner
        .world()
        .update_resource(|state: &mut ScenarioState<Population, Command>| {
            for index in 0..count {
                state.commands.push(vec![u64::try_from(index).unwrap(); 8]);
            }
        });
}

#[test]
fn nested_snapshots_preserve_queued_commands_rng_and_partitioned_continuation() {
    let mut first = populated(256);
    enqueue(&mut first, 1024);
    let retained = first.snapshot().unwrap();
    let independent = retained.clone();
    first.run_ticks(64).unwrap();
    let expected = first.snapshot().unwrap();
    let mut second = populated(256);
    second.restore(&retained).unwrap();
    for ticks in [1, 7, 16, 40] {
        second.run_ticks(ticks).unwrap();
    }
    assert_eq!(second.snapshot().unwrap().state(), expected.state());
    assert_eq!(second.snapshot().unwrap().completed_ticks(), 64);
    assert_eq!(retained.state(), independent.state());
    assert_eq!(retained.state().commands.len(), 1024);
    assert!(expected.state().commands.is_empty());
    second.restore(&retained).unwrap();
    assert_eq!(second.snapshot().unwrap().state(), independent.state());
}
