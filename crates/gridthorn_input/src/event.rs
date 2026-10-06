use crate::KeyCode;
/// Mouse button independent from a platform backend.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MouseButton {
    /// Primary mouse button.
    Left,
    /// Secondary mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Browser or device back button.
    Back,
    /// Browser or device forward button.
    Forward,
    /// Platform-specific button number.
    Other(u16),
}

/// Pressed or released digital input state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ButtonState {
    /// The button is held after this event.
    Pressed,
    /// The button is not held after this event.
    Released,
}

/// Cursor position in physical window pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CursorPosition {
    /// Horizontal physical-pixel coordinate.
    pub x: f64,
    /// Vertical physical-pixel coordinate.
    pub y: f64,
}

/// Engine-owned input event produced by a platform adapter.
#[derive(Clone, Debug, PartialEq)]
pub enum InputEvent {
    /// Controller discovery, input or feedback in arrival order.
    Controller(crate::controller::ControllerEvent),
    /// Committed Unicode text or IME lifecycle; never a physical shortcut.
    Text(crate::TextInputEvent),
    /// Applied text-session state; IME availability is reported separately.
    TextInputChanged {
        /// Whether text delivery is active.
        active: bool,
        /// Request failure, if any.
        error: Option<crate::TextInputError>,
    },
    /// Ordered result of a clipboard operation.
    Clipboard(crate::ClipboardResponse),
    /// Relative physical pointer motion while captured; never a window position.
    PointerMotion {
        /// Horizontal displacement.
        x: f64,
        /// Vertical displacement.
        y: f64,
    },
    /// Full desktop keyboard event; logical keys are not committed text.
    Key(crate::KeyboardEvent),
    /// Effective modifier state changed.
    ModifiersChanged(crate::Modifiers),
    /// Wheel motion with explicit units and gesture phase.
    MouseWheel {
        /// Scroll delta.
        delta: crate::WheelDelta,
        /// Gesture phase.
        phase: crate::ScrollPhase,
    },
    /// Result of a native pointer capture request or cancellation.
    PointerCaptureChanged(crate::PointerCaptureStatus),
    /// Window regained focus.
    FocusGained,
    /// Physical keyboard state changed.
    Keyboard {
        /// Engine-owned physical key.
        key: KeyCode,
        /// New digital state.
        state: ButtonState,
    },
    /// Mouse button state changed.
    MouseButton {
        /// Engine-owned mouse button.
        button: MouseButton,
        /// New digital state.
        state: ButtonState,
    },
    /// Cursor moved inside the window.
    CursorMoved(CursorPosition),
    /// Cursor left the window.
    CursorLeft,
    /// Window focus was lost and held inputs must be released.
    FocusLost,
}
