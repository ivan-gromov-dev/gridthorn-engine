use crate::prelude::*;

#[test]
fn prelude_decodes_and_queues_backend_independent_audio() {
    let clip = AudioClip::from_wav_bytes(&silent_wav()).expect("test clip should decode");
    let mut audio = AudioCommandQueue::new();

    let voice = audio
        .play(clip, PlaybackSettings::default())
        .expect("voice should allocate");
    audio.stop(voice);

    assert_eq!(audio.len(), 2);
    assert_eq!(audio.drain()[1], AudioCommand::Stop { voice });
}

fn silent_wav() -> Vec<u8> {
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
    bytes
}
