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
