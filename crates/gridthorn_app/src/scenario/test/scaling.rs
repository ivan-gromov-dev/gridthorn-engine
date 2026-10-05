use std::{hint::black_box, time::Instant};

use super::*;

/// Manual typed-root cloning probe; no persistence or arbitrary world capture.
#[test]
#[ignore = "manual snapshot scaling probe; run alone with --release"]
fn measure_snapshot_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("snapshot,values,streams,batch,phase,elapsed_ns");
    for values in [1024, 16_384, 262_144] {
        for streams in [1, 128] {
            let mut runner = runtime("economy", 1, FixedStepConfig::default());
            runner
                .world()
                .update_resource(|state: &mut ScenarioState<Vec<u64>, u64>| {
                    state.data = vec![7; values];
                    for index in 1..streams {
                        state.random.register(&format!("extra-{index:04}")).unwrap();
                    }
                    for value in 0..64 {
                        state.commands.push(value);
                    }
                });
            let mut expected_runner = runtime("economy", 1, FixedStepConfig::default());
            let initial = runner.snapshot().unwrap();
            expected_runner.restore(&initial).unwrap();
            expected_runner.run_ticks(1).unwrap();
            let expected = expected_runner.snapshot().unwrap();
            for batch in 0..22 {
                let start = Instant::now();
                let snapshot = runner.snapshot().unwrap();
                let capture = start.elapsed().as_nanos();
                let start = Instant::now();
                let cloned = snapshot.clone();
                let clone = start.elapsed().as_nanos();
                assert_eq!(cloned.state(), initial.state());
                runner.run_ticks(1).unwrap();
                assert_eq!(runner.snapshot().unwrap().state(), expected.state());
                let start = Instant::now();
                runner.restore(&snapshot).unwrap();
                let restore = start.elapsed().as_nanos();
                assert_eq!(runner.snapshot().unwrap().state(), initial.state());
                assert_eq!(snapshot.completed_ticks(), 0);
                if batch >= 2 {
                    for (phase, time) in
                        [("capture", capture), ("clone", clone), ("restore", restore)]
                    {
                        println!("snapshot,{values},{streams},{batch},{phase},{time}");
                    }
                }
            }
        }
    }
}

/// Manual exact-tick runner probe with deterministic scalar systems.
#[test]
#[ignore = "manual fixed schedule scaling probe; run alone with --release"]
fn measure_fixed_schedule_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("fixed_schedule,systems,batch,ticks,elapsed_ns");
    for systems in [0, 16, 256] {
        for batch in 0..22 {
            let mut schedules = ScheduleBuilder::new();
            for _ in 0..systems {
                schedules.add_system(ScheduleStage::FixedUpdate, |world| {
                    world.update_resource(|state: &mut ScenarioState<Vec<u64>, u64>| {
                        state.data[0] += 1;
                    });
                });
            }
            let mut scenario = scenario("scaling", 1);
            scenario.initial.data = vec![0];
            let mut runner =
                ScenarioRuntime::new(schedules.build(), FixedStepConfig::default(), scenario)
                    .unwrap();
            let start = Instant::now();
            let report = runner.run_ticks(1000).unwrap();
            let time = start.elapsed().as_nanos();
            assert_eq!(report.completed_ticks, 1000);
            assert_eq!(runner.snapshot().unwrap().state().data, [systems * 1000]);
            if batch >= 2 {
                println!("fixed_schedule,{systems},{batch},1000,{time}");
            }
        }
    }
}
