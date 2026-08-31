use crate::{Aabb2d, Circle2d, ColliderError, Vec2};

#[test]
fn constructors_reject_invalid_geometry() {
    assert_eq!(
        Aabb2d::new(Vec2::default(), Vec2::new(-1.0, 2.0)),
        Err(ColliderError::NegativeHalfExtent)
    );
    assert_eq!(
        Circle2d::new(Vec2::new(f32::NAN, 0.0), 1.0),
        Err(ColliderError::NonFinite)
    );
    assert_eq!(
        Circle2d::new(Vec2::default(), -1.0),
        Err(ColliderError::NegativeRadius)
    );
}
