use std::{io::Write, sync::Arc, thread, time::Duration};

use super::{style, system};
use crate::{Color, RasterText, TextSystem, UiRect};

/// Account for retained output, span and backend image capacity during release/clone.
#[test]
#[ignore = "manual raster-memory probe; run alone in release with external process sampling"]
fn measure_raster_storage_lifecycle() {
    assert!(
        !std::hint::black_box(cfg!(debug_assertions)),
        "run with --release"
    );
    assert_eq!(std::env::var_os("GRIDTHORN_TEXT_PERFORMANCE"), None);
    let mut service = system();
    let mut settings = style();
    settings.width = Some(600.0);
    let layout = service
        .layout(&"文章を確認する ".repeat(64), &settings)
        .unwrap();
    phase("before_raster", Some(&service), &[]);
    let full = service.rasterize(&layout, 1.0, Color::default()).unwrap();
    let weak = Arc::downgrade(&full.pixels);
    let mut snapshots = vec![full];
    phase("full", Some(&service), &snapshots);
    for _ in 0..32 {
        snapshots.push(snapshots[0].clone());
    }
    assert_eq!(Arc::strong_count(&snapshots[0].pixels), 33);
    phase("cloned_32", Some(&service), &snapshots);
    snapshots.push(
        service
            .rasterize_clipped(
                &layout,
                1.0,
                Color::default(),
                UiRect::new([0.0; 2], [600.0, 40.0], Color::default()).unwrap(),
            )
            .unwrap(),
    );
    phase("clipped", Some(&service), &snapshots);
    service.clear_raster_cache();
    assert!(service.spans.is_none());
    assert!(service.cache.image_cache.is_empty());
    assert!(snapshots[0].pixel_count() > 0);
    phase("raster_cache_cleared", Some(&service), &snapshots);
    snapshots.clear();
    assert!(weak.upgrade().is_none());
    phase("snapshots_dropped", Some(&service), &snapshots);
    drop(layout);
    drop(service);
    phase("service_dropped", None, &[]);
}

fn phase(name: &str, service: Option<&TextSystem>, snapshots: &[RasterText]) {
    let mut output_bytes = 0;
    for (index, snapshot) in snapshots.iter().enumerate() {
        if !snapshots[..index]
            .iter()
            .any(|previous| Arc::ptr_eq(&previous.pixels, &snapshot.pixels))
        {
            output_bytes +=
                snapshot.pixels.capacity() * std::mem::size_of::<super::super::raster::TextPixel>();
        }
    }
    let spans = service.and_then(|service| service.spans.as_ref()).map_or(
        0,
        super::super::raster_spans::GlyphSpans::diagnostic_storage_bytes,
    );
    let images = service.map_or(0, |service| {
        service
            .cache
            .image_cache
            .values()
            .flatten()
            .map(|image| image.data.capacity())
            .sum::<usize>()
    });
    println!(
        "raster_memory_phase,{name},pid={},output_capacity_bytes={output_bytes},span_capacity_bytes={spans},image_data_capacity_bytes={images}",
        std::process::id()
    );
    std::io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
}
