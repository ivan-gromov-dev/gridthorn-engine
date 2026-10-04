use super::{TextError, TextLayout, TextMeasurement, TextSystem};
use crate::presentation::{Color, UiRect};
use std::sync::Arc;

/// Amortize request-local span preparation only across sufficiently large layouts.
const MIN_GLYPHS_FOR_SPAN_REUSE: usize = 256;

/// Keep boundary ink when callers convert bounded logical origins and clips separately.
const CLIP_ROUNDING_MARGIN: f64 = 2.0;

/// DPI-specific immutable text draw data, independent of the font service and GPU.
#[derive(Clone, Debug, PartialEq)]
pub struct RasterText {
    pub(crate) pixels: Arc<[TextPixel]>,
    position: [f32; 2],
    scale: f32,
    measurement: TextMeasurement,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TextPixel {
    pub(crate) position: [f32; 2],
    pub(crate) color: [f32; 4],
    pub(crate) width: u32,
}

impl RasterText {
    #[expect(
        clippy::float_cmp,
        reason = "cached draw data requires exactly matching placement and DPI"
    )]
    pub(crate) fn same_draw_data(&self, other: &Self) -> bool {
        self.position == other.position
            && self.scale == other.scale
            && self.measurement == other.measurement
            && Arc::ptr_eq(&self.pixels, &other.pixels)
    }

    /// Place the snapshot at a logical-pixel top-left origin. Ink overhang is retained.
    ///
    /// # Errors
    /// Rejects non-finite or out-of-budget coordinates.
    pub fn at(mut self, position: [f32; 2]) -> Result<Self, TextError> {
        if !position
            .into_iter()
            .all(|value| value.is_finite() && value.abs() <= 1_000_000.0)
        {
            return Err(TextError::InvalidPosition);
        }
        self.position = position;
        Ok(self)
    }

    /// Logical top-left origin.
    #[must_use]
    pub const fn position(&self) -> [f32; 2] {
        self.position
    }

    /// Physical pixels per logical pixel used for rasterization.
    #[must_use]
    pub const fn scale_factor(&self) -> f32 {
        self.scale
    }

    /// Logical advance bounds of the source layout.
    #[must_use]
    pub const fn measurement(&self) -> TextMeasurement {
        self.measurement
    }

    /// Number of nontransparent raster samples; useful for diagnostics.
    #[must_use]
    pub fn pixel_count(&self) -> usize {
        self.pixels.iter().map(|pixel| pixel.width as usize).sum()
    }
}

impl TextSystem {
    /// Rasterize at the current window DPI, keeping logical layout unchanged.
    ///
    /// Re-rasterize when DPI changes. The output can be submitted as a UI primitive.
    ///
    /// # Errors
    /// Rejects foreign layouts, invalid DPI, excessive output or failed glyph rasterization.
    pub fn rasterize(
        &mut self,
        layout: &TextLayout,
        scale: f32,
        color: Color,
    ) -> Result<RasterText, TextError> {
        self.rasterize_region(layout, scale, color, None)
    }

    /// Omit glyph ink wholly outside a physical-pixel clip relative to layout origin.
    ///
    /// The rectangle color is ignored. Partially intersecting glyphs remain intact;
    /// submit with the same clip for exact edge clipping. Measurement is unchanged.
    /// All glyph images still count toward the normal raster work safety limit.
    ///
    /// # Errors
    /// Returns the same layout, DPI, rasterization and work-limit errors as `rasterize`.
    pub fn rasterize_clipped(
        &mut self,
        layout: &TextLayout,
        scale: f32,
        color: Color,
        clip: UiRect,
    ) -> Result<RasterText, TextError> {
        self.rasterize_region(layout, scale, color, Some(clip))
    }

    fn rasterize_region(
        &mut self,
        layout: &TextLayout,
        scale: f32,
        color: Color,
        clip: Option<UiRect>,
    ) -> Result<RasterText, TextError> {
        let start = self
            .performance
            .as_ref()
            .and_then(|performance| performance.start(1));
        if !scale.is_finite() || scale <= 0.0 || scale > 8.0 {
            return Err(TextError::InvalidMetrics);
        }
        if !Arc::ptr_eq(&layout.owner, &self.owner) {
            return Err(TextError::ForeignLayout);
        }
        let mut pixels: Vec<TextPixel> = Vec::new();
        let mut samples = 0_usize;

        let glyphs: usize = layout.lines().iter().map(|line| line.glyphs.len()).sum();
        let mut spans = super::raster_spans::GlyphSpans::new(
            color.components(),
            glyphs >= MIN_GLYPHS_FOR_SPAN_REUSE,
        );
        for run in layout.buffer.layout_runs() {
            for glyph in run.glyphs {
                let physical = glyph.physical((0.0, run.line_y * scale), scale);
                let Some(image) = self.cache.get_image(&mut self.fonts, physical.cache_key) else {
                    if run.text[glyph.start..glyph.end]
                        .chars()
                        .all(char::is_whitespace)
                    {
                        continue;
                    }
                    return Err(TextError::Rasterization {
                        family: self
                            .fonts
                            .db()
                            .face(glyph.font_id)
                            .map_or_else(String::new, |face| face.families[0].0.clone()),
                        glyph: glyph.glyph_id,
                    });
                };
                samples = samples.saturating_add(
                    image.placement.width as usize * image.placement.height as usize,
                );
                if samples > 1_000_000 {
                    return Err(TextError::TooLarge);
                }
                if let Some(clip) = clip {
                    let origin = [
                        f64::from(physical.x) + f64::from(image.placement.left),
                        f64::from(physical.y) - f64::from(image.placement.top),
                    ];
                    let extent = [image.placement.width, image.placement.height];
                    if (0..2).any(|axis| {
                        let minimum = f64::from(clip.position()[axis]) - CLIP_ROUNDING_MARGIN;
                        let maximum = f64::from(clip.position()[axis])
                            + f64::from(clip.size()[axis])
                            + CLIP_ROUNDING_MARGIN;
                        origin[axis] >= maximum || origin[axis] + f64::from(extent[axis]) <= minimum
                    }) {
                        continue;
                    }
                }
                spans.append(
                    &mut pixels,
                    &mut self.cache,
                    &mut self.fonts,
                    physical.cache_key,
                    [physical.x, physical.y],
                );
            }
        }
        let units = pixels.len();
        #[cfg(test)]
        if std::env::var_os("GRIDTHORN_RASTER_STORAGE_PROBE").is_some() {
            let output_capacity_bytes = pixels.capacity() * std::mem::size_of::<TextPixel>();
            let snapshot_bytes = std::mem::size_of_val(pixels.as_slice());
            let glyph_storage_bytes = spans.diagnostic_storage_bytes();
            println!(
                "raster_storage,clipped={},output_capacity_bytes={output_capacity_bytes},snapshot_bytes={snapshot_bytes},glyph_storage_bytes={glyph_storage_bytes}",
                clip.is_some()
            );
        }
        drop(spans);
        let raster = RasterText {
            pixels: pixels.into(),
            position: [0.0, 0.0],
            scale,
            measurement: layout.measurement,
        };
        if let Some(performance) = &mut self.performance {
            performance.record(1, start, units);
        }
        Ok(raster)
    }
}
