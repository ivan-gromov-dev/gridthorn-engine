use std::{hint::black_box, sync::Arc, time::Instant};

use super::assets;
use crate::{Color, TextStyle, TextSystem, UiRect};

/// Separate fresh-service shaping, warm-font misses, hits and raster output storage.
#[test]
#[ignore = "manual cold Japanese probe; run alone in release without diagnostics"]
fn measure_cold_japanese_phases() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    assert_eq!(std::env::var_os("GRIDTHORN_TEXT_PERFORMANCE"), None);
    cases();
}

/// Attribute backend shaping, diagnostic extraction and raster snapshot construction.
#[test]
#[ignore = "manual Japanese phase probe; enable text diagnostics and run alone in release"]
fn attribute_japanese_phases() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    assert!(std::env::var_os("GRIDTHORN_TEXT_PERFORMANCE").is_some());
    cases();
}

fn cases() {
    let assets = assets();
    println!("cold_japanese,family,repeats,width,sample,operation,elapsed_ns,output_bytes");
    for family in ["Noto Sans", "Noto Sans JP"] {
        for (repeats, width) in [(256, 1.0), (256, 12.0), (256, 600.0), (64, 600.0)] {
            let input = "文章を確認する ".repeat(repeats);
            let changed = format!("{input} 1");
            let mut style = TextStyle::new(family, 20.0);
            style.width = Some(width);
            for sample in 0..20 {
                let start = Instant::now();
                let mut service = TextSystem::new("en-US", &assets).unwrap();
                report(family, repeats, width, sample, "service", start, 0);
                let start = Instant::now();
                let first = black_box(service.layout(&input, &style).unwrap());
                report(family, repeats, width, sample, "cold_layout", start, 0);
                let start = Instant::now();
                let unique = black_box(service.layout(&changed, &style).unwrap());
                report(family, repeats, width, sample, "warm_font_unique", start, 0);
                let start = Instant::now();
                let hit = black_box(service.layout(&changed, &style).unwrap());
                report(family, repeats, width, sample, "layout_hit", start, 0);
                assert!(Arc::ptr_eq(&unique.buffer, &hit.buffer));
                assert_eq!(first.missing_glyphs(), 0);
                assert_eq!(unique.missing_glyphs(), 0);
                if repeats == 64 {
                    let mut expected = None;
                    for operation in ["cold_raster", "warm_raster", "warm_clipped"] {
                        let start = Instant::now();
                        let raster = if operation == "warm_clipped" {
                            service.rasterize_clipped(
                                &first,
                                1.0,
                                Color::rgb(1.0, 1.0, 1.0),
                                UiRect::new([0.0, 0.0], [600.0, 40.0], Color::rgb(1.0, 1.0, 1.0))
                                    .unwrap(),
                            )
                        } else {
                            service.rasterize(&first, 1.0, Color::rgb(1.0, 1.0, 1.0))
                        }
                        .unwrap();
                        let elapsed = start.elapsed().as_nanos();
                        let bytes = std::mem::size_of_val(raster.pixels.as_slice());
                        println!(
                            "cold_japanese,{family},{repeats},{width},{sample},{operation},{elapsed},{bytes}"
                        );
                        if operation == "cold_raster" {
                            expected = Some(raster.clone());
                        } else if operation == "warm_raster" {
                            assert_eq!(expected.as_ref(), Some(&raster));
                        } else {
                            assert!(
                                bytes
                                    < std::mem::size_of_val(
                                        expected.as_ref().unwrap().pixels.as_slice()
                                    )
                            );
                        }
                    }
                }
            }
        }
    }
}

fn report(
    family: &str,
    repeats: usize,
    width: f32,
    sample: usize,
    operation: &str,
    start: Instant,
    bytes: usize,
) {
    let elapsed = start.elapsed().as_nanos();
    println!("cold_japanese,{family},{repeats},{width},{sample},{operation},{elapsed},{bytes}");
}
