use std::{hint::black_box, time::Instant};

use super::population::{enqueue, populated};

/// Whole-history peak includes both scenario roots, queued payloads and retained clones.
#[test]
#[ignore = "manual population CPU/heap probe; run alone with --release --nocapture"]
fn measure_population_retention() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let heap = std::env::var_os("GRIDTHORN_DOMAIN_HEAP").is_some();
    for count in [256, 4096, 16_384] {
        for history in [1, 8, 32] {
            for batch in 0..if heap { 1 } else { 22 } {
                let profile = heap.then(|| {
                    dhat::Profiler::builder()
                        .testing()
                        .trim_backtraces(Some(4))
                        .build()
                });
                let mut runner = populated(count);
                enqueue(&mut runner, 1024);
                let start = Instant::now();
                let mut snapshots = Vec::with_capacity(history);
                for _ in 0..history {
                    snapshots.push(runner.snapshot().unwrap());
                    runner.run_ticks(1).unwrap();
                }
                let capture_ticks = start.elapsed().as_nanos();
                let start = Instant::now();
                let cloned = snapshots[0].clone();
                let clone = start.elapsed().as_nanos();
                let start = Instant::now();
                runner.restore(&cloned).unwrap();
                let restore = start.elapsed().as_nanos();
                assert_eq!(runner.snapshot().unwrap().state(), cloned.state());
                let retained = profile.as_ref().map(|_| dhat::HeapStats::get());
                let start = Instant::now();
                drop(snapshots);
                drop(cloned);
                let release = start.elapsed().as_nanos();
                drop(runner);
                let final_stats = profile.as_ref().map(|_| dhat::HeapStats::get());
                drop(profile);
                if batch >= 2 || heap {
                    println!(
                        "population,{count},{history},{batch},{capture_ticks},{clone},{restore},{release}"
                    );
                }
                if let (Some(retained), Some(final_stats)) = (retained, final_stats) {
                    println!(
                        "population_heap,{count},{history},{},{},{},{},{}",
                        retained.total_blocks,
                        retained.total_bytes,
                        retained.max_bytes,
                        retained.curr_bytes,
                        final_stats.curr_bytes
                    );
                }
            }
        }
    }
}
