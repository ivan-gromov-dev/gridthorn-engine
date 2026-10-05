use super::*;
use crate::text::test::{style, system};

/// Saturation uses direct sampling without exceeding retained limits.
#[test]
fn unique_glyphs_saturate_cache_and_preserve_output() {
    let mut text = system();
    let content: String = (33_u8..=126).map(char::from).collect();
    let layout = text.layout(&content, &style()).unwrap();
    let mut spans = GlyphSpans::new([1.0; 4]);
    let mut output = Vec::new();
    let mut expected = Vec::new();
    for dpi in [1.0, 1.25, 1.5, 1.75] {
        for run in layout.buffer.layout_runs() {
            for glyph in run.glyphs {
                let physical = glyph.physical((0.0, run.line_y * dpi), dpi);
                spans.append(
                    &mut output,
                    &mut text.cache,
                    &mut text.fonts,
                    physical.cache_key,
                    [physical.x, physical.y],
                );
                append_direct(
                    &mut expected,
                    &mut text.cache,
                    &mut text.fonts,
                    physical.cache_key,
                    [physical.x, physical.y],
                    [1.0; 4],
                );
            }
        }
    }
    assert_eq!(spans.entries.len(), ENTRY_LIMIT);
    assert!(spans.spans <= SPAN_LIMIT);
    assert_eq!(
        spans.spans,
        spans.entries.values().map(Vec::len).sum::<usize>()
    );
    assert_ne!(output, []);
    assert_eq!(output, expected);
    spans.finish();
    assert_eq!(spans.scratch.capacity(), 0);
}

#[test]
fn service_span_reuse_tint_change_and_clear_preserve_live_snapshots() {
    let mut text = system();
    let layout = text.layout("Привет 日本語 office", &style()).unwrap();
    let first = text
        .rasterize(&layout, 1.0, crate::Color::default())
        .unwrap();
    let retained = text.spans.as_ref().unwrap().spans;
    assert!(retained > 0);
    assert_eq!(
        text.rasterize(&layout, 1.0, crate::Color::default())
            .unwrap(),
        first
    );
    assert_eq!(text.spans.as_ref().unwrap().spans, retained);
    let tint = crate::Color::rgba(0.3, 0.4, 0.5, 0.6);
    let changed = text.rasterize(&layout, 1.0, tint).unwrap();
    assert_ne!(changed, first);
    assert_eq!(text.spans.as_ref().unwrap().tint, tint.components());
    assert_eq!(text.spans.as_ref().unwrap().scratch.capacity(), 0);
    text.clear_raster_cache();
    assert!(text.spans.is_none());
    assert_eq!(text.rasterize(&layout, 1.0, tint).unwrap(), changed);
    assert_eq!(
        text.rasterize(&layout, 1.0, crate::Color::default())
            .unwrap(),
        first
    );
    drop(text);
    assert!(first.pixel_count() > 0);
}
