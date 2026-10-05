use std::fmt::Write;
use std::hint::black_box;
use std::time::Instant;

use super::{catalog, locale};
use crate::{Localization, MessageId, MessageParameters};

const CALLS: u32 = 100;
const BATCHES: u32 = 50;

/// Manual release probe of warm formatting with bounded catalog growth.
#[test]
#[ignore = "manual localization scaling probe; run alone in release mode"]
fn measure_localization_catalog_and_formatting_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("localization_scaling,locale,messages,operation,batch,calls,elapsed_ns");
    for messages in [16, 256, 4096] {
        for language in ["en-US", "ru", "ar-EG", "ja"] {
            let catalogs = ["en-US", "ru", "ar-EG", "ja"].map(|lang| {
                let mut source = "welcome = Welcome { $name }\nworkers = { $count ->\n    [one] One { NUMBER($count) }\n   *[other] Other { NUMBER($count) }\n}\nvalue = { NUMBER($amount, minimumFractionDigits: 2) }\n".to_owned();
                for index in 0..messages - 4 {
                    writeln!(source, "label-{index} = Label {index}").unwrap();
                }
                source.push_str(if lang == "en-US" { "fallback-only = Fallback\n" } else { "primary-only = Primary\n" });
                let asset = catalog(lang, &source);
                assert_eq!(asset.message_ids().count(), messages);
                asset
            });
            let fallback = if language == "en-US" {
                vec![]
            } else {
                vec![locale("en-US")]
            };
            let service = Localization::new(catalogs, locale(language), fallback).unwrap();
            for operation in [
                "literal",
                "interpolation",
                "plural_number",
                "number_message",
                "number_standalone",
                "fallback",
            ] {
                if operation != "fallback" || language != "en-US" {
                    measure(&service, language, messages, operation);
                }
            }
        }
    }
}

fn measure(service: &Localization, language: &str, messages: usize, operation: &str) {
    let mut parameters = MessageParameters::new();
    let name = match operation {
        "interpolation" => {
            parameters.set_text("name", "Player-日本語").unwrap();
            "welcome".to_owned()
        }
        "plural_number" => {
            parameters.set_number("count", 2.0).unwrap();
            "workers".to_owned()
        }
        "number_message" => {
            parameters.set_number("amount", 12345.5).unwrap();
            "value".to_owned()
        }
        "fallback" => "fallback-only".to_owned(),
        _ => format!("label-{}", messages - 5),
    };
    let id = MessageId::new(&name).unwrap();
    let expected = output(service, &id, &parameters, operation);
    assert_ne!(expected, "");
    if operation != "number_standalone" {
        assert_eq!(
            service.format(&id, &parameters).unwrap().locale,
            locale(if operation == "fallback" {
                "en-US"
            } else {
                language
            })
        );
    }
    run_batch(service, &id, &parameters, operation);
    for batch in 0..BATCHES {
        let start = Instant::now();
        run_batch(service, &id, &parameters, operation);
        let elapsed_ns = start.elapsed().as_nanos();
        println!(
            "localization_scaling,{language},{messages},{operation},{batch},{CALLS},{elapsed_ns}"
        );
    }
    assert_eq!(output(service, &id, &parameters, operation), expected);
}

fn run_batch(
    service: &Localization,
    id: &MessageId,
    parameters: &MessageParameters,
    operation: &str,
) {
    for _ in 0..CALLS {
        black_box(output(service, black_box(id), parameters, operation));
    }
}

fn output(
    service: &Localization,
    id: &MessageId,
    parameters: &MessageParameters,
    operation: &str,
) -> String {
    if operation == "number_standalone" {
        service.format_number(black_box(12345.5), 2, true).unwrap()
    } else {
        service.format(id, parameters).unwrap().text
    }
}
