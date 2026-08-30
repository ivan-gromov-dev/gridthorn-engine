use std::io;
use std::path::PathBuf;

use thiserror::Error;

/// Failure to load a WAV clip from a filesystem path.
#[derive(Debug, Error)]
pub enum AudioClipError {
    /// Source bytes could not be read.
    #[error("failed to read WAV audio clip at {path}")]
    Read {
        /// Requested source path.
        path: PathBuf,
        /// Filesystem failure.
        #[source]
        source: io::Error,
    },
    /// Source bytes were not a supported WAV document.
    #[error("failed to decode WAV audio clip at {path}")]
    Decode {
        /// Requested source path.
        path: PathBuf,
        /// WAV validation failure.
        #[source]
        source: WavDecodeError,
    },
}

/// Failure to decode the provisional PCM16 WAV contract.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WavDecodeError {
    /// The RIFF/WAVE signature is missing.
    #[error("missing RIFF/WAVE header")]
    InvalidHeader,
    /// A declared RIFF chunk extends beyond the available bytes.
    #[error("WAV chunk is truncated")]
    TruncatedChunk,
    /// No usable `fmt ` chunk exists.
    #[error("missing WAV format chunk")]
    MissingFormat,
    /// No sample data exists.
    #[error("missing WAV data chunk")]
    MissingData,
    /// Only interleaved PCM16 is currently accepted.
    #[error("unsupported WAV encoding {format} with {bits_per_sample} bits per sample")]
    UnsupportedEncoding {
        /// WAV format tag.
        format: u16,
        /// Declared sample precision.
        bits_per_sample: u16,
    },
    /// Channel count must be non-zero.
    #[error("WAV channel count must be greater than zero")]
    InvalidChannels,
    /// Sample rate must be non-zero.
    #[error("WAV sample rate must be greater than zero")]
    InvalidSampleRate,
    /// Interleaved sample data must contain complete frames.
    #[error("WAV sample data is not aligned to complete interleaved frames")]
    MisalignedData,
}
