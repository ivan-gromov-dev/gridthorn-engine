use super::{catalog, locale, service, text};
use crate::{Localization, MessageId, MessageParameters};

const PLURAL: &str = "count = { $n ->\n    [0] exact-zero\n    [zero] zero\n    [one] one\n    [two] two\n    [few] few\n    [many] many\n   *[other] other\n}";

#[test]
fn cardinal_rules_cover_english_russian_arabic_and_japanese() {
    for (language, cases) in [
        (
            "en",
            vec![
                (0.0, "exact-zero"),
                (1.0, "one"),
                (2.0, "other"),
                (1.5, "other"),
            ],
        ),
        (
            "ru",
            vec![
                (1.0, "one"),
                (2.0, "few"),
                (5.0, "many"),
                (11.0, "many"),
                (21.0, "one"),
                (1.5, "other"),
            ],
        ),
        (
            "ar",
            vec![
                (0.0, "exact-zero"),
                (1.0, "one"),
                (2.0, "two"),
                (3.0, "few"),
                (11.0, "many"),
                (100.0, "other"),
            ],
        ),
        ("ja", vec![(1.0, "other"), (2.0, "other")]),
    ] {
        let runtime = service(language, PLURAL);
        for (number, expected) in cases {
            let mut parameters = MessageParameters::new();
            parameters.set_number("n", number).unwrap();
            assert_eq!(
                text(&runtime, "count", &parameters),
                expected,
                "{language}: {number}"
            );
        }
    }
}

#[test]
fn string_selection_ordinal_rules_and_local_message_references_work() {
    let runtime = service(
        "en",
        "brand = Gridthorn\nwelcome = { brand }: { $role ->\n    [worker] Worker\n   *[other] Visitor\n}\nrank = { NUMBER($n, type: \"ordinal\") ->\n    [one] st\n    [two] nd\n    [few] rd\n   *[other] th\n}",
    );
    let mut parameters = MessageParameters::new();
    parameters.set_text("role", "worker").unwrap();
    assert!(text(&runtime, "welcome", &parameters).contains("Worker"));
    parameters.set_text("role", "unknown").unwrap();
    assert!(text(&runtime, "welcome", &parameters).contains("Visitor"));
    for (number, expected) in [
        (1.0, "st"),
        (2.0, "nd"),
        (3.0, "rd"),
        (11.0, "th"),
        (22.0, "nd"),
    ] {
        parameters.set_number("n", number).unwrap();
        assert_eq!(text(&runtime, "rank", &parameters), expected);
    }
}

#[test]
fn numbers_use_catalog_locale_and_preserve_bidi_isolation() {
    let mut runtime = Localization::new(
        [
            catalog("ru", "hello = Привет, { $name }!"),
            catalog("en", "amount = { NUMBER($n, minimumFractionDigits: 2) }"),
        ],
        locale("ru"),
        vec![locale("en")],
    )
    .unwrap();
    assert_eq!(
        runtime.format_number(12345.5, 2, true).unwrap(),
        "12\u{a0}345,50"
    );
    assert_eq!(
        runtime.format_number(-12345.5, 2, false).unwrap(),
        "-12345,50"
    );
    let mut parameters = MessageParameters::new();
    parameters.set_number("n", 12345.5).unwrap();
    let result = runtime
        .format(&MessageId::new("amount").unwrap(), &parameters)
        .unwrap();
    assert_eq!(result.locale, locale("en"));
    assert!(result.text.contains("12,345.50"));
    parameters.set_text("name", "علي { $n }").unwrap();
    assert_eq!(
        text(&runtime, "hello", &parameters),
        "Привет, \u{2068}علي { $n }\u{2069}!"
    );
    runtime.install_catalog(catalog("ar-EG", "a = A")).unwrap();
    runtime.select_locale(locale("ar-EG"), vec![]).unwrap();
    assert_eq!(runtime.format_number(1234.5, 2, true).unwrap(), "١٬٢٣٤٫٥٠");
}

#[test]
fn precision_affects_plural_operands_and_grouping_is_configurable() {
    let runtime = service(
        "en",
        "p = { NUMBER($n, minimumFractionDigits: 2) ->\n    [one] one\n   *[other] other\n}\nn = { NUMBER($n, useGrouping: \"false\", minimumFractionDigits: 2) }",
    );
    let mut parameters = MessageParameters::new();
    parameters.set_number("n", 1.0).unwrap();
    assert_eq!(text(&runtime, "p", &parameters), "other");
    parameters.set_number("n", 1234.5).unwrap();
    assert!(text(&runtime, "n", &parameters).contains("1234.50"));
}

#[test]
fn invalid_parameter_updates_preserve_previous_value() {
    let runtime = service("en", "a = { NUMBER($n) }");
    let mut parameters = MessageParameters::new();
    parameters.set_number("n", 2.0).unwrap();
    for number in [
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        1e13,
        0.000_000_1,
    ] {
        assert!(parameters.set_number("n", number).is_err());
        assert!(runtime.format_number(number, 0, true).is_err());
    }
    assert!(text(&runtime, "a", &parameters).contains('2'));
    assert!(parameters.set_text("bad name", "text").is_err());
    assert!(parameters.set_text("n", "x".repeat(64 * 1024 + 1)).is_err());
    assert!(runtime.format_number(1.0, 7, true).is_err());
    parameters.set_text("n", "text").unwrap();
    assert!(
        runtime
            .format(&MessageId::new("a").unwrap(), &parameters)
            .is_err()
    );
}

#[test]
fn fallback_plural_rules_and_referenced_numeric_parameters_use_resolved_catalog() {
    let runtime = Localization::new(
        [catalog("ru", "local = Местный"), catalog("en", "count = { $n ->\n    [one] one\n    [many] many\n   *[other] other\n}\nnumber = { NUMBER($n) }\nreference = { number }")],
        locale("ru"), vec![locale("en")],
    ).unwrap();
    let mut parameters = MessageParameters::new();
    parameters.set_number("n", 5.0).unwrap();
    assert_eq!(text(&runtime, "count", &parameters), "other");
    parameters.set_text("n", "five").unwrap();
    assert!(
        runtime
            .format(&MessageId::new("reference").unwrap(), &parameters)
            .is_err()
    );
}
