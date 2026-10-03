use cosmic_text::{Attrs, Buffer, Family, FontSystem, Metrics, Shaping, SwashCache, fontdb};
use gridthorn_assets::FontAsset;
use std::sync::Arc;

use super::{
    TextAlignment, TextError, TextGlyph, TextLayout, TextLine, TextMeasurement, TextStyle, TextWrap,
};

/// Presentation-only font database and shaping/raster cache. No system fonts are loaded.
///
/// Supply all fallback assets at construction; create a new service after a font reload.
/// Mutable access serializes cache work; immutable layouts/snapshots can cross threads.
pub struct TextSystem {
    pub(super) fonts: FontSystem,
    pub(super) cache: SwashCache,
    pub(super) owner: Arc<()>,
    families: Vec<String>,
}

impl TextSystem {
    /// Construct an isolated font database with an explicit shaping locale.
    ///
    /// # Errors
    /// Returns [`TextError::NoFonts`] when no assets are supplied.
    pub fn new(locale: impl Into<String>, assets: &[FontAsset]) -> Result<Self, TextError> {
        if assets.is_empty() {
            return Err(TextError::NoFonts);
        }
        let mut database = fontdb::Database::new();
        let mut families = Vec::new();
        for asset in assets {
            database.load_font_data(asset.bytes().to_vec());
            families.extend_from_slice(asset.families());
        }
        families.sort();
        families.dedup();
        database.set_sans_serif_family(&families[0]);
        Ok(Self {
            fonts: FontSystem::new_with_locale_and_db(locale.into(), database),
            cache: SwashCache::new(),
            owner: Arc::new(()),
            families,
        })
    }

    /// Shape UTF-8 text with contextual substitution, combining marks and Unicode bidi.
    ///
    /// # Errors
    /// Rejects unknown primary families, invalid metrics and oversized input/layout.
    pub fn layout(&mut self, text: &str, style: &TextStyle) -> Result<TextLayout, TextError> {
        style.validate()?;
        if text.len() > 65536 {
            return Err(TextError::TooLarge);
        }
        if !self.families.contains(&style.family) {
            return Err(TextError::UnknownFamily(style.family.clone()));
        }
        let mut buffer = Buffer::new(
            &mut self.fonts,
            Metrics::new(style.font_size, style.line_height),
        );
        buffer.set_size(style.width, None);
        buffer.set_wrap(match style.wrap {
            TextWrap::None => cosmic_text::Wrap::None,
            TextWrap::Word => cosmic_text::Wrap::Word,
            TextWrap::Glyph => cosmic_text::Wrap::Glyph,
            TextWrap::WordOrGlyph => cosmic_text::Wrap::WordOrGlyph,
        });
        let alignment = match style.alignment {
            TextAlignment::Start => None,
            TextAlignment::Left => Some(cosmic_text::Align::Left),
            TextAlignment::Right => Some(cosmic_text::Align::Right),
            TextAlignment::Center => Some(cosmic_text::Align::Center),
        };
        buffer.set_text(
            text,
            &Attrs::new().family(Family::Name(&style.family)),
            Shaping::Advanced,
            alignment,
        );
        buffer.shape_until_scroll(&mut self.fonts, false);
        let mut lines = Vec::new();
        let mut measurement = TextMeasurement::default();
        for run in buffer.layout_runs() {
            measurement.width = measurement.width.max(run.line_w);
            measurement.height = measurement.height.max(run.line_top + run.line_height);
            if measurement.width > 65536.0 || measurement.height > 65536.0 {
                return Err(TextError::TooLarge);
            }
            let glyphs = run
                .glyphs
                .iter()
                .map(|glyph| TextGlyph {
                    family: self
                        .fonts
                        .db()
                        .face(glyph.font_id)
                        .map_or_else(String::new, |face| face.families[0].0.clone()),
                    glyph_id: glyph.glyph_id,
                    cluster: glyph.start..glyph.end,
                    position: [
                        glyph.x + glyph.x_offset * glyph.font_size,
                        run.line_y + glyph.y - glyph.y_offset * glyph.font_size,
                    ],
                    advance: glyph.w,
                    right_to_left: glyph.level.is_rtl(),
                })
                .collect();
            lines.push(TextLine {
                paragraph: run.line_i,
                right_to_left: run.rtl,
                width: run.line_w,
                top: run.line_top,
                baseline: run.line_y,
                glyphs,
            });
        }
        Ok(TextLayout {
            owner: self.owner.clone(),
            buffer,
            lines,
            measurement,
        })
    }

    /// Drop raster cache allocations. Existing immutable snapshots remain usable.
    pub fn clear_raster_cache(&mut self) {
        self.cache = SwashCache::new();
    }
}
