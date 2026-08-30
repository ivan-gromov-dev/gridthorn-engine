use std::time::Duration;

use crate::prelude::{Camera2d, Color, RenderFrame, Sprite, TimingOverlay};

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
