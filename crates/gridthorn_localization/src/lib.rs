//! Provisional headless presentation localization, independent of simulation.

mod catalog;
mod identity;
mod runtime;

pub use catalog::{CatalogAsset, CatalogError};
pub use identity::{LocaleId, LocalizationIdError, MessageId};
pub use runtime::{Localization, LocalizationError, LocalizedMessage, MessageParameters};
