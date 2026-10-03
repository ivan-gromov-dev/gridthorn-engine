use std::collections::BTreeMap;

use super::{UiCompositionError, UiTheme};
use gridthorn_render::TextSystem;

const LIMIT: usize = 1024;
const TEXT_BYTES_LIMIT: usize = 1024 * 1024;

/// Per-arrangement measurements share one immutable theme and font service.
pub(super) struct TextMeasurements<'a, 'b> {
    pub theme: &'a UiTheme,
    text: &'a mut Option<&'b mut TextSystem>,
    cache: MeasurementCache,
}

#[derive(Default)]
struct MeasurementCache {
    sizes: BTreeMap<String, BTreeMap<u32, [f32; 2]>>,
    entries: usize,
    text_bytes: usize,
}

impl<'a, 'b> TextMeasurements<'a, 'b> {
    pub(super) fn new(theme: &'a UiTheme, text: &'a mut Option<&'b mut TextSystem>) -> Self {
        Self {
            theme,
            text,
            cache: MeasurementCache::default(),
        }
    }

    pub(super) fn size(&mut self, value: &str, width: f32) -> Result<[f32; 2], UiCompositionError> {
        if self.theme.text.is_none() {
            return super::layout::text_size(value, width, self.theme, self.text);
        }
        let width = width.max(1.0);
        self.cache.measure(value, width, || {
            super::layout::text_size(value, width, self.theme, self.text)
        })
    }
}

impl MeasurementCache {
    fn measure(
        &mut self,
        value: &str,
        width: f32,
        measure: impl FnOnce() -> Result<[f32; 2], UiCompositionError>,
    ) -> Result<[f32; 2], UiCompositionError> {
        let key = width.to_bits();
        if let Some(size) = self.sizes.get(value).and_then(|widths| widths.get(&key)) {
            return Ok(*size);
        }
        let size = measure()?;
        let added_bytes = if self.sizes.contains_key(value) {
            0
        } else {
            value.len()
        };
        if self.entries < LIMIT && added_bytes <= TEXT_BYTES_LIMIT - self.text_bytes {
            self.sizes
                .entry(value.to_owned())
                .or_default()
                .insert(key, size);
            self.entries += 1;
            self.text_bytes += added_bytes;
        }
        Ok(size)
    }
}

#[cfg(test)]
#[path = "test/text_measurement.rs"]
mod test;
