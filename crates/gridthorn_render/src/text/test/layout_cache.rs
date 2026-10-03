use super::*;
use crate::text::test::{style, system};
use crate::{Color, TextAlignment, TextError, TextWrap};
use std::sync::Arc;

#[test]
fn service_reuses_only_matching_text_and_complete_style() {
    let mut text = system();
    let settings = style();
    let first = text
        .layout("Привет مرحبًا 日本語 e\u{301}", &settings)
        .unwrap();
    let repeated = text
        .layout("Привет مرحبًا 日本語 e\u{301}", &settings)
        .unwrap();
    assert!(Arc::ptr_eq(&first.buffer, &repeated.buffer));
    assert!(Arc::ptr_eq(&first.lines, &repeated.lines));
    let changed = text.layout("different", &settings).unwrap();
    assert!(!Arc::ptr_eq(&first.buffer, &changed.buffer));
    let mut variants = vec![settings.clone(); 6];
    variants[0].width = Some(60.0);
    variants[1].font_size = 28.0;
    variants[2].line_height = 60.0;
    variants[3].wrap = TextWrap::None;
    variants[4].alignment = TextAlignment::Center;
    variants[5].family = "Noto Sans Arabic".into();
    for variant in variants {
        let changed = text
            .layout("Привет مرحبًا 日本語 e\u{301}", &variant)
            .unwrap();
        assert!(!Arc::ptr_eq(&first.buffer, &changed.buffer));
    }
    let other = system()
        .layout("Привет مرحبًا 日本語 e\u{301}", &settings)
        .unwrap();
    assert!(!Arc::ptr_eq(&first.buffer, &other.buffer));
    assert!(matches!(
        text.rasterize(&other, 1.0, Color::rgba(1.0, 1.0, 1.0, 1.0)),
        Err(TextError::ForeignLayout)
    ));
    let mut invalid = settings.clone();
    invalid.font_size = f32::NAN;
    assert!(matches!(
        text.layout("Привет مرحبًا 日本語 e\u{301}", &invalid),
        Err(TextError::InvalidMetrics)
    ));
}

#[test]
fn lru_eviction_preserves_live_layouts_and_geometry_limits() {
    let mut text = system();
    let settings = style();
    let original = text.layout("keep", &settings).unwrap();
    for index in 0..ENTRY_LIMIT - 1 {
        text.layout(&format!("item {index}"), &settings).unwrap();
    }
    let retained = text.layout("keep", &settings).unwrap();
    assert!(Arc::ptr_eq(&original.buffer, &retained.buffer));
    text.layout("new item", &settings).unwrap();
    assert!(text.layouts.get("item 0", &settings).is_none());
    assert!(text.layouts.get("keep", &settings).is_some());
    let mut cache = LayoutCache::default();
    for index in 0..ENTRY_LIMIT + 4 {
        cache.insert(&format!("{index}"), &settings, &original);
    }
    assert_eq!(cache.entries.len(), ENTRY_LIMIT);
    assert!(
        cache.key_bytes <= KEY_BYTES_LIMIT
            && cache.glyphs <= GLYPH_LIMIT
            && cache.lines <= LINE_LIMIT
    );
    cache.insert(&"x".repeat(KEY_BYTES_LIMIT + 1), &settings, &original);
    assert_eq!(cache.entries.len(), ENTRY_LIMIT);
    text.layouts = LayoutCache::default();
    assert_eq!(
        text.rasterize(&original, 2.0, Color::rgba(1.0, 1.0, 1.0, 1.0))
            .unwrap(),
        text.rasterize(&retained, 2.0, Color::rgba(1.0, 1.0, 1.0, 1.0))
            .unwrap()
    );
}

#[test]
fn aggregate_geometry_and_key_budgets_evict_without_retaining_oversized_entries() {
    let original = system().layout("sample", &style()).unwrap();
    let mut heavy = original.clone();
    let mut line = original.lines()[0].clone();
    line.glyphs = vec![line.glyphs[0].clone(); 2048];
    heavy.lines = vec![line].into();
    let mut cache = LayoutCache::default();
    for index in 0..20 {
        cache.insert(&format!("heavy {index}"), &style(), &heavy);
    }
    assert_eq!(cache.entries.len(), 8);
    assert_eq!(cache.glyphs, GLYPH_LIMIT);
    assert!(cache.get("heavy 0", &style()).is_none());
    let mut blank = original.lines()[0].clone();
    blank.glyphs.clear();
    heavy.lines = vec![blank.clone(); LINE_LIMIT + 1].into();
    cache.insert("oversized", &style(), &heavy);
    assert!(cache.get("oversized", &style()).is_none());
    heavy.lines = vec![blank; 128].into();
    let mut cache = LayoutCache::default();
    for index in 0..20 {
        cache.insert(&format!("lines {index}"), &style(), &heavy);
    }
    assert_eq!(cache.entries.len(), 8);
    assert_eq!(cache.lines, LINE_LIMIT);
    let mut cache = LayoutCache::default();
    for index in 0..20 {
        cache.insert(
            &format!("{index}{}", "x".repeat(16 * 1024)),
            &style(),
            &original,
        );
    }
    assert!(cache.key_bytes <= KEY_BYTES_LIMIT);
    assert!(cache.entries.len() < 20);
}
