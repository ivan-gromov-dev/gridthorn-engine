use std::{hint::black_box, time::Instant};

use gridthorn_simulation::GameCommandQueue;

/// Draining releases payloads while the caller-owned queue retains reusable capacity.
#[test]
#[ignore = "manual queue CPU/heap probe; run alone with --release"]
fn measure_command_queue_retention() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let heap = std::env::var_os("GRIDTHORN_DOMAIN_HEAP").is_some();
    for count in [1024, 65_536] {
        for batch in 0..if heap { 1 } else { 22 } {
            let profile = heap.then(|| {
                dhat::Profiler::builder()
                    .testing()
                    .trim_backtraces(Some(4))
                    .build()
            });
            let mut queue = GameCommandQueue::new();
            for index in 0..count {
                queue.push(vec![index; 8]);
            }
            let filled = profile.as_ref().map(|_| dhat::HeapStats::get());
            let start = Instant::now();
            for (index, payload) in queue.drain().enumerate() {
                assert_eq!(payload, [index; 8]);
            }
            let elapsed = start.elapsed().as_nanos();
            assert!(queue.is_empty());
            let drained = profile.as_ref().map(|_| dhat::HeapStats::get());
            drop(queue);
            let released = profile.as_ref().map(|_| dhat::HeapStats::get().curr_bytes);
            drop(profile);
            if batch >= 2 || heap {
                println!("command_queue,{count},{batch},{elapsed}");
            }
            if let (Some(filled), Some(drained)) = (filled, drained) {
                println!(
                    "command_queue_heap,{count},{},{},{},{},{}",
                    filled.total_blocks,
                    filled.total_bytes,
                    filled.max_bytes,
                    drained.curr_bytes,
                    released.unwrap()
                );
            }
        }
    }
}
