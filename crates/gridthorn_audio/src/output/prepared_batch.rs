use std::sync::Arc;

use kira::{
    Frame,
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

use super::{AudioClip, AudioOutputError, linear_gain, sound_data};

const MAX_ENTRIES: usize = 16;
const MAX_FRAME_BYTES: usize = 1024 * 1024;

/// Reuses conversions only within one command batch, retaining at most 1 MiB of frames.
/// Stored source clips keep sample addresses valid until the batch is dropped.
#[derive(Default)]
pub(super) struct PreparedBatch {
    entries: Vec<(AudioClip, Arc<[Frame]>)>,
    frame_bytes: usize,
}

impl PreparedBatch {
    pub(super) fn sound(
        &mut self,
        clip: &AudioClip,
        volume: f32,
        looping: bool,
    ) -> Result<StaticSoundData, AudioOutputError> {
        if let Some((_, frames)) = self.entries.iter().find(|(source, _)| {
            source.samples().as_ptr() == clip.samples().as_ptr()
                && source.samples().len() == clip.samples().len()
                && source.channels() == clip.channels()
                && source.sample_rate() == clip.sample_rate()
        }) {
            let mut settings = StaticSoundSettings::default().volume(linear_gain(volume));
            if looping {
                settings = settings.loop_region(..);
            }
            return Ok(StaticSoundData {
                sample_rate: clip.sample_rate(),
                frames: Arc::clone(frames),
                settings,
                slice: None,
            });
        }
        let sound = sound_data(clip, volume, looping)?;
        let bytes = std::mem::size_of_val(sound.frames.as_ref());
        if self.entries.len() < MAX_ENTRIES && bytes <= MAX_FRAME_BYTES - self.frame_bytes {
            self.entries.push((clip.clone(), Arc::clone(&sound.frames)));
            self.frame_bytes += bytes;
        }
        Ok(sound)
    }
}
