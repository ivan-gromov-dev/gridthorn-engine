use std::path::PathBuf;

/// Catalog decoding or static validation failure; no partial asset is returned.
#[derive(Debug, thiserror::Error)]
pub enum CatalogError {
    /// File could not be read.
    #[error("could not read localization catalog '{}': {source}", path.display())]
    Read {
        /// Attempted file path.
        path: PathBuf,
        /// Operating-system failure.
        source: std::io::Error,
    },
    /// Resource bound exceeded before parsing.
    #[error("localization catalog exceeds the 1 MiB limit")]
    TooLarge,
    /// Catalog is not UTF-8.
    #[error("localization catalog is not UTF-8: {0}")]
    Encoding(String),
    /// Fluent parser rejected the catalog.
    #[error("invalid Fluent catalog for {locale}: {diagnostics}")]
    Syntax {
        /// Declared catalog language.
        locale: String,
        /// Parser diagnostics with byte offsets.
        diagnostics: String,
    },
    /// A valid Fluent construct violates the supported runtime contract.
    #[error("invalid catalog for {locale}, entry '{entry}': {reason}")]
    Validation {
        /// Declared catalog language.
        locale: String,
        /// Message or term responsible for the failure.
        entry: String,
        /// Actionable validation diagnostic.
        reason: String,
    },
}
