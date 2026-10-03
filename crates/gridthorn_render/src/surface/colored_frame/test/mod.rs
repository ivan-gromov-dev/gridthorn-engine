use super::*;
use crate::{Camera2d, Color, Sprite, TextStyle, TextSystem, UiRect};
use gridthorn_assets::FontAsset;

fn extent() -> SurfaceExtent {
    SurfaceExtent {
        width: 800,
        height: 600,
    }
}

fn assert_fresh_geometry(cache: &ColoredFrame, frame: &RenderFrame, extent: SurfaceExtent) {
    let fresh = FrameGeometry::new(frame, extent.width, extent.height);
    assert_eq!(
        bytemuck::cast_slice::<_, u8>(&cache.geometry.vertices),
        bytemuck::cast_slice::<_, u8>(&fresh.vertices)
    );
    assert_eq!(cache.geometry.world_vertex_count, fresh.world_vertex_count);
}

#[test]
fn changed_camera_sprite_clip_order_overlay_and_extent_rebuild_correct_geometry() {
    let red = UiRect::new([10.0; 2], [30.0; 2], Color::rgb(1.0, 0.0, 0.0)).unwrap();
    let blue = UiRect::new([15.0; 2], [20.0; 2], Color::rgb(0.0, 0.0, 1.0)).unwrap();
    let base = RenderFrame::default().with_ui(vec![red.into(), blue.into()]);
    let mut cache = ColoredFrame::new();
    let sprite = Sprite::new([5.0; 2], [20.0; 2], Color::default());
    let frames = [
        base.clone(),
        RenderFrame::new(Camera2d::default(), vec![sprite]).with_ui(base.ui().to_vec()),
        RenderFrame::new(Camera2d::new([10.0; 2], 300.0), vec![sprite]).with_ui(base.ui().to_vec()),
        RenderFrame::new(
            Camera2d::new([10.0; 2], 300.0),
            vec![Sprite::new([50.0; 2], [20.0; 2], Color::default())],
        )
        .with_ui(base.ui().to_vec()),
        RenderFrame::default().with_ui(vec![blue.into(), red.into()]),
        RenderFrame::default().with_ui(vec![UiPrimitive::Clipped {
            bounds: blue,
            children: vec![red.into()],
        }]),
        base.clone().with_timing_overlay(crate::TimingOverlay::new(
            std::time::Duration::from_millis(20),
            1,
            std::time::Duration::ZERO,
            false,
        )),
        RenderFrame::default(),
    ];
    for frame in frames {
        assert!(cache.prepare(&frame, extent()));
        assert_fresh_geometry(&cache, &frame, extent());
        let allocation = cache.geometry.vertices.as_ptr();
        assert!(!cache.prepare(&frame.clone(), extent()));
        assert_eq!(cache.geometry.vertices.as_ptr(), allocation);
    }
    assert!(cache.geometry.vertices.is_empty());
    assert!(cache.prepare(&base, extent()));
    let resized = SurfaceExtent {
        width: 1600,
        height: 1200,
    };
    assert!(cache.prepare(&base, resized));
    assert_fresh_geometry(&cache, &base, resized);
}

#[test]
fn shared_and_independent_raster_snapshots_invalidate_on_position_dpi_and_color() {
    let asset = FontAsset::from_bytes(
        include_bytes!("../../../text/test/fonts/NotoSans-Regular.ttf").to_vec(),
    )
    .unwrap();
    let mut service = TextSystem::new("en-US", &[asset]).unwrap();
    let layout = service
        .layout("Привет e\u{301}", &TextStyle::new("Noto Sans", 24.0))
        .unwrap();
    let raster = service.rasterize(&layout, 1.0, Color::default()).unwrap();
    let frame = RenderFrame::default().with_ui(vec![raster.clone().into()]);
    let mut cache = ColoredFrame::new();
    assert!(cache.prepare(&frame, extent()));
    assert!(!cache.prepare(&frame.clone(), extent()));
    let independent = service.rasterize(&layout, 1.0, Color::default()).unwrap();
    assert!(cache.prepare(
        &RenderFrame::default().with_ui(vec![independent.into()]),
        extent()
    ));
    for changed in [
        raster.at([50.0; 2]).unwrap(),
        service.rasterize(&layout, 2.0, Color::default()).unwrap(),
        service
            .rasterize(&layout, 2.0, Color::rgb(1.0, 0.0, 0.0))
            .unwrap(),
    ] {
        let frame = RenderFrame::default().with_ui(vec![changed.into()]);
        assert!(cache.prepare(&frame, extent()));
        assert_fresh_geometry(&cache, &frame, extent());
    }
}
