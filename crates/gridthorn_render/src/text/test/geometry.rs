use super::{style, system};
use crate::presentation::FrameGeometry;
use crate::{Color, RenderFrame, UiRect};

#[test]
fn ordered_overlay_uses_physical_dpi_positions_and_clips_offscreen_text() {
    let mut text = system();
    let layout = text.layout("Я", &style()).expect("layout");
    let raster = text
        .rasterize(&layout, 2.0, Color::rgb(0.0, 1.0, 0.0))
        .expect("raster")
        .at([10.0, 20.0])
        .expect("position");
    let first = &raster.pixels[0];
    let expected = [
        (20.0 + first.position[0]) / 200.0 * 2.0 - 1.0,
        1.0 - (40.0 + first.position[1]) / 200.0 * 2.0,
    ];
    let panel = UiRect::new([0.0, 0.0], [200.0, 200.0], Color::rgb(1.0, 0.0, 0.0)).expect("panel");
    let after = UiRect::new([0.0, 0.0], [5.0, 5.0], Color::rgb(0.0, 0.0, 1.0)).expect("after");
    let frame =
        RenderFrame::default().with_ui(vec![panel.into(), raster.clone().into(), after.into()]);
    let geometry = FrameGeometry::new(&frame, 200, 200);
    assert_eq!(geometry.world_vertex_count, 0);
    assert_eq!(geometry.vertices[0].color, [1.0, 0.0, 0.0, 1.0]);
    assert_eq!(geometry.vertices[6].position, expected);
    assert_eq!(
        geometry.vertices.last().expect("after").color,
        [0.0, 0.0, 1.0, 1.0]
    );
    assert!(geometry.vertices[6].color[1] > 0.0);
    let offscreen = RenderFrame::default()
        .with_ui(vec![raster.at([1000.0, 1000.0]).expect("offscreen").into()]);
    assert!(FrameGeometry::new(&offscreen, 200, 200).vertices.is_empty());
    assert!(FrameGeometry::new(&frame, 0, 0).vertices.is_empty());
}
