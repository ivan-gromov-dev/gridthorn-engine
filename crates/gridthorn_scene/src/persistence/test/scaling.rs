use std::{hint::black_box, time::Instant};

use gridthorn_world::ScheduleBuilder;

use super::{Health, registry, scene};
use crate::SceneDocument;

/// Registered scalar scene capture, TOML conversion, preparation and replacement.
#[test]
#[ignore = "manual scene persistence probe; run alone in release mode"]
fn measure_scene_persistence_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("scene_io,entities,operation,sample,elapsed_ns,document_bytes");
    for count in [100, 1000, 10_000] {
        let registry = registry();
        let mut runtime = ScheduleBuilder::new().build();
        let mut world = runtime.world();
        for index in 0..count {
            world.spawn_in_scene(scene(), Health(index + 1));
        }
        let persistent = world.spawn(Health(99));
        for sample in 0..22 {
            let start = Instant::now();
            let document = black_box(registry.capture(&mut world, &scene()).unwrap());
            let capture = start.elapsed().as_nanos();
            let start = Instant::now();
            let encoded = black_box(document.to_toml().unwrap());
            let encode = start.elapsed().as_nanos();
            let start = Instant::now();
            let parsed = black_box(SceneDocument::from_toml(&encoded).unwrap());
            let parse = start.elapsed().as_nanos();
            assert_eq!(parsed, document);
            let start = Instant::now();
            let prepared = black_box(registry.prepare(&parsed).unwrap());
            let prepare = start.elapsed().as_nanos();
            let start = Instant::now();
            let loaded = black_box(prepared.commit(&mut world));
            let commit = start.elapsed().as_nanos();
            assert_eq!(loaded.entities.len(), usize::try_from(count).unwrap());
            assert_eq!(
                world.read_component(persistent, |health: &Health| health.0),
                Some(99)
            );
            let mut actual: Vec<_> = loaded
                .entities
                .iter()
                .map(|entity| {
                    world
                        .read_component(*entity, |health: &Health| health.0)
                        .unwrap()
                })
                .collect();
            actual.sort_unstable();
            assert_eq!(actual, (1..=count).collect::<Vec<_>>());
            if sample >= 2 {
                for (operation, elapsed) in [
                    ("capture", capture),
                    ("encode", encode),
                    ("parse", parse),
                    ("prepare", prepare),
                    ("commit", commit),
                ] {
                    println!(
                        "scene_io,{count},{operation},{},{elapsed},{}",
                        sample - 2,
                        encoded.len()
                    );
                }
            }
        }
    }
    if std::env::var_os("GRIDTHORN_IO_MEMORY").is_some() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
