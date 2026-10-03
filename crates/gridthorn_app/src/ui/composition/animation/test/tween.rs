use super::super::*;
use std::time::Duration;

#[test]
fn easing_samples_and_exact_endpoints() {
    for (easing, halfway) in [
        (UiEasing::Linear, 0.5),
        (UiEasing::EaseIn, 0.25),
        (UiEasing::EaseOut, 0.75),
        (UiEasing::SmoothStep, 0.5),
    ] {
        let mut tween = UiTween::new([0.0], [1.0], Duration::from_secs(1), easing).unwrap();
        assert_eq!(tween.value(), [0.0]);
        assert_eq!(tween.advance(Duration::from_millis(500)), [halfway]);
        assert_eq!(tween.advance(Duration::MAX), [1.0]);
        assert!(tween.is_finished());
        assert_eq!(tween.advance(Duration::MAX), [1.0]);
    }
    let tween = UiTween::new([0.0], [2.0], Duration::ZERO, UiEasing::Linear).unwrap();
    assert_eq!(tween.value(), [2.0]);
    assert!(tween.is_finished());
}

#[test]
fn interruption_pause_resume_and_invalid_retarget_preserve_state() {
    let mut tween = UiTween::new(
        [0.0, 10.0],
        [10.0, 0.0],
        Duration::from_secs(1),
        UiEasing::Linear,
    )
    .unwrap();
    tween.advance(Duration::from_millis(500));
    tween.pause();
    assert!(tween.is_paused());
    assert_eq!(tween.advance(Duration::MAX), [5.0; 2]);
    tween
        .retarget([15.0; 2], Duration::from_secs(1), UiEasing::Linear)
        .unwrap();
    assert_eq!(tween.value(), [5.0; 2]);
    assert_eq!(tween.advance(Duration::from_secs(1)), [5.0; 2]);
    assert!(
        tween
            .retarget([f32::NAN, 0.0], Duration::ZERO, UiEasing::Linear)
            .is_err()
    );
    tween.resume();
    assert_eq!(tween.advance(Duration::from_millis(500)), [10.0; 2]);
    assert!(!tween.is_finished());
    assert_eq!(tween.advance(Duration::from_millis(500)), [15.0; 2]);
}

#[test]
fn extreme_finite_values_do_not_overflow_and_partitioned_time_agrees() {
    let mut extreme = UiTween::new(
        [-f32::MAX],
        [f32::MAX],
        Duration::from_secs(1),
        UiEasing::Linear,
    )
    .unwrap();
    assert_eq!(extreme.advance(Duration::from_millis(500)), [0.0]);
    assert!(UiTween::new([f32::INFINITY], [0.0], Duration::ZERO, UiEasing::Linear).is_err());
    let mut one = UiTween::new([3.0], [9.0], Duration::from_secs(1), UiEasing::SmoothStep).unwrap();
    let mut split = one.clone();
    one.advance(Duration::from_millis(600));
    for _ in 0..6 {
        split.advance(Duration::from_millis(100));
    }
    assert_eq!(one.value(), split.value());
}
