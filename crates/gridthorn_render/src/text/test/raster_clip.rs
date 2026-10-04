use super::{style, system};
use crate::presentation::FrameGeometry;
use crate::{Color, RenderFrame, UiPrimitive, UiRect};

#[test]
fn clipped_raster_preserves_ordered_visible_geometry_at_fractional_origins_and_dpi() {
    let mut text = system();
    let mut settings = style();
    settings.width = Some(180.0);
    let content = "office e\u{301} Привет مرحبًا 日本語\n".repeat(16);
    let layout = text.layout(&content, &settings).unwrap();
    for dpi in [1.0, 1.25, 2.0] {
        for origin in [[10.25, 20.75], [-35.5, -80.25]] {
            let clip = UiRect::new([17.25, 25.5], [130.5, 97.25], Color::default()).unwrap();
            let local = UiRect::new(
                std::array::from_fn(|axis| clip.position()[axis] - origin[axis] * dpi),
                clip.size(),
                Color::default(),
            )
            .unwrap();
            let tint = Color::rgba(0.5, 0.25, 0.75, 0.3);
            let full = text
                .rasterize(&layout, dpi, tint)
                .unwrap()
                .at(origin)
                .unwrap();
            let clipped = text
                .rasterize_clipped(&layout, dpi, tint, local)
                .unwrap()
                .at(origin)
                .unwrap();
            assert_eq!(full.measurement(), clipped.measurement());
            assert!(clipped.pixel_count() < full.pixel_count());
            let frame = |raster: crate::RasterText| {
                RenderFrame::default().with_ui(vec![UiPrimitive::Clipped {
                    bounds: clip,
                    children: vec![raster.into()],
                }])
            };
            let expected = FrameGeometry::new(&frame(full), 400, 400);
            let actual = FrameGeometry::new(&frame(clipped), 400, 400);
            assert!(!actual.vertices.is_empty());
            assert_eq!(
                visible_geometry(&actual),
                visible_geometry(&expected),
                "DPI{dpi} origin{origin:?}"
            );
        }
    }
}

#[expect(
    clippy::float_cmp,
    reason = "exactly collapsed clipped quads have no visible area"
)]
fn visible_geometry(geometry: &FrameGeometry) -> Vec<([f32; 2], [f32; 4])> {
    geometry
        .vertices
        .as_chunks::<6>()
        .0
        .iter()
        .filter(|quad| {
            quad[0].position[0] != quad[2].position[0] && quad[0].position[1] != quad[2].position[1]
        })
        .flat_map(|quad| quad.iter().map(|vertex| (vertex.position, vertex.color)))
        .collect()
}

#[test]
fn offscreen_glyphs_still_enforce_work_budget_and_snapshot_survives() {
    let mut text = system();
    let clip = UiRect::new([10000.0; 2], [1.0; 2], Color::default()).unwrap();
    let normal = text.layout("Я", &style()).unwrap();
    let snapshot = text.rasterize(&normal, 1.0, Color::default()).unwrap();
    assert_eq!(
        text.rasterize_clipped(&normal, 1.0, Color::default(), clip)
            .unwrap()
            .pixel_count(),
        0
    );
    let large = text
        .layout(&"Я".repeat(100), &crate::TextStyle::new("Noto Sans", 200.0))
        .unwrap();
    assert_eq!(
        text.rasterize_clipped(&large, 2.0, Color::default(), clip),
        Err(crate::TextError::TooLarge)
    );
    assert_eq!(
        text.rasterize(&normal, 1.0, Color::default()).unwrap(),
        snapshot
    );
    assert_eq!(
        system().rasterize_clipped(&normal, 1.0, Color::default(), clip),
        Err(crate::TextError::ForeignLayout)
    );
    assert_eq!(
        text.rasterize_clipped(&normal, 0.0, Color::default(), clip),
        Err(crate::TextError::InvalidMetrics)
    );
}
