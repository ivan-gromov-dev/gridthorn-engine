/// Invalid public localization identifiers.
#[derive(Debug, thiserror::Error)]
pub enum LocalizationIdError {
    /// Unsupported or malformed language identifier.
    #[error("invalid localization locale '{0}'")]
    Locale(String),
    /// Malformed message identifier.
    #[error("invalid localization message ID '{0}'")]
    Message(String),
}
