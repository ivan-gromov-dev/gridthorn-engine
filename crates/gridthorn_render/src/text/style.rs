use super::TextError;

/// Unicode line-breaking policy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextWrap {
    /// Preserve authored line breaks only.
    None,
    /// Wrap on word boundaries; long words may exceed the width.
    Word,
    /// Wrap at glyph cluster boundaries.
    Glyph,
    /// Prefer word boundaries and break long words by cluster.
    #[default]
    WordOrGlyph,
}

/// Horizontal alignment within the optional layout width.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextAlignment {
    /// Follow the paragraph's Unicode base direction.
    #[default]
    Start,
    /// Physical left edge.
    Left,
    /// Physical right edge.
    Right,
    /// Centered text.
    Center,
}

/// Game-owned font selection and logical-pixel paragraph metrics.
#[derive(Clone, Debug, PartialEq)]
pub struct TextStyle {
    /// Registered primary family; other supplied assets provide script fallback.
    pub family: String,
    /// Font em size in logical pixels.
    pub font_size: f32,
    /// Distance between line baselines in logical pixels.
    pub line_height: f32,
    /// Optional logical layout width, independent of DPI.
    pub width: Option<f32>,
    /// Wrapping policy.
    pub wrap: TextWrap,
    /// Horizontal alignment.
    pub alignment: TextAlignment,
}

impl TextStyle {
    /// Construct an unbounded, direction-aware style with a 1.4× line height.
    #[must_use]
    pub fn new(family: impl Into<String>, font_size: f32) -> Self {
        Self {
            family: family.into(),
            font_size,
            line_height: font_size * 1.4,
            width: None,
            wrap: TextWrap::default(),
            alignment: TextAlignment::default(),
        }
    }

    pub(super) fn validate(&self) -> Result<(), TextError> {
        let valid = |value: f32, maximum: f32| value.is_finite() && value > 0.0 && value <= maximum;
        if !valid(self.font_size, 1024.0)
            || !valid(self.line_height, 1024.0)
            || self.width.is_some_and(|width| !valid(width, 65536.0))
        {
            return Err(TextError::InvalidMetrics);
        }
        Ok(())
    }
}
