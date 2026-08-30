//! Provisional engine-owned audio assets and playback commands.

mod clip;
mod command;
#[cfg(any(feature = "native-output", test))]
mod output;

pub use clip::{AudioClip, AudioClipError, WavDecodeError};
pub use command::{
    AudioCommand, AudioCommandQueue, AudioQueueError, AudioSettingsError, AudioVoiceId,
    PlaybackSettings,
};
#[cfg(feature = "native-output")]
pub use output::{AudioOutput, AudioOutputError};
