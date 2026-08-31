//! Provisional engine-owned shapes and narrow-phase 2D collision queries.

mod contact;
mod errors;
mod math;
mod shape;

pub use contact::{Contact2d, contact, overlaps};
pub use errors::ColliderError;
pub use math::Vec2;
pub use shape::{Aabb2d, Circle2d, Collider2d};
