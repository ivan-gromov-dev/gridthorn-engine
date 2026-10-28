#[cfg(feature = "native-output")]
mod devices;
mod errors;
#[cfg(feature = "native-output")]
mod native_backend;
mod prepared_batch;

use std::{collections::HashMap, sync::Arc};

use kira::{
    AudioManager, Decibels, Frame, Tween,
    backend::Backend,
    sound::PlaybackState,
    sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
};

pub use errors::AudioOutputError;

use crate::{AudioClip, AudioCommand, AudioCommandQueue, AudioVoiceId};

/// Native audio output service backed by Kira's mixer and platform adapter.
///
/// Native device access is explicit. Decoding clips and queueing commands
/// remain headless-safe.
/// Stream release follows the native host contract. Device enumeration and
/// refresh occur only through explicit calls, never during command processing.
#[cfg(feature = "native-output")]
pub struct AudioOutput {
    backend: OutputBackend<native_backend::NativeBackend>,
    selection: crate::devices::AudioDeviceSelection,
    device: crate::devices::AudioDeviceId,
}

#[cfg(feature = "native-output")]
impl AudioOutput {
    /// Connect to the default operating-system audio output device.
    ///
    /// # Errors
    ///
    /// Returns [`AudioOutputError::Device`] if an output stream
    /// cannot be created.
    pub fn new() -> Result<Self, AudioOutputError> {
        Self::with_device(crate::devices::AudioDeviceSelection::Default)
    }

    /// Consume queued playback commands in FIFO order and release completed voices.
    ///
    /// # Errors
    ///
    /// Returns a playback error when a clip layout is unsupported or the mixer
    /// cannot allocate a voice. This does not query devices or recover streams.
    pub fn process(&mut self, commands: &mut AudioCommandQueue) -> Result<(), AudioOutputError> {
        self.backend.process(commands)
    }

    /// Pause every active voice without discarding its playback position.
    pub fn suspend(&mut self) {
        self.backend.suspend();
    }

    /// Resume voices paused by [`Self::suspend`].
    pub fn resume(&mut self) {
        self.backend.resume();
    }

    /// Number of controlled voices that have not finished, including paused voices.
    #[must_use]
    pub fn active_voice_count(&self) -> usize {
        self.backend.active_voice_count()
    }
}

struct OutputBackend<B: Backend> {
    manager: AudioManager<B>,
    voices: HashMap<AudioVoiceId, StaticSoundHandle>,
    suspended: bool,
}

impl<B: Backend> OutputBackend<B> {
    fn new(manager: AudioManager<B>) -> Self {
        Self {
            manager,
            voices: HashMap::new(),
            suspended: false,
        }
    }

    fn process(&mut self, commands: &mut AudioCommandQueue) -> Result<(), AudioOutputError> {
        self.voices
            .retain(|_, handle| handle.state() != PlaybackState::Stopped);
        let mut prepared = prepared_batch::PreparedBatch::default();
        for command in commands.drain() {
            self.apply(command, &mut prepared)?;
        }
        Ok(())
    }

    fn apply(
        &mut self,
        command: AudioCommand,
        prepared: &mut prepared_batch::PreparedBatch,
    ) -> Result<(), AudioOutputError> {
        match command {
            AudioCommand::Play {
                voice,
                clip,
                settings,
            } => {
                let sound = prepared.sound(&clip, settings.volume(), settings.looping())?;
                let mut handle =
                    self.manager
                        .play(sound)
                        .map_err(|error| AudioOutputError::Play {
                            voice: voice.get(),
                            reason: error.to_string(),
                        })?;
                if self.suspended {
                    handle.pause(Tween::default());
                }
                self.voices.insert(voice, handle);
            }
            AudioCommand::Stop { voice } => {
                if let Some(mut handle) = self.voices.remove(&voice) {
                    handle.stop(Tween::default());
                }
            }
            AudioCommand::SetVolume { voice, volume } => {
                if let Some(handle) = self.voices.get_mut(&voice) {
                    handle.set_volume(linear_gain(volume), Tween::default());
                }
            }
        }
        Ok(())
    }

    fn suspend(&mut self) {
        if self.suspended {
            return;
        }
        self.suspended = true;
        for handle in self.voices.values_mut() {
            handle.pause(Tween::default());
        }
    }

    fn resume(&mut self) {
        if !self.suspended {
            return;
        }
        self.suspended = false;
        for handle in self.voices.values_mut() {
            handle.resume(Tween::default());
        }
    }

    fn active_voice_count(&self) -> usize {
        self.voices
            .values()
            .filter(|handle| handle.state() != PlaybackState::Stopped)
            .count()
    }
}

fn sound_data(
    clip: &AudioClip,
    volume: f32,
    looping: bool,
) -> Result<StaticSoundData, AudioOutputError> {
    let frames = match clip.channels() {
        1 => clip
            .samples()
            .iter()
            .map(|sample| Frame::new(*sample, *sample))
            .collect::<Vec<_>>(),
        2 => clip
            .samples()
            .as_chunks::<2>()
            .0
            .iter()
            .map(|samples| Frame::new(samples[0], samples[1]))
            .collect::<Vec<_>>(),
        channels => return Err(AudioOutputError::UnsupportedChannelCount { channels }),
    };
    let mut settings = StaticSoundSettings::default().volume(linear_gain(volume));
    if looping {
        settings = settings.loop_region(..);
    }
    Ok(StaticSoundData {
        sample_rate: clip.sample_rate(),
        frames: Arc::from(frames),
        settings,
        slice: None,
    })
}

fn linear_gain(gain: f32) -> Decibels {
    if gain == 0.0 {
        Decibels::SILENCE
    } else {
        Decibels(20.0 * gain.log10())
    }
}

#[cfg(test)]
mod test;
