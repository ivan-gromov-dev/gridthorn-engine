use std::{hint::black_box, time::Instant};

use crate::{GridCell, GridFootprint, GridObjectId, PlacementMap};

fn populated(count: usize) -> (PlacementMap, GridFootprint) {
    let footprint = GridFootprint::new(
        (-2..2).flat_map(|column| (-2..2).map(move |row| GridCell::new(column, row))),
    )
    .unwrap();
    let mut map = PlacementMap::new();
    for index in 0..count {
        map.place(
            GridObjectId(u64::try_from(index).unwrap()),
            GridCell::new(i32::try_from(index).unwrap() * 8, -8),
            footprint.clone(),
        )
        .unwrap();
    }
    (map, footprint)
}

fn churn(map: &mut PlacementMap, footprint: &GridFootprint, count: usize) {
    for index in 0..1024.min(count) {
        let id = GridObjectId(u64::try_from(index).unwrap());
        let anchor = GridCell::new(i32::try_from(index).unwrap() * 8, -8);
        assert!(
            map.validate(GridObjectId(u64::MAX), anchor, footprint)
                .is_err()
        );
        assert!(
            map.validate(id, GridCell::new(i32::MAX, 0), footprint)
                .is_err()
        );
        map.relocate(id, GridCell::new(anchor.column, 8), footprint.clone())
            .unwrap();
        map.remove(id).unwrap();
        map.place(id, anchor, footprint.clone()).unwrap();
    }
}

#[test]
fn rejection_and_remove_replace_churn_preserve_all_occupied_cells() {
    let (mut map, footprint) = populated(64);
    let initial = map.clone();
    churn(&mut map, &footprint, 64);
    assert_eq!(
        map.objects().collect::<Vec<_>>(),
        initial.objects().collect::<Vec<_>>()
    );
    for (id, placement) in initial.objects() {
        for cell in placement.footprint().cells_at(placement.anchor()).unwrap() {
            assert_eq!(map.object_at(cell), Some(id));
        }
    }
}

/// Include sparse storage, rejection allocations, clone and remove/replace churn.
#[test]
#[ignore = "manual placement CPU/heap probe; run alone with --release"]
fn measure_placement_churn() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let heap = std::env::var_os("GRIDTHORN_DOMAIN_HEAP").is_some();
    for count in [1024, 16_384, 65_536] {
        for batch in 0..if heap { 1 } else { 22 } {
            let profile = heap.then(|| {
                dhat::Profiler::builder()
                    .testing()
                    .trim_backtraces(Some(4))
                    .build()
            });
            let (mut map, footprint) = populated(count);
            let start = Instant::now();
            churn(&mut map, &footprint, count);
            let elapsed = start.elapsed().as_nanos();
            let start = Instant::now();
            let cloned = map.clone();
            let clone = start.elapsed().as_nanos();
            assert_eq!(map.objects().count(), count);
            let stats = profile.as_ref().map(|_| dhat::HeapStats::get());
            drop(cloned);
            drop(map);
            drop(footprint);
            let released = profile.as_ref().map(|_| dhat::HeapStats::get().curr_bytes);
            drop(profile);
            if batch >= 2 || heap {
                println!("placement_churn,{count},{batch},{elapsed},{clone}");
            }
            if let Some(stats) = stats {
                println!(
                    "placement_churn_heap,{count},{},{},{},{},{}",
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
