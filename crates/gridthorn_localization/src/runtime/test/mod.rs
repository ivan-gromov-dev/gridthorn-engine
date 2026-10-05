mod formatting;
mod scaling;
mod selection;

use crate::{CatalogAsset, LocaleId, Localization, MessageId, MessageParameters};

fn locale(value: &str) -> LocaleId {
    LocaleId::new(value).unwrap()
}

fn catalog(language: &str, source: &str) -> CatalogAsset {
    CatalogAsset::from_source(locale(language), source.to_owned()).unwrap()
}

fn service(language: &str, source: &str) -> Localization {
    Localization::new([catalog(language, source)], locale(language), vec![]).unwrap()
}

fn text(service: &Localization, id: &str, parameters: &MessageParameters) -> String {
    service
        .format(&MessageId::new(id).unwrap(), parameters)
        .unwrap()
        .text
}
mod complex_publication;
mod publication;
