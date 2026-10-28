use thiserror::Error;

/// Failure querying, selecting, or driving a native audio device.
#[derive(Debug, Error)]
pub enum AudioDeviceError {
    /// No device currently satisfies the requested direction and selection.
    #[error("audio device unavailable: {0}")]
    Unavailable(String),
    /// The host rejected an operation; includes the operation and backend diagnostic.
    #[error("audio device operation failed: {0}")]
    Backend(String),
    /// The provisional adapter requires an f32 stream format.
    #[error("audio device has no supported f32 stream configuration")]
    UnsupportedFormat,
    /// Capture storage must have a positive, bounded capacity.
    #[error("capture capacity must be between 1 and 1048576 sample frames")]
    InvalidCapacity,
}
