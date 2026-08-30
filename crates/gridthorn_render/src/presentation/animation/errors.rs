use thiserror::Error;

/// Failure to construct a 2D animation clip.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AnimationClipError {
    /// A clip needs at least one sprite-sheet region.
    #[error("animation clip must contain at least one frame")]
    Empty,
    /// Uniform frame duration must advance time.
    #[error("animation frame duration must be greater than zero")]
    ZeroFrameDuration,
}
