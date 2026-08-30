/// Linear RGBA color used by presentation primitives.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    rgba: [f32; 4],
}

impl Color {
    /// Create an opaque linear RGB color.
    #[must_use]
    pub fn rgb(red: f32, green: f32, blue: f32) -> Self {
        Self::rgba(red, green, blue, 1.0)
    }

    /// Create a linear RGBA color.
    #[must_use]
    pub fn rgba(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            rgba: [red, green, blue, alpha],
        }
    }

    pub(crate) fn components(self) -> [f32; 4] {
        self.rgba
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::rgb(1.0, 1.0, 1.0)
    }
}
