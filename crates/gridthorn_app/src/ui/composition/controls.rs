/// Reusable control content. Game rules and validation remain caller-owned.
#[derive(Clone, Debug, PartialEq)]
pub enum UiControl {
    /// Composition container.
    Panel,
    /// Display-only text.
    Label(String),
    /// Activatable caption.
    Button(String),
    /// Boolean value with caption.
    Toggle {
        /// Caption.
        label: String,
        /// Current value.
        checked: bool,
    },
    /// Finite continuous scalar range, with minimum strictly below maximum.
    Slider {
        /// Lower bound.
        min: f32,
        /// Upper bound.
        max: f32,
        /// Current value.
        value: f32,
    },
    /// Ordered rows with optional single selection.
    List {
        /// Row captions.
        items: Vec<String>,
        /// Selected row index.
        selected: Option<usize>,
    },
    /// UTF-8 value and empty-value placeholder; editing commands arrive from the caller.
    TextField {
        /// Committed value.
        value: String,
        /// Empty-value caption.
        placeholder: String,
    },
}

/// Explicit visual input, independent of native event arbitration.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UiVisualState {
    /// Idle.
    #[default]
    Normal,
    /// Pointer or navigation hover.
    Hovered,
    /// Held interaction.
    Pressed,
    /// Reject control commands and use disabled styling.
    Disabled,
}

/// Explicit value commands; event routing and text selection are a later increment.
#[derive(Clone, Debug, PartialEq)]
pub enum UiCommand {
    /// Activate a button or invert a toggle.
    Activate,
    /// Set a toggle value.
    SetChecked(bool),
    /// Set a slider value; finite input is clamped to its range.
    SetValue(f32),
    /// Set/clear a list selection.
    Select(Option<usize>),
    /// Replace committed UTF-8 text.
    SetText(String),
    /// Append committed UTF-8 text.
    AppendText(String),
    /// Remove the final Unicode scalar (not a grapheme-aware editor).
    PopText,
    /// Set requested nonnegative scroll offset; layout clamps it to content bounds.
    ScrollTo([f32; 2]),
    /// Set visual state.
    Visual(UiVisualState),
}

/// Caller-visible change, suitable for mapping into game commands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiEffect {
    /// No value change or activation; visual/scroll commands also return this effect.
    None,
    /// Button activation.
    Activated,
    /// Control value changed.
    Changed,
}
