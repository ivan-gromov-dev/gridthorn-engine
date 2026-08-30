use std::time::Duration;

use crate::prelude::*;

#[test]
fn prelude_advances_a_sprite_sheet_animation() {
    let frames = vec![
        SpriteRegion::new([0.0, 0.0], [0.5, 1.0]).expect("region should be valid"),
        SpriteRegion::new([0.5, 0.0], [1.0, 1.0]).expect("region should be valid"),
    ];
    let clip = AnimationClip::new(frames, Duration::from_millis(100), AnimationPlayback::Loop)
        .expect("clip should be valid");
    let mut player = AnimationPlayer::new(clip);

    player.advance(Duration::from_millis(100));

    assert_eq!(player.frame_index(), 1);
    assert!(
        player
            .region()
            .min()
            .into_iter()
            .zip([0.5, 0.0])
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
}
