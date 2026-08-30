use kira::{AudioManager, AudioManagerSettings, backend::mock::MockBackend};

use super::{AudioOutputError, OutputBackend, linear_gain, sound_data};
use crate::{AudioClip, AudioCommandQueue, PlaybackSettings};

#[test]
fn processes_voice_controls_without_an_output_device() {
    let manager = AudioManager::<MockBackend>::new(AudioManagerSettings::default())
        .expect("mock backend should initialize");
    let mut output = OutputBackend::new(manager);
    let mut commands = AudioCommandQueue::new();
    let voice = commands
        .play(stereo_clip(), PlaybackSettings::default())
        .expect("voice should allocate");
    commands
        .set_volume(voice, 0.25)
        .expect("volume should validate");

    output.process(&mut commands).expect("commands should play");
    assert_eq!(output.active_voice_count(), 1);
    assert!(commands.is_empty());

    output.suspend();
    output.suspend();
    output.resume();
    output.resume();
    commands.stop(voice);
    output.process(&mut commands).expect("voice should stop");
    assert_eq!(output.active_voice_count(), 0);
}

#[test]
fn converts_mono_to_stereo_and_normalized_gain_to_decibels() {
    let sound = sound_data(&mono_clip(), 0.5, false).expect("mono clip should convert");

    assert_eq!(sound.frames.len(), 2);
    assert_eq!(sound.frames[0], kira::Frame::new(0.25, 0.25));
    assert!((linear_gain(0.5).0 + 6.020_600_3).abs() < 0.000_1);
    assert_eq!(linear_gain(0.0), kira::Decibels::SILENCE);
}

#[test]
fn rejects_clip_layouts_the_stereo_mixer_cannot_represent() {
    let clip = wav_clip(3, &[0, 0, 0]);

    assert!(matches!(
        sound_data(&clip, 1.0, false),
        Err(AudioOutputError::UnsupportedChannelCount { channels: 3 })
    ));
}

fn mono_clip() -> AudioClip {
    wav_clip(1, &[8_192, -8_192])
}

fn stereo_clip() -> AudioClip {
    wav_clip(2, &[8_192, -8_192, 4_096, -4_096])
}

fn wav_clip(channels: u16, samples: &[i16]) -> AudioClip {
    let data_size = u32::try_from(samples.len() * 2).expect("test WAV should fit");
    let byte_rate = 8_000 * u32::from(channels) * 2;
    let block_align = channels * 2;
    let mut bytes = Vec::new();
    bytes.extend(b"RIFF");
    bytes.extend((36 + data_size).to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16_u32.to_le_bytes());
    bytes.extend(1_u16.to_le_bytes());
    bytes.extend(channels.to_le_bytes());
    bytes.extend(8_000_u32.to_le_bytes());
    bytes.extend(byte_rate.to_le_bytes());
    bytes.extend(block_align.to_le_bytes());
    bytes.extend(16_u16.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend(data_size.to_le_bytes());
    for sample in samples {
        bytes.extend(sample.to_le_bytes());
    }
    AudioClip::from_wav_bytes(&bytes).expect("test clip should decode")
}
