use crate::{CatalogAsset, CatalogError, LocaleId, MessageId};
use std::fmt::Write;

fn catalog(source: &str) -> Result<CatalogAsset, CatalogError> {
    CatalogAsset::from_source(LocaleId::new("en-US").unwrap(), source.to_owned())
}

#[test]
fn identities_are_canonical_and_case_sensitive() {
    assert_eq!(LocaleId::new("EN-us").unwrap().as_str(), "en-US");
    for locale in ["", "en-u-nu-arab", "bad@locale", "x-private"] {
        assert!(LocaleId::new(locale).is_err(), "{locale}");
    }
    assert_ne!(
        MessageId::new("Hello").unwrap(),
        MessageId::new("hello").unwrap()
    );
    for id in ["", "-term", "foo.bar", "1id", "hello world", "привет"] {
        assert!(MessageId::new(id).is_err(), "{id}");
    }
}

#[test]
fn valid_assets_share_data_and_list_sorted_messages() {
    let asset = catalog("z = Last\na = { z }\n").unwrap();
    assert_eq!(asset.message_ids().collect::<Vec<_>>(), ["a", "z"]);
    assert!(std::sync::Arc::ptr_eq(
        &asset.resource,
        &asset.clone().resource
    ));
    assert_eq!(asset.locale().as_str(), "en-US");
}

#[test]
fn invalid_catalogs_are_rejected_with_context() {
    for source in [
        "oops = {",
        "a = First\na = Second",
        "a = { missing }",
        "a = { b }\nb = { a }",
        "a = { a }",
        "-term = Brand\na = { -term }",
        "a = Text\n    .title = Title",
        "a = { DATETIME($date) }",
        "a = { NUMBER($n, minimumFractionDigits: 7) }",
        "a = { NUMBER($n, style: \"currency\") }",
        "a = { NUMBER($n, useGrouping: \"maybe\") }",
        "a = { NUMBER($n, minimumFractionDigits: 2, minimumFractionDigits: 3) }",
        "a = { 1000000000001 }",
        "a = { 0.0000001 }",
        "a = { $n ->\n    [one] One\n    [one] Duplicate\n   *[other] Other\n}",
        "a = { $n ->\n    [1] One\n    [1.0] Duplicate\n   *[other] Other\n}",
    ] {
        let error = catalog(source).unwrap_err().to_string();
        assert!(error.contains("en-US"), "{source}: {error}");
    }
}

#[test]
fn missing_references_do_not_panic_even_when_not_first_sorted_entry() {
    assert!(catalog("a = { b }\nb = { missing }").is_err());
}

#[test]
fn size_and_file_errors_are_typed() {
    assert!(matches!(
        catalog(&"x".repeat(1024 * 1024 + 1)),
        Err(CatalogError::TooLarge)
    ));
    assert!(matches!(
        CatalogAsset::load(
            LocaleId::new("en").unwrap(),
            "missing-localization-catalog.ftl"
        ),
        Err(CatalogError::Read { .. })
    ));
}

#[test]
fn excessive_reference_depth_is_rejected() {
    let mut source = String::new();
    for index in 0..34 {
        writeln!(source, "m{index} = {{ m{} }}", index + 1).unwrap();
    }
    source.push_str("m34 = End");
    assert!(catalog(&source).is_err());
}

#[test]
fn parser_recursion_is_bounded_before_backend_parsing() {
    let nested = format!("a = {}1{}", "{".repeat(10_000), "}".repeat(10_000));
    assert!(matches!(
        catalog(&nested),
        Err(CatalogError::Validation { .. })
    ));
    let function = format!(
        "a = {{ {}1{} }}",
        "NUMBER(".repeat(10_000),
        ")".repeat(10_000)
    );
    assert!(matches!(
        catalog(&function),
        Err(CatalogError::Validation { .. })
    ));
    let literal = format!("a = {{ \"{}\" }}", "{".repeat(100));
    assert!(catalog(&literal).is_ok());
    let variant = format!(
        "a = {{ $n ->\n   *[other] \"{}1{}\n}}",
        "{".repeat(10_000),
        "}".repeat(10_000)
    );
    assert!(matches!(
        catalog(&variant),
        Err(CatalogError::Validation { .. })
    ));
}

#[test]
fn file_loading_validates_encoding_size_and_content() {
    let path = std::env::temp_dir().join(format!(
        "gridthorn-localization-catalog-{}.ftl",
        std::process::id()
    ));
    let locale = LocaleId::new("en").unwrap();
    std::fs::write(&path, b"a = Valid").unwrap();
    assert_eq!(
        CatalogAsset::load(locale.clone(), &path)
            .unwrap()
            .message_ids()
            .collect::<Vec<_>>(),
        ["a"]
    );
    std::fs::write(&path, [0xff, 0xfe]).unwrap();
    assert!(matches!(
        CatalogAsset::load(locale.clone(), &path),
        Err(CatalogError::Encoding(_))
    ));
    std::fs::write(&path, vec![b'a'; 1024 * 1024 + 1]).unwrap();
    assert!(matches!(
        CatalogAsset::load(locale.clone(), &path),
        Err(CatalogError::TooLarge)
    ));
    std::fs::write(&path, b"a = {").unwrap();
    assert!(matches!(
        CatalogAsset::load(locale, &path),
        Err(CatalogError::Syntax { .. })
    ));
    std::fs::remove_file(path).unwrap();
}
