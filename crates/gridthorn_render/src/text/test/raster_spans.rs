use super::*;
use crate::text::test::{style, system};

/// Cache saturation falls back to scratch spans without exceeding retained limits.
#[test]
fn unique_glyphs_saturate_request_local_cache_and_preserve_output() {
    let mut text = system();
    let content: String = (33_u8..=126).map(char::from).collect();
    let layout = text.layout(&content, &style()).unwrap();
    let mut spans = GlyphSpans::new([1.0; 4], true);
    let mut output = Vec::new();
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
    assert_ne!(spans.scratch.len(), 0);
}
