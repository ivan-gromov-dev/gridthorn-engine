use thiserror::Error;

/// Failure to validate playback settings.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AudioSettingsError {
    /// Volume is a finite linear gain in the inclusive range zero to one.
    #[error("audio volume must be finite and inside 0.0..=1.0")]
    InvalidVolume,
}

/// Failure to allocate an ordered audio command.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AudioQueueError {
    /// No voice identifier remains in the provisional 64-bit namespace.
    #[error("audio voice identifier space is exhausted")]
    VoiceIdsExhausted,
}
