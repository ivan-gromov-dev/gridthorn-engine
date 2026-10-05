use std::{hint::black_box, time::Instant};

use super::{Codec, runtime};
use crate::ScenarioState;

/// Game-codec costs stay included in complete document and durable-file paths.
#[test]
#[ignore = "manual world-save I/O probe; run alone in release mode"]
fn measure_world_save_io_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("world_save_io,values,operation,sample,elapsed_ns,document_bytes");
    for count in [1024, 16_384, 262_144] {
        measure(count);
    }
    if std::env::var_os("GRIDTHORN_IO_MEMORY").is_some() {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}

fn measure(count: u64) {
    let directory = super::files::Directory::new();
    let path = directory.0.join("scaling.toml");
    let mut source = runtime();
    source
        .world()
        .update_resource(|state: &mut ScenarioState<Vec<u64>, u64>| {
            state.data = (0..count).collect();
            state.commands.push(7);
        });
    let mut target = runtime();
    for sample in 0..22 {
        let start = Instant::now();
        let encoded = black_box(source.save_document(&Codec).unwrap());
        let save_document = start.elapsed().as_nanos();
        let start = Instant::now();
        target.load_document(&encoded, &Codec).unwrap();
        let load_document = start.elapsed().as_nanos();
        let start = Instant::now();
        source.save_file(&path, &Codec).unwrap();
        let save_file = start.elapsed().as_nanos();
        let start = Instant::now();
        target.load_file(&path, &Codec).unwrap();
        let load_file = start.elapsed().as_nanos();
        assert_eq!(target.save_document(&Codec).unwrap(), encoded);
        assert_eq!(std::fs::read(&path).unwrap(), encoded.as_bytes());
        assert_eq!(std::fs::read_dir(&directory.0).unwrap().count(), 1);
        if sample >= 2 {
            for (operation, elapsed) in [
                ("save_document", save_document),
                ("load_document", load_document),
                ("save_file", save_file),
                ("load_file", load_file),
            ] {
                println!(
                    "world_save_io,{count},{operation},{},{elapsed},{}",
                    sample - 2,
                    encoded.len()
                );
            }
        }
    }
}
