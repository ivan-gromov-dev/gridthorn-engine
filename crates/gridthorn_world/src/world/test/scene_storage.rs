use bevy_ecs::world::World;

use crate::{SceneId, WorldAccess};

#[test]
fn scene_storage_supports_empty_entities_multiple_components_and_stale_ids() {
    let mut backend = World::new();
    let mut world = WorldAccess {
        backend: &mut backend,
    };
    let scene = SceneId::new("test").unwrap();
    let entity = world.spawn_empty_in_scene(scene.clone());
    let other = world.spawn_in_scene(scene.clone(), 5_u32);
    world.spawn(0_u32);
    assert_eq!(world.scene_entities(&scene), vec![entity, other]);
    assert!(world.insert_component(entity, 3_u32));
    assert!(world.insert_component(entity, "text".to_owned()));
    assert!(world.insert_component(entity, 7_u32));
    assert_eq!(world.read_component(entity, |value: &u32| *value), Some(7));
    assert_eq!(
        world.read_component(entity, String::clone),
        Some("text".into())
    );
    assert_eq!(world.despawn_scene(&scene), 2);
    assert!(!world.insert_component(entity, 9_u32));
    assert_eq!(world.scene_entities(&scene), []);
}
