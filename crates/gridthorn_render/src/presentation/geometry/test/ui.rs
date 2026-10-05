use super::super::FrameGeometry;
use crate::{Camera2d, Color, RenderFrame, Sprite, TextLabel, UiPrimitive, UiRect};

#[test]
fn nested_clip_does_not_change_preceding_or_following_siblings() {
    let rect = UiRect::new([0.0; 2], [100.0; 2], Color::rgb(1.0, 0.0, 0.0)).unwrap();
    let bounds = UiRect::new([20.0; 2], [40.0; 2], Color::default()).unwrap();
    let frame = RenderFrame::default().with_ui(vec![
        rect.into(),
        UiPrimitive::Clipped {
            bounds,
            children: vec![UiPrimitive::Clipped {
                bounds,
                children: vec![rect.into()],
            }],
        },
        rect.into(),
    ]);
    let geometry = FrameGeometry::new(&frame, 100, 100);
    assert_eq!(geometry.vertices.len(), 18);
    for offset in [0, 12] {
        assert_slice_close(&geometry.vertices[offset].position, &[-1.0, 1.0]);
        assert_slice_close(&geometry.vertices[offset + 2].position, &[1.0, -1.0]);
    }
    assert_slice_close(&geometry.vertices[6].position, &[-0.6, 0.6]);
    assert_slice_close(&geometry.vertices[8].position, &[0.2, -0.2]);
}

#[test]
fn nested_clips_intersect_rectangles_and_bitmap_ink_in_painter_order() {
    let bounds = UiRect::new([20.0, 20.0], [40.0, 40.0], Color::default()).unwrap();
    let inner = UiRect::new([40.0, 0.0], [40.0, 100.0], Color::default()).unwrap();
    let red = UiRect::new([0.0, 0.0], [100.0, 100.0], Color::rgb(1.0, 0.0, 0.0)).unwrap();
    let text = TextLabel::new("W", [35.0, 15.0], 10.0, Color::rgb(0.0, 1.0, 0.0)).unwrap();
    let frame = RenderFrame::default().with_ui(vec![UiPrimitive::Clipped {
        bounds,
        children: vec![UiPrimitive::Clipped {
            bounds: inner,
            children: vec![red.into(), text.into()],
        }],
    }]);
    let geometry = FrameGeometry::new(&frame, 100, 100);
    assert_slice_close(&geometry.vertices[0].position, &[-0.2, 0.6]);
    assert_slice_close(&geometry.vertices[2].position, &[0.2, -0.2]);
    for vertex in &geometry.vertices {
        assert!((-0.200_001..=0.200_001).contains(&vertex.position[0]));
        assert!((-0.200_001..=0.600_001).contains(&vertex.position[1]));
    }
    assert_slice_close(&geometry.vertices[0].color, &[1.0, 0.0, 0.0, 1.0]);
    assert_slice_close(&geometry.vertices[6].color, &[0.0, 1.0, 0.0, 1.0]);
}

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

#[test]
fn separates_world_geometry_from_the_overlay_drawn_after_textures() {
    let frame = RenderFrame::new(
        Camera2d::default(),
        vec![Sprite::new(
            [0.0, 0.0],
            [20.0, 20.0],
            Color::rgb(1.0, 0.0, 0.0),
        )],
    )
    .with_ui(vec![
        UiRect::new([0.0, 0.0], [20.0, 20.0], Color::rgb(0.0, 1.0, 0.0))
            .expect("UI bounds")
            .into(),
    ]);
    let geometry = FrameGeometry::new(&frame, 100, 100);
    assert_eq!(geometry.world_vertex_count, 6);
    assert_eq!(geometry.vertices.len(), 12);
    assert_slice_close(&geometry.vertices[5].color, &[1.0, 0.0, 0.0, 1.0]);
    assert_slice_close(&geometry.vertices[6].color, &[0.0, 1.0, 0.0, 1.0]);
    assert_eq!(
        FrameGeometry::new(
            &RenderFrame::default().with_ui(frame.ui().to_vec()),
            100,
            100
        )
        .world_vertex_count,
        0
    );
}

#[test]
fn timing_diagnostics_remain_above_opaque_ui_panels() {
    let frame = RenderFrame::default()
        .with_ui(vec![
            UiRect::new([0.0, 0.0], [100.0, 100.0], Color::rgb(0.0, 0.0, 0.0))
                .expect("panel")
                .into(),
        ])
        .with_timing_overlay(crate::TimingOverlay::new(
            std::time::Duration::from_millis(20),
            2,
            std::time::Duration::from_millis(35),
            true,
        ));
    let geometry = FrameGeometry::new(&frame, 100, 100);
    assert_eq!(geometry.world_vertex_count, 0);
    assert_slice_close(&geometry.vertices[0].color, &[0.0, 0.0, 0.0, 1.0]);
    assert_slice_close(&geometry.vertices[18].color, &[1.0, 0.18, 0.16, 0.95]);
}
