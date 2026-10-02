use crate::{ButtonState, KeyCode, NamedKey};

/// Platform-scoped identity for keys lacking a standardized mapping.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NativeKey {
    /// No identity supplied.
    Unidentified,
    /// Android code.
    Android(u32),
    /// macOS code.
    MacOS(u16),
    /// Windows code.
    Windows(u16),
    /// XKB code.
    Xkb(u32),
    /// Web key identity.
    Web(String),
}

/// Layout-independent keyboard position.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PhysicalKey {
    /// Standardized position.
    Code(KeyCode),
    /// Native position, meaningful only on its originating platform.
    Unidentified(NativeKey),
}

/// Layout-dependent shortcut identity, separate from text input.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum LogicalKey {
    /// Named function or control key.
    Named(NamedKey),
    /// Character sequence reported by the active layout.
    Character(String),
    /// Dead key, optionally with its accent.
    Dead(Option<char>),
    /// Native logical identity.
    Unidentified(NativeKey),
}

/// Location of a logical key on the keyboard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KeyLocation {
    /// Ordinary key.
    Standard,
    /// Left-side key.
    Left,
    /// Right-side key.
    Right,
    /// Numeric keypad key.
    Numpad,
}

/// Effective aggregate modifier state; left/right identities remain physical keys.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "modifier flags are independent and may be held in any combination"
)]
pub struct Modifiers {
    /// Shift is held.
    pub shift: bool,
    /// Control is held.
    pub control: bool,
    /// Alt is held.
    pub alt: bool,
    /// Super/Windows/Command is held.
    pub super_key: bool,
}

/// One desktop key transition in platform arrival order.
#[derive(Clone, Debug, PartialEq)]
pub struct KeyboardEvent {
    /// Layout-independent identity.
    pub physical_key: PhysicalKey,
    /// Layout-dependent identity; never an IME commit.
    pub logical_key: LogicalKey,
    /// Keyboard location.
    pub location: KeyLocation,
    /// Digital transition.
    pub state: ButtonState,
    /// Whether this is an operating-system repeat.
    pub repeat: bool,
    /// Whether the platform synthesized the event during focus synchronization.
    pub synthetic: bool,
}
