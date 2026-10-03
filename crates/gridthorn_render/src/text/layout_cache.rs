use std::collections::VecDeque;

use super::{TextLayout, TextStyle};

const ENTRY_LIMIT: usize = 64;
const KEY_BYTES_LIMIT: usize = 256 * 1024;
const GLYPH_LIMIT: usize = 16 * 1024;
const LINE_LIMIT: usize = 1024;

struct Entry {
    text: String,
    style: TextStyle,
    layout: TextLayout,
    key_bytes: usize,
    glyphs: usize,
    lines: usize,
}

/// Service-local LRU; limits constrain retained keys/geometry, not opaque backend bytes.
#[derive(Default)]
pub(super) struct LayoutCache {
    entries: VecDeque<Entry>,
    key_bytes: usize,
    glyphs: usize,
    lines: usize,
}

impl LayoutCache {
    pub(super) fn get(&mut self, text: &str, style: &TextStyle) -> Option<TextLayout> {
        let index = self
            .entries
            .iter()
            .position(|entry| entry.text == text && entry.style == *style)?;
        let entry = self.entries.remove(index)?;
        let layout = entry.layout.clone();
        self.entries.push_front(entry);
        Some(layout)
    }

    pub(super) fn insert(&mut self, text: &str, style: &TextStyle, layout: &TextLayout) {
        let key_bytes = text.len().saturating_add(style.family.len());
        let glyphs = layout
            .lines()
            .iter()
            .map(|line| line.glyphs.len())
            .sum::<usize>();
        let lines = layout.lines().len();
        if key_bytes > KEY_BYTES_LIMIT || glyphs > GLYPH_LIMIT || lines > LINE_LIMIT {
            return;
        }
        while self.entries.len() >= ENTRY_LIMIT
            || self.key_bytes + key_bytes > KEY_BYTES_LIMIT
            || self.glyphs + glyphs > GLYPH_LIMIT
            || self.lines + lines > LINE_LIMIT
        {
            let Some(entry) = self.entries.pop_back() else {
                break;
            };
            self.key_bytes -= entry.key_bytes;
            self.glyphs -= entry.glyphs;
            self.lines -= entry.lines;
        }
        self.entries.push_front(Entry {
            text: text.to_owned(),
            style: style.clone(),
            layout: layout.clone(),
            key_bytes,
            glyphs,
            lines,
        });
        self.key_bytes += key_bytes;
        self.glyphs += glyphs;
        self.lines += lines;
    }
}

#[cfg(test)]
#[path = "test/layout_cache.rs"]
mod test;
