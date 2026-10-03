use crate::localization::{CatalogAsset, LocaleId, Localization, MessageId, MessageParameters};

#[test]
fn facade_supports_headless_locale_switch_and_fallback() {
    let en = LocaleId::new("en-US").unwrap();
    let ru = LocaleId::new("ru").unwrap();
    let english = CatalogAsset::from_source(
        en.clone(),
        "hello = Hello, { $name }\nbackup = Fallback".to_owned(),
    )
    .unwrap();
    let russian =
        CatalogAsset::from_source(ru.clone(), "hello = Привет, { $name }".to_owned()).unwrap();
    let mut runtime = Localization::new([english, russian], ru, vec![en.clone()]).unwrap();
    let mut parameters = MessageParameters::new();
    parameters.set_text("name", "Иван").unwrap();
    let id = MessageId::new("hello").unwrap();
    assert!(
        runtime
            .format(&id, &parameters)
            .unwrap()
            .text
            .starts_with("Привет")
    );
    assert_eq!(
        runtime
            .format(&MessageId::new("backup").unwrap(), &parameters)
            .unwrap()
            .locale,
        en
    );
    runtime.select_locale(en, vec![]).unwrap();
    assert!(
        runtime
            .format(&id, &parameters)
            .unwrap()
            .text
            .starts_with("Hello")
    );
    assert_eq!(runtime.format_number(1234.5, 2, true).unwrap(), "1,234.50");
}
