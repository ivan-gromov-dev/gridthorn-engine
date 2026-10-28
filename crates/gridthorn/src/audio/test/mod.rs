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

#[test]
fn bus_controls_are_headless_and_available_through_facade() {
    use crate::audio::{
        AudioBusId, AudioBusSettings, AudioCommand, AudioCommandQueue, AudioSettingsError,
        PlaybackSettings,
    };
    let bus = AudioBusId::Category(u64::MAX);
    assert_eq!(PlaybackSettings::default().bus(), AudioBusId::Master);
    assert_eq!(PlaybackSettings::default().with_bus(bus).bus(), bus);
    assert_eq!(AudioBusSettings::default().gain(), 1.0);
    assert!(!AudioBusSettings::default().muted());
    for gain in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -0.1, 1.1] {
        assert_eq!(
            AudioBusSettings::new(gain, false),
            Err(AudioSettingsError::InvalidVolume)
        );
    }
    let settings = AudioBusSettings::new(0.25, true).unwrap();
    assert_eq!(settings.with_muted(false).gain(), 0.25);
    let mut queue = AudioCommandQueue::new();
    queue.set_bus(bus, settings);
    queue.set_bus(
        AudioBusId::Master,
        AudioBusSettings::new(0.0, false).unwrap(),
    );
    assert!(
        matches!(queue.drain()[0], AudioCommand::SetBus { bus: id, settings: value } if id == bus && value == settings)
    );
}
