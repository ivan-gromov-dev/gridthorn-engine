use std::time::Duration;

use crate::prelude::*;

#[test]
fn prelude_configures_and_observes_fixed_time() {
    let schedules = ScheduleBuilder::new();
    let config = FixedStepConfig::new(Duration::from_millis(10), 2)
        .expect("test configuration should be valid");
    let mut application = ApplicationRuntime::with_fixed_step(schedules.build(), config);

    let timing = application
        .run_timed_frame(Duration::from_millis(25))
        .expect("timed frame should run");

    assert_eq!(timing.fixed_steps(), 2);
    assert_eq!(timing.completed_ticks(), 2);
}

#[test]
fn prelude_builds_a_windowed_timed_runtime() {
    let schedules = ScheduleBuilder::new();
    let runtime = ApplicationRuntime::new(schedules.build());

    let _application = WindowedApplication::new(WindowConfig::default(), runtime);
}

#[test]
fn facade_preserves_preconfigured_pause_and_reports_speed_errors() {
    use crate::{SimulationControl, SimulationSpeed, SimulationSpeedError};
    let mut control = SimulationControl::default();
    control.pause();
    control.set_speed(SimulationSpeed::new(4, 1).unwrap());
    let mut schedules = ScheduleBuilder::new().build();
    schedules.world().insert_resource(control);
    let mut app = ApplicationRuntime::new(schedules);
    assert_eq!(
        app.world().read_resource(|c: &SimulationControl| *c),
        Some(control)
    );
    let paused = app.run_timed_frame(Duration::from_secs(1)).unwrap();
    assert_eq!(paused.fixed_steps(), 0);
    assert_eq!(
        SimulationSpeed::new(0, 1),
        Err(SimulationSpeedError::ZeroRatioTerm)
    );
}
