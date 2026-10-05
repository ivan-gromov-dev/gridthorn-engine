use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use crate::{FixedStepClock, FixedStepConfig, SimulationControl};

/// Manual clock arithmetic probe, excluding execution of assigned schedules.
#[test]
#[ignore = "manual fixed clock probe; run alone with --release"]
fn measure_fixed_clock_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("fixed_clock,kind,batch,frames,elapsed_ns");
    for kind in ["zero", "normal", "catch_up", "paused"] {
        for batch in 0..22 {
            let mut clock =
                FixedStepClock::new(FixedStepConfig::new(Duration::from_millis(10), 8).unwrap());
            let mut control = SimulationControl::default();
            if kind == "paused" {
                control.pause();
            }
            let elapsed = match kind {
                "zero" => Duration::ZERO,
                "catch_up" => Duration::from_millis(200),
                _ => Duration::from_millis(10),
            };
            let start = Instant::now();
            let mut ticks = 0;
            for _ in 0..10_000 {
                ticks += u64::from(
                    black_box(
                        clock
                            .advance_controlled(black_box(elapsed), black_box(control))
                            .unwrap(),
                    )
                    .fixed_steps(),
                );
            }
            let time = start.elapsed().as_nanos();
            assert_eq!(
                ticks,
                match kind {
                    "normal" => 10_000,
                    "catch_up" => 80_000,
                    _ => 0,
                }
            );
            if batch >= 2 {
                println!("fixed_clock,{kind},{batch},10000,{time}");
            }
        }
    }
}
