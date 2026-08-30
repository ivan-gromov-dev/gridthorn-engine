use super::{SpriteRegion, SpriteRegionError};

#[test]
fn accepts_normalized_atlas_regions() {
    let region = SpriteRegion::new([0.25, 0.0], [0.5, 0.5]).expect("region should be valid");

    assert_slice_close(&region.min(), &[0.25, 0.0]);
    assert_slice_close(&region.max(), &[0.5, 0.5]);
}

fn assert_slice_close(actual: &[f32], expected: &[f32]) {
    assert!(
        actual
            .iter()
            .zip(expected)
            .all(|(actual, expected)| (actual - expected).abs() < f32::EPSILON)
    );
}

#[test]
fn rejects_reversed_and_out_of_range_regions() {
    assert_eq!(
        SpriteRegion::new([0.5, 0.0], [0.25, 0.5]),
        Err(SpriteRegionError::InvalidBounds)
    );
    assert_eq!(
        SpriteRegion::new([0.0, 0.0], [1.1, 1.0]),
        Err(SpriteRegionError::InvalidBounds)
    );
}
