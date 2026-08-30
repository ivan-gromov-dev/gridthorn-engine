use super::super::FrameGeometry;
use crate::{Camera2d, Color, RenderFrame, TextLabel, UiPrimitive, UiRect};

#[test]
fn projects_screen_rect_from_top_left_pixels() {
    let rect = UiRect::new([10.0, 20.0], [20.0, 10.0], Color::rgb(1.0, 0.0, 0.0))
        .expect("rectangle should be valid");
    let frame = RenderFrame::default().with_ui(vec![rect.into()]);

    let geometry = FrameGeometry::new(&frame, 100, 100);

    assert_slice_close(&geometry.vertices[0].position, &[-0.8, 0.6]);
    assert_slice_close(&geometry.vertices[2].position, &[-0.4, 0.4]);
}

#[test]
fn emits_bitmap_text_after_ordered_ui_rectangles() {
    let rect =
        UiRect::new([0.0, 0.0], [10.0, 10.0], Color::default()).expect("rectangle should be valid");
    let label = TextLabel::new("I", [20.0, 0.0], 2.0, Color::rgb(0.0, 1.0, 0.0))
        .expect("label should be valid");
    let frame = RenderFrame::new(Camera2d::new([0.0, 0.0], 0.0), Vec::new())
        .with_ui(vec![UiPrimitive::from(rect), UiPrimitive::from(label)]);

    let geometry = FrameGeometry::new(&frame, 100, 100);

    assert_eq!(geometry.vertices.len(), 6 + 11 * 6);
    assert_slice_close(&geometry.vertices[6].color, &[0.0, 1.0, 0.0, 1.0]);
}

fn assert_slice_close(actual: &[f32], expected: &[f32]) {
    assert!(
        actual
            .iter()
            .zip(expected)
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
}
