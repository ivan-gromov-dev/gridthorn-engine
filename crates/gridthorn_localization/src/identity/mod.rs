use std::fmt;

mod errors;
pub use errors::LocalizationIdError;

/// Canonical Unicode language identifier (language, optional script/region/variants).
/// Extensions and private-use tags are outside this provisional catalog contract.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LocaleId(String);

impl LocaleId {
    /// Parse and canonicalize a locale without consulting the operating system.
    ///
    /// # Errors
    /// Rejects malformed identifiers, extensions and private-use tags.
    pub fn new(value: &str) -> Result<Self, LocalizationIdError> {
        let locale: unic_langid::LanguageIdentifier = value
            .parse()
            .map_err(|_| LocalizationIdError::Locale(value.to_owned()))?;
        Ok(Self(locale.to_string()))
    }

    /// Canonical identifier used for exact catalog lookup.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for LocaleId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

/// Case-sensitive Fluent message identifier, independent of its translated text.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId(String);

impl MessageId {
    /// Validate an ASCII letter followed by letters, digits, hyphens or underscores.
    ///
    /// # Errors
    /// Rejects empty identifiers, attributes, term prefixes and invalid characters.
    pub fn new(value: &str) -> Result<Self, LocalizationIdError> {
        if !valid_identifier(value) {
            return Err(LocalizationIdError::Message(value.to_owned()));
        }
        Ok(Self(value.to_owned()))
    }

    /// Original case-sensitive identifier.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub(crate) fn valid_identifier(value: &str) -> bool {
    value
        .as_bytes()
        .first()
        .is_some_and(u8::is_ascii_alphabetic)
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}
