use std::{fmt::Write, hint::black_box, time::Instant};

use super::locale;
use crate::{CatalogAsset, CatalogError, Localization, MessageId, MessageParameters};

/// Fresh services with bounded reference chains, nested selectors and rejected candidates.
#[test]
#[ignore = "manual complex localization publication probe; run alone in release"]
fn measure_complex_localization_publication() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("complex_localization,locale,messages,source_bytes,sample,operation,elapsed_ns");
    for count in [16, 256, 4096] {
        for language in ["en-US", "ru", "ar-EG", "ja"] {
            measure(language, count);
        }
    }
}

fn source(count: usize, revision: usize) -> String {
    let mut input = String::new();
    for index in 0..count {
        if index % 16 == 0 {
            writeln!(input, "node-{index} = Revision {revision} anchor {index}").unwrap();
        } else {
            writeln!(input, "node-{index} = {{ node-{} }} {{ $role ->", index - 1).unwrap();
            input.push_str("    [worker] { $n ->\n        [one] One\n        [few] Few\n       *[other] Other\n    } { NUMBER($amount, minimumFractionDigits: 2) }\n   *[other] Visitor\n}\n");
        }
    }
    input
}

fn measure(language: &str, count: usize) {
    let selected = locale(language);
    let id = MessageId::new(&format!("node-{}", count - 1)).unwrap();
    let anchor = count - 16;
    let mut parameters = MessageParameters::new();
    parameters.set_text("role", "worker").unwrap();
    parameters.set_number("n", 2.0).unwrap();
    parameters.set_number("amount", 12345.5).unwrap();
    for sample in 0..20 {
        let input = source(count, sample);
        let bytes = input.len();
        let start = Instant::now();
        let asset = CatalogAsset::from_source(selected.clone(), input).unwrap();
        let validation = start.elapsed().as_nanos();
        assert_eq!(asset.message_ids().count(), count);
        let old_asset = asset.clone();
        let start = Instant::now();
        let mut runtime = Localization::new([asset], selected.clone(), vec![]).unwrap();
        let construction = start.elapsed().as_nanos();
        let start = Instant::now();
        let old = black_box(runtime.format(&id, &parameters).unwrap());
        let first = start.elapsed().as_nanos();
        assert_eq!(old.locale, selected);
        assert!(
            old.text
                .contains(&format!("Revision {sample} anchor {anchor}"))
        );
        assert!(
            old.text
                .contains(if language == "ru" { "Few" } else { "Other" })
        );
        let input = source(count, sample + 1);
        let invalid = input.replacen(
            &format!("node-0 = Revision {} anchor 0", sample + 1),
            "node-0 = { node-0 }",
            1,
        );
        let start = Instant::now();
        let candidate = CatalogAsset::from_source(selected.clone(), input).unwrap();
        let candidate_validation = start.elapsed().as_nanos();
        let start = Instant::now();
        runtime.replace_catalog(candidate).unwrap();
        let replacement = start.elapsed().as_nanos();
        let published = runtime.format(&id, &parameters).unwrap();
        assert!(
            published
                .text
                .contains(&format!("Revision {} anchor {anchor}", sample + 1))
        );
        let start = Instant::now();
        let rejected = CatalogAsset::from_source(selected.clone(), invalid);
        let rejection = start.elapsed().as_nanos();
        assert!(matches!(
            rejected,
            Err(CatalogError::Validation { reason, .. }) if reason.contains("cyclic message reference")
        ));
        assert_eq!(runtime.format(&id, &parameters).unwrap(), published);
        assert!(
            old.text
                .contains(&format!("Revision {sample} anchor {anchor}"))
        );
        assert_eq!(old_asset.message_ids().count(), count);
        assert_eq!(runtime.locale_chain(), std::slice::from_ref(&selected));
        for (operation, elapsed) in [
            ("validate", validation),
            ("construct", construction),
            ("first_format", first),
            ("candidate_validate", candidate_validation),
            ("replace", replacement),
            ("reject_candidate", rejection),
        ] {
            println!(
                "complex_localization,{language},{count},{bytes},{sample},{operation},{elapsed}"
            );
        }
    }
}
