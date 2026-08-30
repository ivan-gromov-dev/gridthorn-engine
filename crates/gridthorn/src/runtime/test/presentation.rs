use std::time::Duration;

use crate::prelude::{
    Camera2d, Color, RenderFrame, Sprite, TextLabel, TimingOverlay, UiPrimitive, UiRect,
};

#[test]
fn prelude_builds_an_engine_owned_sprite_frame() {
    let frame = RenderFrame::new(
        Camera2d::default(),
        vec![Sprite::new(
            [0.0, 0.0],
            [32.0, 32.0],
            Color::rgb(0.9, 0.4, 0.2),
        )],
    );

    assert_eq!(frame.sprites().len(), 1);
}

#[test]
fn prelude_attaches_timing_diagnostics_to_a_render_frame() {
    let frame = RenderFrame::default().with_timing_overlay(TimingOverlay::new(
        Duration::from_millis(16),
        1,
        Duration::ZERO,
        false,
    ));

    assert_eq!(frame.timing_overlay().expect("overlay").fixed_steps(), 1);
}

#[test]
fn prelude_builds_ordered_runtime_ui() {
    let panel = UiRect::new([8.0, 8.0], [120.0, 24.0], Color::rgba(0.0, 0.0, 0.0, 0.8))
        .expect("panel should be valid");
    let label = TextLabel::new("SCORE 10", [12.0, 12.0], 2.0, Color::default())
        .expect("label should be valid");
    let frame =
        RenderFrame::default().with_ui(vec![UiPrimitive::from(panel), UiPrimitive::from(label)]);

    assert_eq!(frame.ui().len(), 2);
}
