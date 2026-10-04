use std::{hint::black_box, time::Instant};

use bevy_ecs::world::World;

use crate::{EntityId, SceneId, WorldAccess};

struct Position(u64);
struct Velocity(u64);
struct Inventory([u64; 8]);

/// Public component insertion/replacement and scene-partition churn.
#[test]
#[ignore = "manual structural world probe; run alone in release mode"]
fn measure_structural_world_churn() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("structural_world,entities,partitions,operation,sample,elapsed_ns");
    for count in [1000, 10_000, 100_000] {
        for partitions in [1, 16, 64] {
            exercise(count, partitions, 51, true);
        }
    }
}

#[test]
fn mixed_components_survive_partition_replacement_and_reject_stale_ids() {
    exercise(120, 4, 3, false);
}

fn exercise(count: usize, partitions: usize, cycles: usize, report: bool) {
    let mut backend = World::new();
    let mut world = WorldAccess {
        backend: &mut backend,
    };
    let scenes = (0..partitions)
        .map(|index| SceneId::new(format!("partition-{index}")).unwrap())
        .collect::<Vec<_>>();
    let persistent = populate(&mut world, count, None);
    for scene in &scenes {
        populate(&mut world, count / 10 / partitions, Some(scene));
    }
    for cycle in 0..cycles {
        let scene = &scenes[cycle % partitions];
        let stale = world.scene_entities(scene);
        let start = Instant::now();
        for (index, entity) in persistent.iter().enumerate() {
            assert!(world.insert_component(*entity, Velocity(index as u64)));
            if index.is_multiple_of(2) {
                assert!(world.insert_component(*entity, Inventory([cycle as u64; 8])));
            }
        }
        let insert = start.elapsed().as_nanos();
        let start = Instant::now();
        let mut visits = [0; 3];
        world.for_each_component_mut(|_, value: &mut Position| {
            value.0 += 1;
            visits[0] += 1;
        });
        world.for_each_component_mut(|_, value: &mut Velocity| {
            value.0 += 1;
            visits[1] += 1;
        });
        world.for_each_component_mut(|_, value: &mut Inventory| {
            value.0[0] += 1;
            visits[2] += 1;
        });
        let traverse = start.elapsed().as_nanos();
        let start = Instant::now();
        for index in 0..count {
            let entity = persistent[(index * 7919) % count];
            assert!(world.update_component(entity, |value: &mut Position| value.0 += 1));
            black_box(
                world
                    .read_component(entity, |value: &Velocity| value.0)
                    .unwrap(),
            );
        }
        let access = start.elapsed().as_nanos();
        let start = Instant::now();
        let removed = world.despawn_scene(scene);
        let removal = start.elapsed().as_nanos();
        let start = Instant::now();
        populate(&mut world, count / 10 / partitions, Some(scene));
        let spawn = start.elapsed().as_nanos();
        assert_eq!(removed, count / 10 / partitions);
        assert_eq!(visits[0], count + partitions * (count / 10 / partitions));
        for entity in stale {
            assert!(!world.insert_component(entity, Position(99)));
            assert_eq!(
                world.read_component(entity, |value: &Position| value.0),
                None
            );
        }
        for (index, entity) in persistent.iter().enumerate() {
            assert_eq!(
                world.read_component(*entity, |value: &Position| value.0),
                Some(2 * (cycle as u64 + 1))
            );
            assert_eq!(
                world.read_component(*entity, |value: &Velocity| value.0),
                Some(index as u64 + 1)
            );
        }
        if report {
            for (operation, elapsed) in [
                ("insert_replace", insert),
                ("traverse", traverse),
                ("random_access", access),
                ("partition_remove", removal),
                ("partition_spawn", spawn),
            ] {
                println!("structural_world,{count},{partitions},{operation},{cycle},{elapsed}");
            }
        }
    }
    for scene in scenes {
        assert_eq!(world.despawn_scene(&scene), count / 10 / partitions);
    }
    let mut remaining = 0;
    world.for_each_component_mut(|_, _: &mut Position| remaining += 1);
    assert_eq!(remaining, count);
}

fn populate(world: &mut WorldAccess<'_>, count: usize, scene: Option<&SceneId>) -> Vec<EntityId> {
    (0..count)
        .map(|index| {
            let entity = if let Some(scene) = scene {
                world.spawn_in_scene(scene.clone(), Position(0))
            } else {
                world.spawn(Position(0))
            };
            if !index.is_multiple_of(3) {
                assert!(world.insert_component(entity, Velocity(0)));
            }
            if index.is_multiple_of(3) {
                assert!(world.insert_component(entity, Inventory([0; 8])));
            }
            entity
        })
        .collect()
}
