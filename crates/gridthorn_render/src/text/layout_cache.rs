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

#[derive(Default)]
struct Diagnostics {
    hits: usize,
    misses: usize,
    evictions: usize,
    bypassed: usize,
    peak: [usize; 4],
}

/// Service-local LRU; limits constrain retained keys/geometry, not opaque backend bytes.
#[derive(Default)]
pub(super) struct LayoutCache {
    diagnostics: Option<Diagnostics>,
    entries: VecDeque<Entry>,
    key_bytes: usize,
    glyphs: usize,
    lines: usize,
}

impl LayoutCache {
    pub(super) fn new(diagnostics: bool) -> Self {
        Self {
            diagnostics: diagnostics.then(Diagnostics::default),
            entries: VecDeque::new(),
            key_bytes: 0,
            glyphs: 0,
            lines: 0,
        }
    }

    pub(super) fn get(&mut self, text: &str, style: &TextStyle) -> Option<TextLayout> {
        let index = self
            .entries
            .iter()
            .position(|entry| entry.text == text && entry.style == *style);
        if let Some(diagnostics) = &mut self.diagnostics {
            if index.is_some() {
                diagnostics.hits += 1;
            } else {
                diagnostics.misses += 1;
            }
        }
        let index = index?;
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
            if let Some(diagnostics) = &mut self.diagnostics {
                diagnostics.bypassed += 1;
            }
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
            if let Some(diagnostics) = &mut self.diagnostics {
                diagnostics.evictions += 1;
            }
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
        if let Some(diagnostics) = &mut self.diagnostics {
            for (peak, value) in diagnostics.peak.iter_mut().zip([
                self.entries.len(),
                self.key_bytes,
                self.glyphs,
                self.lines,
            ]) {
                *peak = (*peak).max(value);
            }
        }
    }
}

impl Drop for LayoutCache {
    fn drop(&mut self) {
        if let Some(diagnostics) = &self.diagnostics {
            let [entries, keys, glyphs, lines] = diagnostics.peak;
            eprintln!(
                "text_layout_cache,hits={},misses={},evictions={},bypassed={},entries={},key_bytes={},glyphs={},lines={},peak_entries={entries},peak_key_bytes={keys},peak_glyphs={glyphs},peak_lines={lines}",
                diagnostics.hits,
                diagnostics.misses,
                diagnostics.evictions,
                diagnostics.bypassed,
                self.entries.len(),
                self.key_bytes,
                self.glyphs,
                self.lines
            );
        }
    }
}

#[cfg(test)]
#[path = "test/layout_cache.rs"]
mod test;
