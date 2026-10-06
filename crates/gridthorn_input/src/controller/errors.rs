/// Recoverable controller discovery or feedback failure.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ControllerError {
    /// Window is unfocused; feedback is not submitted in the background.
    #[error("controller feedback requires window focus")]
    Unfocused,
    /// Device connection no longer exists.
    #[error("controller is disconnected")]
    Disconnected,
    /// Backend does not support requested feedback.
    #[error("controller rumble is unsupported")]
    Unsupported,
    /// Magnitudes/duration or dead-zone threshold are invalid.
    #[error("invalid controller parameters")]
    InvalidParameters,
    /// Native backend failed with context.
    #[error("controller backend: {0}")]
    Platform(String),
}
