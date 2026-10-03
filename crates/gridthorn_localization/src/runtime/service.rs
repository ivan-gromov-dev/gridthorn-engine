use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use fluent_bundle::{
    FluentArgs, FluentResource,
    concurrent::FluentBundle,
    types::{FluentNumber, FluentNumberOptions},
};
use intl_memoizer::Memoizable;

use super::{
    LocalizationError, MessageParameters,
    formatting::{NumberFormatter, format_value},
    parameters::{ParameterValue, parameter_error},
};
use crate::{CatalogAsset, LocaleId, MessageId, catalog::validation::numeric};

/// Complete translated text with the catalog locale that supplied it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalizedMessage {
    /// Text includes Fluent bidi isolation around interpolations; preserve it.
    pub text: String,
    /// Effective language for plural rules and numeric formatting.
    pub locale: LocaleId,
}

/// Explicit presentation locale/catalog service, usable without a window or GPU.
/// No OS locale detection, I/O during lookup, global state or automatic reload.
/// `Send + Sync`; catalog/selection mutations require exclusive mutable access.
pub struct Localization {
    catalogs: BTreeMap<LocaleId, FluentBundle<Arc<FluentResource>>>,
    numeric_parameters: BTreeMap<LocaleId, BTreeMap<String, BTreeSet<String>>>,
    chain: Vec<LocaleId>,
}

impl Localization {
    /// Prepare all catalogs and select a locale with an ordered explicit fallback.
    ///
    /// # Errors
    /// Rejects duplicate catalogs, missing lookup locales and formatter failures.
    pub fn new(
        catalogs: impl IntoIterator<Item = CatalogAsset>,
        locale: LocaleId,
        fallback: Vec<LocaleId>,
    ) -> Result<Self, LocalizationError> {
        let mut service = Self {
            catalogs: BTreeMap::new(),
            numeric_parameters: BTreeMap::new(),
            chain: Vec::new(),
        };
        for catalog in catalogs {
            service.install_catalog(catalog)?;
        }
        service.select_locale(locale, fallback)?;
        Ok(service)
    }

    /// Install a new locale. Existing catalogs require `replace_catalog`.
    ///
    /// # Errors
    /// Duplicate locale or formatter failure leaves the installed set unchanged.
    pub fn install_catalog(&mut self, catalog: CatalogAsset) -> Result<(), LocalizationError> {
        if self.catalogs.contains_key(catalog.locale()) {
            return Err(LocalizationError::DuplicateCatalog(
                catalog.locale().clone(),
            ));
        }
        let bundle = prepare(&catalog)?;
        self.numeric_parameters
            .insert(catalog.locale.clone(), catalog.numeric_parameters);
        self.catalogs.insert(catalog.locale, bundle);
        Ok(())
    }

    /// Atomically replace an installed locale after preparing a validated asset.
    /// Existing returned strings and asset clones remain valid.
    ///
    /// # Errors
    /// Missing locale or preparation failure preserves the old catalog and selection.
    pub fn replace_catalog(&mut self, catalog: CatalogAsset) -> Result<(), LocalizationError> {
        if !self.catalogs.contains_key(catalog.locale()) {
            return Err(LocalizationError::MissingLocale(catalog.locale().clone()));
        }
        let bundle = prepare(&catalog)?;
        self.numeric_parameters
            .insert(catalog.locale.clone(), catalog.numeric_parameters);
        self.catalogs.insert(catalog.locale, bundle);
        Ok(())
    }

    /// Change the lookup chain atomically. No implicit parent-language matching.
    ///
    /// # Errors
    /// Unknown/repeated locales leave the previous selection intact.
    pub fn select_locale(
        &mut self,
        locale: LocaleId,
        fallback: Vec<LocaleId>,
    ) -> Result<(), LocalizationError> {
        let chain: Vec<_> = std::iter::once(locale).chain(fallback).collect();
        let mut seen = BTreeSet::new();
        for locale in &chain {
            if !self.catalogs.contains_key(locale) {
                return Err(LocalizationError::MissingLocale(locale.clone()));
            }
            if !seen.insert(locale) {
                return Err(LocalizationError::DuplicateLocale(locale.clone()));
            }
        }
        self.chain = chain;
        Ok(())
    }

