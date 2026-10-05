use std::{
    hint::black_box,
    thread,
    time::{Duration, Instant},
};

use cpal::traits::{DeviceTrait, HostTrait};
use kira::{
    backend::{DefaultBackend, cpal::cpal},
    sound::PlaybackState,
};

use super::{
    AudioCommandQueue, AudioManager, AudioManagerSettings, OutputBackend, PlaybackSettings,
    wav_clip,
};

#[cfg(target_os = "windows")]
mod loopback;

/// Measures silent native mixer progress and explicit lifecycle completion, not acoustic latency.
#[test]
#[ignore = "requires a native output device; run alone in release mode"]
fn measure_native_output_lifecycle() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let device = cpal::default_host()
        .default_output_device()
        .expect("default output device");
    println!(
        "native_device,{:?},{:?}",
        device.description(),
        device.default_output_config()
    );
    let start = Instant::now();
    let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())
        .expect("native stream");
    println!("native_audio,init,0,{}", start.elapsed().as_nanos());
    let mut output = OutputBackend::new(manager);
    let mut queue = AudioCommandQueue::new();
    let clip = wav_clip(2, &vec![0; 1600]);
    let cycles = std::env::var("GRIDTHORN_AUDIO_PROBE_CYCLES")
        .map_or(120, |value| value.parse::<usize>().expect("cycle count"));
    let voice_count = std::env::var("GRIDTHORN_AUDIO_PROBE_VOICES")
        .map_or(1, |value| value.parse::<usize>().expect("voice count"));
    assert!((1..=64).contains(&voice_count));
    let looping_clip = wav_clip(2, &vec![0; 160_000]);
    println!("native_audio,ready,{voice_count},{cycles}");
    for cycle in 0..cycles {
        let voices = (0..voice_count)
            .map(|_| {
                queue
                    .play(
                        looping_clip.clone(),
                        PlaybackSettings::new(0.0, true).unwrap(),
                    )
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let voice = voices[0];
        let start = Instant::now();
        output.process(&mut queue).unwrap();
        println!("native_audio,submit,{cycle},{}", start.elapsed().as_nanos());
        wait_until(|| voices.iter().all(|id| output.voices[id].position() > 0.0));
        println!(
            "native_audio,mixer_progress,{cycle},{}",
            start.elapsed().as_nanos()
        );
        let start = Instant::now();
        output.suspend();
        wait_until(|| {
            voices
                .iter()
                .all(|id| output.voices[id].state() == PlaybackState::Paused)
        });
        println!(
            "native_audio,pause_complete,{cycle},{}",
            start.elapsed().as_nanos()
        );
        thread::sleep(Duration::from_millis(25));
        let paused = output.voices[&voice].position();
        thread::sleep(Duration::from_millis(25));
        assert_eq!(paused, output.voices[&voice].position());
        let start = Instant::now();
        output.resume();
        wait_until(|| {
            voices
                .iter()
                .all(|id| output.voices[id].state() == PlaybackState::Playing)
        });
        println!(
            "native_audio,resume_complete,{cycle},{}",
            start.elapsed().as_nanos()
        );
        for voice in voices {
            queue.stop(voice);
        }
        output.process(&mut queue).unwrap();
        let natural = queue
            .play(clip.clone(), PlaybackSettings::new(0.0, false).unwrap())
            .unwrap();
        output.process(&mut queue).unwrap();
        wait_until(|| output.voices[&natural].state() == PlaybackState::Stopped);
        output.process(&mut queue).unwrap();
        assert!(output.voices.is_empty());
        assert!(queue.is_empty());
        thread::sleep(Duration::from_millis(100));
        report_backend(&mut output, cycle);
    }
    if let Ok(gate) = std::env::var("GRIDTHORN_AUDIO_RESUME_GATE") {
        measure_manual_sleep(&mut output, &mut queue, clip, &gate);
    }
    let voice = queue
        .play(looping_clip, PlaybackSettings::new(0.0, true).unwrap())
        .unwrap();
    output.process(&mut queue).unwrap();
    wait_until(|| output.voices[&voice].position() > 0.0);
    let start = Instant::now();
    drop(output);
    println!("native_audio,shutdown,0,{}", start.elapsed().as_nanos());
}

fn report_backend(output: &mut OutputBackend<DefaultBackend>, cycle: usize) {
    while let Some(usage) = output.manager.backend_mut().pop_cpu_usage() {
        println!("native_cpu,{cycle},{usage}");
    }
    assert!(output.manager.backend_mut().pop_error().is_none());
    assert_eq!(
        output.manager.backend_mut().num_stream_errors_discarded(),
        Some(0)
    );
}

fn measure_manual_sleep(
    output: &mut OutputBackend<DefaultBackend>,
    queue: &mut AudioCommandQueue,
    clip: crate::AudioClip,
    gate: &str,
) {
    let minimum_progress = 1.0 / f64::from(clip.sample_rate());
    let voice = queue
        .play(clip, PlaybackSettings::new(0.0, true).unwrap())
        .unwrap();
    output.process(queue).unwrap();
    wait_until(|| output.voices[&voice].position() > 0.0);
    output.suspend();
    wait_until(|| output.voices[&voice].state() == PlaybackState::Paused);
    thread::sleep(Duration::from_millis(25));
    let paused = output.voices[&voice].position();
    println!("native_audio,manual_sleep_ready,0,{paused}");
    while !std::path::Path::new(gate).exists() {
        thread::sleep(Duration::from_millis(100));
    }
    assert_eq!(paused, output.voices[&voice].position());
    let start = Instant::now();
    output.resume();
    wait_until(|| output.voices[&voice].state() == PlaybackState::Playing);
    println!(
        "native_audio,post_sleep_resume,0,{}",
        start.elapsed().as_nanos()
    );
    wait_until(|| (output.voices[&voice].position() - paused).abs() > minimum_progress);
    while let Some(error) = output.manager.backend_mut().pop_error() {
        println!("native_stream_recovered_error,{error:?}");
    }
    queue.stop(voice);
    output.process(queue).unwrap();
}

fn wait_until(mut complete: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !complete() {
        assert!(
            Instant::now() < deadline,
            "native mixer failed to progress within five seconds"
        );
        thread::sleep(Duration::from_millis(1));
    }
}

/// Distinguishes synchronous drop submission from deferred native renderer data release.
#[test]
#[ignore = "requires a native output device; run alone in release mode"]
fn measure_native_output_release() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let clip = wav_clip(2, &vec![0; 160_000]);
    for batch in 0..22 {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
        let mut output = OutputBackend::new(manager);
        let sound = super::sound_data(&clip, 0.0, true).unwrap();
        let retained = std::sync::Arc::downgrade(&sound.frames);
        let handle = output.manager.play(sound).unwrap();
        wait_until(|| handle.position() > 0.0);
        let start = Instant::now();
        drop(output);
        let signal = start.elapsed().as_nanos();
        wait_until(|| retained.upgrade().is_none());
        let release = start.elapsed().as_nanos();
        if batch >= 2 {
            println!("native_release,{batch},{signal},{release}");
        }
    }
}
