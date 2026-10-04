use std::{hint::black_box, time::Instant};

use crate::{GridCell, GridFootprint, GridObjectId, PlacementMap};

/// Manual sparse occupancy probe; terrain policy remains caller-owned.
#[test]
#[ignore = "manual placement scaling probe; run alone with --release"]
fn measure_placement_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("placement,objects,footprint,batch,phase,elapsed_ns");
    for count in [1024_i32, 16_384, 65_536] {
        for width in [1_i32, 16] {
            let footprint =
                GridFootprint::new((0..width).map(|column| GridCell::new(column, 0))).unwrap();
            let mut map = PlacementMap::new();
            for index in 0..count {
                map.place(
                    GridObjectId(u64::try_from(index).unwrap()),
                    GridCell::new(index * 32, 0),
                    footprint.clone(),
                )
                .unwrap();
            }
            for batch in 0..22 {
                let start = Instant::now();
                let hits = (0..1024)
                    .filter(|index| {
                        map.object_at(black_box(GridCell::new((index * 61 % count) * 32, 0)))
                            .is_some()
                    })
                    .count();
                let lookup = start.elapsed().as_nanos();
                assert_eq!(hits, 1024);
                let start = Instant::now();
                for index in 0..1024 {
                    black_box(
                        map.validate(
                            GridObjectId(u64::MAX),
                            GridCell::new(index * 32, 2),
                            &footprint,
                        )
                        .unwrap(),
                    );
                }
                let validate = start.elapsed().as_nanos();
                let start = Instant::now();
                for index in 0..1024 {
                    let id = GridObjectId(u64::try_from(index).unwrap());
                    map.relocate(id, GridCell::new(index * 32, 1), footprint.clone())
                        .unwrap();
                    map.relocate(id, GridCell::new(index * 32, 0), footprint.clone())
                        .unwrap();
                }
                let relocate = start.elapsed().as_nanos();
                assert_eq!(map.objects().len(), usize::try_from(count).unwrap());
                for index in 0..1024 {
                    assert_eq!(
                        map.object_at(GridCell::new(index * 32, 0)),
                        Some(GridObjectId(u64::try_from(index).unwrap()))
                    );
                    assert_eq!(map.object_at(GridCell::new(index * 32, 1)), None);
                }
                if batch >= 2 {
                    for (phase, elapsed) in [
                        ("lookup_1024", lookup),
                        ("validate_1024", validate),
                        ("relocate_2048", relocate),
                    ] {
                        println!("placement,{count},{width},{batch},{phase},{elapsed}");
                    }
                }
            }
        }
    }
}