    /// Selected locale followed by its explicit ordered fallback catalogs.
    #[must_use]
    pub fn locale_chain(&self) -> &[LocaleId] {
        &self.chain
    }

    /// Resolve a message, using fallback only when its ID is absent.
    ///
    /// # Errors
    /// Missing IDs and resolver/parameter errors return contextual errors, never
    /// partial output. A broken selected translation does not silently fall back.
    pub fn format(
        &self,
        id: &MessageId,
        parameters: &MessageParameters,
    ) -> Result<LocalizedMessage, LocalizationError> {
        let mut arguments = FluentArgs::new();
        for (name, value) in &parameters.0 {
            match value {
                ParameterValue::Text(value) => arguments.set(name, value.as_str()),
                ParameterValue::Number(value) => arguments.set(name, *value),
            }
        }
        for locale in &self.chain {
            let bundle = &self.catalogs[locale];
            let Some(message) = bundle.get_message(id.as_str()) else {
                continue;
            };
            for name in &self.numeric_parameters[locale][id.as_str()] {
                if matches!(parameters.0.get(name), Some(ParameterValue::Text(_))) {
                    return Err(LocalizationError::Format {
                        locale: locale.clone(),
                        message: id.as_str().to_owned(),
                        diagnostics: format!(
                            "parameter '{name}' must be numeric because the catalog uses NUMBER"
                        ),
                    });
                }
            }
            let Some(pattern) = message.value() else {
                continue;
            };
            let mut errors = Vec::new();
            let text = bundle
                .format_pattern(pattern, Some(&arguments), &mut errors)
                .into_owned();
            if !errors.is_empty() {
                return Err(LocalizationError::Format {
                    locale: locale.clone(),
                    message: id.as_str().to_owned(),
                    diagnostics: format!("{errors:?}"),
                });
            }
            return Ok(LocalizedMessage {
                text,
                locale: locale.clone(),
            });
        }
        Err(LocalizationError::MissingMessage {
            message: id.as_str().to_owned(),
            locales: self.chain.clone(),
        })
    }

    /// Format a number in the selected locale with optional grouping and minimum
    /// fractional width (0..=6). Does not round or truncate supplied decimals.
    ///
    /// # Errors
    /// Rejects numbers outside the parameter contract and unsupported precision.
    pub fn format_number(
        &self,
        value: f64,
        minimum_fraction_digits: u8,
        grouping: bool,
    ) -> Result<String, LocalizationError> {
        numeric(&value.to_string()).map_err(|reason| parameter_error("number", &reason))?;
        if minimum_fraction_digits > 6 {
            return Err(parameter_error(
                "minimum_fraction_digits",
                "must be at most six",
            ));
        }
        let locale = &self.chain[0];
        let language = locale
            .as_str()
            .parse()
            .map_err(|error| formatter_error(locale, format!("{error}")))?;
        let formatter = NumberFormatter::construct(language, (grouping,))
            .map_err(|reason| formatter_error(locale, reason))?;
        let options = FluentNumberOptions {
            minimum_fraction_digits: Some(usize::from(minimum_fraction_digits)),
            ..Default::default()
        };
        formatter
            .format(&FluentNumber::new(value, options))
            .map_err(|reason| formatter_error(locale, reason))
    }
}

fn formatter_error(locale: &LocaleId, reason: String) -> LocalizationError {
    LocalizationError::Formatter {
        locale: locale.clone(),
        reason,
    }
}

fn prepare(catalog: &CatalogAsset) -> Result<FluentBundle<Arc<FluentResource>>, LocalizationError> {
    let language: unic_langid::LanguageIdentifier = catalog
        .locale
        .as_str()
        .parse()
        .map_err(|error| formatter_error(&catalog.locale, format!("{error}")))?;
    for grouping in [true, false] {
        NumberFormatter::construct(language.clone(), (grouping,))
            .map_err(|reason| formatter_error(&catalog.locale, reason))?;
    }
    let mut bundle = FluentBundle::new_concurrent(vec![language]);
    bundle.set_formatter(Some(format_value));
    bundle
        .add_builtins()
        .map_err(|error| formatter_error(&catalog.locale, format!("{error:?}")))?;
    bundle
        .add_resource(Arc::clone(&catalog.resource))
        .map_err(|errors| formatter_error(&catalog.locale, format!("{errors:?}")))?;
    Ok(bundle)
}
