use crate::{AudioBusId, AudioBusSettings, AudioVoiceId};
use kira::{Tween, sound::static_sound::StaticSoundHandle};
use std::collections::HashMap;

#[derive(Default)]
pub(super) struct BusMixer {
    settings: HashMap<AudioBusId, AudioBusSettings>,
    voices: HashMap<AudioVoiceId, (AudioBusId, f32)>,
}
impl BusMixer {
    pub(super) fn gain(&self, bus: AudioBusId, volume: f32) -> f32 {
        let master = self
            .settings
            .get(&AudioBusId::Master)
            .copied()
            .unwrap_or_default()
            .effective_gain();
        let category = if bus == AudioBusId::Master {
            1.0
        } else {
            self.settings
                .get(&bus)
                .copied()
                .unwrap_or_default()
                .effective_gain()
        };
        volume * master * category
    }
    pub(super) fn insert_voice(&mut self, voice: AudioVoiceId, bus: AudioBusId, volume: f32) {
        self.voices.insert(voice, (bus, volume));
    }
    pub(super) fn remove_voice(&mut self, voice: AudioVoiceId) {
        self.voices.remove(&voice);
    }
    pub(super) fn retain(&mut self, voices: &HashMap<AudioVoiceId, StaticSoundHandle>) {
        self.voices.retain(|voice, _| voices.contains_key(voice));
    }
    pub(super) fn set_voice_volume(&mut self, voice: AudioVoiceId, volume: f32) -> f32 {
        let Some((bus, stored)) = self.voices.get_mut(&voice) else {
            return volume;
        };
        *stored = volume;
        let bus = *bus;
        self.gain(bus, volume)
    }
    pub(super) fn set(
        &mut self,
        bus: AudioBusId,
        settings: AudioBusSettings,
        voices: &mut HashMap<AudioVoiceId, StaticSoundHandle>,
    ) {
        if settings == AudioBusSettings::default() {
            self.settings.remove(&bus);
        } else {
            self.settings.insert(bus, settings);
        }
        for (voice, (route, volume)) in &self.voices {
            if (bus == AudioBusId::Master || bus == *route)
                && let Some(handle) = voices.get_mut(voice)
            {
                handle.set_volume(
                    super::linear_gain(self.gain(*route, *volume)),
                    Tween::default(),
                );
            }
        }
    }
}
