/// Presentation request validation and capability failures.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum PresentationError {
    /// Caps must fit a nonzero monotonic nanosecond interval.
    #[error("frame-rate limit must be between 1 and 1000000000 FPS, received {0}")]
    InvalidFrameRate(u32),
    /// Renderer-free windows cannot configure GPU presentation.
    #[error("GPU presentation is unavailable for this window")]
    Unavailable,
    /// Explicit policies never silently fall back to another mode.
    #[error("present mode {0:?} is unsupported by this surface/adapter")]
    UnsupportedMode(super::PresentMode),
    /// Request correlation cannot wrap.
    #[error("presentation request identifiers are exhausted")]
    RequestIdsExhausted,
}
