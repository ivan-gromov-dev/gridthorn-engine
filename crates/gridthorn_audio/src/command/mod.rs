mod errors;

use std::collections::VecDeque;

pub use errors::{AudioQueueError, AudioSettingsError};

use crate::AudioClip;

/// Engine-owned identifier for one requested playback voice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AudioVoiceId(u64);

impl AudioVoiceId {
    /// Numeric identifier used by diagnostics and backend adapters.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Validated settings for a new playback voice.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlaybackSettings {
    volume: f32,
    looping: bool,
}

impl PlaybackSettings {
    /// Construct playback settings with a normalized linear volume.
    ///
    /// # Errors
    ///
    /// Returns [`AudioSettingsError::InvalidVolume`] unless volume is finite
    /// and inside `0.0..=1.0`.
    pub fn new(volume: f32, looping: bool) -> Result<Self, AudioSettingsError> {
        validate_volume(volume)?;
        Ok(Self { volume, looping })
    }

    /// Normalized linear playback gain.
    #[must_use]
    pub const fn volume(self) -> f32 {
        self.volume
    }

    /// Whether playback restarts after its final frame.
    #[must_use]
    pub const fn looping(self) -> bool {
        self.looping
    }
}

impl Default for PlaybackSettings {
    fn default() -> Self {
        Self {
            volume: 1.0,
            looping: false,
        }
    }
}

/// Ordered request consumed later by an audio output service.
#[derive(Clone, Debug, PartialEq)]
pub enum AudioCommand {
    /// Start a decoded clip as a new voice.
    Play {
        /// Allocated voice identifier.
        voice: AudioVoiceId,
        /// Backend-independent decoded samples.
        clip: AudioClip,
        /// Initial playback settings.
        settings: PlaybackSettings,
    },
    /// Stop a voice if it is still active.
    Stop {
        /// Voice to stop.
        voice: AudioVoiceId,
    },
    /// Change a voice's normalized linear gain.
    SetVolume {
        /// Voice to update.
        voice: AudioVoiceId,
        /// New normalized linear gain.
        volume: f32,
    },
}

/// FIFO boundary between game systems and a future output backend.
pub struct AudioCommandQueue {
    next_voice: u64,
    commands: VecDeque<AudioCommand>,
}

impl Default for AudioCommandQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl AudioCommandQueue {
    /// Create an empty queue.
    #[must_use]
    pub fn new() -> Self {
        Self {
            next_voice: 1,
            commands: VecDeque::new(),
        }
    }

    /// Queue playback and return its voice identifier immediately.
    ///
    /// # Errors
    ///
    /// Returns [`AudioQueueError::VoiceIdsExhausted`] after the identifier
    /// namespace is exhausted.
    pub fn play(
        &mut self,
        clip: AudioClip,
        settings: PlaybackSettings,
    ) -> Result<AudioVoiceId, AudioQueueError> {
        let voice = AudioVoiceId(self.next_voice);
        self.next_voice = self
            .next_voice
            .checked_add(1)
            .ok_or(AudioQueueError::VoiceIdsExhausted)?;
        self.commands.push_back(AudioCommand::Play {
            voice,
            clip,
            settings,
        });
        Ok(voice)
    }

    /// Queue an idempotent stop request.
    pub fn stop(&mut self, voice: AudioVoiceId) {
        self.commands.push_back(AudioCommand::Stop { voice });
    }

    /// Queue a normalized volume change.
    ///
    /// # Errors
    ///
    /// Returns [`AudioSettingsError::InvalidVolume`] for an invalid gain.
    pub fn set_volume(
        &mut self,
        voice: AudioVoiceId,
        volume: f32,
    ) -> Result<(), AudioSettingsError> {
        validate_volume(volume)?;
        self.commands
            .push_back(AudioCommand::SetVolume { voice, volume });
        Ok(())
    }

    /// Number of commands awaiting a backend service.
    #[must_use]
    pub fn len(&self) -> usize {
        self.commands.len()
    }

    /// Whether no commands await a backend service.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.commands.is_empty()
    }

    /// Remove all queued commands in request order.
    #[must_use]
    pub fn drain(&mut self) -> Vec<AudioCommand> {
        self.commands.drain(..).collect()
    }
}

fn validate_volume(volume: f32) -> Result<(), AudioSettingsError> {
    if volume.is_finite() && (0.0..=1.0).contains(&volume) {
        Ok(())
    } else {
        Err(AudioSettingsError::InvalidVolume)
    }
}

#[cfg(test)]
mod test;
