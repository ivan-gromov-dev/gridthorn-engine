mod errors;

use std::fs;
use std::path::Path;
use std::sync::Arc;

pub use errors::{AudioClipError, WavDecodeError};

/// Decoded interleaved audio samples independent of an output backend.
#[derive(Clone, Debug, PartialEq)]
pub struct AudioClip {
    sample_rate: u32,
    channels: u16,
    samples: Arc<[f32]>,
}

impl AudioClip {
    /// Load and decode an interleaved PCM16 WAV file.
    ///
    /// # Errors
    ///
    /// Returns a contextual read or WAV validation failure.
    pub fn load_wav(path: impl AsRef<Path>) -> Result<Self, AudioClipError> {
        let path = path.as_ref();
        let bytes = fs::read(path).map_err(|source| AudioClipError::Read {
            path: path.to_path_buf(),
            source,
        })?;
        Self::from_wav_bytes(&bytes).map_err(|source| AudioClipError::Decode {
            path: path.to_path_buf(),
            source,
        })
    }

    /// Decode interleaved PCM16 WAV bytes without accessing an output device.
    ///
    /// # Errors
    ///
    /// Returns a typed error for malformed or unsupported WAV data.
    pub fn from_wav_bytes(bytes: &[u8]) -> Result<Self, WavDecodeError> {
        let document = WavDocument::parse(bytes)?;
        let frame_bytes = usize::from(document.channels) * 2;
        if document.data.len() % frame_bytes != 0 {
            return Err(WavDecodeError::MisalignedData);
        }
        let samples = document
            .data
            .as_chunks::<2>()
            .0
            .iter()
            .map(|sample| f32::from(i16::from_le_bytes([sample[0], sample[1]])) / 32_768.0)
            .collect::<Vec<_>>();
        Ok(Self {
            sample_rate: document.sample_rate,
            channels: document.channels,
            samples: Arc::from(samples),
        })
    }

    /// Samples per second for every channel.
    #[must_use]
    pub const fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// Number of interleaved channels.
    #[must_use]
    pub const fn channels(&self) -> u16 {
        self.channels
    }

    /// Number of complete interleaved sample frames.
    #[must_use]
    pub fn frame_count(&self) -> usize {
        self.samples.len() / usize::from(self.channels)
    }

    /// Normalized interleaved samples in `-1.0..1.0`.
    #[must_use]
    pub fn samples(&self) -> &[f32] {
        &self.samples
    }
}

struct WavDocument<'bytes> {
    sample_rate: u32,
    channels: u16,
    data: &'bytes [u8],
}

impl<'bytes> WavDocument<'bytes> {
    fn parse(bytes: &'bytes [u8]) -> Result<Self, WavDecodeError> {
        if bytes.len() < 12 || &bytes[0..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
            return Err(WavDecodeError::InvalidHeader);
        }
        let mut format = None;
        let mut data = None;
        let mut offset = 12_usize;
        while offset + 8 <= bytes.len() {
            let identifier = &bytes[offset..offset + 4];
            let size = u32::from_le_bytes(
                bytes[offset + 4..offset + 8]
                    .try_into()
                    .expect("fixed chunk-size field"),
            ) as usize;
            let start = offset + 8;
            let end = start
                .checked_add(size)
                .ok_or(WavDecodeError::TruncatedChunk)?;
            if end > bytes.len() {
                return Err(WavDecodeError::TruncatedChunk);
            }
            match identifier {
                b"fmt " => format = Some(parse_format(&bytes[start..end])?),
                b"data" => data = Some(&bytes[start..end]),
                _ => {}
            }
            offset = end.saturating_add(size % 2);
        }
        let format = format.ok_or(WavDecodeError::MissingFormat)?;
        let data = data.ok_or(WavDecodeError::MissingData)?;
        Ok(Self {
            sample_rate: format.sample_rate,
            channels: format.channels,
            data,
        })
    }
}

struct WavFormat {
    sample_rate: u32,
    channels: u16,
}

fn parse_format(bytes: &[u8]) -> Result<WavFormat, WavDecodeError> {
    if bytes.len() < 16 {
        return Err(WavDecodeError::TruncatedChunk);
    }
    let encoding = u16::from_le_bytes([bytes[0], bytes[1]]);
    let channels = u16::from_le_bytes([bytes[2], bytes[3]]);
    let sample_rate = u32::from_le_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]);
    let bits_per_sample = u16::from_le_bytes([bytes[14], bytes[15]]);
    if encoding != 1 || bits_per_sample != 16 {
        return Err(WavDecodeError::UnsupportedEncoding {
            format: encoding,
            bits_per_sample,
        });
    }
    if channels == 0 {
        return Err(WavDecodeError::InvalidChannels);
    }
    if sample_rate == 0 {
        return Err(WavDecodeError::InvalidSampleRate);
    }
    Ok(WavFormat {
        sample_rate,
        channels,
    })
}

#[cfg(test)]
mod test;
