use std::{
    collections::{BTreeMap, BTreeSet},
    io::Read,
    path::Path,
    sync::Arc,
};

use fluent_bundle::FluentResource;

use crate::LocaleId;

mod bounds;
mod errors;
pub(crate) mod validation;
pub use errors::CatalogError;

/// Immutable validated UTF-8 Fluent catalog for one explicitly declared locale.
/// Clones share data. Loading starts no threads and does not publish runtime state.
#[derive(Clone, Debug)]
pub struct CatalogAsset {
    pub(crate) locale: LocaleId,
    pub(crate) resource: Arc<FluentResource>,
    messages: BTreeSet<String>,
    pub(crate) numeric_parameters: BTreeMap<String, BTreeSet<String>>,
}

impl CatalogAsset {
    /// Validate syntax, identifiers, references, cycles and supported expressions.
    ///
    /// # Errors
    /// Rejects catalogs over 1 MiB, invalid Fluent and unsupported constructs.
    pub fn from_source(locale: LocaleId, source: String) -> Result<Self, CatalogError> {
        if source.len() > 1024 * 1024 {
            return Err(CatalogError::TooLarge);
        }
        bounds::validate_parser_depth(&locale, &source)?;
        let resource =
            FluentResource::try_new(source).map_err(|(_, errors)| CatalogError::Syntax {
                locale: locale.to_string(),
                diagnostics: format!("{errors:?}"),
            })?;
        let numeric_parameters = validation::validate(&locale, &resource)?;
        let messages = numeric_parameters.keys().cloned().collect();
        Ok(Self {
            locale,
            resource: Arc::new(resource),
            messages,
            numeric_parameters,
        })
    }

    /// Read at most 1 MiB of UTF-8 data and validate it before returning an asset.
    ///
    /// # Errors
    /// Reports path-specific I/O errors, encoding, syntax and validation failures.
    pub fn load(locale: LocaleId, path: impl AsRef<Path>) -> Result<Self, CatalogError> {
        let path = path.as_ref();
        let file = std::fs::File::open(path).map_err(|source| CatalogError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        let mut bytes = Vec::new();
        file.take(1024 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|source| CatalogError::Read {
                path: path.to_path_buf(),
                source,
            })?;
        if bytes.len() > 1024 * 1024 {
            return Err(CatalogError::TooLarge);
        }
        let source =
            String::from_utf8(bytes).map_err(|error| CatalogError::Encoding(error.to_string()))?;
        Self::from_source(locale, source)
    }

    /// Locale used for lookup, plural rules and number formatting.
    #[must_use]
    pub fn locale(&self) -> &LocaleId {
        &self.locale
    }

    /// Sorted catalog message IDs; translations need not contain identical IDs.
    pub fn message_ids(&self) -> impl Iterator<Item = &str> {
        self.messages.iter().map(String::as_str)
    }
}

#[cfg(test)]
mod test;
