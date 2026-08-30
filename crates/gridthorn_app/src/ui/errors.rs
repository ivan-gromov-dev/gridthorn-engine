use thiserror::Error;

/// Failure to construct a runtime UI interaction target.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum UiButtonError {
    /// Positions must contain finite pixel coordinates.
    #[error("UI button position must contain finite pixel coordinates")]
    InvalidPosition,
    /// Dimensions must be finite and greater than zero.
    #[error("UI button size must contain finite positive dimensions")]
    InvalidSize,
}
