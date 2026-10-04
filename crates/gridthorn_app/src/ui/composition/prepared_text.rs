use std::collections::BTreeMap;

use gridthorn_render::{TextLayout, TextStyle, TextSystem};

use super::UiCompositionError;

const ENTRY_LIMIT: usize = 1024;
const KEY_BYTES_LIMIT: usize = 256 * 1024;
const GLYPH_LIMIT: usize = 16 * 1024;
const LINE_LIMIT: usize = 4096;

/// One immutable theme and font service across arrangement, geometry and paint.
/// Retention ends before returning the detached UI snapshot.
pub(super) struct PreparedText<'a> {
    pub service: Option<&'a mut TextSystem>,
    layouts: BTreeMap<String, BTreeMap<Option<u32>, (TextStyle, TextLayout)>>,
    retained: [usize; 4],
}

impl<'a> PreparedText<'a> {
    pub(super) fn new(service: Option<&'a mut TextSystem>) -> Self {
        Self {
            service,
            layouts: BTreeMap::new(),
            retained: [0; 4],
        }
    }

    pub(super) fn layout(
        &mut self,
        value: &str,
        style: &TextStyle,
    ) -> Result<TextLayout, UiCompositionError> {
        let width = style.width.map(f32::to_bits);
        if let Some((cached_style, layout)) = self
            .layouts
            .get(value)
            .and_then(|widths| widths.get(&width))
            && cached_style == style
        {
            return Ok(layout.clone());
        }
        let layout = self
            .service
            .as_deref_mut()
            .ok_or(UiCompositionError::InvalidMetrics(
                "asset font theme requires TextSystem",
            ))?
            .layout(value, style)?;
        let added = [
            1,
            if self.layouts.contains_key(value) {
                0
            } else {
                value.len()
            } + style.family.len(),
            layout.lines().iter().map(|line| line.glyphs.len()).sum(),
            layout.lines().len(),
        ];
        if self.retain(added) {
            self.layouts
                .entry(value.to_owned())
                .or_default()
                .insert(width, (style.clone(), layout.clone()));
        }
        Ok(layout)
    }

    fn retain(&mut self, added: [usize; 4]) -> bool {
        let limits = [ENTRY_LIMIT, KEY_BYTES_LIMIT, GLYPH_LIMIT, LINE_LIMIT];
        if !(0..4).all(|index| added[index] <= limits[index] - self.retained[index]) {
            return false;
        }
        for (retained, added) in self.retained.iter_mut().zip(added) {
            *retained += added;
        }
        true
    }
}

#[cfg(test)]
#[path = "test/prepared_text.rs"]
mod test;
