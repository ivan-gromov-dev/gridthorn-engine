use std::{hint::black_box, time::Instant};

use crate::{
    ChunkSize, GridCell, GridFootprint, GridObjectId, NavigationBounds, PathSearch, PlacementMap,
    TileLayerId, TileMap, search_path,
};

fn terrain(side: i32) -> (TileMap<u32>, PlacementMap) {
    let mut tiles = TileMap::new(ChunkSize::new(16, 16).unwrap());
    tiles.add_layer(TileLayerId(0)).unwrap();
    let mut occupants = PlacementMap::new();
    let footprint = GridFootprint::single_cell();
    for row in 0..side {
        for column in 0..side {
            let cell = GridCell::new(column - side / 2, row - side / 2);
            tiles
                .set_tile(
                    TileLayerId(0),
                    cell,
                    Some(1 + u32::try_from((column + row) % 5).unwrap()),
                )
                .unwrap();
            if column % 8 == 4 && row % 8 != 7 {
                occupants
                    .place(
                        GridObjectId(u64::try_from(row * side + column).unwrap()),
                        cell,
                        footprint.clone(),
                    )
                    .unwrap();
            }
        }
    }
    (tiles, occupants)
}

fn queries(
    tiles: &TileMap<u32>,
    occupants: &PlacementMap,
    side: i32,
    budget: usize,
) -> Vec<PathSearch> {
    let low = -side / 2;
    let bounds = NavigationBounds::new(
        GridCell::new(low, low),
        GridCell::new(low + side - 1, low + side - 1),
    )
    .unwrap();
    (0..8)
        .map(|index| {
            search_path(
                bounds,
                GridCell::new(low, low + index),
                GridCell::new(low + side - 1, low + side - 1 - index),
                budget,
                |cell| {
                    if occupants.object_at(cell).is_some() {
                        None
                    } else {
                        tiles.tile(TileLayerId(0), cell).copied()
                    }
                },
            )
            .unwrap()
        })
        .collect()
}

#[test]
fn inhabited_routes_repeat_complete_ordered_diagnostics() {
    let (tiles, occupants) = terrain(32);
    for budget in [32, 256, 1024] {
        let first = queries(&tiles, &occupants, 32, budget);
        assert_eq!(queries(&tiles, &occupants, 32, budget), first);
        assert!(first.iter().all(|result| result.visited.len() <= budget));
        if budget == 1024 {
            assert!(
                first
                    .iter()
                    .all(|result| result.status == crate::PathStatus::Found)
            );
        }
    }
}

/// Eight weighted maze queries against signed sparse tiles and occupied cells.
#[test]
#[ignore = "manual inhabited navigation CPU/heap probe; run alone with --release"]
fn measure_inhabited_navigation() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let heap = std::env::var_os("GRIDTHORN_DOMAIN_HEAP").is_some();
    for side in [64_i32, 128, 256] {
        for budget in [256, 1024, usize::try_from(side * side).unwrap()] {
            let (tiles, occupants) = terrain(side);
            let expected = queries(&tiles, &occupants, side, budget);
            for batch in 0..if heap { 1 } else { 22 } {
                let profile = heap.then(|| {
                    dhat::Profiler::builder()
                        .testing()
                        .trim_backtraces(Some(4))
                        .build()
                });
                let start = Instant::now();
                let results = queries(black_box(&tiles), black_box(&occupants), side, budget);
                let elapsed = start.elapsed().as_nanos();
                assert_eq!(results, expected);
                let stats = profile.as_ref().map(|_| dhat::HeapStats::get());
                drop(results);
                let released = profile.as_ref().map(|_| dhat::HeapStats::get().curr_bytes);
                drop(profile);
                if batch >= 2 || heap {
                    println!("inhabited,{side},{budget},{batch},{elapsed}");
                }
                if let Some(stats) = stats {
                    println!(
                        "inhabited_heap,{side},{budget},{},{},{},{},{}",
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
