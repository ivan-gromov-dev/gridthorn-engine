use std::{hint::black_box, time::Instant};

use crate::{ChunkSize, GridCell, TileLayerId, TileMap};

fn cells(count: i32, spread: bool) -> impl Iterator<Item = GridCell> {
    (0..count).map(move |index| {
        if spread {
            GridCell::new(index * 16 - count * 8, -17)
        } else {
            GridCell::new(index % 256 - 128, index / 256 - 128)
        }
    })
}

/// Sparse one-cell chunks and compact storage have deliberately different envelopes.
#[test]
#[ignore = "manual tile storage CPU/heap probe; run alone with --release"]
fn measure_tile_retention() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let heap = std::env::var_os("GRIDTHORN_DOMAIN_HEAP").is_some();
    for spread in [false, true] {
        for count in [1024, 16_384, 65_536] {
            for batch in 0..if heap { 1 } else { 22 } {
                let profile = heap.then(|| {
                    dhat::Profiler::builder()
                        .testing()
                        .trim_backtraces(Some(4))
                        .build()
                });
                let mut map = TileMap::new(ChunkSize::new(16, 16).unwrap());
                map.add_layer(TileLayerId(0)).unwrap();
                let start = Instant::now();
                for cell in cells(count, spread) {
                    map.set_tile(TileLayerId(0), cell, Some(7_u64)).unwrap();
                }
                let insert = start.elapsed().as_nanos();
                let start = Instant::now();
                let cloned = map.clone();
                let clone = start.elapsed().as_nanos();
                let start = Instant::now();
                for cell in cells(count, spread) {
                    assert_eq!(map.tile(TileLayerId(0), cell), Some(&7));
                    assert_eq!(map.set_tile(TileLayerId(0), cell, None).unwrap(), Some(7));
                }
                let remove = start.elapsed().as_nanos();
                assert_eq!(map.layer(TileLayerId(0)).unwrap().chunks().count(), 0);
                assert_eq!(
                    cloned.layer(TileLayerId(0)).unwrap().tiles().count(),
                    usize::try_from(count).unwrap()
                );
                let stats = profile.as_ref().map(|_| dhat::HeapStats::get());
                drop(cloned);
                drop(map);
                let released = profile.as_ref().map(|_| dhat::HeapStats::get().curr_bytes);
                drop(profile);
                if batch >= 2 || heap {
                    println!("tile_retention,{spread},{count},{batch},{insert},{clone},{remove}");
                }
                if let Some(stats) = stats {
                    println!(
                        "tile_retention_heap,{spread},{count},{},{},{},{},{}",
                        stats.total_blocks,
                        stats.total_bytes,
                        stats.max_bytes,
                        stats.curr_bytes,
                        released.unwrap()
                    );
                }
            }
        }
    }
}
