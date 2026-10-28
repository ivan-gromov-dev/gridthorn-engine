use crate::devices::{
    AudioDeviceError, AudioDeviceId, AudioDeviceKind, AudioDeviceSelection, backend_error, config,
    resolve,
};
use kira::backend::{
    Backend, Renderer,
    cpal::cpal::{
        self,
        traits::{DeviceTrait, StreamTrait},
    },
};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc::{Receiver, SyncSender, sync_channel},
};

pub(super) struct NativeBackend {
    stream: Option<cpal::Stream>,
    device: AudioDeviceId,
    renderer: Arc<Mutex<Option<Renderer>>>,
    failed: Arc<AtomicBool>,
    selection: AudioDeviceSelection,
    errors: Receiver<String>,
    error_sender: SyncSender<String>,
}

impl Backend for NativeBackend {
    type Settings = AudioDeviceSelection;
    type Error = AudioDeviceError;

    fn setup(selection: Self::Settings, _: usize) -> Result<(Self, u32), Self::Error> {
        let (device, id) = resolve(&selection, AudioDeviceKind::Output)?;
        let rate = config(&device, AudioDeviceKind::Output)?.sample_rate;
        let (error_sender, errors) = sync_channel(8);
        Ok((
            Self {
                stream: None,
                device: id.clone(),
                renderer: Arc::new(Mutex::new(None)),
                failed: Arc::new(AtomicBool::new(false)),
                selection: AudioDeviceSelection::Device(id),
                errors,
                error_sender,
            },
            rate,
        ))
    }

    fn start(&mut self, renderer: Renderer) -> Result<(), Self::Error> {
        *self.renderer.lock().map_err(backend_error)? = Some(renderer);
        self.connect(self.selection.clone()).map(|_| ())
    }
}

impl NativeBackend {
    #[cfg(test)]
    pub(super) fn simulate_stream_failure(&self) {
        self.failed.store(true, Ordering::Release);
    }

    pub(super) fn device(&self) -> &AudioDeviceId {
        &self.device
    }

    pub(super) fn take_error(&self) -> Option<AudioDeviceError> {
        self.errors.try_recv().ok().map(AudioDeviceError::Backend)
    }

    pub(super) fn failed(&self) -> bool {
        self.failed.load(Ordering::Acquire)
    }

    pub(super) fn connect(
        &mut self,
        selection: AudioDeviceSelection,
    ) -> Result<AudioDeviceId, AudioDeviceError> {
        let (device, id) = resolve(&selection, AudioDeviceKind::Output)?;
        let config = config(&device, AudioDeviceKind::Output)?;
        let channels = config.channels;
        let renderer = self.renderer.clone();
        let failed = Arc::new(AtomicBool::new(false));
        let callback_failed = failed.clone();
        let error_sender = self.error_sender.clone();
        let enabled = Arc::new(AtomicBool::new(false));
        let callback_enabled = enabled.clone();
        let stream = device
            .build_output_stream(
                config,
                move |data: &mut [f32], _| {
                    data.fill(0.0);
                    if callback_enabled.load(Ordering::Acquire)
                        && let Ok(mut renderer) = renderer.try_lock()
                        && let Some(renderer) = renderer.as_mut()
                    {
                        renderer.on_start_processing();
                        renderer.process(data, channels);
                    }
                },
                move |error| {
                    let _ = error_sender.try_send(format!("output stream: {error}"));
                    if crate::devices::requires_reconnect(error.kind()) {
                        callback_failed.store(true, Ordering::Release);
                    }
                },
                None,
            )
            .map_err(backend_error)?;
        stream.play().map_err(backend_error)?;
        let mut renderer = self.renderer.lock().map_err(backend_error)?;
        self.stream.take();
        if let Some(renderer) = renderer.as_mut() {
            renderer.on_change_sample_rate(config.sample_rate);
        }
        self.failed = failed;
        self.stream = Some(stream);
        self.selection = selection;
        self.device = id.clone();
        enabled.store(true, Ordering::Release);
        Ok(id)
    }
}
