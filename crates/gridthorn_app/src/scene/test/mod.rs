use gridthorn_world::{SceneId, ScheduleBuilder, ScheduleStage};

use super::SceneController;
use crate::ApplicationRuntime;

#[derive(Debug, PartialEq, Eq)]
struct Actor(&'static str);

#[derive(Default)]
struct Trace(Vec<&'static str>);

fn scene(identifier: &str) -> SceneId {
    SceneId::new(identifier).expect("test scene should be valid")
}

#[test]
fn activates_initial_scene_once_before_fixed_updates() {
    let mut schedules = ScheduleBuilder::new();
    schedules
        .add_system(ScheduleStage::SceneTransition, |world| {
            let active = world
                .read_resource(|scenes: &SceneController| scenes.current().clone())
                .expect("scene controller should exist");
            world.spawn_in_scene(active, Actor("player"));
            world.update_resource(|trace: &mut Trace| trace.0.push("scene"));
        })
        .add_system(ScheduleStage::FixedUpdate, |world| {
            world.update_resource(|trace: &mut Trace| trace.0.push("fixed"));
        });
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime
        .world()
        .insert_resource(SceneController::new(scene("game")));
    runtime.world().insert_resource(Trace::default());

    runtime.run_frame(2).expect("first frame should run");
    runtime.run_frame(1).expect("second frame should run");

    assert_eq!(
        runtime
            .world()
            .read_resource(|trace: &Trace| trace.0.clone()),
        Some(vec!["scene", "fixed", "fixed", "fixed"])
    );
}

#[test]
fn removes_exited_scene_entities_before_loading_the_replacement() {
    let menu = scene("menu");
    let game = scene("game");
    let mut schedules = ScheduleBuilder::new();
    schedules.add_system(ScheduleStage::SceneTransition, |world| {
        let active = world
            .read_resource(|scenes: &SceneController| scenes.current().clone())
            .expect("scene controller should exist");
        let actor = if active.as_str() == "menu" {
            Actor("cursor")
        } else {
            Actor("player")
        };
        world.spawn_in_scene(active, actor);
    });
    let mut runtime = ApplicationRuntime::new(schedules.build());
    runtime
        .world()
        .insert_resource(SceneController::new(menu.clone()));
    runtime.run_frame(0).expect("menu frame should run");
    let persistent = runtime.world().spawn(Actor("music"));

    runtime
        .world()
        .update_resource(|scenes: &mut SceneController| scenes.request_switch(game.clone()));
    runtime.run_frame(0).expect("game frame should run");

    let mut actors = Vec::new();
    runtime
        .world()
        .for_each_component_mut(|_, actor: &mut Actor| actors.push(actor.0));
    actors.sort_unstable();
    assert_eq!(actors, vec!["music", "player"]);
    assert_eq!(
        runtime
            .world()
            .read_component(persistent, |actor: &Actor| actor.0),
        Some("music")
    );
    let change = runtime
        .world()
        .read_resource(|scenes: &SceneController| scenes.change().cloned())
        .flatten()
        .expect("scene change should be observable for the frame");
    assert_eq!(change.exited(), Some(&menu));
    assert_eq!(change.entered(), &game);
}
