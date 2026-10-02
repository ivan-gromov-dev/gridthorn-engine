/// Engine-owned `KeyCode` values for desktop keyboard input.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum KeyCode {
    /// A future unmapped backend key.
    Unidentified,
    /// The `Backquote` key.
    Backquote,
    /// The `Backslash` key.
    Backslash,
    /// The `BracketLeft` key.
    BracketLeft,
    /// The `BracketRight` key.
    BracketRight,
    /// The `Comma` key.
    Comma,
    /// The `Digit0` key.
    Digit0,
    /// The `Digit1` key.
    Digit1,
    /// The `Digit2` key.
    Digit2,
    /// The `Digit3` key.
    Digit3,
    /// The `Digit4` key.
    Digit4,
    /// The `Digit5` key.
    Digit5,
    /// The `Digit6` key.
    Digit6,
    /// The `Digit7` key.
    Digit7,
    /// The `Digit8` key.
    Digit8,
    /// The `Digit9` key.
    Digit9,
    /// The `Equal` key.
    Equal,
    /// The `IntlBackslash` key.
    IntlBackslash,
    /// The `IntlRo` key.
    IntlRo,
    /// The `IntlYen` key.
    IntlYen,
    /// The `KeyA` key.
    KeyA,
    /// The `KeyB` key.
    KeyB,
    /// The `KeyC` key.
    KeyC,
    /// The `KeyD` key.
    KeyD,
    /// The `KeyE` key.
    KeyE,
    /// The `KeyF` key.
    KeyF,
    /// The `KeyG` key.
    KeyG,
    /// The `KeyH` key.
    KeyH,
    /// The `KeyI` key.
    KeyI,
    /// The `KeyJ` key.
    KeyJ,
    /// The `KeyK` key.
    KeyK,
    /// The `KeyL` key.
    KeyL,
    /// The `KeyM` key.
    KeyM,
    /// The `KeyN` key.
    KeyN,
    /// The `KeyO` key.
    KeyO,
    /// The `KeyP` key.
    KeyP,
    /// The `KeyQ` key.
    KeyQ,
    /// The `KeyR` key.
    KeyR,
    /// The `KeyS` key.
    KeyS,
    /// The `KeyT` key.
    KeyT,
    /// The `KeyU` key.
    KeyU,
    /// The `KeyV` key.
    KeyV,
    /// The `KeyW` key.
    KeyW,
    /// The `KeyX` key.
    KeyX,
    /// The `KeyY` key.
    KeyY,
    /// The `KeyZ` key.
    KeyZ,
    /// The `Minus` key.
    Minus,
    /// The `Period` key.
    Period,
    /// The `Quote` key.
    Quote,
    /// The `Semicolon` key.
    Semicolon,
    /// The `Slash` key.
    Slash,
    /// The `AltLeft` key.
    AltLeft,
    /// The `AltRight` key.
    AltRight,
    /// The `Backspace` key.
    Backspace,
    /// The `CapsLock` key.
    CapsLock,
    /// The `ContextMenu` key.
    ContextMenu,
    /// The `ControlLeft` key.
    ControlLeft,
    /// The `ControlRight` key.
    ControlRight,
    /// The `Enter` key.
    Enter,
    /// The `SuperLeft` key.
    SuperLeft,
    /// The `SuperRight` key.
    SuperRight,
    /// The `ShiftLeft` key.
    ShiftLeft,
    /// The `ShiftRight` key.
    ShiftRight,
    /// The `Space` key.
    Space,
    /// The `Tab` key.
    Tab,
    /// The `Convert` key.
    Convert,
    /// The `KanaMode` key.
    KanaMode,
    /// The `Lang1` key.
    Lang1,
    /// The `Lang2` key.
    Lang2,
    /// The `Lang3` key.
    Lang3,
    /// The `Lang4` key.
    Lang4,
    /// The `Lang5` key.
    Lang5,
    /// The `NonConvert` key.
    NonConvert,
    /// The `Delete` key.
    Delete,
    /// The `End` key.
    End,
    /// The `Help` key.
    Help,
    /// The `Home` key.
    Home,
    /// The `Insert` key.
    Insert,
    /// The `PageDown` key.
    PageDown,
    /// The `PageUp` key.
    PageUp,
    /// The `ArrowDown` key.
    ArrowDown,
    /// The `ArrowLeft` key.
    ArrowLeft,
    /// The `ArrowRight` key.
    ArrowRight,
    /// The `ArrowUp` key.
    ArrowUp,
    /// The `NumLock` key.
    NumLock,
    /// The `Numpad0` key.
    Numpad0,
    /// The `Numpad1` key.
    Numpad1,
    /// The `Numpad2` key.
    Numpad2,
    /// The `Numpad3` key.
    Numpad3,
    /// The `Numpad4` key.
    Numpad4,
    /// The `Numpad5` key.
    Numpad5,
    /// The `Numpad6` key.
    Numpad6,
    /// The `Numpad7` key.
    Numpad7,
    /// The `Numpad8` key.
    Numpad8,
    /// The `Numpad9` key.
    Numpad9,
    /// The `NumpadAdd` key.
    NumpadAdd,
    /// The `NumpadBackspace` key.
    NumpadBackspace,
    /// The `NumpadClear` key.
    NumpadClear,
    /// The `NumpadClearEntry` key.
    NumpadClearEntry,
    /// The `NumpadComma` key.
    NumpadComma,
    /// The `NumpadDecimal` key.
    NumpadDecimal,
    /// The `NumpadDivide` key.
    NumpadDivide,
    /// The `NumpadEnter` key.
    NumpadEnter,
    /// The `NumpadEqual` key.
    NumpadEqual,
    /// The `NumpadHash` key.
    NumpadHash,
    /// The `NumpadMemoryAdd` key.
    NumpadMemoryAdd,
    /// The `NumpadMemoryClear` key.
    NumpadMemoryClear,
    /// The `NumpadMemoryRecall` key.
    NumpadMemoryRecall,
    /// The `NumpadMemoryStore` key.
    NumpadMemoryStore,
    /// The `NumpadMemorySubtract` key.
    NumpadMemorySubtract,
    /// The `NumpadMultiply` key.
    NumpadMultiply,
    /// The `NumpadParenLeft` key.
    NumpadParenLeft,
    /// The `NumpadParenRight` key.
    NumpadParenRight,
    /// The `NumpadStar` key.
    NumpadStar,
    /// The `NumpadSubtract` key.
    NumpadSubtract,
    /// The `Escape` key.
    Escape,
    /// The `Fn` key.
    Fn,
    /// The `FnLock` key.
    FnLock,
    /// The `PrintScreen` key.
    PrintScreen,
    /// The `ScrollLock` key.
    ScrollLock,
    /// The `Pause` key.
    Pause,
    /// The `BrowserBack` key.
    BrowserBack,
    /// The `BrowserFavorites` key.
    BrowserFavorites,
    /// The `BrowserForward` key.
    BrowserForward,
    /// The `BrowserHome` key.
    BrowserHome,
    /// The `BrowserRefresh` key.
    BrowserRefresh,
    /// The `BrowserSearch` key.
    BrowserSearch,
    /// The `BrowserStop` key.
    BrowserStop,
    /// The `Eject` key.
    Eject,
    /// The `LaunchApp1` key.
    LaunchApp1,
    /// The `LaunchApp2` key.
    LaunchApp2,
    /// The `LaunchMail` key.
    LaunchMail,
    /// The `MediaPlayPause` key.
    MediaPlayPause,
    /// The `MediaSelect` key.
    MediaSelect,
    /// The `MediaStop` key.
    MediaStop,
    /// The `MediaTrackNext` key.
    MediaTrackNext,
    /// The `MediaTrackPrevious` key.
    MediaTrackPrevious,
    /// The `Power` key.
    Power,
    /// The `Sleep` key.
    Sleep,
    /// The `AudioVolumeDown` key.
    AudioVolumeDown,
    /// The `AudioVolumeMute` key.
    AudioVolumeMute,
    /// The `AudioVolumeUp` key.
    AudioVolumeUp,
    /// The `WakeUp` key.
    WakeUp,
    /// The `Meta` key.
    Meta,
    /// The `Hyper` key.
    Hyper,
    /// The `Turbo` key.
    Turbo,
    /// The `Abort` key.
    Abort,
    /// The `Resume` key.
    Resume,
    /// The `Suspend` key.
    Suspend,
    /// The `Again` key.
    Again,
    /// The `Copy` key.
    Copy,
    /// The `Cut` key.
    Cut,
    /// The `Find` key.
    Find,
    /// The `Open` key.
    Open,
    /// The `Paste` key.
    Paste,
    /// The `Props` key.
    Props,
    /// The `Select` key.
    Select,
    /// The `Undo` key.
    Undo,
    /// The `Hiragana` key.
    Hiragana,
    /// The `Katakana` key.
    Katakana,
    /// The `F1` key.
    F1,
    /// The `F2` key.
    F2,
    /// The `F3` key.
    F3,
    /// The `F4` key.
    F4,
    /// The `F5` key.
    F5,
    /// The `F6` key.
    F6,
    /// The `F7` key.
    F7,
    /// The `F8` key.
    F8,
    /// The `F9` key.
    F9,
    /// The `F10` key.
    F10,
    /// The `F11` key.
    F11,
    /// The `F12` key.
    F12,
    /// The `F13` key.
    F13,
    /// The `F14` key.
    F14,
    /// The `F15` key.
    F15,
    /// The `F16` key.
    F16,
    /// The `F17` key.
    F17,
    /// The `F18` key.
    F18,
    /// The `F19` key.
    F19,
    /// The `F20` key.
    F20,
    /// The `F21` key.
    F21,
    /// The `F22` key.
    F22,
    /// The `F23` key.
    F23,
    /// The `F24` key.
    F24,
    /// The `F25` key.
    F25,
    /// The `F26` key.
    F26,
    /// The `F27` key.
    F27,
    /// The `F28` key.
    F28,
    /// The `F29` key.
    F29,
    /// The `F30` key.
    F30,
    /// The `F31` key.
    F31,
    /// The `F32` key.
    F32,
    /// The `F33` key.
    F33,
    /// The `F34` key.
    F34,
    /// The `F35` key.
    F35,
}
