use thiserror::Error;

/// Invalid projection configuration or coordinate conversion.
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum GridError {
    /// Dimensions must be finite and strictly positive.
    #[error("grid tile width and height must be finite and strictly positive")]
    InvalidDimensions,
    /// The origin, input point, or conversion result is not finite.
    #[error("grid projection origin, point, and conversion results must be finite")]
    NonFinite,
    /// The inverse result cannot identify a signed 32-bit cell.
    #[error("grid projection point lies outside the signed 32-bit cell range")]
    CellOutOfRange,
}
