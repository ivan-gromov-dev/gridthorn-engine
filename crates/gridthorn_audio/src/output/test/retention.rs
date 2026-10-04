use std::sync::Arc;

use super::*;
use crate::output::prepared_batch::PreparedBatch;

#[test]
fn reuses_shared_samples_with_independent_playback_settings() {
    let clip = stereo_clip();
    let mut batch = PreparedBatch::default();
    let first = batch.sound(&clip, 1.0, false).unwrap();
    let second = batch.sound(&clip.clone(), 0.25, true).unwrap();
    assert!(Arc::ptr_eq(&first.frames, &second.frames));
    assert_ne!(first.settings, second.settings);
    assert_eq!(
        &*first.frames,
        &*sound_data(&clip, 1.0, false).unwrap().frames
    );
    let separate = batch.sound(&stereo_clip(), 1.0, false).unwrap();
    assert!(!Arc::ptr_eq(&first.frames, &separate.frames));
    let next = PreparedBatch::default().sound(&clip, 1.0, false).unwrap();
    assert!(!Arc::ptr_eq(&first.frames, &next.frames));
}

#[test]
fn oversized_frames_bypass_reuse() {
    let clip = wav_clip(2, &vec![0; 300_000]);
    let mut batch = PreparedBatch::default();
    let first = batch.sound(&clip, 1.0, false).unwrap();
    let second = batch.sound(&clip, 1.0, false).unwrap();
    assert!(!Arc::ptr_eq(&first.frames, &second.frames));
}

#[test]
fn conversion_reuse_respects_entry_and_combined_byte_limits() {
    let mut batch = PreparedBatch::default();
    for _ in 0..16 {
        batch.sound(&mono_clip(), 1.0, false).unwrap();
    }
    let overflow = mono_clip();
    let first = batch.sound(&overflow, 1.0, false).unwrap();
    let second = batch.sound(&overflow, 1.0, false).unwrap();
    assert!(!Arc::ptr_eq(&first.frames, &second.frames));

    let mut batch = PreparedBatch::default();
    let retained = wav_clip(2, &vec![0; 160_000]);
    let overflow = wav_clip(2, &vec![1; 160_000]);
    let first = batch.sound(&retained, 1.0, false).unwrap();
    let second = batch.sound(&retained, 1.0, false).unwrap();
    assert!(Arc::ptr_eq(&first.frames, &second.frames));
    let first = batch.sound(&overflow, 1.0, false).unwrap();
    let second = batch.sound(&overflow, 1.0, false).unwrap();
    assert!(!Arc::ptr_eq(&first.frames, &second.frames));
}

#[test]
fn repeated_completed_playback_does_not_accumulate_handles() {
    let manager = AudioManager::<MockBackend>::new(AudioManagerSettings::default()).unwrap();
    let mut output = OutputBackend::new(manager);
    let mut queue = AudioCommandQueue::new();
    for _ in 0..128 {
        queue
            .play(mono_clip(), PlaybackSettings::default())
            .unwrap();
        output.process(&mut queue).unwrap();
        let backend = output.manager.backend_mut();
        backend.on_start_processing();
        for _ in 0..4 {
            backend.process();
        }
        output.process(&mut queue).unwrap();
        assert!(output.voices.is_empty());
    }
}

#[test]
fn reclaims_completed_voices_on_empty_batches_but_preserves_loops() {
    let manager = AudioManager::<MockBackend>::new(AudioManagerSettings::default()).unwrap();
    let mut output = OutputBackend::new(manager);
    let mut queue = AudioCommandQueue::new();
    queue
        .play(stereo_clip(), PlaybackSettings::default())
        .unwrap();
    let looping = queue
        .play(stereo_clip(), PlaybackSettings::new(1.0, true).unwrap())
        .unwrap();
    output.process(&mut queue).unwrap();
    let backend = output.manager.backend_mut();
    backend.on_start_processing();
    for _ in 0..4 {
        backend.process();
    }
    assert_eq!(output.active_voice_count(), 1);
    output.process(&mut queue).unwrap();
    assert_eq!(output.voices.len(), 1);
    assert!(output.voices.contains_key(&looping));
    output.suspend();
    output.process(&mut queue).unwrap();
    assert_eq!(output.voices.len(), 1);
}
