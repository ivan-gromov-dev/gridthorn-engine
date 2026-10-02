/// Recoverable native pointer capture failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PointerCaptureError {
    /// Capture was requested without a focused window.
    Unfocused,
    /// The platform rejected a capture mode.
    Platform {
        /// Native diagnostic, including unsupported capabilities.
        message: String,
    },
}

impl std::fmt::Display for PointerCaptureError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unfocused => formatter.write_str("pointer capture requires a focused window"),
            Self::Platform { message } => write!(formatter, "pointer capture failed: {message}"),
        }
    }
}

impl std::error::Error for PointerCaptureError {}
