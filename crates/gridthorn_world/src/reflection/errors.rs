use thiserror::Error;

/// Contextual failures from reflection registration and inspection.
#[derive(Debug, Error, Eq, PartialEq)]
pub enum ReflectionError {
    /// Type or field names are empty or contain whitespace.
    #[error("reflection type '{type_name}' has invalid identifier '{name}'")]
    InvalidName { type_name: String, name: String },
    /// A type identifier or Rust type was already registered for this role.
    #[error("reflection type '{0}' is already registered for this role")]
    DuplicateType(String),
    /// Field names must be unique within a type.
    #[error("reflection type '{type_name}' repeats field '{field}'")]
    DuplicateField { type_name: String, field: String },
    /// The requested type is not registered for this role.
    #[error("reflection type '{0}' is not registered for this role")]
    UnknownType(String),
    /// A callback returned values inconsistent with registered metadata.
    #[error("reflection type '{0}' returned values inconsistent with its field metadata")]
    InvalidValues(String),
}
