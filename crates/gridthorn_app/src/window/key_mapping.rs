#[allow(
    clippy::too_many_lines,
    reason = "exhaustive standardized key conversion table"
)]
pub(super) fn map_keycode(key: winit::keyboard::KeyCode) -> gridthorn_input::KeyCode {
    match key {
        winit::keyboard::KeyCode::Backquote => gridthorn_input::KeyCode::Backquote,
        winit::keyboard::KeyCode::Backslash => gridthorn_input::KeyCode::Backslash,
        winit::keyboard::KeyCode::BracketLeft => gridthorn_input::KeyCode::BracketLeft,
        winit::keyboard::KeyCode::BracketRight => gridthorn_input::KeyCode::BracketRight,
        winit::keyboard::KeyCode::Comma => gridthorn_input::KeyCode::Comma,
        winit::keyboard::KeyCode::Digit0 => gridthorn_input::KeyCode::Digit0,
        winit::keyboard::KeyCode::Digit1 => gridthorn_input::KeyCode::Digit1,
        winit::keyboard::KeyCode::Digit2 => gridthorn_input::KeyCode::Digit2,
        winit::keyboard::KeyCode::Digit3 => gridthorn_input::KeyCode::Digit3,
        winit::keyboard::KeyCode::Digit4 => gridthorn_input::KeyCode::Digit4,
        winit::keyboard::KeyCode::Digit5 => gridthorn_input::KeyCode::Digit5,
        winit::keyboard::KeyCode::Digit6 => gridthorn_input::KeyCode::Digit6,
        winit::keyboard::KeyCode::Digit7 => gridthorn_input::KeyCode::Digit7,
        winit::keyboard::KeyCode::Digit8 => gridthorn_input::KeyCode::Digit8,
        winit::keyboard::KeyCode::Digit9 => gridthorn_input::KeyCode::Digit9,
        winit::keyboard::KeyCode::Equal => gridthorn_input::KeyCode::Equal,
        winit::keyboard::KeyCode::IntlBackslash => gridthorn_input::KeyCode::IntlBackslash,
        winit::keyboard::KeyCode::IntlRo => gridthorn_input::KeyCode::IntlRo,
        winit::keyboard::KeyCode::IntlYen => gridthorn_input::KeyCode::IntlYen,
        winit::keyboard::KeyCode::KeyA => gridthorn_input::KeyCode::KeyA,
        winit::keyboard::KeyCode::KeyB => gridthorn_input::KeyCode::KeyB,
        winit::keyboard::KeyCode::KeyC => gridthorn_input::KeyCode::KeyC,
        winit::keyboard::KeyCode::KeyD => gridthorn_input::KeyCode::KeyD,
        winit::keyboard::KeyCode::KeyE => gridthorn_input::KeyCode::KeyE,
        winit::keyboard::KeyCode::KeyF => gridthorn_input::KeyCode::KeyF,
        winit::keyboard::KeyCode::KeyG => gridthorn_input::KeyCode::KeyG,
        winit::keyboard::KeyCode::KeyH => gridthorn_input::KeyCode::KeyH,
        winit::keyboard::KeyCode::KeyI => gridthorn_input::KeyCode::KeyI,
        winit::keyboard::KeyCode::KeyJ => gridthorn_input::KeyCode::KeyJ,
        winit::keyboard::KeyCode::KeyK => gridthorn_input::KeyCode::KeyK,
        winit::keyboard::KeyCode::KeyL => gridthorn_input::KeyCode::KeyL,
        winit::keyboard::KeyCode::KeyM => gridthorn_input::KeyCode::KeyM,
        winit::keyboard::KeyCode::KeyN => gridthorn_input::KeyCode::KeyN,
        winit::keyboard::KeyCode::KeyO => gridthorn_input::KeyCode::KeyO,
        winit::keyboard::KeyCode::KeyP => gridthorn_input::KeyCode::KeyP,
        winit::keyboard::KeyCode::KeyQ => gridthorn_input::KeyCode::KeyQ,
        winit::keyboard::KeyCode::KeyR => gridthorn_input::KeyCode::KeyR,
        winit::keyboard::KeyCode::KeyS => gridthorn_input::KeyCode::KeyS,
        winit::keyboard::KeyCode::KeyT => gridthorn_input::KeyCode::KeyT,
        winit::keyboard::KeyCode::KeyU => gridthorn_input::KeyCode::KeyU,
        winit::keyboard::KeyCode::KeyV => gridthorn_input::KeyCode::KeyV,
        winit::keyboard::KeyCode::KeyW => gridthorn_input::KeyCode::KeyW,
        winit::keyboard::KeyCode::KeyX => gridthorn_input::KeyCode::KeyX,
        winit::keyboard::KeyCode::KeyY => gridthorn_input::KeyCode::KeyY,
        winit::keyboard::KeyCode::KeyZ => gridthorn_input::KeyCode::KeyZ,
        winit::keyboard::KeyCode::Minus => gridthorn_input::KeyCode::Minus,
        winit::keyboard::KeyCode::Period => gridthorn_input::KeyCode::Period,
        winit::keyboard::KeyCode::Quote => gridthorn_input::KeyCode::Quote,
        winit::keyboard::KeyCode::Semicolon => gridthorn_input::KeyCode::Semicolon,
        winit::keyboard::KeyCode::Slash => gridthorn_input::KeyCode::Slash,
        winit::keyboard::KeyCode::AltLeft => gridthorn_input::KeyCode::AltLeft,
        winit::keyboard::KeyCode::AltRight => gridthorn_input::KeyCode::AltRight,
        winit::keyboard::KeyCode::Backspace => gridthorn_input::KeyCode::Backspace,
        winit::keyboard::KeyCode::CapsLock => gridthorn_input::KeyCode::CapsLock,
        winit::keyboard::KeyCode::ContextMenu => gridthorn_input::KeyCode::ContextMenu,
        winit::keyboard::KeyCode::ControlLeft => gridthorn_input::KeyCode::ControlLeft,
        winit::keyboard::KeyCode::ControlRight => gridthorn_input::KeyCode::ControlRight,
        winit::keyboard::KeyCode::Enter => gridthorn_input::KeyCode::Enter,
        winit::keyboard::KeyCode::SuperLeft => gridthorn_input::KeyCode::SuperLeft,
        winit::keyboard::KeyCode::SuperRight => gridthorn_input::KeyCode::SuperRight,
        winit::keyboard::KeyCode::ShiftLeft => gridthorn_input::KeyCode::ShiftLeft,
        winit::keyboard::KeyCode::ShiftRight => gridthorn_input::KeyCode::ShiftRight,
        winit::keyboard::KeyCode::Space => gridthorn_input::KeyCode::Space,
        winit::keyboard::KeyCode::Tab => gridthorn_input::KeyCode::Tab,
        winit::keyboard::KeyCode::Convert => gridthorn_input::KeyCode::Convert,
        winit::keyboard::KeyCode::KanaMode => gridthorn_input::KeyCode::KanaMode,
        winit::keyboard::KeyCode::Lang1 => gridthorn_input::KeyCode::Lang1,
        winit::keyboard::KeyCode::Lang2 => gridthorn_input::KeyCode::Lang2,
        winit::keyboard::KeyCode::Lang3 => gridthorn_input::KeyCode::Lang3,
        winit::keyboard::KeyCode::Lang4 => gridthorn_input::KeyCode::Lang4,
        winit::keyboard::KeyCode::Lang5 => gridthorn_input::KeyCode::Lang5,
        winit::keyboard::KeyCode::NonConvert => gridthorn_input::KeyCode::NonConvert,
        winit::keyboard::KeyCode::Delete => gridthorn_input::KeyCode::Delete,
        winit::keyboard::KeyCode::End => gridthorn_input::KeyCode::End,
        winit::keyboard::KeyCode::Help => gridthorn_input::KeyCode::Help,
        winit::keyboard::KeyCode::Home => gridthorn_input::KeyCode::Home,
        winit::keyboard::KeyCode::Insert => gridthorn_input::KeyCode::Insert,
        winit::keyboard::KeyCode::PageDown => gridthorn_input::KeyCode::PageDown,
        winit::keyboard::KeyCode::PageUp => gridthorn_input::KeyCode::PageUp,
        winit::keyboard::KeyCode::ArrowDown => gridthorn_input::KeyCode::ArrowDown,
        winit::keyboard::KeyCode::ArrowLeft => gridthorn_input::KeyCode::ArrowLeft,
        winit::keyboard::KeyCode::ArrowRight => gridthorn_input::KeyCode::ArrowRight,
        winit::keyboard::KeyCode::ArrowUp => gridthorn_input::KeyCode::ArrowUp,
        winit::keyboard::KeyCode::NumLock => gridthorn_input::KeyCode::NumLock,
        winit::keyboard::KeyCode::Numpad0 => gridthorn_input::KeyCode::Numpad0,
        winit::keyboard::KeyCode::Numpad1 => gridthorn_input::KeyCode::Numpad1,
        winit::keyboard::KeyCode::Numpad2 => gridthorn_input::KeyCode::Numpad2,
        winit::keyboard::KeyCode::Numpad3 => gridthorn_input::KeyCode::Numpad3,
        winit::keyboard::KeyCode::Numpad4 => gridthorn_input::KeyCode::Numpad4,
        winit::keyboard::KeyCode::Numpad5 => gridthorn_input::KeyCode::Numpad5,
        winit::keyboard::KeyCode::Numpad6 => gridthorn_input::KeyCode::Numpad6,
        winit::keyboard::KeyCode::Numpad7 => gridthorn_input::KeyCode::Numpad7,
        winit::keyboard::KeyCode::Numpad8 => gridthorn_input::KeyCode::Numpad8,
        winit::keyboard::KeyCode::Numpad9 => gridthorn_input::KeyCode::Numpad9,
        winit::keyboard::KeyCode::NumpadAdd => gridthorn_input::KeyCode::NumpadAdd,
        winit::keyboard::KeyCode::NumpadBackspace => gridthorn_input::KeyCode::NumpadBackspace,
        winit::keyboard::KeyCode::NumpadClear => gridthorn_input::KeyCode::NumpadClear,
        winit::keyboard::KeyCode::NumpadClearEntry => gridthorn_input::KeyCode::NumpadClearEntry,
        winit::keyboard::KeyCode::NumpadComma => gridthorn_input::KeyCode::NumpadComma,
        winit::keyboard::KeyCode::NumpadDecimal => gridthorn_input::KeyCode::NumpadDecimal,
        winit::keyboard::KeyCode::NumpadDivide => gridthorn_input::KeyCode::NumpadDivide,
        winit::keyboard::KeyCode::NumpadEnter => gridthorn_input::KeyCode::NumpadEnter,
        winit::keyboard::KeyCode::NumpadEqual => gridthorn_input::KeyCode::NumpadEqual,
        winit::keyboard::KeyCode::NumpadHash => gridthorn_input::KeyCode::NumpadHash,
        winit::keyboard::KeyCode::NumpadMemoryAdd => gridthorn_input::KeyCode::NumpadMemoryAdd,
        winit::keyboard::KeyCode::NumpadMemoryClear => gridthorn_input::KeyCode::NumpadMemoryClear,
        winit::keyboard::KeyCode::NumpadMemoryRecall => {
            gridthorn_input::KeyCode::NumpadMemoryRecall
        }
        winit::keyboard::KeyCode::NumpadMemoryStore => gridthorn_input::KeyCode::NumpadMemoryStore,
        winit::keyboard::KeyCode::NumpadMemorySubtract => {
            gridthorn_input::KeyCode::NumpadMemorySubtract
        }
        winit::keyboard::KeyCode::NumpadMultiply => gridthorn_input::KeyCode::NumpadMultiply,
        winit::keyboard::KeyCode::NumpadParenLeft => gridthorn_input::KeyCode::NumpadParenLeft,
        winit::keyboard::KeyCode::NumpadParenRight => gridthorn_input::KeyCode::NumpadParenRight,
        winit::keyboard::KeyCode::NumpadStar => gridthorn_input::KeyCode::NumpadStar,
        winit::keyboard::KeyCode::NumpadSubtract => gridthorn_input::KeyCode::NumpadSubtract,
        winit::keyboard::KeyCode::Escape => gridthorn_input::KeyCode::Escape,
        winit::keyboard::KeyCode::Fn => gridthorn_input::KeyCode::Fn,
        winit::keyboard::KeyCode::FnLock => gridthorn_input::KeyCode::FnLock,
        winit::keyboard::KeyCode::PrintScreen => gridthorn_input::KeyCode::PrintScreen,
        winit::keyboard::KeyCode::ScrollLock => gridthorn_input::KeyCode::ScrollLock,
        winit::keyboard::KeyCode::Pause => gridthorn_input::KeyCode::Pause,
        winit::keyboard::KeyCode::BrowserBack => gridthorn_input::KeyCode::BrowserBack,
        winit::keyboard::KeyCode::BrowserFavorites => gridthorn_input::KeyCode::BrowserFavorites,
        winit::keyboard::KeyCode::BrowserForward => gridthorn_input::KeyCode::BrowserForward,
        winit::keyboard::KeyCode::BrowserHome => gridthorn_input::KeyCode::BrowserHome,
        winit::keyboard::KeyCode::BrowserRefresh => gridthorn_input::KeyCode::BrowserRefresh,
        winit::keyboard::KeyCode::BrowserSearch => gridthorn_input::KeyCode::BrowserSearch,
        winit::keyboard::KeyCode::BrowserStop => gridthorn_input::KeyCode::BrowserStop,
        winit::keyboard::KeyCode::Eject => gridthorn_input::KeyCode::Eject,
        winit::keyboard::KeyCode::LaunchApp1 => gridthorn_input::KeyCode::LaunchApp1,
        winit::keyboard::KeyCode::LaunchApp2 => gridthorn_input::KeyCode::LaunchApp2,
        winit::keyboard::KeyCode::LaunchMail => gridthorn_input::KeyCode::LaunchMail,
        winit::keyboard::KeyCode::MediaPlayPause => gridthorn_input::KeyCode::MediaPlayPause,
        winit::keyboard::KeyCode::MediaSelect => gridthorn_input::KeyCode::MediaSelect,
        winit::keyboard::KeyCode::MediaStop => gridthorn_input::KeyCode::MediaStop,
        winit::keyboard::KeyCode::MediaTrackNext => gridthorn_input::KeyCode::MediaTrackNext,
        winit::keyboard::KeyCode::MediaTrackPrevious => {
            gridthorn_input::KeyCode::MediaTrackPrevious
        }
        winit::keyboard::KeyCode::Power => gridthorn_input::KeyCode::Power,
        winit::keyboard::KeyCode::Sleep => gridthorn_input::KeyCode::Sleep,
        winit::keyboard::KeyCode::AudioVolumeDown => gridthorn_input::KeyCode::AudioVolumeDown,
        winit::keyboard::KeyCode::AudioVolumeMute => gridthorn_input::KeyCode::AudioVolumeMute,
        winit::keyboard::KeyCode::AudioVolumeUp => gridthorn_input::KeyCode::AudioVolumeUp,
        winit::keyboard::KeyCode::WakeUp => gridthorn_input::KeyCode::WakeUp,
        winit::keyboard::KeyCode::Meta => gridthorn_input::KeyCode::Meta,
        winit::keyboard::KeyCode::Hyper => gridthorn_input::KeyCode::Hyper,
        winit::keyboard::KeyCode::Turbo => gridthorn_input::KeyCode::Turbo,
        winit::keyboard::KeyCode::Abort => gridthorn_input::KeyCode::Abort,
        winit::keyboard::KeyCode::Resume => gridthorn_input::KeyCode::Resume,
        winit::keyboard::KeyCode::Suspend => gridthorn_input::KeyCode::Suspend,
        winit::keyboard::KeyCode::Again => gridthorn_input::KeyCode::Again,
        winit::keyboard::KeyCode::Copy => gridthorn_input::KeyCode::Copy,
        winit::keyboard::KeyCode::Cut => gridthorn_input::KeyCode::Cut,
        winit::keyboard::KeyCode::Find => gridthorn_input::KeyCode::Find,
        winit::keyboard::KeyCode::Open => gridthorn_input::KeyCode::Open,
        winit::keyboard::KeyCode::Paste => gridthorn_input::KeyCode::Paste,
        winit::keyboard::KeyCode::Props => gridthorn_input::KeyCode::Props,
        winit::keyboard::KeyCode::Select => gridthorn_input::KeyCode::Select,
        winit::keyboard::KeyCode::Undo => gridthorn_input::KeyCode::Undo,
        winit::keyboard::KeyCode::Hiragana => gridthorn_input::KeyCode::Hiragana,
        winit::keyboard::KeyCode::Katakana => gridthorn_input::KeyCode::Katakana,
        winit::keyboard::KeyCode::F1 => gridthorn_input::KeyCode::F1,
        winit::keyboard::KeyCode::F2 => gridthorn_input::KeyCode::F2,
        winit::keyboard::KeyCode::F3 => gridthorn_input::KeyCode::F3,
        winit::keyboard::KeyCode::F4 => gridthorn_input::KeyCode::F4,
        winit::keyboard::KeyCode::F5 => gridthorn_input::KeyCode::F5,
        winit::keyboard::KeyCode::F6 => gridthorn_input::KeyCode::F6,
        winit::keyboard::KeyCode::F7 => gridthorn_input::KeyCode::F7,
        winit::keyboard::KeyCode::F8 => gridthorn_input::KeyCode::F8,
        winit::keyboard::KeyCode::F9 => gridthorn_input::KeyCode::F9,
        winit::keyboard::KeyCode::F10 => gridthorn_input::KeyCode::F10,
        winit::keyboard::KeyCode::F11 => gridthorn_input::KeyCode::F11,
        winit::keyboard::KeyCode::F12 => gridthorn_input::KeyCode::F12,
        winit::keyboard::KeyCode::F13 => gridthorn_input::KeyCode::F13,
        winit::keyboard::KeyCode::F14 => gridthorn_input::KeyCode::F14,
        winit::keyboard::KeyCode::F15 => gridthorn_input::KeyCode::F15,
        winit::keyboard::KeyCode::F16 => gridthorn_input::KeyCode::F16,
        winit::keyboard::KeyCode::F17 => gridthorn_input::KeyCode::F17,
        winit::keyboard::KeyCode::F18 => gridthorn_input::KeyCode::F18,
        winit::keyboard::KeyCode::F19 => gridthorn_input::KeyCode::F19,
        winit::keyboard::KeyCode::F20 => gridthorn_input::KeyCode::F20,
        winit::keyboard::KeyCode::F21 => gridthorn_input::KeyCode::F21,
        winit::keyboard::KeyCode::F22 => gridthorn_input::KeyCode::F22,
        winit::keyboard::KeyCode::F23 => gridthorn_input::KeyCode::F23,
        winit::keyboard::KeyCode::F24 => gridthorn_input::KeyCode::F24,
        winit::keyboard::KeyCode::F25 => gridthorn_input::KeyCode::F25,
        winit::keyboard::KeyCode::F26 => gridthorn_input::KeyCode::F26,
        winit::keyboard::KeyCode::F27 => gridthorn_input::KeyCode::F27,
        winit::keyboard::KeyCode::F28 => gridthorn_input::KeyCode::F28,
        winit::keyboard::KeyCode::F29 => gridthorn_input::KeyCode::F29,
        winit::keyboard::KeyCode::F30 => gridthorn_input::KeyCode::F30,
        winit::keyboard::KeyCode::F31 => gridthorn_input::KeyCode::F31,
        winit::keyboard::KeyCode::F32 => gridthorn_input::KeyCode::F32,
        winit::keyboard::KeyCode::F33 => gridthorn_input::KeyCode::F33,
        winit::keyboard::KeyCode::F34 => gridthorn_input::KeyCode::F34,
        winit::keyboard::KeyCode::F35 => gridthorn_input::KeyCode::F35,
        _ => gridthorn_input::KeyCode::Unidentified,
    }
}

