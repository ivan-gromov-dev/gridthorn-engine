use crate::prelude::*;

#[test]
fn prelude_exposes_frame_boundary_game_state_transitions() {
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::Input, |world| {
        world.update_resource(|states: &mut GameStateStack| {
            states.request_push(
                GameStateId::new("paused").expect("test state identifier should be valid"),
            );
        });
    });
    let mut application = ApplicationRuntime::new(schedules.build());
    application.world().insert_resource(GameStateStack::new(
        GameStateId::new("playing").expect("test state identifier should be valid"),
    ));

    application.run_frame(0).expect("frame should run");

    assert_eq!(
        application
            .world()
            .read_resource(|states: &GameStateStack| states.current().as_str().to_owned()),
        Some("paused".to_owned())
    );
}
