use super::{style, system};
use crate::{Color, TextError};

#[test]
fn zero_width_and_bidi_format_controls_do_not_fail_rasterization() {
    let mut text = system();
    for content in [
        "a\u{200B}b",
        "a\u{200D}b",
        "a\u{2067}العربية\u{2069}b",
        "\u{200E}\u{200F}",
    ] {
        let layout = text.layout(content, &style()).expect("control layout");
        text.rasterize(&layout, 1.0, Color::default())
            .expect("control rasterization");
    }
}

#[test]
fn excessive_raster_work_returns_an_error_and_previous_snapshot_survives() {
    let mut text = system();
    let original = text.layout("Я", &style()).expect("normal layout");
    let snapshot = text
        .rasterize(&original, 1.0, Color::default())
        .expect("normal raster");
    let large = text
        .layout(&"Я".repeat(100), &crate::TextStyle::new("Noto Sans", 200.0))
        .expect("large layout");
    assert_eq!(
        text.rasterize(&large, 2.0, Color::default()),
        Err(TextError::TooLarge)
    );
    assert_eq!(
        text.rasterize(&original, 1.0, Color::default())
            .expect("last good"),
        snapshot
    );
}

#[test]
fn dpi_rerasterization_preserves_layout_and_increases_detail() {
    let mut text = system();
    let layout = text
        .layout("Привет العربية 日本語 e\u{301}", &style())
        .expect("layout");
    let one = text.rasterize(&layout, 1.0, Color::default()).expect("1x");
    let two = text.rasterize(&layout, 2.0, Color::default()).expect("2x");
    assert_eq!(one.measurement(), two.measurement());
    assert!(one.pixel_count() > 100);
    assert!(two.pixel_count() > one.pixel_count() * 2);
    assert_eq!(two.scale_factor(), 2.0);
    assert!(
        one.pixels
            .iter()
            .any(|pixel| pixel.color[3] > 0.0 && pixel.color[3] < 1.0)
    );
    text.clear_raster_cache();
    assert_eq!(
        text.rasterize(&layout, 1.0, Color::default())
            .expect("rebuild"),
        one
    );
}

#[test]
fn rejects_foreign_layout_invalid_dpi_and_position() {
    let mut text = system();
    let layout = text.layout("hello", &style()).expect("layout");
    assert_eq!(
        system().rasterize(&layout, 1.0, Color::default()),
        Err(TextError::ForeignLayout)
    );
    for dpi in [0.0, -1.0, f32::NAN, f32::INFINITY, 8.01] {
        assert_eq!(
            text.rasterize(&layout, dpi, Color::default()),
            Err(TextError::InvalidMetrics)
        );
    }
    assert_eq!(
        text.rasterize(&layout, 1.0, Color::default())
            .expect("raster")
            .at([f32::NAN, 0.0]),
        Err(TextError::InvalidPosition)
    );
}

#[test]
fn blank_text_has_measurement_and_no_ink_and_tint_alpha_is_applied() {
    let mut text = system();
    let blank = text.layout(" \n", &style()).expect("blank");
    assert_eq!(
        text.rasterize(&blank, 1.0, Color::default())
            .expect("blank raster")
            .pixel_count(),
        0
    );
    let label = text.layout("Я", &style()).expect("glyph");
    let raster = text
        .rasterize(&label, 1.0, Color::rgba(0.5, 0.0, 1.0, 0.25))
        .expect("tint");
    assert!(
        raster
            .pixels
            .iter()
            .all(|pixel| (pixel.color[0] - 0.5).abs() < f32::EPSILON
                && pixel.color[1].abs() < f32::EPSILON
                && pixel.color[3] <= 0.25)
    );
}
