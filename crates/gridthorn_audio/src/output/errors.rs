use thiserror::Error;

/// Failure while starting or driving the native audio output service.
#[derive(Debug, Error)]
pub enum AudioOutputError {
    /// The operating system did not provide a usable output stream.
    #[error("failed to initialize the audio output device: {0}")]
    DeviceInitialization(String),
    /// The clip layout cannot be represented by the provisional stereo mixer.
    #[error("audio output supports mono or stereo clips, not {channels} channels")]
    UnsupportedChannelCount {
        /// Channel count found in the decoded clip.
        channels: u16,
    },
    /// The mixer has no remaining capacity for a requested voice.
    #[error("audio mixer could not start voice {voice}: {reason}")]
    Play {
        /// Engine-owned identifier associated with the failed command.
        voice: u64,
        /// Backend diagnostic kept behind the engine error contract.
        reason: String,
    },
}
