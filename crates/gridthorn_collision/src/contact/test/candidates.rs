use std::{hint::black_box, time::Instant};

use super::{aabb, circle, contact};
use crate::Collider2d;

fn groups(count: usize) -> Vec<[Collider2d; 4]> {
    (0..count)
        .map(|index| {
            let x = f32::from(u16::try_from(index).unwrap()) * 8.0;
            [
                aabb([x, 0.0], [1.0, 1.0]),
                circle([x + 1.5, 1.5], 1.0),
                circle([x, 0.0], 0.5),
                circle([x + 2.0, 0.5], 0.75),
            ]
        })
        .collect()
}

fn selected(groups: &[[Collider2d; 4]]) -> usize {
    groups
        .iter()
        .map(|group| {
            let mut hits = 0;
            for (index, first) in group.iter().enumerate() {
                for second in &group[index + 1..] {
                    hits += usize::from(contact(black_box(*first), black_box(*second)).is_some());
                }
            }
            hits
        })
        .sum()
}

#[test]
fn caller_partition_has_same_contacts_as_all_pairs() {
    let groups = groups(64);
    let shapes: Vec<_> = groups.iter().flatten().copied().collect();
    let mut hits = 0;
    for (index, first) in shapes.iter().enumerate() {
        for second in &shapes[index + 1..] {
            hits += usize::from(contact(*first, *second).is_some());
        }
    }
    assert_eq!(selected(&groups), hits);
    assert_eq!(hits, 3 * groups.len());
}

/// Caller partitions separated four-shape clusters; no engine broad phase is implied.
#[test]
#[ignore = "manual mixed candidate CPU/heap probe; run alone with --release"]
fn measure_collision_candidates() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let heap = std::env::var_os("GRIDTHORN_DOMAIN_HEAP").is_some();
    for count in [256, 4096, 65_536] {
        let groups = groups(count);
        for batch in 0..if heap { 1 } else { 22 } {
            let profile = heap.then(|| {
                dhat::Profiler::builder()
                    .testing()
                    .trim_backtraces(Some(4))
                    .build()
            });
            let start = Instant::now();
            let hits = selected(&groups);
            let elapsed = start.elapsed().as_nanos();
            assert_eq!(hits, count * 3);
            let stats = profile.as_ref().map(|_| dhat::HeapStats::get());
            drop(profile);
            if batch >= 2 || heap {
                println!("collision_candidates,{count},{batch},{elapsed},{hits}");
            }
            if let Some(stats) = stats {
                println!(
                    "collision_candidates_heap,{count},{},{},{},{}",
                    stats.total_blocks, stats.total_bytes, stats.max_bytes, stats.curr_bytes
                );
            }
        }
    }
}