#[allow(
    clippy::too_many_lines,
    reason = "exhaustive standardized key conversion table"
)]
pub(super) fn map_namedkey(key: winit::keyboard::NamedKey) -> gridthorn_input::NamedKey {
    match key {
        winit::keyboard::NamedKey::Alt => gridthorn_input::NamedKey::Alt,
        winit::keyboard::NamedKey::AltGraph => gridthorn_input::NamedKey::AltGraph,
        winit::keyboard::NamedKey::CapsLock => gridthorn_input::NamedKey::CapsLock,
        winit::keyboard::NamedKey::Control => gridthorn_input::NamedKey::Control,
        winit::keyboard::NamedKey::Fn => gridthorn_input::NamedKey::Fn,
        winit::keyboard::NamedKey::FnLock => gridthorn_input::NamedKey::FnLock,
        winit::keyboard::NamedKey::NumLock => gridthorn_input::NamedKey::NumLock,
        winit::keyboard::NamedKey::ScrollLock => gridthorn_input::NamedKey::ScrollLock,
        winit::keyboard::NamedKey::Shift => gridthorn_input::NamedKey::Shift,
        winit::keyboard::NamedKey::Symbol => gridthorn_input::NamedKey::Symbol,
        winit::keyboard::NamedKey::SymbolLock => gridthorn_input::NamedKey::SymbolLock,
        winit::keyboard::NamedKey::Meta => gridthorn_input::NamedKey::Meta,
        winit::keyboard::NamedKey::Hyper => gridthorn_input::NamedKey::Hyper,
        winit::keyboard::NamedKey::Super => gridthorn_input::NamedKey::Super,
        winit::keyboard::NamedKey::Enter => gridthorn_input::NamedKey::Enter,
        winit::keyboard::NamedKey::Tab => gridthorn_input::NamedKey::Tab,
        winit::keyboard::NamedKey::Space => gridthorn_input::NamedKey::Space,
        winit::keyboard::NamedKey::ArrowDown => gridthorn_input::NamedKey::ArrowDown,
        winit::keyboard::NamedKey::ArrowLeft => gridthorn_input::NamedKey::ArrowLeft,
        winit::keyboard::NamedKey::ArrowRight => gridthorn_input::NamedKey::ArrowRight,
        winit::keyboard::NamedKey::ArrowUp => gridthorn_input::NamedKey::ArrowUp,
        winit::keyboard::NamedKey::End => gridthorn_input::NamedKey::End,
        winit::keyboard::NamedKey::Home => gridthorn_input::NamedKey::Home,
        winit::keyboard::NamedKey::PageDown => gridthorn_input::NamedKey::PageDown,
        winit::keyboard::NamedKey::PageUp => gridthorn_input::NamedKey::PageUp,
        winit::keyboard::NamedKey::Backspace => gridthorn_input::NamedKey::Backspace,
        winit::keyboard::NamedKey::Clear => gridthorn_input::NamedKey::Clear,
        winit::keyboard::NamedKey::Copy => gridthorn_input::NamedKey::Copy,
        winit::keyboard::NamedKey::CrSel => gridthorn_input::NamedKey::CrSel,
        winit::keyboard::NamedKey::Cut => gridthorn_input::NamedKey::Cut,
        winit::keyboard::NamedKey::Delete => gridthorn_input::NamedKey::Delete,
        winit::keyboard::NamedKey::EraseEof => gridthorn_input::NamedKey::EraseEof,
        winit::keyboard::NamedKey::ExSel => gridthorn_input::NamedKey::ExSel,
        winit::keyboard::NamedKey::Insert => gridthorn_input::NamedKey::Insert,
        winit::keyboard::NamedKey::Paste => gridthorn_input::NamedKey::Paste,
        winit::keyboard::NamedKey::Redo => gridthorn_input::NamedKey::Redo,
        winit::keyboard::NamedKey::Undo => gridthorn_input::NamedKey::Undo,
        winit::keyboard::NamedKey::Accept => gridthorn_input::NamedKey::Accept,
        winit::keyboard::NamedKey::Again => gridthorn_input::NamedKey::Again,
        winit::keyboard::NamedKey::Attn => gridthorn_input::NamedKey::Attn,
        winit::keyboard::NamedKey::Cancel => gridthorn_input::NamedKey::Cancel,
        winit::keyboard::NamedKey::ContextMenu => gridthorn_input::NamedKey::ContextMenu,
        winit::keyboard::NamedKey::Escape => gridthorn_input::NamedKey::Escape,
        winit::keyboard::NamedKey::Execute => gridthorn_input::NamedKey::Execute,
        winit::keyboard::NamedKey::Find => gridthorn_input::NamedKey::Find,
        winit::keyboard::NamedKey::Help => gridthorn_input::NamedKey::Help,
        winit::keyboard::NamedKey::Pause => gridthorn_input::NamedKey::Pause,
        winit::keyboard::NamedKey::Play => gridthorn_input::NamedKey::Play,
        winit::keyboard::NamedKey::Props => gridthorn_input::NamedKey::Props,
        winit::keyboard::NamedKey::Select => gridthorn_input::NamedKey::Select,
        winit::keyboard::NamedKey::ZoomIn => gridthorn_input::NamedKey::ZoomIn,
        winit::keyboard::NamedKey::ZoomOut => gridthorn_input::NamedKey::ZoomOut,
        winit::keyboard::NamedKey::BrightnessDown => gridthorn_input::NamedKey::BrightnessDown,
        winit::keyboard::NamedKey::BrightnessUp => gridthorn_input::NamedKey::BrightnessUp,
        winit::keyboard::NamedKey::Eject => gridthorn_input::NamedKey::Eject,
        winit::keyboard::NamedKey::LogOff => gridthorn_input::NamedKey::LogOff,
        winit::keyboard::NamedKey::Power => gridthorn_input::NamedKey::Power,
        winit::keyboard::NamedKey::PowerOff => gridthorn_input::NamedKey::PowerOff,
        winit::keyboard::NamedKey::PrintScreen => gridthorn_input::NamedKey::PrintScreen,
        winit::keyboard::NamedKey::Hibernate => gridthorn_input::NamedKey::Hibernate,
        winit::keyboard::NamedKey::Standby => gridthorn_input::NamedKey::Standby,
        winit::keyboard::NamedKey::WakeUp => gridthorn_input::NamedKey::WakeUp,
        winit::keyboard::NamedKey::AllCandidates => gridthorn_input::NamedKey::AllCandidates,
        winit::keyboard::NamedKey::Alphanumeric => gridthorn_input::NamedKey::Alphanumeric,
        winit::keyboard::NamedKey::CodeInput => gridthorn_input::NamedKey::CodeInput,
        winit::keyboard::NamedKey::Compose => gridthorn_input::NamedKey::Compose,
        winit::keyboard::NamedKey::Convert => gridthorn_input::NamedKey::Convert,
        winit::keyboard::NamedKey::FinalMode => gridthorn_input::NamedKey::FinalMode,
        winit::keyboard::NamedKey::GroupFirst => gridthorn_input::NamedKey::GroupFirst,
        winit::keyboard::NamedKey::GroupLast => gridthorn_input::NamedKey::GroupLast,
        winit::keyboard::NamedKey::GroupNext => gridthorn_input::NamedKey::GroupNext,
        winit::keyboard::NamedKey::GroupPrevious => gridthorn_input::NamedKey::GroupPrevious,
        winit::keyboard::NamedKey::ModeChange => gridthorn_input::NamedKey::ModeChange,
        winit::keyboard::NamedKey::NextCandidate => gridthorn_input::NamedKey::NextCandidate,
        winit::keyboard::NamedKey::NonConvert => gridthorn_input::NamedKey::NonConvert,
        winit::keyboard::NamedKey::PreviousCandidate => {
            gridthorn_input::NamedKey::PreviousCandidate
        }
        winit::keyboard::NamedKey::Process => gridthorn_input::NamedKey::Process,
        winit::keyboard::NamedKey::SingleCandidate => gridthorn_input::NamedKey::SingleCandidate,
        winit::keyboard::NamedKey::HangulMode => gridthorn_input::NamedKey::HangulMode,
        winit::keyboard::NamedKey::HanjaMode => gridthorn_input::NamedKey::HanjaMode,
        winit::keyboard::NamedKey::JunjaMode => gridthorn_input::NamedKey::JunjaMode,
        winit::keyboard::NamedKey::Eisu => gridthorn_input::NamedKey::Eisu,
        winit::keyboard::NamedKey::Hankaku => gridthorn_input::NamedKey::Hankaku,
        winit::keyboard::NamedKey::Hiragana => gridthorn_input::NamedKey::Hiragana,
        winit::keyboard::NamedKey::HiraganaKatakana => gridthorn_input::NamedKey::HiraganaKatakana,
        winit::keyboard::NamedKey::KanaMode => gridthorn_input::NamedKey::KanaMode,
        winit::keyboard::NamedKey::KanjiMode => gridthorn_input::NamedKey::KanjiMode,
        winit::keyboard::NamedKey::Katakana => gridthorn_input::NamedKey::Katakana,
        winit::keyboard::NamedKey::Romaji => gridthorn_input::NamedKey::Romaji,
        winit::keyboard::NamedKey::Zenkaku => gridthorn_input::NamedKey::Zenkaku,
        winit::keyboard::NamedKey::ZenkakuHankaku => gridthorn_input::NamedKey::ZenkakuHankaku,
        winit::keyboard::NamedKey::Soft1 => gridthorn_input::NamedKey::Soft1,
        winit::keyboard::NamedKey::Soft2 => gridthorn_input::NamedKey::Soft2,
        winit::keyboard::NamedKey::Soft3 => gridthorn_input::NamedKey::Soft3,
        winit::keyboard::NamedKey::Soft4 => gridthorn_input::NamedKey::Soft4,
        winit::keyboard::NamedKey::ChannelDown => gridthorn_input::NamedKey::ChannelDown,
        winit::keyboard::NamedKey::ChannelUp => gridthorn_input::NamedKey::ChannelUp,
        winit::keyboard::NamedKey::Close => gridthorn_input::NamedKey::Close,
        winit::keyboard::NamedKey::MailForward => gridthorn_input::NamedKey::MailForward,
        winit::keyboard::NamedKey::MailReply => gridthorn_input::NamedKey::MailReply,
        winit::keyboard::NamedKey::MailSend => gridthorn_input::NamedKey::MailSend,
        winit::keyboard::NamedKey::MediaClose => gridthorn_input::NamedKey::MediaClose,
        winit::keyboard::NamedKey::MediaFastForward => gridthorn_input::NamedKey::MediaFastForward,
        winit::keyboard::NamedKey::MediaPause => gridthorn_input::NamedKey::MediaPause,
        winit::keyboard::NamedKey::MediaPlay => gridthorn_input::NamedKey::MediaPlay,
        winit::keyboard::NamedKey::MediaPlayPause => gridthorn_input::NamedKey::MediaPlayPause,
        winit::keyboard::NamedKey::MediaRecord => gridthorn_input::NamedKey::MediaRecord,
        winit::keyboard::NamedKey::MediaRewind => gridthorn_input::NamedKey::MediaRewind,
        winit::keyboard::NamedKey::MediaStop => gridthorn_input::NamedKey::MediaStop,
        winit::keyboard::NamedKey::MediaTrackNext => gridthorn_input::NamedKey::MediaTrackNext,
        winit::keyboard::NamedKey::MediaTrackPrevious => {
            gridthorn_input::NamedKey::MediaTrackPrevious
        }
        winit::keyboard::NamedKey::New => gridthorn_input::NamedKey::New,
        winit::keyboard::NamedKey::Open => gridthorn_input::NamedKey::Open,
        winit::keyboard::NamedKey::Print => gridthorn_input::NamedKey::Print,
        winit::keyboard::NamedKey::Save => gridthorn_input::NamedKey::Save,
        winit::keyboard::NamedKey::SpellCheck => gridthorn_input::NamedKey::SpellCheck,
        winit::keyboard::NamedKey::Key11 => gridthorn_input::NamedKey::Key11,
        winit::keyboard::NamedKey::Key12 => gridthorn_input::NamedKey::Key12,
        winit::keyboard::NamedKey::AudioBalanceLeft => gridthorn_input::NamedKey::AudioBalanceLeft,
        winit::keyboard::NamedKey::AudioBalanceRight => {
            gridthorn_input::NamedKey::AudioBalanceRight
        }
        winit::keyboard::NamedKey::AudioBassBoostDown => {
            gridthorn_input::NamedKey::AudioBassBoostDown
        }
        winit::keyboard::NamedKey::AudioBassBoostToggle => {
            gridthorn_input::NamedKey::AudioBassBoostToggle
        }
        winit::keyboard::NamedKey::AudioBassBoostUp => gridthorn_input::NamedKey::AudioBassBoostUp,
        winit::keyboard::NamedKey::AudioFaderFront => gridthorn_input::NamedKey::AudioFaderFront,
        winit::keyboard::NamedKey::AudioFaderRear => gridthorn_input::NamedKey::AudioFaderRear,
        winit::keyboard::NamedKey::AudioSurroundModeNext => {
            gridthorn_input::NamedKey::AudioSurroundModeNext
        }
        winit::keyboard::NamedKey::AudioTrebleDown => gridthorn_input::NamedKey::AudioTrebleDown,
        winit::keyboard::NamedKey::AudioTrebleUp => gridthorn_input::NamedKey::AudioTrebleUp,
        winit::keyboard::NamedKey::AudioVolumeDown => gridthorn_input::NamedKey::AudioVolumeDown,
        winit::keyboard::NamedKey::AudioVolumeUp => gridthorn_input::NamedKey::AudioVolumeUp,
        winit::keyboard::NamedKey::AudioVolumeMute => gridthorn_input::NamedKey::AudioVolumeMute,
        winit::keyboard::NamedKey::MicrophoneToggle => gridthorn_input::NamedKey::MicrophoneToggle,
        winit::keyboard::NamedKey::MicrophoneVolumeDown => {
            gridthorn_input::NamedKey::MicrophoneVolumeDown
        }
        winit::keyboard::NamedKey::MicrophoneVolumeUp => {
            gridthorn_input::NamedKey::MicrophoneVolumeUp
        }
        winit::keyboard::NamedKey::MicrophoneVolumeMute => {
            gridthorn_input::NamedKey::MicrophoneVolumeMute
        }
        winit::keyboard::NamedKey::SpeechCorrectionList => {
            gridthorn_input::NamedKey::SpeechCorrectionList
        }
        winit::keyboard::NamedKey::SpeechInputToggle => {
            gridthorn_input::NamedKey::SpeechInputToggle
        }
        winit::keyboard::NamedKey::LaunchApplication1 => {
            gridthorn_input::NamedKey::LaunchApplication1
        }
        winit::keyboard::NamedKey::LaunchApplication2 => {
            gridthorn_input::NamedKey::LaunchApplication2
        }
        winit::keyboard::NamedKey::LaunchCalendar => gridthorn_input::NamedKey::LaunchCalendar,
        winit::keyboard::NamedKey::LaunchContacts => gridthorn_input::NamedKey::LaunchContacts,
        winit::keyboard::NamedKey::LaunchMail => gridthorn_input::NamedKey::LaunchMail,
        winit::keyboard::NamedKey::LaunchMediaPlayer => {
            gridthorn_input::NamedKey::LaunchMediaPlayer
        }
        winit::keyboard::NamedKey::LaunchMusicPlayer => {
            gridthorn_input::NamedKey::LaunchMusicPlayer
        }
        winit::keyboard::NamedKey::LaunchPhone => gridthorn_input::NamedKey::LaunchPhone,
        winit::keyboard::NamedKey::LaunchScreenSaver => {
            gridthorn_input::NamedKey::LaunchScreenSaver
        }
        winit::keyboard::NamedKey::LaunchSpreadsheet => {
            gridthorn_input::NamedKey::LaunchSpreadsheet
        }
        winit::keyboard::NamedKey::LaunchWebBrowser => gridthorn_input::NamedKey::LaunchWebBrowser,
        winit::keyboard::NamedKey::LaunchWebCam => gridthorn_input::NamedKey::LaunchWebCam,
        winit::keyboard::NamedKey::LaunchWordProcessor => {
            gridthorn_input::NamedKey::LaunchWordProcessor
        }
        winit::keyboard::NamedKey::BrowserBack => gridthorn_input::NamedKey::BrowserBack,
        winit::keyboard::NamedKey::BrowserFavorites => gridthorn_input::NamedKey::BrowserFavorites,
        winit::keyboard::NamedKey::BrowserForward => gridthorn_input::NamedKey::BrowserForward,
        winit::keyboard::NamedKey::BrowserHome => gridthorn_input::NamedKey::BrowserHome,
        winit::keyboard::NamedKey::BrowserRefresh => gridthorn_input::NamedKey::BrowserRefresh,
        winit::keyboard::NamedKey::BrowserSearch => gridthorn_input::NamedKey::BrowserSearch,
        winit::keyboard::NamedKey::BrowserStop => gridthorn_input::NamedKey::BrowserStop,
        winit::keyboard::NamedKey::AppSwitch => gridthorn_input::NamedKey::AppSwitch,
        winit::keyboard::NamedKey::Call => gridthorn_input::NamedKey::Call,
        winit::keyboard::NamedKey::Camera => gridthorn_input::NamedKey::Camera,
        winit::keyboard::NamedKey::CameraFocus => gridthorn_input::NamedKey::CameraFocus,
        winit::keyboard::NamedKey::EndCall => gridthorn_input::NamedKey::EndCall,
        winit::keyboard::NamedKey::GoBack => gridthorn_input::NamedKey::GoBack,
        winit::keyboard::NamedKey::GoHome => gridthorn_input::NamedKey::GoHome,
        winit::keyboard::NamedKey::HeadsetHook => gridthorn_input::NamedKey::HeadsetHook,
        winit::keyboard::NamedKey::LastNumberRedial => gridthorn_input::NamedKey::LastNumberRedial,
        winit::keyboard::NamedKey::Notification => gridthorn_input::NamedKey::Notification,
        winit::keyboard::NamedKey::MannerMode => gridthorn_input::NamedKey::MannerMode,
        winit::keyboard::NamedKey::VoiceDial => gridthorn_input::NamedKey::VoiceDial,
        winit::keyboard::NamedKey::TV => gridthorn_input::NamedKey::TV,
        winit::keyboard::NamedKey::TV3DMode => gridthorn_input::NamedKey::TV3DMode,
        winit::keyboard::NamedKey::TVAntennaCable => gridthorn_input::NamedKey::TVAntennaCable,
        winit::keyboard::NamedKey::TVAudioDescription => {
            gridthorn_input::NamedKey::TVAudioDescription
        }
        winit::keyboard::NamedKey::TVAudioDescriptionMixDown => {
            gridthorn_input::NamedKey::TVAudioDescriptionMixDown
        }
        winit::keyboard::NamedKey::TVAudioDescriptionMixUp => {
            gridthorn_input::NamedKey::TVAudioDescriptionMixUp
        }
        winit::keyboard::NamedKey::TVContentsMenu => gridthorn_input::NamedKey::TVContentsMenu,
        winit::keyboard::NamedKey::TVDataService => gridthorn_input::NamedKey::TVDataService,
        winit::keyboard::NamedKey::TVInput => gridthorn_input::NamedKey::TVInput,
        winit::keyboard::NamedKey::TVInputComponent1 => {
            gridthorn_input::NamedKey::TVInputComponent1
        }
        winit::keyboard::NamedKey::TVInputComponent2 => {
            gridthorn_input::NamedKey::TVInputComponent2
        }
        winit::keyboard::NamedKey::TVInputComposite1 => {
            gridthorn_input::NamedKey::TVInputComposite1
        }
        winit::keyboard::NamedKey::TVInputComposite2 => {
            gridthorn_input::NamedKey::TVInputComposite2
        }
        winit::keyboard::NamedKey::TVInputHDMI1 => gridthorn_input::NamedKey::TVInputHDMI1,
        winit::keyboard::NamedKey::TVInputHDMI2 => gridthorn_input::NamedKey::TVInputHDMI2,
        winit::keyboard::NamedKey::TVInputHDMI3 => gridthorn_input::NamedKey::TVInputHDMI3,
        winit::keyboard::NamedKey::TVInputHDMI4 => gridthorn_input::NamedKey::TVInputHDMI4,
        winit::keyboard::NamedKey::TVInputVGA1 => gridthorn_input::NamedKey::TVInputVGA1,
        winit::keyboard::NamedKey::TVMediaContext => gridthorn_input::NamedKey::TVMediaContext,
        winit::keyboard::NamedKey::TVNetwork => gridthorn_input::NamedKey::TVNetwork,
        winit::keyboard::NamedKey::TVNumberEntry => gridthorn_input::NamedKey::TVNumberEntry,
        winit::keyboard::NamedKey::TVPower => gridthorn_input::NamedKey::TVPower,
        winit::keyboard::NamedKey::TVRadioService => gridthorn_input::NamedKey::TVRadioService,
        winit::keyboard::NamedKey::TVSatellite => gridthorn_input::NamedKey::TVSatellite,
        winit::keyboard::NamedKey::TVSatelliteBS => gridthorn_input::NamedKey::TVSatelliteBS,
        winit::keyboard::NamedKey::TVSatelliteCS => gridthorn_input::NamedKey::TVSatelliteCS,
        winit::keyboard::NamedKey::TVSatelliteToggle => {
            gridthorn_input::NamedKey::TVSatelliteToggle
        }
        winit::keyboard::NamedKey::TVTerrestrialAnalog => {
            gridthorn_input::NamedKey::TVTerrestrialAnalog
        }
        winit::keyboard::NamedKey::TVTerrestrialDigital => {
            gridthorn_input::NamedKey::TVTerrestrialDigital
        }
        winit::keyboard::NamedKey::TVTimer => gridthorn_input::NamedKey::TVTimer,
        winit::keyboard::NamedKey::AVRInput => gridthorn_input::NamedKey::AVRInput,
        winit::keyboard::NamedKey::AVRPower => gridthorn_input::NamedKey::AVRPower,
        winit::keyboard::NamedKey::ColorF0Red => gridthorn_input::NamedKey::ColorF0Red,
        winit::keyboard::NamedKey::ColorF1Green => gridthorn_input::NamedKey::ColorF1Green,
        winit::keyboard::NamedKey::ColorF2Yellow => gridthorn_input::NamedKey::ColorF2Yellow,
        winit::keyboard::NamedKey::ColorF3Blue => gridthorn_input::NamedKey::ColorF3Blue,
        winit::keyboard::NamedKey::ColorF4Grey => gridthorn_input::NamedKey::ColorF4Grey,
        winit::keyboard::NamedKey::ColorF5Brown => gridthorn_input::NamedKey::ColorF5Brown,
        winit::keyboard::NamedKey::ClosedCaptionToggle => {
            gridthorn_input::NamedKey::ClosedCaptionToggle
        }
        winit::keyboard::NamedKey::Dimmer => gridthorn_input::NamedKey::Dimmer,
        winit::keyboard::NamedKey::DisplaySwap => gridthorn_input::NamedKey::DisplaySwap,
        winit::keyboard::NamedKey::DVR => gridthorn_input::NamedKey::DVR,
        winit::keyboard::NamedKey::Exit => gridthorn_input::NamedKey::Exit,
        winit::keyboard::NamedKey::FavoriteClear0 => gridthorn_input::NamedKey::FavoriteClear0,
        winit::keyboard::NamedKey::FavoriteClear1 => gridthorn_input::NamedKey::FavoriteClear1,
        winit::keyboard::NamedKey::FavoriteClear2 => gridthorn_input::NamedKey::FavoriteClear2,
        winit::keyboard::NamedKey::FavoriteClear3 => gridthorn_input::NamedKey::FavoriteClear3,
        winit::keyboard::NamedKey::FavoriteRecall0 => gridthorn_input::NamedKey::FavoriteRecall0,
        winit::keyboard::NamedKey::FavoriteRecall1 => gridthorn_input::NamedKey::FavoriteRecall1,
        winit::keyboard::NamedKey::FavoriteRecall2 => gridthorn_input::NamedKey::FavoriteRecall2,
        winit::keyboard::NamedKey::FavoriteRecall3 => gridthorn_input::NamedKey::FavoriteRecall3,
        winit::keyboard::NamedKey::FavoriteStore0 => gridthorn_input::NamedKey::FavoriteStore0,
        winit::keyboard::NamedKey::FavoriteStore1 => gridthorn_input::NamedKey::FavoriteStore1,
        winit::keyboard::NamedKey::FavoriteStore2 => gridthorn_input::NamedKey::FavoriteStore2,
        winit::keyboard::NamedKey::FavoriteStore3 => gridthorn_input::NamedKey::FavoriteStore3,
        winit::keyboard::NamedKey::Guide => gridthorn_input::NamedKey::Guide,
        winit::keyboard::NamedKey::GuideNextDay => gridthorn_input::NamedKey::GuideNextDay,
        winit::keyboard::NamedKey::GuidePreviousDay => gridthorn_input::NamedKey::GuidePreviousDay,
        winit::keyboard::NamedKey::Info => gridthorn_input::NamedKey::Info,
        winit::keyboard::NamedKey::InstantReplay => gridthorn_input::NamedKey::InstantReplay,
        winit::keyboard::NamedKey::Link => gridthorn_input::NamedKey::Link,
        winit::keyboard::NamedKey::ListProgram => gridthorn_input::NamedKey::ListProgram,
        winit::keyboard::NamedKey::LiveContent => gridthorn_input::NamedKey::LiveContent,
        winit::keyboard::NamedKey::Lock => gridthorn_input::NamedKey::Lock,
        winit::keyboard::NamedKey::MediaApps => gridthorn_input::NamedKey::MediaApps,
        winit::keyboard::NamedKey::MediaAudioTrack => gridthorn_input::NamedKey::MediaAudioTrack,
        winit::keyboard::NamedKey::MediaLast => gridthorn_input::NamedKey::MediaLast,
        winit::keyboard::NamedKey::MediaSkipBackward => {
            gridthorn_input::NamedKey::MediaSkipBackward
        }
        winit::keyboard::NamedKey::MediaSkipForward => gridthorn_input::NamedKey::MediaSkipForward,
        winit::keyboard::NamedKey::MediaStepBackward => {
            gridthorn_input::NamedKey::MediaStepBackward
        }
        winit::keyboard::NamedKey::MediaStepForward => gridthorn_input::NamedKey::MediaStepForward,
        winit::keyboard::NamedKey::MediaTopMenu => gridthorn_input::NamedKey::MediaTopMenu,
        winit::keyboard::NamedKey::NavigateIn => gridthorn_input::NamedKey::NavigateIn,
        winit::keyboard::NamedKey::NavigateNext => gridthorn_input::NamedKey::NavigateNext,
        winit::keyboard::NamedKey::NavigateOut => gridthorn_input::NamedKey::NavigateOut,
        winit::keyboard::NamedKey::NavigatePrevious => gridthorn_input::NamedKey::NavigatePrevious,
        winit::keyboard::NamedKey::NextFavoriteChannel => {
            gridthorn_input::NamedKey::NextFavoriteChannel
        }
        winit::keyboard::NamedKey::NextUserProfile => gridthorn_input::NamedKey::NextUserProfile,
        winit::keyboard::NamedKey::OnDemand => gridthorn_input::NamedKey::OnDemand,
        winit::keyboard::NamedKey::Pairing => gridthorn_input::NamedKey::Pairing,
        winit::keyboard::NamedKey::PinPDown => gridthorn_input::NamedKey::PinPDown,
        winit::keyboard::NamedKey::PinPMove => gridthorn_input::NamedKey::PinPMove,
        winit::keyboard::NamedKey::PinPToggle => gridthorn_input::NamedKey::PinPToggle,
        winit::keyboard::NamedKey::PinPUp => gridthorn_input::NamedKey::PinPUp,
        winit::keyboard::NamedKey::PlaySpeedDown => gridthorn_input::NamedKey::PlaySpeedDown,
        winit::keyboard::NamedKey::PlaySpeedReset => gridthorn_input::NamedKey::PlaySpeedReset,
        winit::keyboard::NamedKey::PlaySpeedUp => gridthorn_input::NamedKey::PlaySpeedUp,
        winit::keyboard::NamedKey::RandomToggle => gridthorn_input::NamedKey::RandomToggle,
        winit::keyboard::NamedKey::RcLowBattery => gridthorn_input::NamedKey::RcLowBattery,
        winit::keyboard::NamedKey::RecordSpeedNext => gridthorn_input::NamedKey::RecordSpeedNext,
        winit::keyboard::NamedKey::RfBypass => gridthorn_input::NamedKey::RfBypass,
        winit::keyboard::NamedKey::ScanChannelsToggle => {
            gridthorn_input::NamedKey::ScanChannelsToggle
        }
        winit::keyboard::NamedKey::ScreenModeNext => gridthorn_input::NamedKey::ScreenModeNext,
        winit::keyboard::NamedKey::Settings => gridthorn_input::NamedKey::Settings,
        winit::keyboard::NamedKey::SplitScreenToggle => {
            gridthorn_input::NamedKey::SplitScreenToggle
        }
        winit::keyboard::NamedKey::STBInput => gridthorn_input::NamedKey::STBInput,
        winit::keyboard::NamedKey::STBPower => gridthorn_input::NamedKey::STBPower,
        winit::keyboard::NamedKey::Subtitle => gridthorn_input::NamedKey::Subtitle,
        winit::keyboard::NamedKey::Teletext => gridthorn_input::NamedKey::Teletext,
        winit::keyboard::NamedKey::VideoModeNext => gridthorn_input::NamedKey::VideoModeNext,
        winit::keyboard::NamedKey::Wink => gridthorn_input::NamedKey::Wink,
        winit::keyboard::NamedKey::ZoomToggle => gridthorn_input::NamedKey::ZoomToggle,
        winit::keyboard::NamedKey::F1 => gridthorn_input::NamedKey::F1,
        winit::keyboard::NamedKey::F2 => gridthorn_input::NamedKey::F2,
        winit::keyboard::NamedKey::F3 => gridthorn_input::NamedKey::F3,
        winit::keyboard::NamedKey::F4 => gridthorn_input::NamedKey::F4,
        winit::keyboard::NamedKey::F5 => gridthorn_input::NamedKey::F5,
        winit::keyboard::NamedKey::F6 => gridthorn_input::NamedKey::F6,
        winit::keyboard::NamedKey::F7 => gridthorn_input::NamedKey::F7,
        winit::keyboard::NamedKey::F8 => gridthorn_input::NamedKey::F8,
        winit::keyboard::NamedKey::F9 => gridthorn_input::NamedKey::F9,
        winit::keyboard::NamedKey::F10 => gridthorn_input::NamedKey::F10,
        winit::keyboard::NamedKey::F11 => gridthorn_input::NamedKey::F11,
        winit::keyboard::NamedKey::F12 => gridthorn_input::NamedKey::F12,
        winit::keyboard::NamedKey::F13 => gridthorn_input::NamedKey::F13,
        winit::keyboard::NamedKey::F14 => gridthorn_input::NamedKey::F14,
        winit::keyboard::NamedKey::F15 => gridthorn_input::NamedKey::F15,
        winit::keyboard::NamedKey::F16 => gridthorn_input::NamedKey::F16,
        winit::keyboard::NamedKey::F17 => gridthorn_input::NamedKey::F17,
        winit::keyboard::NamedKey::F18 => gridthorn_input::NamedKey::F18,
        winit::keyboard::NamedKey::F19 => gridthorn_input::NamedKey::F19,
        winit::keyboard::NamedKey::F20 => gridthorn_input::NamedKey::F20,
        winit::keyboard::NamedKey::F21 => gridthorn_input::NamedKey::F21,
        winit::keyboard::NamedKey::F22 => gridthorn_input::NamedKey::F22,
        winit::keyboard::NamedKey::F23 => gridthorn_input::NamedKey::F23,
        winit::keyboard::NamedKey::F24 => gridthorn_input::NamedKey::F24,
        winit::keyboard::NamedKey::F25 => gridthorn_input::NamedKey::F25,
        winit::keyboard::NamedKey::F26 => gridthorn_input::NamedKey::F26,
        winit::keyboard::NamedKey::F27 => gridthorn_input::NamedKey::F27,
        winit::keyboard::NamedKey::F28 => gridthorn_input::NamedKey::F28,
        winit::keyboard::NamedKey::F29 => gridthorn_input::NamedKey::F29,
        winit::keyboard::NamedKey::F30 => gridthorn_input::NamedKey::F30,
        winit::keyboard::NamedKey::F31 => gridthorn_input::NamedKey::F31,
        winit::keyboard::NamedKey::F32 => gridthorn_input::NamedKey::F32,
        winit::keyboard::NamedKey::F33 => gridthorn_input::NamedKey::F33,
        winit::keyboard::NamedKey::F34 => gridthorn_input::NamedKey::F34,
        winit::keyboard::NamedKey::F35 => gridthorn_input::NamedKey::F35,
        _ => gridthorn_input::NamedKey::Unidentified,
    }
}
