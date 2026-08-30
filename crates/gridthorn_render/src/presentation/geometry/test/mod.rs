use super::FrameGeometry;
use crate::{Camera2d, Color, RenderFrame, Sprite};

#[test]
fn projects_centered_sprite_into_clip_space() {
    let frame = RenderFrame::new(
        Camera2d::new([0.0, 0.0], 100.0),
        vec![Sprite::new(
            [0.0, 0.0],
            [20.0, 10.0],
            Color::rgb(1.0, 0.5, 0.25),
        )],
    );

    let geometry = FrameGeometry::new(&frame, 200, 100);

    assert_eq!(geometry.vertices.len(), 6);
    assert_slice_close(&geometry.vertices[0].position, &[-0.1, 0.1]);
    assert_slice_close(&geometry.vertices[2].position, &[0.1, -0.1]);
    assert_slice_close(&geometry.vertices[0].color, &[1.0, 0.5, 0.25, 1.0]);
}

fn assert_slice_close(actual: &[f32], expected: &[f32]) {
    assert_eq!(actual.len(), expected.len());
    assert!(
        actual
            .iter()
            .zip(expected)
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
}

#[test]
fn ignores_invalid_camera_and_sprite_extents() {
    let invalid_camera = RenderFrame::new(Camera2d::new([0.0, 0.0], 0.0), Vec::new());
    let invalid_sprite = RenderFrame::new(
        Camera2d::default(),
        vec![Sprite::new([0.0, 0.0], [0.0, 10.0], Color::default())],
    );

    assert!(
        FrameGeometry::new(&invalid_camera, 100, 100)
            .vertices
            .is_empty()
    );
    assert!(
        FrameGeometry::new(&invalid_sprite, 100, 100)
            .vertices
            .is_empty()
    );
}
