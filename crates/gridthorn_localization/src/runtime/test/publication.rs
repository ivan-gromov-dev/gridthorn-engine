use std::{fmt::Write, hint::black_box, time::Instant};

use super::locale;
use crate::{CatalogAsset, Localization, MessageId, MessageParameters};

/// Fresh assets/services, with validation, publication and first format separated.
#[test]
#[ignore = "manual cold localization probe; run alone in release mode"]
fn measure_cold_localization_publication() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("cold_localization,locale,messages,operation,sample,elapsed_ns");
    for count in [16, 256, 4096] {
        for language in ["en-US", "ru", "ar-EG", "ja"] {
            measure(language, count);
        }
    }
}

fn source(count: usize, revision: usize) -> String {
    let mut source = String::from("value = { NUMBER($amount, minimumFractionDigits: 2) }\n");
    for index in 1..count {
        writeln!(source, "label-{index} = Revision {revision} label {index}").unwrap();
    }
    source
}

fn measure(language: &str, count: usize) {
    let selected = locale(language);
    let id = MessageId::new("value").unwrap();
    let label = MessageId::new(&format!("label-{}", count - 1)).unwrap();
    let mut parameters = MessageParameters::new();
    parameters.set_number("amount", 12345.5).unwrap();
    for sample in 0..51 {
        let input = source(count, sample);
        let start = Instant::now();
        let asset = CatalogAsset::from_source(selected.clone(), input).unwrap();
        let validation = start.elapsed().as_nanos();
        assert_eq!(asset.message_ids().count(), count);
        let start = Instant::now();
        let mut service = Localization::new([asset], selected.clone(), vec![]).unwrap();
        let construction = start.elapsed().as_nanos();
        let start = Instant::now();
        let formatted = black_box(service.format(&id, &parameters).unwrap());
        let first_format = start.elapsed().as_nanos();
        assert_eq!(formatted.locale, selected);
        assert_ne!(formatted.text, "");
        let candidate =
            CatalogAsset::from_source(selected.clone(), source(count, sample + 1)).unwrap();
        let old = service.format(&label, &MessageParameters::new()).unwrap();
        let start = Instant::now();
        service.replace_catalog(candidate).unwrap();
        let replacement = start.elapsed().as_nanos();
        assert_eq!(service.locale_chain(), std::slice::from_ref(&selected));
        assert_eq!(old.text, format!("Revision {sample} label {}", count - 1));
        assert_eq!(
            service
                .format(&label, &MessageParameters::new())
                .unwrap()
                .text,
            format!("Revision {} label {}", sample + 1, count - 1)
        );
        if sample > 0 {
            for (operation, elapsed) in [
                ("validate", validation),
                ("construct", construction),
                ("first_number", first_format),
                ("replace", replacement),
            ] {
                println!("cold_localization,{language},{count},{operation},{sample},{elapsed}");
            }
        }
    }
}
