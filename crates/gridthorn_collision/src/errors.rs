use thiserror::Error;

/// Errors returned while constructing a 2D collider.
#[derive(Clone, Copy, Debug, Error, PartialEq)]
pub enum ColliderError {
    /// A position or dimension contains `NaN` or infinity.
    #[error("collider coordinates and dimensions must be finite")]
    NonFinite,
    /// An axis-aligned box half extent is negative.
    #[error("axis-aligned box half extents must be non-negative")]
    NegativeHalfExtent,
    /// A circle radius is negative.
    #[error("circle radius must be non-negative")]
    NegativeRadius,
}
