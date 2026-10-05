use super::{style, system};
use crate::{Color, TextLayout, TextSystem};

/// Expanded spans must preserve the backend's ordered samples at fractional DPI.
#[test]
fn reused_glyph_spans_preserve_order_coverage_tint_and_fractional_origins() {
    let ascii: String = (32_u8..=126).map(char::from).collect();
    let content = format!("{ascii}\nПривет مرحبًا 日本語 e\u{301} office\n{ascii}\n{ascii}");
    let mut text = system();
    let mut settings = style();
    settings.width = Some(600.0);
    let layout = text.layout(&content, &settings).unwrap();
    assert!(
        layout
            .lines()
            .iter()
            .map(|line| line.glyphs.len())
            .sum::<usize>()
            >= 256
    );
    for dpi in [1.0, 1.25, 2.0] {
        for color in [
            Color::default(),
            Color::rgba(0.5, 0.25, 0.75, 0.3),
            Color::rgba(0.0, 0.0, 0.0, 0.0),
        ] {
            let expected = backend_samples(&mut text, &layout, dpi, color);
            let raster = text.rasterize(&layout, dpi, color).unwrap();
            let actual: Vec<_> = raster
                .pixels
                .iter()
                .flat_map(|span| {
                    (0..span.width).map(move |offset| {
                        (
                            [
                                span.position[0] + f32::from(u16::try_from(offset).unwrap()),
                                span.position[1],
                            ],
                            span.color,
                        )
                    })
                })
                .collect();
            assert_eq!(actual, expected);
            text.clear_raster_cache();
            assert_eq!(text.rasterize(&layout, dpi, color).unwrap(), raster);
        }
    }
}

#[expect(
    clippy::cast_precision_loss,
    reason = "bounded raster coordinates match the backend sample reference"
)]
fn backend_samples(
    text: &mut TextSystem,
    layout: &TextLayout,
    dpi: f32,
    color: Color,
) -> Vec<([f32; 2], [f32; 4])> {
    let mut expected = Vec::new();
    let tint = color.components();
    for run in layout.buffer.layout_runs() {
        for glyph in run.glyphs {
            let physical = glyph.physical((0.0, run.line_y * dpi), dpi);
            text.cache.with_pixels(
                &mut text.fonts,
                physical.cache_key,
                cosmic_text::Color::rgb(255, 255, 255),
                |x, y, sample| {
                    let [r, g, b, a] = sample.as_rgba();
                    if a != 0 {
                        expected.push((
                            [(physical.x + x) as f32, (physical.y + y) as f32],
                            [
                                f32::from(r) / 255.0 * tint[0],
                                f32::from(g) / 255.0 * tint[1],
                                f32::from(b) / 255.0 * tint[2],
                                f32::from(a) / 255.0 * tint[3],
                            ],
                        ));
                    }
                },
            );
        }
    }
    expected
}
