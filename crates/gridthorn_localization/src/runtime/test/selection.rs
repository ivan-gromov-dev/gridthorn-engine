use super::{catalog, locale, service, text};
use crate::{Localization, LocalizationError, MessageId, MessageParameters};

#[test]
fn fallback_order_and_selected_locale_are_explicit() {
    let mut runtime = Localization::new(
        [
            catalog("ru", "hello = Привет"),
            catalog("en-US", "hello = Hello\nmissing = English"),
            catalog("ja", "missing = 日本語"),
        ],
        locale("ru"),
        vec![locale("ja"), locale("en-US")],
    )
    .unwrap();
    let empty = MessageParameters::new();
    assert_eq!(text(&runtime, "hello", &empty), "Привет");
    let result = runtime
        .format(&MessageId::new("missing").unwrap(), &empty)
        .unwrap();
    assert_eq!(result.text, "日本語");
    assert_eq!(result.locale, locale("ja"));
    runtime
        .select_locale(locale("en-US"), vec![locale("ru")])
        .unwrap();
    assert_eq!(text(&runtime, "hello", &empty), "Hello");
    assert!(matches!(
        runtime.format(&MessageId::new("absent").unwrap(), &empty),
        Err(LocalizationError::MissingMessage { .. })
    ));
}

#[test]
fn invalid_selection_and_duplicate_install_leave_state_intact() {
    let mut runtime = service("en", "a = Old");
    assert!(matches!(
        runtime.select_locale(locale("en-GB"), vec![]),
        Err(LocalizationError::MissingLocale(_))
    ));
    assert!(matches!(
        runtime.select_locale(locale("en"), vec![locale("en")]),
        Err(LocalizationError::DuplicateLocale(_))
    ));
    assert!(matches!(
        runtime.install_catalog(catalog("en", "a = New")),
        Err(LocalizationError::DuplicateCatalog(_))
    ));
    assert_eq!(runtime.locale_chain(), [locale("en")]);
    assert_eq!(text(&runtime, "a", &MessageParameters::new()), "Old");
    assert!(
        Localization::new(
            [catalog("en", "a = A"), catalog("en", "b = B")],
            locale("en"),
            vec![]
        )
        .is_err()
    );
}

#[test]
fn replacement_and_install_are_visible_only_after_explicit_publication() {
    let old = catalog("en", "a = Old");
    let mut runtime = Localization::new([old.clone()], locale("en"), vec![]).unwrap();
    let returned = text(&runtime, "a", &MessageParameters::new());
    assert!(crate::CatalogAsset::from_source(locale("en"), "a = {".to_owned()).is_err());
    assert_eq!(text(&runtime, "a", &MessageParameters::new()), "Old");
    runtime.replace_catalog(catalog("en", "a = New")).unwrap();
    assert_eq!(text(&runtime, "a", &MessageParameters::new()), "New");
    assert_eq!(returned, "Old");
    assert!(runtime.replace_catalog(catalog("ru", "a = А")).is_err());
    runtime.install_catalog(catalog("ru", "a = А")).unwrap();
    runtime
        .select_locale(locale("ru"), vec![locale("en")])
        .unwrap();
    assert_eq!(text(&runtime, "a", &MessageParameters::new()), "А");
    assert_eq!(old.message_ids().collect::<Vec<_>>(), ["a"]);
}

#[test]
fn missing_arguments_do_not_fall_back_or_return_partial_output() {
    let runtime = Localization::new(
        [
            catalog("ru", "a = Привет, { $name }"),
            catalog("en", "a = Hello"),
        ],
        locale("ru"),
        vec![locale("en")],
    )
    .unwrap();
    let error = runtime
        .format(&MessageId::new("a").unwrap(), &MessageParameters::new())
        .unwrap_err();
    assert!(matches!(error, LocalizationError::Format { .. }));
    assert!(error.to_string().contains("name"));
}

#[test]
fn presentation_service_and_assets_are_send_sync() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Localization>();
    assert_send_sync::<crate::CatalogAsset>();
    assert_send_sync::<MessageParameters>();
}
