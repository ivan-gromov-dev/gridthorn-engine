use super::{AudioClip, WavDecodeError};

#[test]
fn decodes_interleaved_pcm16_samples() {
    let bytes = wav_bytes(1, 16, 2, 48_000, &[i16::MIN, 0, i16::MAX, 16_384]);

    let clip = AudioClip::from_wav_bytes(&bytes).expect("WAV should decode");

    assert_eq!(clip.sample_rate(), 48_000);
    assert_eq!(clip.channels(), 2);
    assert_eq!(clip.frame_count(), 2);
    assert!((clip.samples()[0] + 1.0).abs() < f32::EPSILON);
    assert!((clip.samples()[3] - 0.5).abs() < f32::EPSILON);
}

#[test]
fn rejects_unsupported_and_truncated_documents() {
    let unsupported = wav_bytes(1, 8, 1, 44_100, &[0]);
    assert_eq!(
        AudioClip::from_wav_bytes(&unsupported),
        Err(WavDecodeError::UnsupportedEncoding {
            format: 1,
            bits_per_sample: 8,
        })
    );

    let mut truncated = wav_bytes(1, 16, 1, 44_100, &[0]);
    truncated.pop();
    assert_eq!(
        AudioClip::from_wav_bytes(&truncated),
        Err(WavDecodeError::TruncatedChunk)
    );
}

pub(crate) fn wav_bytes(
    format: u16,
    bits_per_sample: u16,
    channels: u16,
    sample_rate: u32,
    samples: &[i16],
) -> Vec<u8> {
    let data = samples
        .iter()
        .flat_map(|sample| sample.to_le_bytes())
        .collect::<Vec<_>>();
    let byte_rate = sample_rate * u32::from(channels) * u32::from(bits_per_sample) / 8;
    let block_align = channels * bits_per_sample / 8;
    let riff_size = 36 + u32::try_from(data.len()).expect("test data length");
    let mut bytes = Vec::new();
    bytes.extend(b"RIFF");
    bytes.extend(riff_size.to_le_bytes());
    bytes.extend(b"WAVEfmt ");
    bytes.extend(16_u32.to_le_bytes());
    bytes.extend(format.to_le_bytes());
    bytes.extend(channels.to_le_bytes());
    bytes.extend(sample_rate.to_le_bytes());
    bytes.extend(byte_rate.to_le_bytes());
    bytes.extend(block_align.to_le_bytes());
    bytes.extend(bits_per_sample.to_le_bytes());
    bytes.extend(b"data");
    bytes.extend(
        u32::try_from(data.len())
            .expect("test data length")
            .to_le_bytes(),
    );
    bytes.extend(data);
    bytes
}
