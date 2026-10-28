use super::*;
use crate::{AudioBusId, AudioBusSettings};
use kira::backend::{Backend, Renderer};

struct RecordingBackend {
    renderer: Option<Renderer>,
}
impl Backend for RecordingBackend {
    type Settings = ();
    type Error = ();
    fn setup((): (), _: usize) -> Result<(Self, u32), ()> {
        Ok((Self { renderer: None }, 8_000))
    }
    fn start(&mut self, renderer: Renderer) -> Result<(), ()> {
        self.renderer = Some(renderer);
        Ok(())
    }
}
fn sample(output: &mut OutputBackend<RecordingBackend>) -> f32 {
    let renderer = output.manager.backend_mut().renderer.as_mut().unwrap();
    renderer.on_start_processing();
    let mut samples = vec![0.0; 8_000];
    renderer.process(&mut samples, 2);
    samples[7_998]
}
fn near(actual: f32, expected: f32) {
    assert!((actual - expected).abs() < 0.0001, "{actual} != {expected}");
}

#[test]
fn renders_bus_controls_for_existing_future_and_direct_voices() {
    let manager = AudioManager::<RecordingBackend>::new(AudioManagerSettings::default()).unwrap();
    let mut output = OutputBackend::new(manager);
    let mut queue = AudioCommandQueue::new();
    let category = AudioBusId::Category(7);
    let other = AudioBusId::Category(8);
    let half = AudioBusSettings::new(0.5, false).unwrap();
    queue.set_bus(AudioBusId::Master, half);
    queue.set_bus(category, half);
    let voice = queue
        .play(
            wav_clip(2, &[8_192; 64]),
            PlaybackSettings::new(0.5, true).unwrap().with_bus(category),
        )
        .unwrap();
    output.process(&mut queue).unwrap();
    near(sample(&mut output), 0.03125);
    queue.set_bus(other, AudioBusSettings::new(0.0, true).unwrap());
    queue.set_bus(category, half.with_muted(true));
    output.process(&mut queue).unwrap();
    near(sample(&mut output), 0.0);
    assert_eq!(output.active_voice_count(), 1);
    queue.set_volume(voice, 1.0).unwrap();
    queue.set_bus(category, half);
    output.process(&mut queue).unwrap();
    near(sample(&mut output), 0.0625);
    queue.set_bus(AudioBusId::Master, half.with_muted(true));
    let direct = queue
        .play(
            wav_clip(2, &[8_192; 64]),
            PlaybackSettings::new(1.0, true).unwrap(),
        )
        .unwrap();
    output.process(&mut queue).unwrap();
    near(sample(&mut output), 0.0);
    output.suspend();
    queue.set_bus(AudioBusId::Master, half);
    output.process(&mut queue).unwrap();
    sample(&mut output);
    output.resume();
    near(sample(&mut output), 0.1875);
    queue.stop(voice);
    output.process(&mut queue).unwrap();
    near(sample(&mut output), 0.125);
    queue.set_bus(AudioBusId::Master, AudioBusSettings::default());
    output.process(&mut queue).unwrap();
    near(sample(&mut output), 0.25);
    queue.stop(direct);
    output.process(&mut queue).unwrap();
    near(sample(&mut output), 0.0);
}

#[test]
fn completed_voice_routing_is_released() {
    let manager = AudioManager::<RecordingBackend>::new(AudioManagerSettings::default()).unwrap();
    let mut output = OutputBackend::new(manager);
    let mut queue = AudioCommandQueue::new();
    queue
        .play(
            stereo_clip(),
            PlaybackSettings::default().with_bus(AudioBusId::Category(1)),
        )
        .unwrap();
    output.process(&mut queue).unwrap();
    sample(&mut output);
    output.process(&mut queue).unwrap();
    assert_eq!(output.active_voice_count(), 0);
}
