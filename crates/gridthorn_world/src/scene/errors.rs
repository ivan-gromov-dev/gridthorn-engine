use thiserror::Error;

/// Failure to construct a scene identifier.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum SceneIdError {
    /// Scene identifiers must contain at least one non-whitespace character.
    #[error("scene identifier must not be empty")]
    EmptyIdentifier,
}
