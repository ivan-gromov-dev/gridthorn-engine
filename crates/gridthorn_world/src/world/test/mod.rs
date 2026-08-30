use bevy_ecs::world::World;

use super::WorldAccess;

#[derive(Debug, Eq, PartialEq)]
struct Position(i32);

#[test]
fn stores_domain_components_without_exposing_backend_types() {
    let mut backend = World::new();
    let mut world = WorldAccess {
        backend: &mut backend,
    };
    let entity = world.spawn(Position(2));

    assert!(world.update_component(entity, |position: &mut Position| position.0 += 3));

    let mut positions = Vec::new();
    world.for_each_component_mut(|_, position: &mut Position| positions.push(position.0));
    assert_eq!(positions, vec![5]);
}

#[test]
fn reads_one_component_through_the_entity_boundary() {
    let mut backend = World::new();
    let mut world = WorldAccess {
        backend: &mut backend,
    };
    let entity = world.spawn(Position(3));

    assert_eq!(
        world.read_component(entity, |position: &Position| position.0),
        Some(3)
    );
}

#[test]
fn stores_typed_resources_behind_gridthorn_access() {
    let mut backend = World::new();
    let mut world = WorldAccess {
        backend: &mut backend,
    };
    world.insert_resource(4_u32);

    assert!(world.update_resource(|value: &mut u32| *value += 2));
    assert_eq!(world.read_resource(|value: &u32| *value), Some(6));
}
