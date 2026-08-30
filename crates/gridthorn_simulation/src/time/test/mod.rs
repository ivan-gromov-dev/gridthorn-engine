use std::time::Duration;

use super::{FixedStepClock, FixedStepConfig, FixedStepConfigError};

fn clock(step_millis: u64, max_catch_up_steps: u32) -> FixedStepClock {
    let config = FixedStepConfig::new(Duration::from_millis(step_millis), max_catch_up_steps)
        .expect("test configuration should be valid");
    FixedStepClock::new(config)
}

#[test]
fn rejects_configuration_that_cannot_advance() {
    assert_eq!(
        FixedStepConfig::new(Duration::ZERO, 1),
        Err(FixedStepConfigError::ZeroDuration)
    );
    assert_eq!(
        FixedStepConfig::new(Duration::from_millis(10), 0),
        Err(FixedStepConfigError::ZeroCatchUpLimit)
    );
}

#[test]
fn accumulates_partial_frames_into_integer_tick_indices() {
    let mut clock = clock(10, 4);

    let first = clock
        .advance(Duration::from_millis(6))
        .expect("time should advance");
    let second = clock
        .advance(Duration::from_millis(15))
        .expect("time should advance");

    assert_eq!(first.fixed_steps(), 0);
    assert_eq!(first.accumulated_lag(), Duration::from_millis(6));
    assert_eq!(second.fixed_steps(), 2);
    assert_eq!(second.first_tick_index(), 0);
    assert_eq!(second.completed_ticks(), 2);
    assert_eq!(second.accumulated_lag(), Duration::from_millis(1));
}

#[test]
fn reports_overload_and_preserves_backlog_for_later_frames() {
    let mut clock = clock(10, 2);

    let overloaded = clock
        .advance(Duration::from_millis(55))
        .expect("time should advance");
    let catch_up = clock
        .advance(Duration::ZERO)
        .expect("backlog should advance");

    assert_eq!(overloaded.fixed_steps(), 2);
    assert!(overloaded.overloaded());
    assert_eq!(overloaded.accumulated_lag(), Duration::from_millis(35));
    assert_eq!(catch_up.fixed_steps(), 2);
    assert!(catch_up.overloaded());
    assert_eq!(catch_up.first_tick_index(), 2);
    assert_eq!(catch_up.accumulated_lag(), Duration::from_millis(15));
}
