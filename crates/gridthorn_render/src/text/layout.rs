use std::{ops::Range, sync::Arc};

/// Logical advance bounds; ink can extend outside these bounds.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TextMeasurement {
    /// Maximum line advance in logical pixels.
    pub width: f32,
    /// Sum of laid-out line heights in logical pixels, including empty lines.
    pub height: f32,
}

/// Engine-owned glyph diagnostic, with a UTF-8 cluster range within its paragraph.
#[derive(Clone, Debug, PartialEq)]
pub struct TextGlyph {
    /// Resolved family, including script fallback.
    pub family: String,
    /// Font-local glyph index; zero identifies missing coverage.
    pub glyph_id: u16,
    /// Cluster byte range in the original paragraph (not the complete input).
    pub cluster: Range<usize>,
    /// Glyph origin in logical pixels.
    pub position: [f32; 2],
    /// Advance width in logical pixels.
    pub advance: f32,
    /// Unicode embedding direction of this glyph.
    pub right_to_left: bool,
}

/// One visual line, retaining its source paragraph and directional information.
#[derive(Clone, Debug, PartialEq)]
pub struct TextLine {
    /// Original paragraph index before wrapping.
    pub paragraph: usize,
    /// Paragraph's Unicode base direction.
    pub right_to_left: bool,
    /// Logical advance width.
    pub width: f32,
    /// Top of the line in logical pixels.
    pub top: f32,
    /// Baseline in logical pixels.
    pub baseline: f32,
    /// Glyphs in draw order, with visual coordinates after bidi reordering.
    pub glyphs: Vec<TextGlyph>,
}

/// Immutable shaped layout. Cloneable and thread-transferable; tied to its text service.
#[derive(Clone, Debug)]
pub struct TextLayout {
    pub(super) owner: Arc<()>,
    pub(super) buffer: cosmic_text::Buffer,
    pub(super) lines: Vec<TextLine>,
    pub(super) measurement: TextMeasurement,
}

impl TextLayout {
    /// Logical advance measurement, unaffected by raster DPI.
    #[must_use]
    pub const fn measurement(&self) -> TextMeasurement {
        self.measurement
    }

    /// Visual lines, including fallback and cluster diagnostics.
    #[must_use]
    pub fn lines(&self) -> &[TextLine] {
        &self.lines
    }

    /// Count missing shaped glyphs. Missing coverage renders the font's .notdef glyph.
    #[must_use]
    pub fn missing_glyphs(&self) -> usize {
        self.lines
            .iter()
            .flat_map(|line| &line.glyphs)
            .filter(|glyph| glyph.glyph_id == 0)
            .count()
    }
}
