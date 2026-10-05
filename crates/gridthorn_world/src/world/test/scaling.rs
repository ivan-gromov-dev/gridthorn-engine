use std::{hint::black_box, time::Instant};

use bevy_ecs::world::World;

use crate::{SceneId, WorldAccess};

struct Position(u64);
struct Velocity(u64);
struct Inventory([u64; 8]);

/// Mixed domain populations and scene replacement through engine-owned access.
#[test]
#[ignore = "manual mixed world probe; run alone in release mode"]
fn measure_mixed_world_and_scene_churn() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("mixed_world,entities,operation,sample,elapsed_ns");
    for count in [1000, 10_000, 100_000] {
        let mut backend = World::new();
        let mut world = WorldAccess {
            backend: &mut backend,
        };
        let scene = SceneId::new("churn").unwrap();
        populate(&mut world, count, None);
        populate(&mut world, count / 10, Some(&scene));
        traverse(&mut world);
        for sample in 0..50 {
            let start = Instant::now();
            traverse(&mut world);
            let traversal = start.elapsed().as_nanos();
            let start = Instant::now();
            let removed = world.despawn_scene(&scene);
            let removal = start.elapsed().as_nanos();
            assert_eq!(removed, count / 10);
            let start = Instant::now();
            populate(&mut world, count / 10, Some(&scene));
            let spawn = start.elapsed().as_nanos();
            for (operation, elapsed) in [
                ("traverse", traversal),
                ("scene_remove", removal),
                ("scene_spawn", spawn),
            ] {
                println!("mixed_world,{count},{operation},{sample},{elapsed}");
            }
        }
        assert_eq!(world.despawn_scene(&scene), count / 10);
        let mut visited = 0;
        world.for_each_component_mut(|_, _: &mut Position| visited += 1);
        world.for_each_component_mut(|_, _: &mut Velocity| visited += 1);
        world.for_each_component_mut(|_, _: &mut Inventory| visited += 1);
        assert_eq!(visited, count);
    }
}

fn populate(world: &mut WorldAccess<'_>, count: usize, scene: Option<&SceneId>) {
    for index in 0..count {
        match index % 3 {
            0 => {
                spawn(world, Position(0), scene);
            }
            1 => {
                spawn(world, Velocity(1), scene);
            }
            _ => {
                spawn(world, Inventory([0; 8]), scene);
            }
        }
    }
}

fn spawn<T: Send + Sync + 'static>(world: &mut WorldAccess<'_>, value: T, scene: Option<&SceneId>) {
    black_box(if let Some(scene) = scene {
        world.spawn_in_scene(scene.clone(), value)
    } else {
        world.spawn(value)
    });
}

fn traverse(world: &mut WorldAccess<'_>) {
    world.for_each_component_mut(|_, value: &mut Position| value.0 += black_box(1));
    world.for_each_component_mut(|_, value: &mut Velocity| value.0 += black_box(1));
    world.for_each_component_mut(|_, value: &mut Inventory| value.0[0] += black_box(1));
}
