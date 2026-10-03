use gridthorn_render::{Color, TextStyle};

/// Logical-pixel sizing relative to the parent's content box.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum UiLength {
    /// Intrinsic content plus padding.
    #[default]
    Auto,
    /// Fixed logical pixels.
    Pixels(f32),
    /// Fraction of the parent's available extent, from zero to one.
    Fraction(f32),
    /// Equal share of remaining space in a row/column; full extent in an overlay.
    Fill,
}

/// Child placement order; children paint in declaration order.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UiFlow {
    /// Children share the content box and use their anchor.
    #[default]
    Overlay,
    /// Children advance left to right.
    Row,
    /// Children advance top to bottom.
    Column,
}

/// Alignment along an axis; in a flow only the cross-axis anchor applies.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UiAnchor {
    /// Near edge.
    #[default]
    Start,
    /// Center.
    Center,
    /// Far edge.
    End,
}

impl UiAnchor {
    pub(super) const fn factor(self) -> f32 {
        match self {
            Self::Start => 0.0,
            Self::Center => 0.5,
            Self::End => 1.0,
        }
    }
}

/// Game-owned per-node style. All geometry uses logical pixels.
#[derive(Clone, Debug)]
pub struct UiStyle {
    /// Width and height policies.
    pub size: [UiLength; 2],
    /// Minimum width/height, including padding.
    pub min_size: [f32; 2],
    /// Maximum width/height, including padding.
    pub max_size: [f32; 2],
    /// Insets: left, top, right, bottom.
    pub padding: [f32; 4],
    /// Space between flow children.
    pub gap: f32,
    /// Composition direction.
    pub flow: UiFlow,
    /// Horizontal and vertical anchor.
    pub anchor: [UiAnchor; 2],
    /// Offset after layout; does not alter flow advance.
    pub offset: [f32; 2],
    /// Clip descendants to the content box.
    pub clip: bool,
    /// Enable clamped two-axis scrolling, implying content clipping.
    pub scroll: bool,
    /// Optional background override.
    pub background: Option<Color>,
    /// Optional foreground override.
    pub foreground: Option<Color>,
}

impl Default for UiStyle {
    fn default() -> Self {
        Self {
            size: [UiLength::Auto; 2],
            min_size: [0.0; 2],
            max_size: [65536.0; 2],
            padding: [0.0; 4],
            gap: 0.0,
            flow: UiFlow::Overlay,
            anchor: [UiAnchor::Start; 2],
            offset: [0.0; 2],
            clip: false,
            scroll: false,
            background: None,
            foreground: None,
        }
    }
}

/// Reusable theme for controls; node colors override this palette.
#[derive(Clone, Debug)]
pub struct UiTheme {
    /// Normal control surface.
    pub surface: Color,
    /// Hover surface.
    pub hovered: Color,
    /// Held surface.
    pub pressed: Color,
    /// Disabled surface.
    pub disabled: Color,
    /// Selected indicator and slider fill.
    pub accent: Color,
    /// Text color.
    pub foreground: Color,
    /// Optional asset-font style; otherwise use the diagnostic bitmap font.
    pub text: Option<TextStyle>,
    /// Bitmap font pixel scale in logical units.
    pub bitmap_scale: f32,
    /// Height of a list row in logical pixels.
    pub row_height: f32,
}

impl Default for UiTheme {
    fn default() -> Self {
        Self {
            surface: Color::rgb(0.12, 0.15, 0.2),
            hovered: Color::rgb(0.2, 0.25, 0.32),
            pressed: Color::rgb(0.08, 0.1, 0.14),
            disabled: Color::rgb(0.2, 0.2, 0.2),
            accent: Color::rgb(0.2, 0.65, 0.9),
            foreground: Color::default(),
            text: None,
            bitmap_scale: 2.0,
            row_height: 28.0,
        }
    }
}
