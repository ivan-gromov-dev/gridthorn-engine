use crate::LocaleId;

/// Recoverable localization failures; formatting never returns partial text.
#[derive(Debug, thiserror::Error)]
pub enum LocalizationError {
    /// A configured lookup locale has no installed catalog.
    #[error("no localization catalog installed for {0}")]
    MissingLocale(LocaleId),
    /// Fallback configuration contains a repeated locale.
    #[error("duplicate locale in localization lookup chain: {0}")]
    DuplicateLocale(LocaleId),
    /// A catalog is already installed; replacement must be explicit.
    #[error("localization catalog already installed for {0}")]
    DuplicateCatalog(LocaleId),
    /// Message was absent from the entire lookup chain.
    #[error("message '{message}' is absent from localization chain {locales:?}")]
    MissingMessage {
        /// Requested message ID.
        message: String,
        /// Exact lookup order.
        locales: Vec<LocaleId>,
    },
    /// Invalid parameter name or value.
    #[error("invalid localization parameter '{name}': {reason}")]
    Parameter {
        /// Parameter name.
        name: String,
        /// Validation diagnostic.
        reason: String,
    },
    /// Backend construction or number conversion failed.
    #[error("localization formatter for {locale} failed: {reason}")]
    Formatter {
        /// Effective locale.
        locale: LocaleId,
        /// Backend diagnostic, without exposing implementation types.
        reason: String,
    },
    /// The resolved message could not be formatted.
    #[error("message '{message}' in {locale} could not be formatted: {diagnostics}")]
    Format {
        /// Resolved catalog locale.
        locale: LocaleId,
        /// Requested message ID.
        message: String,
        /// Missing variables, type errors or resolver diagnostics.
        diagnostics: String,
    },
}
