use std::collections::HashMap;

use cosmic_text::{CacheKey, FontSystem, SwashCache};

use super::raster::TextPixel;

const ENTRY_LIMIT: usize = 128;
const SPAN_LIMIT: usize = 65_536;

/// Tint-specific glyph spans reused only during one raster request.
#[derive(Default)]
pub(super) struct GlyphSpans {
    entries: HashMap<CacheKey, Vec<GlyphSpan>>,
    spans: usize,
    scratch: Vec<GlyphSpan>,
    tint: [f32; 4],
    reuse: bool,
}

impl GlyphSpans {
    #[cfg(test)]
    pub(super) fn diagnostic_storage_bytes(&self) -> usize {
        self.scratch.capacity() * std::mem::size_of::<GlyphSpan>()
            + self
                .entries
                .values()
                .map(|spans| spans.capacity() * std::mem::size_of::<GlyphSpan>())
                .sum::<usize>()
    }

    pub(super) fn new(tint: [f32; 4], reuse: bool) -> Self {
        Self {
            tint,
            reuse,
            ..Self::default()
        }
    }
    pub(super) fn append(
        &mut self,
        output: &mut Vec<TextPixel>,
        cache: &mut SwashCache,
        fonts: &mut FontSystem,
        key: CacheKey,
        position: [i32; 2],
    ) {
        if !self.reuse {
            append_direct(output, cache, fonts, key, position, self.tint);
            return;
        }
        if let Some(pixels) = self.entries.get(&key) {
            append_translated(output, pixels, position);
            return;
        }
        self.scratch.clear();
        let tint = self.tint;
        cache.with_pixels(
            fonts,
            key,
            cosmic_text::Color::rgb(255, 255, 255),
            |x, y, sample| {
                let [red, green, blue, alpha] = sample.as_rgba();
                if alpha != 0 {
                    push_glyph_span(
                        &mut self.scratch,
                        GlyphSpan {
                            position: [x, y],
                            color: [
                                f32::from(red) / 255.0 * tint[0],
                                f32::from(green) / 255.0 * tint[1],
                                f32::from(blue) / 255.0 * tint[2],
                                f32::from(alpha) / 255.0 * tint[3],
                            ],
                            width: 1,
                        },
                    );
                }
            },
        );
        append_translated(output, &self.scratch, position);
        if self.entries.len() < ENTRY_LIMIT && self.spans + self.scratch.len() <= SPAN_LIMIT {
            self.spans += self.scratch.len();
            self.entries.insert(key, std::mem::take(&mut self.scratch));
        }
    }
}

/// Small requests avoid allocating a glyph-span working set.
#[expect(
    clippy::cast_precision_loss,
    reason = "bounded raster pixel coordinates become GPU f32 positions"
)]
fn append_direct(
    output: &mut Vec<TextPixel>,
    cache: &mut SwashCache,
    fonts: &mut FontSystem,
    key: CacheKey,
    position: [i32; 2],
    tint: [f32; 4],
) {
    cache.with_pixels(
        fonts,
        key,
        cosmic_text::Color::rgb(255, 255, 255),
        |x, y, sample| {
            let [red, green, blue, alpha] = sample.as_rgba();
            if alpha != 0 {
                push_span(
                    output,
                    TextPixel {
                        position: [(position[0] + x) as f32, (position[1] + y) as f32],
                        color: [
                            f32::from(red) / 255.0 * tint[0],
                            f32::from(green) / 255.0 * tint[1],
                            f32::from(blue) / 255.0 * tint[2],
                            f32::from(alpha) / 255.0 * tint[3],
                        ],
                        width: 1,
                    },
                );
            }
        },
    );
}

#[expect(
    clippy::cast_precision_loss,
    reason = "bounded raster pixel coordinates become GPU f32 positions"
)]
fn append_translated(output: &mut Vec<TextPixel>, pixels: &[GlyphSpan], position: [i32; 2]) {
    for pixel in pixels {
        let translated = TextPixel {
            position: [
                (position[0] + pixel.position[0]) as f32,
                (position[1] + pixel.position[1]) as f32,
            ],
            color: pixel.color,
            width: pixel.width,
        };
        push_span(output, translated);
    }
}

#[expect(
    clippy::float_cmp,
    reason = "merge only exactly identical raster colors and adjacent integer positions"
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "bounded glyph-image spans use exact f32 integer widths"
)]
fn push_span(output: &mut Vec<TextPixel>, pixel: TextPixel) {
    if let Some(previous) = output.last_mut().filter(|previous| {
        previous.position[1] == pixel.position[1]
            && previous.position[0] + previous.width as f32 == pixel.position[0]
            && previous.color == pixel.color
    }) {
        previous.width += pixel.width;
    } else {
        output.push(pixel);
    }
}

struct GlyphSpan {
    position: [i32; 2],
    color: [f32; 4],
    width: u32,
}

#[expect(
    clippy::float_cmp,
    reason = "merge only exactly identical raster colors"
)]
fn push_glyph_span(output: &mut Vec<GlyphSpan>, pixel: GlyphSpan) {
    if let Some(previous) = output.last_mut().filter(|previous| {
        previous.position[1] == pixel.position[1]
            && i64::from(previous.position[0]) + i64::from(previous.width)
                == i64::from(pixel.position[0])
            && previous.color == pixel.color
    }) {
        previous.width += pixel.width;
    } else {
        output.push(pixel);
    }
}

#[cfg(test)]
#[path = "test/raster_spans.rs"]
mod test;
