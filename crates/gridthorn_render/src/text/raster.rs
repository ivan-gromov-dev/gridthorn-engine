use super::{TextError, TextLayout, TextMeasurement, TextSystem};
use crate::presentation::Color;
use std::sync::Arc;

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
    #[expect(
        clippy::cast_precision_loss,
        reason = "bounded raster pixel coordinates become GPU f32 positions"
    )]
    #[expect(
        clippy::float_cmp,
        reason = "merge only exactly identical raster colors and integer pixel positions"
    )]
    pub fn rasterize(
        &mut self,
        layout: &TextLayout,
        scale: f32,
        color: Color,
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
        let tint = color.components();
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
                self.cache.with_pixels(
                    &mut self.fonts,
                    physical.cache_key,
                    cosmic_text::Color::rgb(255, 255, 255),
                    |x, y, sample| {
                        let [red, green, blue, alpha] = sample.as_rgba();
                        if alpha != 0 {
                            let pixel = TextPixel {
                                position: [(physical.x + x) as f32, (physical.y + y) as f32],
                                color: [
                                    f32::from(red) / 255.0 * tint[0],
                                    f32::from(green) / 255.0 * tint[1],
                                    f32::from(blue) / 255.0 * tint[2],
                                    f32::from(alpha) / 255.0 * tint[3],
                                ],
                                width: 1,
                            };
                            if let Some(previous) = pixels.last_mut().filter(|previous| {
                                previous.position[1] == pixel.position[1]
                                    && previous.position[0] + previous.width as f32
                                        == pixel.position[0]
                                    && previous.color == pixel.color
                            }) {
                                previous.width += 1;
                            } else {
                                pixels.push(pixel);
                            }
                        }
                    },
                );
            }
        }
        let units = pixels.len();
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
