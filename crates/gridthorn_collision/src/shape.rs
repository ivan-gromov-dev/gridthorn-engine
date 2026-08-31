use crate::{ColliderError, Vec2};

/// Axis-aligned box described by its center and non-negative half extents.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Aabb2d {
    center: Vec2,
    half_extents: Vec2,
}

impl Aabb2d {
    /// Creates a box after validating that all values are finite and extents are non-negative.
    ///
    /// # Errors
    ///
    /// Returns [`ColliderError::NonFinite`] for non-finite values and
    /// [`ColliderError::NegativeHalfExtent`] for a negative half extent.
    pub fn new(center: Vec2, half_extents: Vec2) -> Result<Self, ColliderError> {
        if !center.is_finite() || !half_extents.is_finite() {
            return Err(ColliderError::NonFinite);
        }
        if half_extents.x < 0.0 || half_extents.y < 0.0 {
            return Err(ColliderError::NegativeHalfExtent);
        }
        Ok(Self {
            center,
            half_extents,
        })
    }

    /// Center of the box in world coordinates.
    #[must_use]
    pub const fn center(self) -> Vec2 {
        self.center
    }

    /// Non-negative distances from the center to each edge.
    #[must_use]
    pub const fn half_extents(self) -> Vec2 {
        self.half_extents
    }
}

/// Circle described by its center and non-negative radius.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle2d {
    center: Vec2,
    radius: f32,
}

impl Circle2d {
    /// Creates a circle after validating that its values are finite and non-negative.
    ///
    /// # Errors
    ///
    /// Returns [`ColliderError::NonFinite`] for non-finite values and
    /// [`ColliderError::NegativeRadius`] for a negative radius.
    pub fn new(center: Vec2, radius: f32) -> Result<Self, ColliderError> {
        if !center.is_finite() || !radius.is_finite() {
            return Err(ColliderError::NonFinite);
        }
        if radius < 0.0 {
            return Err(ColliderError::NegativeRadius);
        }
        Ok(Self { center, radius })
    }

    /// Center of the circle in world coordinates.
    #[must_use]
    pub const fn center(self) -> Vec2 {
        self.center
    }

    /// Non-negative circle radius.
    #[must_use]
    pub const fn radius(self) -> f32 {
        self.radius
    }
}

/// Supported basic 2D collider shapes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Collider2d {
    /// Axis-aligned box collider.
    Aabb(Aabb2d),
    /// Circle collider.
    Circle(Circle2d),
}

impl From<Aabb2d> for Collider2d {
    fn from(value: Aabb2d) -> Self {
        Self::Aabb(value)
    }
}

impl From<Circle2d> for Collider2d {
    fn from(value: Circle2d) -> Self {
        Self::Circle(value)
    }
}

#[cfg(test)]
mod test;
