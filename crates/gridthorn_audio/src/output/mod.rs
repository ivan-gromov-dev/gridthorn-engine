mod errors;

use std::{collections::HashMap, sync::Arc};

use kira::{
    AudioManager, AudioManagerSettings, Decibels, Frame, Tween,
    backend::{Backend, DefaultBackend},
    sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
};

pub use errors::AudioOutputError;

use crate::{AudioClip, AudioCommand, AudioCommandQueue, AudioVoiceId};

/// Native audio output service backed by Kira's mixer and platform adapter.
///
/// Constructing this service is the only audio operation that requires an
/// output device. Decoding clips and queueing commands remain headless-safe.
pub struct AudioOutput {
    backend: OutputBackend<DefaultBackend>,
}

impl AudioOutput {
    /// Connect to the default operating-system audio output device.
    ///
    /// # Errors
    ///
    /// Returns [`AudioOutputError::DeviceInitialization`] if an output stream
    /// cannot be created.
    pub fn new() -> Result<Self, AudioOutputError> {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
            .map_err(|error| AudioOutputError::DeviceInitialization(error.to_string()))?;
        Ok(Self {
            backend: OutputBackend::new(manager),
        })
    }

    /// Consume queued playback commands in FIFO order.
    ///
    /// # Errors
    ///
    /// Returns an error when a clip layout is unsupported or the mixer cannot
    /// allocate a requested voice.
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

    /// Number of voices currently controlled by the output service.
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
        for command in commands.drain() {
            self.apply(command)?;
        }
        Ok(())
    }

    fn apply(&mut self, command: AudioCommand) -> Result<(), AudioOutputError> {
        match command {
            AudioCommand::Play {
                voice,
                clip,
                settings,
            } => {
                let sound = sound_data(&clip, settings.volume(), settings.looping())?;
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
        self.voices.len()
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
