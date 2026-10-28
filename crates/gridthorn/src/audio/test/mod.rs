#[cfg(feature = "audio-capture")]
#[test]
fn capture_capacity_error_is_available_through_public_facade() {
    use crate::audio::{
        AudioCapture,
        devices::{AudioDeviceError, AudioDeviceSelection},
    };
    assert!(matches!(
        AudioCapture::start(AudioDeviceSelection::Default, 0),
        Err(AudioDeviceError::InvalidCapacity)
    ));
}
