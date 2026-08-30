use crate::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Actor(&'static str);

#[test]
fn prelude_switches_scene_owned_entities_as_one_frame_boundary() {
    let menu = SceneId::new("menu").expect("test scene should be valid");
    let game = SceneId::new("game").expect("test scene should be valid");
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::SceneTransition, |world| {
        let active = world
            .read_resource(|scenes: &SceneController| scenes.current().clone())
            .expect("scene controller should exist");
        world.spawn_in_scene(active, Actor("active scene actor"));
    });
    let mut application = ApplicationRuntime::new(schedules.build());
    application
        .world()
        .insert_resource(SceneController::new(menu));

    application.run_frame(0).expect("menu frame should run");
    application
        .world()
        .update_resource(|scenes: &mut SceneController| scenes.request_switch(game.clone()));
    application.run_frame(0).expect("game frame should run");

    assert_eq!(
        application
            .world()
            .read_resource(|scenes: &SceneController| scenes.current().clone()),
        Some(game)
    );
    let mut actor_count = 0;
    application
        .world()
        .for_each_component_mut(|_, _: &mut Actor| actor_count += 1);
    assert_eq!(actor_count, 1);
}
