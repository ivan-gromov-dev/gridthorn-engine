use super::{AudioOutput, AudioOutputError};
use crate::devices::{
    AudioDeviceError, AudioDeviceId, AudioDeviceKind, AudioDeviceSelection, resolve,
};

impl AudioOutput {
    /// Connect to a chosen endpoint. Explicit device loss never falls back silently.
    ///
    /// # Errors
    /// Returns device lookup, format, or stream initialization diagnostics.
    pub fn with_device(selection: AudioDeviceSelection) -> Result<Self, AudioOutputError> {
        let mut manager = kira::AudioManager::<super::native_backend::NativeBackend>::new(
            kira::AudioManagerSettings {
                backend_settings: selection.clone(),
                ..Default::default()
            },
        )
        .map_err(AudioOutputError::Device)?;
        let device = manager.backend_mut().device().clone();
        Ok(Self {
            backend: super::OutputBackend::new(manager),
            selection,
            device,
        })
    }

    /// Current requested policy; failed switches leave it unchanged.
    #[must_use]
    pub fn selection(&self) -> &AudioDeviceSelection {
        &self.selection
    }

    /// Last successfully connected endpoint, not proof of current availability.
    #[must_use]
    pub fn device(&self) -> &AudioDeviceId {
        &self.device
    }

    /// Switch output while preserving voice handles, playback positions and suspension.
    ///
    /// # Errors
    /// A failed request leaves the previous stream and selection intact.
    pub fn select_device(
        &mut self,
        selection: AudioDeviceSelection,
    ) -> Result<(), AudioDeviceError> {
        let device = self
            .backend
            .manager
            .backend_mut()
            .connect(selection.clone())?;
        self.device = device;
        self.selection = selection;
        Ok(())
    }

    /// Take the oldest native stream diagnostic, including errors followed by successful recovery.
    /// At most eight pending diagnostics are retained.
    pub fn take_device_error(&mut self) -> Option<AudioDeviceError> {
        self.backend.manager.backend_mut().take_error()
    }

    /// Poll default-device changes and retry failed streams, preserving the mixer.
    /// Call explicitly when checking settings or handling a stream diagnostic.
    /// Returns whether reconnection occurred; command processing never calls this.
    ///
    /// # Errors
    /// Missing devices and failed reconnections are retryable on a later call.
    /// Explicit selections wait for that endpoint to return; default follows the OS.
    pub fn refresh_device(&mut self) -> Result<bool, AudioDeviceError> {
        let (_, device) = resolve(&self.selection, AudioDeviceKind::Output)?;
        if device != self.device || self.backend.manager.backend_mut().failed() {
            self.select_device(self.selection.clone())?;
            return Ok(true);
        }
        Ok(false)
    }
}
