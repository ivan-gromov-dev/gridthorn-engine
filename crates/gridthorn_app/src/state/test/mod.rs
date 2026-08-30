use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use super::{GameStateChange, GameStateError, GameStateId, GameStateStack};
use crate::ApplicationRuntime;

fn state(identifier: &str) -> GameStateId {
    GameStateId::new(identifier).expect("test state should be valid")
}

#[test]
fn rejects_empty_identifiers_and_root_pop() {
    assert_eq!(GameStateId::new("  "), Err(GameStateError::EmptyIdentifier));

    let mut states = GameStateStack::new(state("playing"));
    assert_eq!(states.request_pop(), Err(GameStateError::CannotPopRoot));
}

#[test]
fn applies_input_transitions_before_fixed_and_presentation_stages() {
    #[derive(Default)]
    struct Trace(Vec<(ScheduleStage, String)>);

    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Input, |world| {
        world.update_resource(|states: &mut GameStateStack| {
            states.request_push(state("paused"));
        });
    });
    for stage in [ScheduleStage::FixedUpdate, ScheduleStage::Update] {
        schedules.add_system(stage, move |world| {
            let current = world
                .read_resource(|states: &GameStateStack| states.current().as_str().to_owned())
                .expect("state stack should exist");
            world.update_resource(|trace: &mut Trace| trace.0.push((stage, current)));
        });
    }
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime
        .world()
        .insert_resource(GameStateStack::new(state("playing")));
    runtime.world().insert_resource(Trace::default());

    runtime.run_frame(1).expect("frame should run");

    assert_eq!(
        runtime
            .world()
            .read_resource(|trace: &Trace| trace.0.clone()),
        Some(vec![
            (ScheduleStage::FixedUpdate, "paused".to_owned()),
            (ScheduleStage::Update, "paused".to_owned()),
        ])
    );
    assert_eq!(
        runtime
            .world()
            .read_resource(|states: &GameStateStack| states.changes().to_vec()),
        Some(vec![GameStateChange::Push {
            suspended: state("playing"),
            entered: state("paused"),
        }])
    );
}

#[test]
fn defers_update_transitions_until_the_next_frame() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Update, |world| {
        world.update_resource(|states: &mut GameStateStack| {
            if states.current().as_str() == "playing" {
                states.request_set(state("game_over"));
            }
        });
    });
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime
        .world()
        .insert_resource(GameStateStack::new(state("playing")));

    runtime.run_frame(0).expect("first frame should run");
    assert_eq!(
        runtime
            .world()
            .read_resource(|states: &GameStateStack| states.current().clone()),
        Some(state("playing"))
    );

    runtime.run_frame(0).expect("second frame should run");
    assert_eq!(
        runtime
            .world()
            .read_resource(|states: &GameStateStack| states.current().clone()),
        Some(state("game_over"))
    );
}

#[test]
fn applies_multiple_transitions_in_request_order() {
    let mut states = GameStateStack::new(state("menu"));
    states.request_push(state("playing"));
    states.request_set(state("paused"));
    states.request_pop().expect("queued push permits pop");

    states.apply_pending();

    assert_eq!(states.states(), &[state("menu")]);
    assert_eq!(states.changes().len(), 3);
}
