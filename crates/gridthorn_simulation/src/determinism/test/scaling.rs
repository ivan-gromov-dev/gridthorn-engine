use std::{hint::black_box, time::Instant};

use crate::{DeterministicRng, RandomStreams};

/// Manual direct versus named-stream draw probe preserving `SplitMix64` outputs.
#[test]
#[ignore = "manual RNG scaling probe; run alone with --release"]
fn measure_rng_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("rng,streams,batch,phase,draws,elapsed_ns");
    for count in [1, 64, 1024] {
        let mut initial = RandomStreams::new(42);
        let names = (0..count)
            .map(|index| format!("stream-{index:04}"))
            .collect::<Vec<_>>();
        for name in &names {
            initial.register(name).unwrap();
        }
        for batch in 0..22 {
            let mut registry = initial.clone();
            let mut direct = initial
                .states()
                .map(|(_, state)| DeterministicRng::new(state))
                .collect::<Vec<_>>();
            let start = Instant::now();
            let mut named_sum = 0;
            for index in 0..16_384 {
                named_sum ^=
                    black_box(registry.next_u64(black_box(&names[index % count])).unwrap());
            }
            let named_time = start.elapsed().as_nanos();
            let start = Instant::now();
            let mut direct_sum = 0;
            for index in 0..16_384 {
                direct_sum ^= black_box(direct[index % count].next_u64());
            }
            let direct_time = start.elapsed().as_nanos();
            assert_eq!(named_sum, direct_sum);
            assert_eq!(
                registry
                    .states()
                    .map(|(_, state)| state)
                    .collect::<Vec<_>>(),
                direct
                    .iter()
                    .map(DeterministicRng::state)
                    .collect::<Vec<_>>()
            );
            if batch >= 2 {
                for (phase, time) in [("named", named_time), ("direct", direct_time)] {
                    println!("rng,{count},{batch},{phase},16384,{time}");
                }
            }
        }
    }
}
