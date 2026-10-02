use super::clock;
use crate::{
    FixedStepClock, FixedStepConfig, SimulationControl, SimulationSpeed, SimulationSpeedError,
    TimeError,
};
use std::time::Duration;

#[test]
fn validates_and_normalizes_positive_ratios() {
    assert_eq!(
        SimulationSpeed::new(0, 1),
        Err(SimulationSpeedError::ZeroRatioTerm)
    );
    assert_eq!(
        SimulationSpeed::new(1, 0),
        Err(SimulationSpeedError::ZeroRatioTerm)
    );
    let speed = SimulationSpeed::new(6, 4).unwrap();
    assert_eq!((speed.numerator(), speed.denominator()), (3, 2));
}

#[test]
fn pause_freezes_overloaded_backlog_and_resume_ignores_paused_time() {
    let mut clock = clock(10, 2);
    let mut control = SimulationControl::default();
    clock.advance(Duration::from_millis(55)).unwrap();
    control.pause();
    let paused = clock.advance_controlled(Duration::MAX, control).unwrap();
    assert_eq!(paused.fixed_steps(), 0);
    assert_eq!(paused.completed_ticks(), 2);
    assert_eq!(paused.accumulated_lag(), Duration::from_millis(35));
    assert_eq!(paused.frame_elapsed(), Duration::MAX);
    assert!(!paused.overloaded());
    control.resume();
    let resumed = clock.advance_controlled(Duration::ZERO, control).unwrap();
    assert_eq!(resumed.first_tick_index(), 2);
    assert_eq!(resumed.fixed_steps(), 2);
    assert_eq!(resumed.accumulated_lag(), Duration::from_millis(15));
}

#[test]
fn speed_changes_preserve_lag_and_fixed_duration() {
    let mut clock = clock(10, 4);
    let mut control = SimulationControl::default();
    control.set_speed(SimulationSpeed::new(1, 2).unwrap());
    let slow = clock
        .advance_controlled(Duration::from_millis(15), control)
        .unwrap();
    assert_eq!(slow.fixed_steps(), 0);
    assert_eq!(slow.accumulated_lag(), Duration::from_micros(7500));
    control.set_speed(SimulationSpeed::new(2, 1).unwrap());
    let fast = clock
        .advance_controlled(Duration::from_millis(15), control)
        .unwrap();
    assert_eq!(fast.fixed_steps(), 3);
    assert_eq!(fast.accumulated_lag(), Duration::from_micros(7500));
    assert_eq!(fast.frame_elapsed(), Duration::from_millis(15));
    assert_eq!(clock.fixed_step(), Duration::from_millis(10));
}

#[test]
fn fractional_time_is_independent_of_frame_partition() {
    let config = FixedStepConfig::new(Duration::from_nanos(1), 100).unwrap();
    let mut partitioned = FixedStepClock::new(config);
    let mut whole = FixedStepClock::new(config);
    let mut control = SimulationControl::default();
    control.set_speed(SimulationSpeed::new(1, 3).unwrap());
    for _ in 0..8 {
        partitioned
            .advance_controlled(Duration::from_nanos(1), control)
            .unwrap();
    }
    let final_part = partitioned
        .advance_controlled(Duration::from_nanos(1), control)
        .unwrap();
    let final_whole = whole
        .advance_controlled(Duration::from_nanos(9), control)
        .unwrap();
    assert_eq!(final_part.completed_ticks(), final_whole.completed_ticks());
    assert_eq!(final_part.accumulated_lag(), final_whole.accumulated_lag());
}

#[test]
fn overflow_leaves_clock_and_fractional_remainder_unchanged() {
    let mut clock = clock(10, 4);
    let mut control = SimulationControl::default();
    control.set_speed(SimulationSpeed::new(2, 1).unwrap());
    assert_eq!(
        clock.advance_controlled(Duration::MAX, control),
        Err(TimeError::ElapsedArithmeticOverflow)
    );
    let next = clock.advance(Duration::from_millis(10)).unwrap();
    assert_eq!(next.first_tick_index(), 0);
    assert_eq!(next.fixed_steps(), 1);
}

#[test]
fn explicit_steps_ignore_pause_speed_and_preserve_lag() {
    let mut clock = clock(10, 1);
    clock.advance(Duration::from_millis(25)).unwrap();
    let stepped = clock.advance_steps(10).unwrap();
    assert_eq!(stepped.fixed_steps(), 10);
    assert_eq!(stepped.first_tick_index(), 1);
    assert_eq!(stepped.accumulated_lag(), Duration::from_millis(15));
}

#[test]
fn equivalent_speed_and_pause_preserve_fractional_nanoseconds() {
    let config = FixedStepConfig::new(Duration::from_nanos(1), 4).unwrap();
    let mut clock = FixedStepClock::new(config);
    let mut control = SimulationControl::default();
    control.set_speed(SimulationSpeed::new(1, 3).unwrap());
    clock
        .advance_controlled(Duration::from_nanos(1), control)
        .unwrap();
    control.pause();
    clock.advance_controlled(Duration::MAX, control).unwrap();
    control.resume();
    control.set_speed(SimulationSpeed::new(2, 6).unwrap());
    let frame = clock
        .advance_controlled(Duration::from_nanos(2), control)
        .unwrap();
    assert_eq!(frame.fixed_steps(), 1);
    assert_eq!(frame.accumulated_lag(), Duration::ZERO);
}

#[test]
fn speed_change_discards_only_subnanosecond_remainder() {
    let config = FixedStepConfig::new(Duration::from_nanos(2), 4).unwrap();
    let mut clock = FixedStepClock::new(config);
    let mut control = SimulationControl::default();
    control.set_speed(SimulationSpeed::new(1, 2).unwrap());
    clock
        .advance_controlled(Duration::from_nanos(3), control)
        .unwrap();
    control.set_speed(SimulationSpeed::new(1, 3).unwrap());
    let frame = clock
        .advance_controlled(Duration::from_nanos(2), control)
        .unwrap();
    assert_eq!(frame.fixed_steps(), 0);
    assert_eq!(frame.accumulated_lag(), Duration::from_nanos(1));
}
