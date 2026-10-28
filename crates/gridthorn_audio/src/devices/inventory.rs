use super::AudioDeviceError;
use kira::backend::cpal::cpal::{
    self,
    traits::{DeviceTrait, HostTrait},
};

/// Opaque native device identity, scoped to the current host and direction.
/// Re-enumerate after reconnection; persistence across OS sessions is not promised.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AudioDeviceId(String);

/// Direction of a native audio endpoint.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioDeviceKind {
    /// Playback endpoint.
    Output,
    /// Recording endpoint. Enumeration does not open a recording stream.
    Capture,
}

/// Requested endpoint policy. Explicit devices never silently fall back.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum AudioDeviceSelection {
    /// Follow the system default on explicit refresh.
    #[default]
    Default,
    /// Use the endpoint returned by enumeration.
    Device(AudioDeviceId),
}

/// One endpoint in a fresh native device inventory.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AudioDeviceInfo {
    /// Host-scoped opaque identity.
    pub id: AudioDeviceId,
    /// Human-readable OS label, not an identity.
    pub name: String,
    /// Endpoint direction.
    pub kind: AudioDeviceKind,
    /// Whether the OS currently reports this endpoint as the default.
    pub is_default: bool,
}

/// Query playback and capture endpoints without opening streams.
///
/// # Errors
/// Returns contextual host or device identity query failures.
pub fn audio_devices() -> Result<Vec<AudioDeviceInfo>, AudioDeviceError> {
    let host = cpal::default_host();
    let mut result = Vec::new();
    for kind in [AudioDeviceKind::Output, AudioDeviceKind::Capture] {
        let default = default_device(&host, kind)
            .map(|d| identity(&d, kind))
            .transpose()?;
        for device in host.devices().map_err(backend_error)? {
            let supported = match kind {
                AudioDeviceKind::Output => device.supports_output(),
                AudioDeviceKind::Capture => device.supports_input(),
            };
            if supported {
                let id = identity(&device, kind)?;
                result.push(AudioDeviceInfo {
                    is_default: default.as_ref() == Some(&id),
                    id,
                    name: device.to_string(),
                    kind,
                });
            }
        }
    }
    Ok(result)
}

pub(crate) fn backend_error(error: impl std::fmt::Display) -> AudioDeviceError {
    AudioDeviceError::Backend(error.to_string())
}

fn default_device(host: &cpal::Host, kind: AudioDeviceKind) -> Option<cpal::Device> {
    match kind {
        AudioDeviceKind::Output => host.default_output_device(),
        AudioDeviceKind::Capture => host.default_input_device(),
    }
}

pub(crate) fn identity(
    device: &cpal::Device,
    kind: AudioDeviceKind,
) -> Result<AudioDeviceId, AudioDeviceError> {
    Ok(AudioDeviceId(format!(
        "{kind:?}:{}",
        device.id().map_err(backend_error)?
    )))
}

pub(crate) fn resolve(
    selection: &AudioDeviceSelection,
    kind: AudioDeviceKind,
) -> Result<(cpal::Device, AudioDeviceId), AudioDeviceError> {
    let host = cpal::default_host();
    let device = match selection {
        AudioDeviceSelection::Default => default_device(&host, kind),
        AudioDeviceSelection::Device(id) => host
            .devices()
            .map_err(backend_error)?
            .find(|d| identity(d, kind).is_ok_and(|found| &found == id)),
    }
    .ok_or_else(|| AudioDeviceError::Unavailable(format!("{kind:?} {selection:?}")))?;
    let id = identity(&device, kind)?;
    Ok((device, id))
}

pub(crate) fn config(
    device: &cpal::Device,
    kind: AudioDeviceKind,
) -> Result<cpal::StreamConfig, AudioDeviceError> {
    let configs: Vec<_> = match kind {
        AudioDeviceKind::Output => device
            .supported_output_configs()
            .map_err(backend_error)?
            .collect(),
        AudioDeviceKind::Capture => device
            .supported_input_configs()
            .map_err(backend_error)?
            .collect(),
    };
    let default = match kind {
        AudioDeviceKind::Output => device.default_output_config(),
        AudioDeviceKind::Capture => device.default_input_config(),
    }
    .map_err(backend_error)?;
    if default.sample_format() == cpal::SampleFormat::F32 {
        return Ok(default.config());
    }
    configs
        .into_iter()
        .find(|c| c.sample_format() == cpal::SampleFormat::F32)
        .map(|c| {
            let rate = 48_000_u32.clamp(c.min_sample_rate(), c.max_sample_rate());
            c.with_sample_rate(rate).config()
        })
        .ok_or(AudioDeviceError::UnsupportedFormat)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

pub(crate) fn requires_reconnect(kind: cpal::ErrorKind) -> bool {
    !matches!(
        kind,
        cpal::ErrorKind::Xrun | cpal::ErrorKind::RealtimeDenied | cpal::ErrorKind::DeviceChanged
    )
}
