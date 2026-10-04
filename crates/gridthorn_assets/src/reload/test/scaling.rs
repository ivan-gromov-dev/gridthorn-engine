use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use super::{Fixture, id};
use crate::{AssetReloader, AssetStore};

/// Separate synchronous scans, worker round trips and completed-result publication.
#[test]
#[ignore = "manual asset I/O probe; run alone in release mode"]
fn measure_asset_reload_io_and_publication() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("asset_io,kind,count,units,state,operation,sample,elapsed_ns");
    for (count, size) in [(16, 4096), (128, 4096), (1024, 4096), (128, 65536)] {
        measure("raw", count, size);
    }
    measure("texture", 16, 256);
    if std::env::var_os("GRIDTHORN_IO_MEMORY").is_some() {
        std::thread::sleep(Duration::from_millis(500));
    }
}

fn measure(kind: &str, count: usize, units: usize) {
    let fixture = Fixture::new();
    let names: Vec<_> = (0..count)
        .map(|index| {
            format!(
                "asset-{index:04}.{}",
                if kind == "raw" { "bin" } else { "ppm" }
            )
        })
        .collect();
    let ids: Vec<_> = names.iter().map(|name| id(name)).collect();
    let mut source = payload(kind, units, 0);
    let mut store = AssetStore::new(&fixture.0);
    for (name, asset) in names.iter().zip(&ids) {
        fixture.write(name, &source);
        if kind == "raw" {
            store.load_source(asset.clone()).unwrap();
        } else {
            store.load_texture(asset.clone()).unwrap();
        }
    }
    for index in 0..count - 1 {
        store
            .set_dependencies(&ids[index], &[ids[index + 1].clone()])
            .unwrap();
    }
    let mut worker = AssetReloader::new(store.snapshot()).unwrap();
    for state in ["unchanged", "leaf_changed"] {
        for sample in 0..22 {
            let old = worker.assets().source(&ids[count - 1]).unwrap();
            if state == "leaf_changed" {
                source = payload(kind, units, if sample % 2 == 0 { 170 } else { 187 });
                fixture.write(&names[count - 1], &source);
            }
            let start = Instant::now();
            let changed = store.reload_changed().unwrap();
            let synchronous = start.elapsed().as_nanos();
            let start = Instant::now();
            assert!(worker.request_reload().unwrap());
            let request = start.elapsed().as_nanos();
            let start = Instant::now();
            let (published, publication, empty_max) = await_publication(&mut worker);
            let round_trip = start.elapsed().as_nanos();
            let expected = if state == "unchanged" {
                vec![]
            } else {
                ids.iter().rev().cloned().collect()
            };
            assert_eq!(changed, expected);
            assert_eq!(published, expected);
            assert_eq!(
                worker.assets().source(&ids[count - 1]).unwrap().as_ref(),
                source
            );
            if state == "leaf_changed" {
                assert_ne!(old.as_ref(), source);
            }
            if sample >= 2 {
                for (operation, elapsed) in [
                    ("sync_scan", synchronous),
                    ("request", request),
                    ("request_to_poll", round_trip),
                    ("ready_poll", publication),
                    ("empty_poll_max", empty_max),
                ] {
                    println!(
                        "asset_io,{kind},{count},{units},{state},{operation},{},{elapsed}",
                        sample - 2
                    );
                }
            }
        }
    }
    let start = Instant::now();
    worker.shutdown().unwrap();
    println!(
        "asset_shutdown,{kind},{count},{units},{}",
        start.elapsed().as_nanos()
    );
}

fn payload(kind: &str, units: usize, value: u8) -> Vec<u8> {
    if kind == "raw" {
        return vec![value; units];
    }
    let mut bytes = format!("P6\n{units} {units}\n255\n").into_bytes();
    bytes.extend(vec![value; units * units * 3]);
    bytes
}

fn await_publication(worker: &mut AssetReloader) -> (Vec<crate::AssetId>, u128, u128) {
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut empty_max = 0;
    loop {
        let start = Instant::now();
        let ready = worker.poll().unwrap();
        let elapsed = start.elapsed().as_nanos();
        if let Some(changed) = ready {
            return (changed, elapsed, empty_max);
        }
        empty_max = empty_max.max(elapsed);
        assert!(Instant::now() < deadline, "worker timeout");
        std::thread::sleep(Duration::from_millis(1));
    }
}
