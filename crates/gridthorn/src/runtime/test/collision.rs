use crate::prelude::{Aabb2d, Circle2d, Collider2d, Vec2};
use crate::{Contact2d, contact, overlaps};

#[test]
fn facade_exposes_basic_collision_queries() {
    let player: Collider2d = Circle2d::new(Vec2::new(1.5, 0.0), 1.0)
        .expect("player collider should be valid")
        .into();
    let wall: Collider2d = Aabb2d::new(Vec2::default(), Vec2::new(1.0, 4.0))
        .expect("wall collider should be valid")
        .into();

    assert!(overlaps(player, wall));
    assert_eq!(
        contact(player, wall).map(Contact2d::normal),
        Some(Vec2::new(1.0, 0.0))
    );
}
