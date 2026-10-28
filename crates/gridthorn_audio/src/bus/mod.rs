//! Flat caller-defined category buses feeding master.
//!
//! ```
//! use gridthorn_audio::{AudioBusId, AudioBusSettings, AudioCommandQueue, PlaybackSettings};
//! let category = AudioBusId::Category(42);
//! let mut commands = AudioCommandQueue::new();
//! let controls = AudioBusSettings::new(0.5, false)?;
//! commands.set_bus(category, controls.with_muted(true));
//! let playback = PlaybackSettings::default().with_bus(category);
//! assert_eq!(playback.bus(), category);
//! # Ok::<(), gridthorn_audio::AudioSettingsError>(())
//! ```
use crate::AudioSettingsError;

/// Routing identity; categories require no registration or engine-owned names.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AudioBusId {
    /// Final bus applied to every voice exactly once.
    #[default]
    Master,
    /// Caller-defined category feeding master.
    Category(u64),
}

/// Independent normalized linear gain and mute controls.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioBusSettings {
    gain: f32,
    muted: bool,
}
impl AudioBusSettings {
    /// Construct controls with finite gain inside `0.0..=1.0`.
    ///
    /// # Errors
    /// Returns [`AudioSettingsError::InvalidVolume`] for invalid gain.
    pub fn new(gain: f32, muted: bool) -> Result<Self, AudioSettingsError> {
        crate::command::validate_volume(gain)?;
        Ok(Self { gain, muted })
    }
    /// Configured gain, preserved while muted.
    #[must_use]
    pub const fn gain(self) -> f32 {
        self.gain
    }
    /// Whether this bus suppresses its voices without pausing playback.
    #[must_use]
    pub const fn muted(self) -> bool {
        self.muted
    }
    /// Change mute while preserving gain.
    #[must_use]
    pub const fn with_muted(mut self, muted: bool) -> Self {
        self.muted = muted;
        self
    }
    #[cfg(any(feature = "native-output", test))]
    pub(crate) fn effective_gain(self) -> f32 {
        if self.muted { 0.0 } else { self.gain }
    }
}
impl Default for AudioBusSettings {
    fn default() -> Self {
        Self {
            gain: 1.0,
            muted: false,
        }
    }
}
