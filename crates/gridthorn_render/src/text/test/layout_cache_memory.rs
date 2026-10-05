use std::{io::Write, sync::Arc, thread, time::Duration};

use super::*;
use crate::text::test::system;

/// Phase holds allow an external process sampler to observe resident/private bytes.
#[test]
#[ignore = "manual process-memory probe; run alone in release with external sampling"]
fn measure_layout_cache_memory() {
    assert!(
        !std::hint::black_box(cfg!(debug_assertions)),
        "run with --release"
    );
    assert_eq!(std::env::var_os("GRIDTHORN_TEXT_PERFORMANCE"), None);
    let mut text = system();
    let inputs = [
        "文章を確認する ".repeat(256),
        format!("{} 0", "文章を確認する ".repeat(256)),
        format!("{} 1", "文章を確認する ".repeat(256)),
    ];
    let mut settings = crate::TextStyle::new("Noto Sans", 20.0);
    for width in [600.0, 1.0, 12.0] {
        settings.width = Some(width);
        for input in &inputs {
            text.layout(input, &settings).unwrap();
        }
    }
    text.layouts = LayoutCache::default();
    phase("warm_empty", Some(&text.layouts));
    settings.width = Some(600.0);
    for input in &inputs {
        text.layout(input, &settings).unwrap();
    }
    phase("normal_fields", Some(&text.layouts));
    let buffers: Vec<_> = [1.0, 12.0]
        .into_iter()
        .map(|width| {
            settings.width = Some(width);
            let layout = text.layout(&inputs[0], &settings).unwrap();
            Arc::downgrade(&layout.buffer)
        })
        .collect();
    assert!(buffers.iter().all(|buffer| buffer.upgrade().is_some()));
    phase("narrow_fields", Some(&text.layouts));
    for index in 0..20 {
        text.layout(&format!("{}{index}", "\n".repeat(255)), &settings)
            .unwrap();
    }
    assert!(buffers.iter().all(|buffer| buffer.upgrade().is_none()));
    phase("line_pressure", Some(&text.layouts));
    text.layouts = LayoutCache::default();
    phase("cache_cleared", Some(&text.layouts));
    drop(text);
    phase("service_dropped", None);
}

fn phase(name: &str, cache: Option<&LayoutCache>) {
    let diagnostic_bytes = cache.map_or(0, diagnostic_capacity);
    let counts = cache.map_or([0; 4], |cache| {
        assert!(cache.entries.len() <= ENTRY_LIMIT);
        assert!(cache.key_bytes <= KEY_BYTES_LIMIT);
        assert!(cache.glyphs <= GLYPH_LIMIT);
        assert!(cache.lines <= LINE_LIMIT);
        [
            cache.entries.len(),
            cache.key_bytes,
            cache.glyphs,
            cache.lines,
        ]
    });
    let [entries, keys, glyphs, lines] = counts;
    println!(
        "cache_memory_phase,{name},pid={},entries={entries},keys={keys},glyphs={glyphs},lines={lines},diagnostic_capacity_bytes={diagnostic_bytes}",
        std::process::id()
    );
    std::io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(1));
}

/// Retained engine-owned capacity, excluding backend buffers, Arc headers and allocator overhead.
fn diagnostic_capacity(cache: &LayoutCache) -> usize {
    cache.entries.capacity() * std::mem::size_of::<Entry>()
        + cache
            .entries
            .iter()
            .map(|entry| {
                entry.text.capacity()
                    + entry.style.family.capacity()
                    + std::mem::size_of_val(entry.layout.lines())
                    + entry
                        .layout
                        .lines()
                        .iter()
                        .map(|line| {
                            line.glyphs.capacity() * std::mem::size_of::<crate::TextGlyph>()
                                + line
                                    .glyphs
                                    .iter()
                                    .map(|glyph| glyph.family.capacity())
                                    .sum::<usize>()
                        })
                        .sum::<usize>()
            })
            .sum::<usize>()
}
