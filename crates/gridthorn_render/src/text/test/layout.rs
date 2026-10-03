use super::{assets, style, system};
use crate::{TextAlignment, TextError, TextStyle, TextSystem, TextWrap};

#[test]
fn word_only_can_overflow_while_glyph_wrapping_breaks_long_words() {
    let mut text = system();
    let mut settings = style();
    settings.width = Some(80.0);
    settings.wrap = TextWrap::Word;
    let word = text
        .layout("extraordinary", &settings)
        .expect("word wrapping");
    assert!(word.measurement().width > 80.0);
    settings.wrap = TextWrap::Glyph;
    let glyph = text
        .layout("extraordinary", &settings)
        .expect("glyph wrapping");
    assert!(glyph.lines().len() > 1);
    assert!(glyph.measurement().width <= 80.01);
    for width in [0.0, -1.0, f32::NAN, f32::INFINITY] {
        settings.width = Some(width);
        assert!(matches!(
            text.layout("a", &settings),
            Err(TextError::InvalidMetrics)
        ));
    }
}

#[test]
fn validates_assets_primary_family_and_metrics() {
    assert!(matches!(
        TextSystem::new("en", &[]),
        Err(TextError::NoFonts)
    ));
    let fonts = assets();
    assert!(
        fonts[0]
            .families()
            .iter()
            .any(|family| family == "Noto Sans")
    );
    let mut text = system();
    assert!(matches!(
        text.layout("hello", &TextStyle::new("unknown", 24.0)),
        Err(TextError::UnknownFamily(_))
    ));
    for value in [0.0, -1.0, f32::NAN, f32::INFINITY, 1025.0] {
        assert!(matches!(
            text.layout("hello", &TextStyle::new("Noto Sans", value)),
            Err(TextError::InvalidMetrics)
        ));
    }
    assert!(matches!(
        text.layout(&"x".repeat(65537), &style()),
        Err(TextError::TooLarge)
    ));
}

#[test]
fn shapes_cyrillic_combining_marks_and_ligatures() {
    let mut text = system();
    let cyrillic = text
        .layout("Привет, мир! Ёжик", &style())
        .expect("Cyrillic");
    assert_eq!(cyrillic.missing_glyphs(), 0);
    assert!(cyrillic.measurement().width > 100.0);
    let combined = text
        .layout("e\u{301}", &style())
        .expect("combining sequence");
    assert_eq!(combined.lines()[0].glyphs.len(), 1);
    assert_eq!(combined.lines()[0].glyphs[0].cluster, 0..3);
    let ligature = text.layout("ffi", &style()).expect("ligature");
    assert!(ligature.lines()[0].glyphs.len() < 3);
    assert_eq!(ligature.lines()[0].glyphs[0].cluster, 0..3);
}

#[test]
fn selects_fallback_for_arabic_and_japanese_and_reports_missing_coverage() {
    let mut text = system();
    let layout = text
        .layout("Русский العربية 日本語", &style())
        .expect("multilingual");
    assert_eq!(layout.missing_glyphs(), 0);
    let families: Vec<_> = layout.lines()[0]
        .glyphs
        .iter()
        .map(|glyph| glyph.family.as_str())
        .collect();
    assert!(families.contains(&"Noto Sans"));
    assert!(families.contains(&"Noto Sans Arabic"));
    assert!(families.contains(&"Noto Sans JP"));
    assert!(
        text.layout("\u{10FFFF}", &style())
            .expect("missing coverage")
            .missing_glyphs()
            > 0
    );
}

#[test]
fn reorders_mixed_bidi_and_contextually_shapes_arabic() {
    let mut text = system();
    let mixed = text
        .layout("abc العربية 123", &style())
        .expect("mixed bidi");
    let glyphs = &mixed.lines()[0].glyphs;
    assert!(glyphs.iter().any(|glyph| glyph.right_to_left));
    assert!(glyphs.iter().any(|glyph| !glyph.right_to_left));
    let rtl: Vec<_> = glyphs.iter().filter(|glyph| glyph.right_to_left).collect();
    assert!(
        rtl.windows(2)
            .any(|pair| pair[0].cluster.start < pair[1].cluster.start
                && pair[0].position[0] > pair[1].position[0]
                || pair[0].cluster.start > pair[1].cluster.start
                    && pair[0].position[0] < pair[1].position[0])
    );
    let isolated = text.layout("ب", &style()).expect("isolated").lines()[0].glyphs[0].glyph_id;
    let joined = text.layout("بب", &style()).expect("joined");
    assert!(joined.lines()[0].right_to_left);
    assert!(
        joined.lines()[0]
            .glyphs
            .iter()
            .all(|glyph| glyph.glyph_id != isolated)
    );
}

#[test]
fn measures_wraps_long_words_and_retains_empty_lines() {
    let mut text = system();
    let mut wrapped = style();
    wrapped.width = Some(100.0);
    let layout = text
        .layout("Привет мир extraordinary 日本語日本語", &wrapped)
        .expect("wrap");
    assert!(layout.lines().len() > 2);
    assert!(layout.lines().iter().all(|line| line.width <= 100.01));
    wrapped.wrap = TextWrap::None;
    let unwrapped = text
        .layout("extraordinaryextraordinary", &wrapped)
        .expect("no wrap");
    assert_eq!(unwrapped.lines().len(), 1);
    assert!(unwrapped.measurement().width > 100.0);
    let lines = text.layout("a\n\nb\n", &style()).expect("newlines");
    assert_eq!(lines.lines().len(), 4);
    assert!((lines.measurement().height - 4.0 * style().line_height).abs() < 0.01);
}

#[test]
fn alignment_changes_origin_without_changing_advance() {
    let mut text = system();
    let mut settings = style();
    settings.width = Some(300.0);
    settings.alignment = TextAlignment::Left;
    let left = text.layout("Привет", &settings).expect("left");
    settings.alignment = TextAlignment::Right;
    let right = text.layout("Привет", &settings).expect("right");
    assert_eq!(left.measurement(), right.measurement());
    assert!(right.lines()[0].glyphs[0].position[0] > left.lines()[0].glyphs[0].position[0] + 100.0);
}
