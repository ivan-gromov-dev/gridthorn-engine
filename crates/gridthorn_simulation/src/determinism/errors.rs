use thiserror::Error;

/// Invalid registration or access to a named authoritative random stream.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum RandomStreamError {
    /// Names must contain non-whitespace text.
    #[error("random stream name must not be empty or have surrounding whitespace")]
    InvalidName,
    /// A registered stream must never be implicitly reseeded.
    #[error("random stream `{0}` is already registered")]
    Duplicate(String),
    /// Consumption requires explicit registration.
    #[error("random stream `{0}` is not registered")]
    Missing(String),
}
