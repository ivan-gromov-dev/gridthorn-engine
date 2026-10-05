use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicU64, Ordering},
};

use cpal::traits::StreamTrait;

use super::*;

/// Measures Play-to-WASAPI-loopback detection, without microphone capture or recordings.
#[test]
#[ignore = "requires quiet Windows output; emits short low-volume test tones"]
fn measure_native_loopback_latency() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let device = cpal::default_host().default_output_device().unwrap();
    let config = device.default_output_config().unwrap();
    assert_eq!(config.sample_format(), cpal::SampleFormat::F32);
    assert_eq!(config.sample_rate(), 48_000);
    assert_eq!(config.channels(), 2);
    let origin = Instant::now();
    let detected = Arc::new(AtomicU64::new(0));
    let errors = Arc::new(AtomicBool::new(false));
    let peak = Arc::new(AtomicU64::new(0));
    let callback_peak = peak.clone();
    let callback_detected = detected.clone();
    let callback_errors = errors.clone();
    let capture = device
        .build_input_stream(
            config.config(),
            move |samples: &[f32], _| {
                let (present, magnitude) = tone_measurement(samples);
                callback_peak.fetch_max(magnitude.to_bits(), Ordering::Relaxed);
                if present {
                    let now = u64::try_from(origin.elapsed().as_nanos()).unwrap();
                    let _ = callback_detected.compare_exchange(
                        0,
                        now,
                        Ordering::SeqCst,
                        Ordering::SeqCst,
                    );
                }
            },
            move |error| {
                eprintln!("loopback stream error: {error}");
                callback_errors.store(true, Ordering::SeqCst);
            },
            None,
        )
        .unwrap();
    capture.play().unwrap();
    let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let mut output = OutputBackend::new(manager);
    let samples = (0..800_u16)
        .flat_map(|frame| {
            let value = [0, 2121, 3000, 2121, 0, -2121, -3000, -2121][usize::from(frame % 8)];
            [value, value]
        })
        .collect::<Vec<_>>();
    let clip = wav_clip(2, &samples);
    let mut queue = AudioCommandQueue::new();
    thread::sleep(Duration::from_secs(1));
    println!(
        "native_loopback,startup_error,{}",
        errors.swap(false, Ordering::SeqCst)
    );
    for batch in 0..22 {
        detected.store(0, Ordering::SeqCst);
        thread::sleep(Duration::from_millis(250));
        assert_eq!(
            detected.load(Ordering::SeqCst),
            0,
            "other output contains the test tone"
        );
        let voice = queue
            .play(clip.clone(), PlaybackSettings::new(0.05, false).unwrap())
            .unwrap();
        let request = u64::try_from(origin.elapsed().as_nanos()).unwrap();
        peak.store(0, Ordering::Relaxed);
        output.process(&mut queue).unwrap();
        await_detection(&detected, &peak, &output.voices[&voice]);
        let observed = detected.load(Ordering::SeqCst);
        assert!(observed >= request);
        assert!(!errors.load(Ordering::SeqCst));
        if batch >= 2 {
            println!("native_loopback,{batch},{}", observed - request);
        }
        wait_until(|| output.voices[&voice].state() == PlaybackState::Stopped);
        output.process(&mut queue).unwrap();
        thread::sleep(Duration::from_millis(100));
        assert!(output.voices.is_empty());
        assert!(!errors.load(Ordering::SeqCst));
        assert!(output.manager.backend_mut().pop_error().is_none());
    }
}

#[test]
fn loopback_detector_requires_a_coherent_finite_test_tone() {
    let tone = |period| {
        (0..96_u16)
            .flat_map(|frame| {
                let phase = f32::from(frame % period) * std::f32::consts::TAU / f32::from(period);
                let sample = phase.sin() * 0.001;
                [sample, sample]
            })
            .collect::<Vec<_>>()
    };
    assert!(tone_measurement(&tone(48)).0);
    assert!(!tone_measurement(&tone(16)).0);
    assert!(!tone_measurement(&[0.0; 192]).0);
    assert!(!tone_measurement(&[1.0; 192]).0);
    assert!(!tone_measurement(&[f32::NAN; 192]).0);
}

fn tone_measurement(samples: &[f32]) -> (bool, f64) {
    let mut present = false;
    let mut peak = 0.0_f64;
    for window in samples.as_chunks::<192>().0 {
        let (mut real, mut imaginary) = (0.0_f64, 0.0_f64);
        let mut energy = 0.0_f64;
        for (index, frame) in window.as_chunks::<2>().0.iter().enumerate() {
            let phase = f64::from(u32::try_from(index).unwrap()) * std::f64::consts::TAU / 48.0;
            let mono = f64::from(frame[0].midpoint(frame[1]));
            real += mono * phase.cos();
            imaginary += mono * phase.sin();
            energy += mono * mono;
        }
        let magnitude = real.hypot(imaginary) / 96.0;
        let rms = (energy / 96.0).sqrt();
        if magnitude.is_finite() {
            peak = peak.max(magnitude);
            present |= magnitude > 0.000_025 && magnitude > rms * 0.65;
        }
    }
    (present, peak)
}

fn await_detection(
    detected: &AtomicU64,
    peak: &AtomicU64,
    voice: &kira::sound::static_sound::StaticSoundHandle,
) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while detected.load(Ordering::SeqCst) == 0 && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(1));
    }
    assert_ne!(
        detected.load(Ordering::SeqCst),
        0,
        "loopback tone not detected: state={:?}, position={}, peak={}",
        voice.state(),
        voice.position(),
        f64::from_bits(peak.load(Ordering::Relaxed))
    );
}
