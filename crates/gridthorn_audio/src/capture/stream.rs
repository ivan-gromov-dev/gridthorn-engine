use crate::devices::{
    AudioDeviceError, AudioDeviceId, AudioDeviceKind, AudioDeviceSelection, backend_error, config,
    resolve,
};
use kira::backend::cpal::cpal::{
    self,
    traits::{DeviceTrait, StreamTrait},
};
use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
        mpsc::{Receiver, sync_channel},
    },
};

/// Interleaved normalized f32 capture samples from one stream configuration.
#[derive(Debug)]
pub struct CapturedAudio {
    /// Interleaved sample frames in native channel order.
    pub samples: Vec<f32>,
    /// Samples per second per channel.
    pub sample_rate: u32,
    /// Samples per interleaved frame.
    pub channels: u16,
    /// Frames dropped since the previous drain due to overflow or contention.
    pub dropped_frames: u64,
}

struct CaptureBuffer {
    samples: Mutex<VecDeque<f32>>,
    capacity: usize,
    dropped: AtomicU64,
}

impl CaptureBuffer {
    fn push(&self, samples: &[f32], channels: usize) {
        let Ok(mut buffer) = self.samples.try_lock() else {
            self.dropped
                .fetch_add((samples.len() / channels) as u64, Ordering::Relaxed);
            return;
        };
        let available = (self.capacity - buffer.len()) / channels * channels;
        let accepted = available.min(samples.len() / channels * channels);
        buffer.extend(samples[..accepted].iter().copied());
        self.dropped.fetch_add(
            ((samples.len() - accepted) / channels) as u64,
            Ordering::Relaxed,
        );
    }
}

/// Explicitly started recording stream. Dropping it stops capture.
/// No recording occurs during device enumeration or output construction.
pub struct AudioCapture {
    _stream: cpal::Stream,
    buffer: Arc<CaptureBuffer>,
    failed: Arc<AtomicBool>,
    selection: AudioDeviceSelection,
    device: AudioDeviceId,
    config: cpal::StreamConfig,
    capacity_frames: usize,
    errors: Receiver<String>,
}

impl AudioCapture {
    /// Start recording with bounded storage, in sample frames (1..=1048576).
    ///
    /// # Errors
    /// Reports invalid capacity, permission denial, missing devices, unsupported
    /// f32 configuration or stream failures with backend context.
    pub fn start(
        selection: AudioDeviceSelection,
        capacity_frames: usize,
    ) -> Result<Self, AudioDeviceError> {
        if !(1..=1_048_576).contains(&capacity_frames) {
            return Err(AudioDeviceError::InvalidCapacity);
        }
        let (device, id) = resolve(&selection, AudioDeviceKind::Capture)?;
        let config = config(&device, AudioDeviceKind::Capture)?;
        let capacity = capacity_frames * usize::from(config.channels);
        let buffer = Arc::new(CaptureBuffer {
            samples: Mutex::new(VecDeque::with_capacity(capacity)),
            capacity,
            dropped: AtomicU64::new(0),
        });
        let callback_buffer = buffer.clone();
        let failed = Arc::new(AtomicBool::new(false));
        let callback_failed = failed.clone();
        let channels = usize::from(config.channels);
        let (error_sender, errors) = sync_channel(8);
        let stream = device
            .build_input_stream(
                config,
                move |samples: &[f32], _| callback_buffer.push(samples, channels),
                move |error| {
                    let _ = error_sender.try_send(format!("capture stream: {error}"));
                    if crate::devices::requires_reconnect(error.kind()) {
                        callback_failed.store(true, Ordering::Release);
                    }
                },
                None,
            )
            .map_err(backend_error)?;
        stream.play().map_err(backend_error)?;
        Ok(Self {
            _stream: stream,
            buffer,
            failed,
            selection,
            device: id,
            config,
            capacity_frames,
            errors,
        })
    }

    /// Last connected endpoint; call refresh to check current availability.
    #[must_use]
    pub fn device(&self) -> &AudioDeviceId {
        &self.device
    }

    /// Requested capture endpoint policy.
    #[must_use]
    pub fn selection(&self) -> &AudioDeviceSelection {
        &self.selection
    }

    /// Take a pending native stream diagnostic. At most eight are retained.
    pub fn take_device_error(&self) -> Option<AudioDeviceError> {
        self.errors.try_recv().ok().map(AudioDeviceError::Backend)
    }

    /// Take buffered audio. This never restarts recording or changes configuration.
    ///
    /// # Errors
    /// Returns a retryable stream failure until refresh successfully reconnects.
    pub fn drain(&mut self) -> Result<CapturedAudio, AudioDeviceError> {
        if self.failed.load(Ordering::Acquire) {
            return Err(AudioDeviceError::Backend(
                "capture stream failed; call refresh_device to retry".into(),
            ));
        }
        let samples = self
            .buffer
            .samples
            .lock()
            .map_err(backend_error)?
            .drain(..)
            .collect();
        Ok(CapturedAudio {
            samples,
            sample_rate: self.config.sample_rate,
            channels: self.config.channels,
            dropped_frames: self.buffer.dropped.swap(0, Ordering::Relaxed),
        })
    }

    /// Switch recording endpoint, discarding old buffered samples only on success.
    ///
    /// # Errors
    /// A rejected switch preserves the previous stream and selection.
    pub fn select_device(
        &mut self,
        selection: AudioDeviceSelection,
    ) -> Result<(), AudioDeviceError> {
        *self = Self::start(selection, self.capacity_frames)?;
        Ok(())
    }

    /// Follow a changed default or retry a failed stream. Returns whether restarted.
    /// Buffered samples are cleared on restart so configurations cannot be mixed.
    ///
    /// # Errors
    /// Missing explicit devices never fall back; call again after reconnection.
    pub fn refresh_device(&mut self) -> Result<bool, AudioDeviceError> {
        let (_, id) = resolve(&self.selection, AudioDeviceKind::Capture)?;
        if id != self.device || self.failed.load(Ordering::Acquire) {
            self.select_device(self.selection.clone())?;
            return Ok(true);
        }
        Ok(false)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
