use super::assets;
use crate::{TextStyle, TextSystem};
use std::{hint::black_box, sync::Arc, time::Instant};

/// Separate repeated narrow layouts, primary/fallback fonts and retained cache identity.
#[test]
#[ignore = "manual narrow-layout probe; run alone in release without diagnostics"]
fn measure_narrow_japanese_layout() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    assert_eq!(std::env::var_os("GRIDTHORN_TEXT_PERFORMANCE"), None);
    let assets = assets();
    let input = "文章を確認する ".repeat(256);
    println!("narrow_layout,family,width,operation,sample,elapsed_ns,lines,glyphs,hit");
    for family in ["Noto Sans", "Noto Sans JP"] {
        for width in [1.0, 12.0, 600.0] {
            let mut service = TextSystem::new("en-US", &assets).unwrap();
            let mut style = TextStyle::new(family, 20.0);
            style.width = Some(width);
            let mut previous = None;
            for sample in 0..110 {
                let start = Instant::now();
                let layout = black_box(service.layout(&input, &style).unwrap());
                let elapsed = start.elapsed().as_nanos();
                let hit = previous
                    .as_ref()
                    .and_then(std::sync::Weak::upgrade)
                    .is_some_and(|buffer| Arc::ptr_eq(&buffer, &layout.buffer));
                previous = Some(Arc::downgrade(&layout.buffer));
                assert_eq!(layout.missing_glyphs(), 0);
                if sample >= 10 {
                    let glyphs: usize = layout.lines().iter().map(|line| line.glyphs.len()).sum();
                    println!(
                        "narrow_layout,{family},{width},layout,{},{},{},{glyphs},{hit}",
                        sample - 10,
                        elapsed,
                        layout.lines().len()
                    );
                }
            }
        }
    }
}
