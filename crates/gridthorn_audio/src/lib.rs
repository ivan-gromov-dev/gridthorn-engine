//! Provisional engine-owned audio assets and playback commands.

mod clip;
mod command;

pub use clip::{AudioClip, AudioClipError, WavDecodeError};
pub use command::{
    AudioCommand, AudioCommandQueue, AudioQueueError, AudioSettingsError, AudioVoiceId,
    PlaybackSettings,
};
