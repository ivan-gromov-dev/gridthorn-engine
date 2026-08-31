use crate::{Aabb2d, Circle2d, Collider2d, Vec2};

/// Minimum translation needed to separate the first collider from the second.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Contact2d {
    normal: Vec2,
    penetration: f32,
}

impl Contact2d {
    /// Unit direction in which the first collider can be moved out of the second.
    #[must_use]
    pub const fn normal(self) -> Vec2 {
        self.normal
    }

    /// Non-negative distance to move the first collider along [`Self::normal`].
    #[must_use]
    pub const fn penetration(self) -> f32 {
        self.penetration
    }
}

/// Returns whether two colliders overlap or touch.
#[must_use]
pub fn overlaps(first: Collider2d, second: Collider2d) -> bool {
    contact(first, second).is_some()
}

/// Computes a contact that separates the first collider from the second.
///
/// Touching shapes return a contact with zero penetration. When multiple
/// minimum translations are equal, the horizontal axis is selected.
#[must_use]
pub fn contact(first: Collider2d, second: Collider2d) -> Option<Contact2d> {
    match (first, second) {
        (Collider2d::Aabb(first), Collider2d::Aabb(second)) => aabb_contact(first, second),
        (Collider2d::Circle(first), Collider2d::Circle(second)) => circle_contact(first, second),
        (Collider2d::Circle(circle), Collider2d::Aabb(aabb)) => circle_aabb_contact(circle, aabb),
        (Collider2d::Aabb(aabb), Collider2d::Circle(circle)) => circle_aabb_contact(circle, aabb)
            .map(|contact| Contact2d {
                normal: -contact.normal,
                penetration: contact.penetration,
            }),
    }
}

fn aabb_contact(first: Aabb2d, second: Aabb2d) -> Option<Contact2d> {
    let delta = Vec2::new(
        first.center().x - second.center().x,
        first.center().y - second.center().y,
    );
    let overlap_x = first.half_extents().x + second.half_extents().x - delta.x.abs();
    let overlap_y = first.half_extents().y + second.half_extents().y - delta.y.abs();
    if overlap_x < 0.0 || overlap_y < 0.0 {
        return None;
    }
    if overlap_x <= overlap_y {
        Some(Contact2d {
            normal: Vec2::new(non_zero_sign(delta.x), 0.0),
            penetration: overlap_x,
        })
    } else {
        Some(Contact2d {
            normal: Vec2::new(0.0, non_zero_sign(delta.y)),
            penetration: overlap_y,
        })
    }
}

fn circle_contact(first: Circle2d, second: Circle2d) -> Option<Contact2d> {
    let delta = Vec2::new(
        first.center().x - second.center().x,
        first.center().y - second.center().y,
    );
    let radius = first.radius() + second.radius();
    let distance_squared = delta.length_squared();
    if distance_squared > radius * radius {
        return None;
    }
    if distance_squared == 0.0 {
        return Some(Contact2d {
            normal: Vec2::new(1.0, 0.0),
            penetration: radius,
        });
    }
    let distance = distance_squared.sqrt();
    Some(Contact2d {
        normal: Vec2::new(delta.x / distance, delta.y / distance),
        penetration: radius - distance,
    })
}

fn circle_aabb_contact(circle: Circle2d, aabb: Aabb2d) -> Option<Contact2d> {
    let offset = Vec2::new(
        circle.center().x - aabb.center().x,
        circle.center().y - aabb.center().y,
    );
    let closest = Vec2::new(
        offset
            .x
            .clamp(-aabb.half_extents().x, aabb.half_extents().x),
        offset
            .y
            .clamp(-aabb.half_extents().y, aabb.half_extents().y),
    );
    let delta = Vec2::new(offset.x - closest.x, offset.y - closest.y);
    let distance_squared = delta.length_squared();
    if distance_squared > circle.radius() * circle.radius() {
        return None;
    }
    if distance_squared > 0.0 {
        let distance = distance_squared.sqrt();
        return Some(Contact2d {
            normal: Vec2::new(delta.x / distance, delta.y / distance),
            penetration: circle.radius() - distance,
        });
    }
    let distance_x = aabb.half_extents().x - offset.x.abs();
    let distance_y = aabb.half_extents().y - offset.y.abs();
    if distance_x <= distance_y {
        Some(Contact2d {
            normal: Vec2::new(non_zero_sign(offset.x), 0.0),
            penetration: circle.radius() + distance_x,
        })
    } else {
        Some(Contact2d {
            normal: Vec2::new(0.0, non_zero_sign(offset.y)),
            penetration: circle.radius() + distance_y,
        })
    }
}

fn non_zero_sign(value: f32) -> f32 {
    if value < 0.0 { -1.0 } else { 1.0 }
}

#[cfg(test)]
mod test;
