use std::{hint::black_box, time::Instant};

use gridthorn_world::{ScheduleBuilder, ScheduleStage};

use super::{ApplicationRuntime, RuntimeWindowLifecycle, WindowControl, WindowLifecycle};

/// Manual lifecycle callback probe without a native window, GPU or OS suspension.
#[test]
#[ignore = "manual lifecycle scaling probe; run alone with --release"]
fn measure_platform_lifecycle_callbacks() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("platform_callbacks,systems,batch,phase,elapsed_ns");
    for systems in [0, 32, 1024] {
        for batch in 0..22 {
            let mut schedules = ScheduleBuilder::new();
            for _ in 0..systems {
                schedules.add_system(ScheduleStage::Shutdown, |world| {
                    world.update_resource(|count: &mut usize| *count += 1);
                });
            }
            let mut lifecycle =
                RuntimeWindowLifecycle::new(ApplicationRuntime::new(schedules.build()));
            lifecycle.runtime.world().insert_resource(0_usize);
            lifecycle.started(&mut WindowControl::default()).unwrap();
            let start = Instant::now();
            lifecycle.suspended();
            let suspend = start.elapsed().as_nanos();
            assert!(lifecycle.frame_timer.previous.is_none());
            let start = Instant::now();
            lifecycle.resumed();
            let resume = start.elapsed().as_nanos();
            assert!(lifecycle.frame_timer.previous.is_some());
            let start = Instant::now();
            lifecycle.shutdown();
            let shutdown = start.elapsed().as_nanos();
            assert_eq!(
                lifecycle
                    .runtime
                    .world()
                    .read_resource(|count: &usize| *count),
                Some(systems)
            );
            lifecycle.shutdown();
            assert_eq!(
                lifecycle
                    .runtime
                    .world()
                    .read_resource(|count: &usize| *count),
                Some(systems)
            );
            if batch >= 2 {
                for (phase, elapsed) in [
                    ("suspend", suspend),
                    ("resume", resume),
                    ("shutdown", shutdown),
                ] {
                    println!("platform_callbacks,{systems},{batch},{phase},{elapsed}");
                }
            }
        }
    }
}
