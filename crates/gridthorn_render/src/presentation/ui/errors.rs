use thiserror::Error;

/// Failure to construct a screen-space UI primitive.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum UiError {
    /// Positions must contain finite pixel coordinates.
    #[error("UI position must contain finite pixel coordinates")]
    InvalidPosition,
    /// Rectangle dimensions must be finite and greater than zero.
    #[error("UI rectangle size must contain finite positive dimensions")]
    InvalidSize,
    /// Bitmap pixels must have a finite positive display scale.
    #[error("text pixel scale must be finite and greater than zero")]
    InvalidPixelScale,
}
