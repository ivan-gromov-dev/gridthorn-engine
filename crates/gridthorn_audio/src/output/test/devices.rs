use super::*;
use crate::{AudioOutput, devices::AudioDeviceSelection};

#[test]
#[ignore = "requires native output; silent playback and injected stream failure"]
fn reconnect_preserves_voices_and_suspension() {
    let mut output = AudioOutput::new().unwrap();
    let mut commands = AudioCommandQueue::new();
    let bus = crate::AudioBusId::Category(12);
    commands.set_bus(bus, crate::AudioBusSettings::new(0.5, true).unwrap());
    let voice = commands
        .play(
            stereo_clip(),
            PlaybackSettings::new(0.0, true).unwrap().with_bus(bus),
        )
        .unwrap();
    output.process(&mut commands).unwrap();
    output.suspend();
    let position = output.backend.voices[&voice].position();
    output
        .backend
        .manager
        .backend_mut()
        .simulate_stream_failure();
    assert!(output.refresh_device().unwrap());
    assert_eq!(output.active_voice_count(), 1);
    assert!(output.backend.suspended);
    assert!(output.backend.voices[&voice].position() >= position);
    output.select_device(AudioDeviceSelection::Default).unwrap();
    assert_eq!(output.active_voice_count(), 1);
    assert_eq!(output.backend.buses.gain(bus, 1.0), 0.0);
    commands.set_bus(bus, crate::AudioBusSettings::new(0.5, false).unwrap());
    output.process(&mut commands).unwrap();
    assert_eq!(output.backend.buses.gain(bus, 1.0), 0.5);
    output.resume();
    commands.stop(voice);
    output.process(&mut commands).unwrap();
    assert_eq!(output.active_voice_count(), 0);
}

#[test]
#[ignore = "requires native output; command processing must not recover a failed stream"]
fn processing_commands_does_not_poll_or_reconnect() {
    let mut output = AudioOutput::new().unwrap();
    let mut commands = AudioCommandQueue::new();
    output
        .backend
        .manager
        .backend_mut()
        .simulate_stream_failure();
    let voice = commands
        .play(stereo_clip(), PlaybackSettings::new(0.0, true).unwrap())
        .unwrap();
    output.process(&mut commands).unwrap();
    assert!(commands.is_empty());
    assert_eq!(output.active_voice_count(), 1);
    assert!(output.backend.manager.backend_mut().failed());
    output.process(&mut commands).unwrap();
    assert!(output.backend.manager.backend_mut().failed());
    commands.stop(voice);
    output.process(&mut commands).unwrap();
    assert_eq!(output.active_voice_count(), 0);
    assert!(output.refresh_device().unwrap());
    assert!(!output.backend.manager.backend_mut().failed());
}
