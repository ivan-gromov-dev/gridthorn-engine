use crate::{Aabb2d, Circle2d, Collider2d, Vec2, contact, overlaps};

fn aabb(center: [f32; 2], half_extents: [f32; 2]) -> Collider2d {
    Aabb2d::new(
        Vec2::new(center[0], center[1]),
        Vec2::new(half_extents[0], half_extents[1]),
    )
    .expect("box should be valid")
    .into()
}

fn circle(center: [f32; 2], radius: f32) -> Collider2d {
    Circle2d::new(Vec2::new(center[0], center[1]), radius)
        .expect("circle should be valid")
        .into()
}

#[test]
fn separated_shapes_do_not_contact() {
    assert!(!overlaps(
        aabb([0.0, 0.0], [1.0, 1.0]),
        aabb([3.0, 0.0], [1.0, 1.0])
    ));
    assert!(!overlaps(circle([0.0, 0.0], 1.0), circle([3.0, 0.0], 1.0)));
    assert!(!overlaps(
        circle([0.0, 3.0], 1.0),
        aabb([0.0, 0.0], [1.0, 1.0])
    ));
}

#[test]
fn touching_boxes_report_zero_penetration() {
    let contact = contact(aabb([0.0, 0.0], [1.0, 1.0]), aabb([2.0, 0.0], [1.0, 1.0]))
        .expect("touching boxes should contact");

    assert_eq!(contact.normal(), Vec2::new(-1.0, 0.0));
    assert!(contact.penetration().abs() < f32::EPSILON);
}

#[test]
fn circle_contacts_are_symmetric() {
    let first = circle([0.0, 0.0], 2.0);
    let second = circle([3.0, 0.0], 2.0);
    let forward = contact(first, second).expect("circles should overlap");
    let reverse = contact(second, first).expect("circles should overlap");

    assert_eq!(forward.normal(), -reverse.normal());
    assert!((forward.penetration() - 1.0).abs() < f32::EPSILON);
    assert!((forward.penetration() - reverse.penetration()).abs() < f32::EPSILON);
}

#[test]
fn circle_box_contact_handles_corners_and_swapped_order() {
    let circle = circle([1.5, 1.5], 1.0);
    let box_collider = aabb([0.0, 0.0], [1.0, 1.0]);
    let forward = contact(circle, box_collider).expect("circle should overlap the corner");
    let reverse = contact(box_collider, circle).expect("swapped shapes should overlap");

    assert!((forward.penetration() - (1.0 - 0.5_f32.sqrt())).abs() < 0.000_01);
    assert_eq!(forward.normal(), -reverse.normal());
}

#[test]
fn contained_circle_uses_nearest_box_edge() {
    let contact = contact(circle([1.5, 0.0], 1.0), aabb([0.0, 0.0], [2.0, 3.0]))
        .expect("contained circle should contact");

    assert_eq!(contact.normal(), Vec2::new(1.0, 0.0));
    assert!((contact.penetration() - 1.5).abs() < f32::EPSILON);
}
