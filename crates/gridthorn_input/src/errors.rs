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
/// Recoverable text-session failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TextInputError {
    /// Text sessions require a focused window.
    Unfocused,
    /// Candidate anchor contains nonfinite values or negative extents.
    InvalidCursorArea,
}

impl std::fmt::Display for TextInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unfocused => "text input requires a focused window",
            Self::InvalidCursorArea => {
                "IME cursor area requires finite coordinates and nonnegative extents"
            }
        })
    }
}
impl std::error::Error for TextInputError {}

/// Recoverable plain-text clipboard failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClipboardError {
    /// Clipboard access requires a focused window.
    Unfocused,
    /// No text content is available.
    NoText,
    /// Native clipboard initialization or operation failed.
    Platform {
        /// Contextual backend diagnostic.
        message: String,
    },
}
impl std::fmt::Display for ClipboardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unfocused => f.write_str("clipboard access requires a focused window"),
            Self::NoText => f.write_str("clipboard contains no available text"),
            Self::Platform { message } => write!(f, "clipboard access failed: {message}"),
        }
    }
}
impl std::error::Error for ClipboardError {}
