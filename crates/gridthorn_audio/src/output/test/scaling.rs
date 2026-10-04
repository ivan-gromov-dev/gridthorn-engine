use std::{hint::black_box, time::Instant};

use super::{
    AudioCommandQueue, AudioManager, AudioManagerSettings, MockBackend, OutputBackend,
    PlaybackSettings, wav_clip,
};

/// Isolates frame conversion and sound-data construction from manager submission.
#[test]
#[ignore = "manual PCM conversion probe; run alone with --release"]
fn measure_pcm_conversion_reuse() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    let clip = wav_clip(2, &vec![4_096; 160_000]);
    println!("pcm_conversion,mode,batch,elapsed_ns");
    for batch in 0..22 {
        for reuse in [false, true] {
            let mut prepared = crate::output::prepared_batch::PreparedBatch::default();
            let start = Instant::now();
            let sounds = (0..64)
                .map(|_| {
                    if reuse {
                        prepared.sound(black_box(&clip), 0.5, false).unwrap()
                    } else {
                        super::sound_data(black_box(&clip), 0.5, false).unwrap()
                    }
                })
                .collect::<Vec<_>>();
            let elapsed = start.elapsed().as_nanos();
            assert!(sounds.iter().all(|sound| sound.frames.len() == 80_000));
            assert_eq!(sounds[0].frames, sounds[63].frames);
            if batch >= 2 {
                println!("pcm_conversion,{reuse},{batch},{elapsed}");
            }
        }
    }
}

/// Manual control-side probe; the mock backend does not measure device or mixer latency.
#[test]
#[ignore = "manual audio scaling probe; run alone with --release"]
fn measure_audio_control_scaling() {
    assert!(!black_box(cfg!(debug_assertions)), "run with --release");
    println!("audio_control,voices,frames,batch,phase,elapsed_ns");
    for voices in [1, 16, 64] {
        for frames in [256, 8_000, 80_000] {
            let clip = wav_clip(2, &vec![4_096; frames * 2]);
            for batch in 0..22 {
                let manager =
                    AudioManager::<MockBackend>::new(AudioManagerSettings::default()).unwrap();
                let mut output = OutputBackend::new(manager);
                let mut commands = AudioCommandQueue::new();
                let start = Instant::now();
                let ids = (0..voices)
                    .map(|_| {
                        commands
                            .play(clip.clone(), PlaybackSettings::default())
                            .unwrap()
                    })
                    .collect::<Vec<_>>();
                let enqueue = start.elapsed().as_nanos();
                assert_eq!(commands.len(), voices);
                let start = Instant::now();
                output.process(&mut commands).unwrap();
                let play = start.elapsed().as_nanos();
                assert_eq!(output.active_voice_count(), voices);
                assert!(commands.is_empty());
                let start = Instant::now();
                output.suspend();
                let suspend = start.elapsed().as_nanos();
                assert!(output.suspended);
                let start = Instant::now();
                output.resume();
                let resume = start.elapsed().as_nanos();
                assert!(!output.suspended);
                for id in ids {
                    commands.set_volume(id, 0.25).unwrap();
                    commands.stop(id);
                }
                let start = Instant::now();
                output.process(&mut commands).unwrap();
                let controls = start.elapsed().as_nanos();
                assert_eq!(output.active_voice_count(), 0);
                assert!(commands.is_empty());
                let start = Instant::now();
                drop(output);
                let teardown = start.elapsed().as_nanos();
                if batch >= 2 {
                    for (phase, elapsed) in [
                        ("enqueue", enqueue),
                        ("play", play),
                        ("suspend", suspend),
                        ("resume", resume),
                        ("controls", controls),
                        ("teardown", teardown),
                    ] {
                        println!("audio_control,{voices},{frames},{batch},{phase},{elapsed}");
                    }
                }
            }
        }
    }
}
