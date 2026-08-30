use crate::prelude::{Camera2d, Color, RenderFrame, Sprite};

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
