use thiserror::Error;

/// Failure to construct a normalized sprite-sheet region.
#[derive(Debug, Error, PartialEq)]
pub enum SpriteRegionError {
    /// Both corners must be finite, ordered, and inside normalized texture space.
    #[error("sprite region must have finite ordered bounds inside 0.0..=1.0")]
    InvalidBounds,
}
