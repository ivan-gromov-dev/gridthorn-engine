use std::{hint::black_box, sync::Arc, time::Instant};

use super::{assets, style};
use crate::{Color, TextStyle, TextSystem};

/// Individual layout/raster calls under long-text and cache working-set pressure.
#[test]
#[ignore = "manual text pressure probe; run alone in release mode without diagnostics"]
fn measure_long_text_and_cache_pressure() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    assert_eq!(std::env::var_os("GRIDTHORN_TEXT_PERFORMANCE"), None);
    let fonts = assets();
    println!(
        "text_pressure,locale,repeats,working_set,dpi,operation,sample,elapsed_ns,bytes,glyphs,hit"
    );
    for (language, phrase) in [
        ("en-US", "Review text "),
        ("ru", "Проверка текста "),
        ("ar-EG", "مراجعة النص "),
        ("ja", "文章を確認する "),
    ] {
        for repeats in [8, 64, 256] {
            for working_set in [1, 32, 96] {
                let mut text = TextSystem::new(language, &fonts).unwrap();
                let mut settings = style();
                settings.width = Some(600.0);
                let inputs: Vec<_> = (0..working_set)
                    .map(|index| format!("{} {index}", phrase.repeat(repeats)))
                    .collect();
                measure_layout(&mut text, language, repeats, &inputs, &settings);
                if working_set == 1 && repeats <= 64 {
                    for dpi in [1, 2] {
                        measure_raster(&mut text, language, repeats, &inputs[0], &settings, dpi);
                    }
                }
            }
        }
    }
}

fn measure_layout(
    text: &mut TextSystem,
    language: &str,
    repeats: usize,
    inputs: &[String],
    settings: &TextStyle,
) {
    let original = text.layout(&inputs[0], settings).unwrap();
    let expected = original.measurement();
    let mut previous: Vec<_> = inputs
        .iter()
        .map(|input| {
            let layout = text.layout(input, settings).unwrap();
            assert_eq!(layout.missing_glyphs(), 0);
            Arc::downgrade(&layout.buffer)
        })
        .collect();
    for sample in 0..100 {
        let index = sample % inputs.len();
        let start = Instant::now();
        let layout = black_box(text.layout(black_box(&inputs[index]), settings).unwrap());
        let elapsed = start.elapsed().as_nanos();
        let hit = previous[index]
            .upgrade()
            .is_some_and(|buffer| Arc::ptr_eq(&buffer, &layout.buffer));
        previous[index] = Arc::downgrade(&layout.buffer);
        let glyphs: usize = layout.lines().iter().map(|line| line.glyphs.len()).sum();
        println!(
            "text_pressure,{language},{repeats},{},0,layout,{sample},{elapsed},{},{glyphs},{hit}",
            inputs.len(),
            inputs[index].len()
        );
    }
    assert_eq!(original.measurement(), expected);
    assert_eq!(
        text.layout(&inputs[0], settings).unwrap().measurement(),
        expected
    );
}

fn measure_raster(
    text: &mut TextSystem,
    language: &str,
    repeats: usize,
    input: &str,
    settings: &TextStyle,
    dpi: u16,
) {
    let layout = text.layout(input, settings).unwrap();
    let color = Color::rgba(1.0, 1.0, 1.0, 1.0);
    let expected = text.rasterize(&layout, f32::from(dpi), color).unwrap();
    for sample in 0..100 {
        let start = Instant::now();
        let raster = black_box(text.rasterize(&layout, f32::from(dpi), color).unwrap());
        let elapsed = start.elapsed().as_nanos();
        assert_eq!(raster, expected);
        let glyphs: usize = layout.lines().iter().map(|line| line.glyphs.len()).sum();
        println!(
            "text_pressure,{language},{repeats},1,{dpi},raster,{sample},{elapsed},{},{glyphs},true",
            input.len()
        );
    }
}

/// Attribute Japanese misses to fallback family versus line wrapping.
#[test]
#[ignore = "manual Japanese attribution probe; run alone in release mode without diagnostics"]
fn measure_japanese_layout_attribution() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    assert_eq!(std::env::var_os("GRIDTHORN_TEXT_PERFORMANCE"), None);
    let fonts = assets();
    println!(
        "text_pressure,locale,repeats,working_set,dpi,operation,sample,elapsed_ns,bytes,glyphs,hit"
    );
    for (name, family, wrap) in [
        (
            "ja-fallback-wrap",
            "Noto Sans",
            crate::TextWrap::WordOrGlyph,
        ),
        (
            "ja-primary-wrap",
            "Noto Sans JP",
            crate::TextWrap::WordOrGlyph,
        ),
        ("ja-fallback-nowrap", "Noto Sans", crate::TextWrap::None),
        ("ja-primary-nowrap", "Noto Sans JP", crate::TextWrap::None),
    ] {
        let mut text = TextSystem::new("ja", &fonts).unwrap();
        let mut settings = TextStyle::new(family, 24.0);
        settings.width = Some(600.0);
        settings.wrap = wrap;
        let inputs: Vec<_> = (0..96)
            .map(|index| format!("{} {index}", "文章を確認する ".repeat(256)))
            .collect();
        measure_layout(&mut text, name, 256, &inputs, &settings);
    }
}
