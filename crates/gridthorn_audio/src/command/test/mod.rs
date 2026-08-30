use super::{AudioCommand, AudioCommandQueue, AudioSettingsError, PlaybackSettings};
use crate::AudioClip;

#[test]
fn preserves_playback_control_request_order() {
    let clip = silent_clip();
    let mut queue = AudioCommandQueue::new();
    let voice = queue
        .play(clip, PlaybackSettings::default())
        .expect("voice should allocate");
    queue
        .set_volume(voice, 0.5)
        .expect("volume should be valid");
    queue.stop(voice);

    let commands = queue.drain();

    assert!(matches!(commands[0], AudioCommand::Play { voice: id, .. } if id == voice));
    assert!(
        matches!(commands[1], AudioCommand::SetVolume { voice: id, volume } if id == voice && (volume - 0.5).abs() < f32::EPSILON)
    );
    assert_eq!(commands[2], AudioCommand::Stop { voice });
    assert!(queue.is_empty());
}

#[test]
fn rejects_invalid_normalized_volumes_without_queueing() {
    assert_eq!(
        PlaybackSettings::new(f32::NAN, false),
        Err(AudioSettingsError::InvalidVolume)
    );
    let mut queue = AudioCommandQueue::new();
    let voice = queue
        .play(silent_clip(), PlaybackSettings::default())
        .expect("voice should allocate");
    let _commands = queue.drain();

    assert_eq!(
        queue.set_volume(voice, 1.1),
        Err(AudioSettingsError::InvalidVolume)
    );
    assert!(queue.is_empty());
}

fn silent_clip() -> AudioClip {
    let mut bytes = Vec::new();
    bytes.extend(b"RIFF");
    bytes.extend(38_u32.to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16_u32.to_le_bytes());
    bytes.extend(1_u16.to_le_bytes());
    bytes.extend(1_u16.to_le_bytes());
    bytes.extend(8_000_u32.to_le_bytes());
    bytes.extend(16_000_u32.to_le_bytes());
    bytes.extend(2_u16.to_le_bytes());
    bytes.extend(16_u16.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend(2_u32.to_le_bytes());
    bytes.extend(0_i16.to_le_bytes());
    AudioClip::from_wav_bytes(&bytes).expect("test clip should decode")
}
