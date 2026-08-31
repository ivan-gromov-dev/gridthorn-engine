use std::ops::Neg;

/// Engine-owned two-dimensional vector used by collision queries.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec2 {
    /// Horizontal component.
    pub x: f32,
    /// Vertical component.
    pub y: f32,
}

impl Vec2 {
    /// Creates a vector from horizontal and vertical components.
    #[must_use]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub(crate) fn is_finite(self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    pub(crate) fn length_squared(self) -> f32 {
        self.x.mul_add(self.x, self.y * self.y)
    }
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}
