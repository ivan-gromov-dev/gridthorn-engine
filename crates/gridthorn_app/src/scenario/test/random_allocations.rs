use std::{hint::black_box, time::Instant};

use gridthorn_simulation::RandomStreams;

/// Include registry/name ownership; isolate draw allocations from construction.
#[test]
#[ignore = "manual named RNG CPU/heap probe; run alone with --release"]
fn measure_named_rng_allocations() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let heap = std::env::var_os("GRIDTHORN_DOMAIN_HEAP").is_some();
    for count in [1, 64, 1024] {
        let names: Vec<_> = (0..count)
            .map(|i| format!("simulation/экономика/sector/{i:04}/production"))
            .collect();
        for batch in 0..if heap { 1 } else { 22 } {
            let profile = heap.then(|| {
                dhat::Profiler::builder()
                    .testing()
                    .trim_backtraces(Some(4))
                    .build()
            });
            let mut random = RandomStreams::new(42);
            for name in &names {
                random.register(name).unwrap();
            }
            let before = profile.as_ref().map(|_| dhat::HeapStats::get());
            let start = Instant::now();
            let mut checksum = 0;
            for index in 0..65_536 {
                checksum ^= random.next_u64(black_box(&names[index % count])).unwrap();
            }
            let elapsed = start.elapsed().as_nanos();
            black_box(checksum);
            let after = profile.as_ref().map(|_| dhat::HeapStats::get());
            assert_eq!(random.states().count(), count);
            drop(random);
            let released = profile.as_ref().map(|_| dhat::HeapStats::get().curr_bytes);
            drop(profile);
            if batch >= 2 || heap {
                println!("named_rng_owned,{count},{batch},{elapsed},{checksum}");
            }
            if let (Some(before), Some(after)) = (before, after) {
                println!(
                    "named_rng_owned_heap,{count},{},{},{},{},{}",
                    after.total_blocks,
                    after.total_bytes,
                    after.max_bytes,
                    after.total_blocks - before.total_blocks,
                    released.unwrap()
                );
            }
        }
    }
}
