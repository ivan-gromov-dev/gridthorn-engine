mod errors;

pub use errors::UiError;

use super::Color;

/// One colored rectangle in top-left-origin screen pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiRect {
    position: [f32; 2],
    size: [f32; 2],
    color: Color,
}

impl UiRect {
    /// Construct a screen-space rectangle.
    ///
    /// # Errors
    ///
    /// Returns [`UiError::InvalidPosition`] for non-finite coordinates and
    /// [`UiError::InvalidSize`] for non-finite or non-positive dimensions.
    pub fn new(position: [f32; 2], size: [f32; 2], color: Color) -> Result<Self, UiError> {
        if !position.into_iter().all(f32::is_finite) {
            return Err(UiError::InvalidPosition);
        }
        if !size
            .into_iter()
            .all(|dimension| dimension.is_finite() && dimension > 0.0)
        {
            return Err(UiError::InvalidSize);
        }
        Ok(Self {
            position,
            size,
            color,
        })
    }

    /// Top-left position in physical screen pixels.
    #[must_use]
    pub const fn position(self) -> [f32; 2] {
        self.position
    }

    /// Width and height in physical screen pixels.
    #[must_use]
    pub const fn size(self) -> [f32; 2] {
        self.size
    }

    /// Fill color.
    #[must_use]
    pub const fn color(self) -> Color {
        self.color
    }
}

/// Bitmap text in top-left-origin screen pixels.
#[derive(Clone, Debug, PartialEq)]
pub struct TextLabel {
    text: String,
    position: [f32; 2],
    pixel_scale: f32,
    color: Color,
}

impl TextLabel {
    /// Construct a bitmap text label.
    ///
    /// Lowercase Latin letters use the matching uppercase glyph. Unsupported
    /// Unicode scalars render as `?`; newlines begin a new text row.
    ///
    /// # Errors
    ///
    /// Returns [`UiError::InvalidPosition`] for non-finite coordinates and
    /// [`UiError::InvalidPixelScale`] for a non-finite or non-positive scale.
    pub fn new(
        text: impl Into<String>,
        position: [f32; 2],
        pixel_scale: f32,
        color: Color,
    ) -> Result<Self, UiError> {
        if !position.into_iter().all(f32::is_finite) {
            return Err(UiError::InvalidPosition);
        }
        if !pixel_scale.is_finite() || pixel_scale <= 0.0 {
            return Err(UiError::InvalidPixelScale);
        }
        Ok(Self {
            text: text.into(),
            position,
            pixel_scale,
            color,
        })
    }

    /// Label text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Top-left position in physical screen pixels.
    #[must_use]
    pub const fn position(&self) -> [f32; 2] {
        self.position
    }

    /// Physical size of one 5x7 bitmap pixel.
    #[must_use]
    pub const fn pixel_scale(&self) -> f32 {
        self.pixel_scale
    }

    /// Glyph color.
    #[must_use]
    pub const fn color(&self) -> Color {
        self.color
    }
}

/// Ordered screen-space primitive rendered after world presentation.
#[derive(Clone, Debug, PartialEq)]
pub enum UiPrimitive {
    /// Filled rectangle.
    Rect(UiRect),
    /// Built-in 5x7 bitmap text.
    Text(TextLabel),
}

impl From<UiRect> for UiPrimitive {
    fn from(rect: UiRect) -> Self {
        Self::Rect(rect)
    }
}

impl From<TextLabel> for UiPrimitive {
    fn from(text: TextLabel) -> Self {
        Self::Text(text)
    }
}

#[cfg(test)]
mod test;
