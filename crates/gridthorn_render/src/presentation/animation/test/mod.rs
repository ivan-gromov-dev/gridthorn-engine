use std::time::Duration;

use super::{AnimationClip, AnimationClipError, AnimationPlayback, AnimationPlayer};
use crate::SpriteRegion;

fn regions() -> Vec<SpriteRegion> {
    vec![
        SpriteRegion::new([0.0, 0.0], [0.5, 1.0]).expect("first region should be valid"),
        SpriteRegion::new([0.5, 0.0], [1.0, 1.0]).expect("second region should be valid"),
    ]
}

#[test]
fn rejects_clips_that_cannot_advance() {
    assert_eq!(
        AnimationClip::new(
            Vec::new(),
            Duration::from_millis(100),
            AnimationPlayback::Loop
        ),
        Err(AnimationClipError::Empty)
    );
    assert_eq!(
        AnimationClip::new(regions(), Duration::ZERO, AnimationPlayback::Loop),
        Err(AnimationClipError::ZeroFrameDuration)
    );
}

#[test]
fn loops_across_large_elapsed_updates_without_iteration() {
    let clip = AnimationClip::new(
        regions(),
        Duration::from_millis(100),
        AnimationPlayback::Loop,
    )
    .expect("clip should be valid");
    let mut player = AnimationPlayer::new(clip);

    player.advance(Duration::from_millis(10_150));

    assert_eq!(player.frame_index(), 1);
    assert!(!player.is_finished());
}

#[test]
fn one_shot_holds_final_frame_and_can_restart() {
    let clip = AnimationClip::new(
        regions(),
        Duration::from_millis(50),
        AnimationPlayback::Once,
    )
    .expect("clip should be valid");
    let mut player = AnimationPlayer::new(clip);

    player.advance(Duration::from_millis(100));
    assert_eq!(player.frame_index(), 1);
    assert!(player.is_finished());

    player.restart();
    assert_eq!(player.frame_index(), 0);
    assert!(!player.is_finished());
}

#[test]
fn pause_preserves_the_current_frame() {
    let clip = AnimationClip::new(
        regions(),
        Duration::from_millis(10),
        AnimationPlayback::Loop,
    )
    .expect("clip should be valid");
    let mut player = AnimationPlayer::new(clip);
    player.pause();

    player.advance(Duration::from_secs(1));

    assert_eq!(player.frame_index(), 0);
    assert!(!player.is_playing());
}
