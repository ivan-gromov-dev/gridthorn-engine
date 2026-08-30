/// Supported physical keyboard keys.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum KeyCode {
    /// Physical W key.
    KeyW,
    /// Physical A key.
    KeyA,
    /// Physical S key.
    KeyS,
    /// Physical D key.
    KeyD,
    /// Up arrow key.
    ArrowUp,
    /// Down arrow key.
    ArrowDown,
    /// Left arrow key.
    ArrowLeft,
    /// Right arrow key.
    ArrowRight,
    /// Space key.
    Space,
    /// Enter key.
    Enter,
    /// Escape key.
    Escape,
}

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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputEvent {
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
