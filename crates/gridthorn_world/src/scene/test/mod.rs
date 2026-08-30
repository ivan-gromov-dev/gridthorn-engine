use bevy_ecs::world::World;

use super::{SceneId, SceneIdError};
use crate::WorldAccess;

#[derive(Debug, Eq, PartialEq)]
struct Actor(&'static str);

fn scene(identifier: &str) -> SceneId {
    SceneId::new(identifier).expect("test scene should be valid")
}

#[test]
fn rejects_empty_scene_identifiers() {
    assert_eq!(SceneId::new("\t"), Err(SceneIdError::EmptyIdentifier));
}

#[test]
fn despawns_only_entities_owned_by_the_selected_scene() {
    let mut backend = World::new();
    let mut world = WorldAccess {
        backend: &mut backend,
    };
    let menu_actor = world.spawn_in_scene(scene("menu"), Actor("cursor"));
    let game_actor = world.spawn_in_scene(scene("game"), Actor("player"));
    let persistent_actor = world.spawn(Actor("music"));

    assert_eq!(world.despawn_scene(&scene("menu")), 1);

    assert_eq!(
        world.read_component(menu_actor, |actor: &Actor| actor.0),
        None
    );
    assert_eq!(
        world.read_component(game_actor, |actor: &Actor| actor.0),
        Some("player")
    );
    assert_eq!(
        world.read_component(persistent_actor, |actor: &Actor| actor.0),
        Some("music")
    );
}
