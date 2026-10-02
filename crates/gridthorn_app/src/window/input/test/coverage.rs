use super::super::*;
#[test]
#[allow(
    clippy::too_many_lines,
    reason = "exhaustive physical key conversion contract"
)]
fn maps_every_standardized_physical_key() {
    assert_eq!(map_key_code(KeyCode::Backquote), EngineKeyCode::Backquote);
    assert_eq!(map_key_code(KeyCode::Backslash), EngineKeyCode::Backslash);
    assert_eq!(
        map_key_code(KeyCode::BracketLeft),
        EngineKeyCode::BracketLeft
    );
    assert_eq!(
        map_key_code(KeyCode::BracketRight),
        EngineKeyCode::BracketRight
    );
    assert_eq!(map_key_code(KeyCode::Comma), EngineKeyCode::Comma);
    assert_eq!(map_key_code(KeyCode::Digit0), EngineKeyCode::Digit0);
    assert_eq!(map_key_code(KeyCode::Digit1), EngineKeyCode::Digit1);
    assert_eq!(map_key_code(KeyCode::Digit2), EngineKeyCode::Digit2);
    assert_eq!(map_key_code(KeyCode::Digit3), EngineKeyCode::Digit3);
    assert_eq!(map_key_code(KeyCode::Digit4), EngineKeyCode::Digit4);
    assert_eq!(map_key_code(KeyCode::Digit5), EngineKeyCode::Digit5);
    assert_eq!(map_key_code(KeyCode::Digit6), EngineKeyCode::Digit6);
    assert_eq!(map_key_code(KeyCode::Digit7), EngineKeyCode::Digit7);
    assert_eq!(map_key_code(KeyCode::Digit8), EngineKeyCode::Digit8);
    assert_eq!(map_key_code(KeyCode::Digit9), EngineKeyCode::Digit9);
    assert_eq!(map_key_code(KeyCode::Equal), EngineKeyCode::Equal);
    assert_eq!(
        map_key_code(KeyCode::IntlBackslash),
        EngineKeyCode::IntlBackslash
    );
    assert_eq!(map_key_code(KeyCode::IntlRo), EngineKeyCode::IntlRo);
    assert_eq!(map_key_code(KeyCode::IntlYen), EngineKeyCode::IntlYen);
    assert_eq!(map_key_code(KeyCode::KeyA), EngineKeyCode::KeyA);
    assert_eq!(map_key_code(KeyCode::KeyB), EngineKeyCode::KeyB);
    assert_eq!(map_key_code(KeyCode::KeyC), EngineKeyCode::KeyC);
    assert_eq!(map_key_code(KeyCode::KeyD), EngineKeyCode::KeyD);
    assert_eq!(map_key_code(KeyCode::KeyE), EngineKeyCode::KeyE);
    assert_eq!(map_key_code(KeyCode::KeyF), EngineKeyCode::KeyF);
    assert_eq!(map_key_code(KeyCode::KeyG), EngineKeyCode::KeyG);
    assert_eq!(map_key_code(KeyCode::KeyH), EngineKeyCode::KeyH);
    assert_eq!(map_key_code(KeyCode::KeyI), EngineKeyCode::KeyI);
    assert_eq!(map_key_code(KeyCode::KeyJ), EngineKeyCode::KeyJ);
    assert_eq!(map_key_code(KeyCode::KeyK), EngineKeyCode::KeyK);
    assert_eq!(map_key_code(KeyCode::KeyL), EngineKeyCode::KeyL);
    assert_eq!(map_key_code(KeyCode::KeyM), EngineKeyCode::KeyM);
    assert_eq!(map_key_code(KeyCode::KeyN), EngineKeyCode::KeyN);
    assert_eq!(map_key_code(KeyCode::KeyO), EngineKeyCode::KeyO);
    assert_eq!(map_key_code(KeyCode::KeyP), EngineKeyCode::KeyP);
    assert_eq!(map_key_code(KeyCode::KeyQ), EngineKeyCode::KeyQ);
    assert_eq!(map_key_code(KeyCode::KeyR), EngineKeyCode::KeyR);
    assert_eq!(map_key_code(KeyCode::KeyS), EngineKeyCode::KeyS);
    assert_eq!(map_key_code(KeyCode::KeyT), EngineKeyCode::KeyT);
    assert_eq!(map_key_code(KeyCode::KeyU), EngineKeyCode::KeyU);
    assert_eq!(map_key_code(KeyCode::KeyV), EngineKeyCode::KeyV);
    assert_eq!(map_key_code(KeyCode::KeyW), EngineKeyCode::KeyW);
    assert_eq!(map_key_code(KeyCode::KeyX), EngineKeyCode::KeyX);
    assert_eq!(map_key_code(KeyCode::KeyY), EngineKeyCode::KeyY);
    assert_eq!(map_key_code(KeyCode::KeyZ), EngineKeyCode::KeyZ);
    assert_eq!(map_key_code(KeyCode::Minus), EngineKeyCode::Minus);
    assert_eq!(map_key_code(KeyCode::Period), EngineKeyCode::Period);
    assert_eq!(map_key_code(KeyCode::Quote), EngineKeyCode::Quote);
    assert_eq!(map_key_code(KeyCode::Semicolon), EngineKeyCode::Semicolon);
    assert_eq!(map_key_code(KeyCode::Slash), EngineKeyCode::Slash);
    assert_eq!(map_key_code(KeyCode::AltLeft), EngineKeyCode::AltLeft);
    assert_eq!(map_key_code(KeyCode::AltRight), EngineKeyCode::AltRight);
    assert_eq!(map_key_code(KeyCode::Backspace), EngineKeyCode::Backspace);
    assert_eq!(map_key_code(KeyCode::CapsLock), EngineKeyCode::CapsLock);
    assert_eq!(
        map_key_code(KeyCode::ContextMenu),
        EngineKeyCode::ContextMenu
    );
    assert_eq!(
        map_key_code(KeyCode::ControlLeft),
        EngineKeyCode::ControlLeft
    );
    assert_eq!(
        map_key_code(KeyCode::ControlRight),
        EngineKeyCode::ControlRight
    );
    assert_eq!(map_key_code(KeyCode::Enter), EngineKeyCode::Enter);
    assert_eq!(map_key_code(KeyCode::SuperLeft), EngineKeyCode::SuperLeft);
    assert_eq!(map_key_code(KeyCode::SuperRight), EngineKeyCode::SuperRight);
    assert_eq!(map_key_code(KeyCode::ShiftLeft), EngineKeyCode::ShiftLeft);
    assert_eq!(map_key_code(KeyCode::ShiftRight), EngineKeyCode::ShiftRight);
    assert_eq!(map_key_code(KeyCode::Space), EngineKeyCode::Space);
    assert_eq!(map_key_code(KeyCode::Tab), EngineKeyCode::Tab);
    assert_eq!(map_key_code(KeyCode::Convert), EngineKeyCode::Convert);
    assert_eq!(map_key_code(KeyCode::KanaMode), EngineKeyCode::KanaMode);
    assert_eq!(map_key_code(KeyCode::Lang1), EngineKeyCode::Lang1);
    assert_eq!(map_key_code(KeyCode::Lang2), EngineKeyCode::Lang2);
    assert_eq!(map_key_code(KeyCode::Lang3), EngineKeyCode::Lang3);
    assert_eq!(map_key_code(KeyCode::Lang4), EngineKeyCode::Lang4);
    assert_eq!(map_key_code(KeyCode::Lang5), EngineKeyCode::Lang5);
    assert_eq!(map_key_code(KeyCode::NonConvert), EngineKeyCode::NonConvert);
    assert_eq!(map_key_code(KeyCode::Delete), EngineKeyCode::Delete);
    assert_eq!(map_key_code(KeyCode::End), EngineKeyCode::End);
    assert_eq!(map_key_code(KeyCode::Help), EngineKeyCode::Help);
    assert_eq!(map_key_code(KeyCode::Home), EngineKeyCode::Home);
    assert_eq!(map_key_code(KeyCode::Insert), EngineKeyCode::Insert);
    assert_eq!(map_key_code(KeyCode::PageDown), EngineKeyCode::PageDown);
    assert_eq!(map_key_code(KeyCode::PageUp), EngineKeyCode::PageUp);
    assert_eq!(map_key_code(KeyCode::ArrowDown), EngineKeyCode::ArrowDown);
    assert_eq!(map_key_code(KeyCode::ArrowLeft), EngineKeyCode::ArrowLeft);
    assert_eq!(map_key_code(KeyCode::ArrowRight), EngineKeyCode::ArrowRight);
    assert_eq!(map_key_code(KeyCode::ArrowUp), EngineKeyCode::ArrowUp);
    assert_eq!(map_key_code(KeyCode::NumLock), EngineKeyCode::NumLock);
    assert_eq!(map_key_code(KeyCode::Numpad0), EngineKeyCode::Numpad0);
    assert_eq!(map_key_code(KeyCode::Numpad1), EngineKeyCode::Numpad1);
    assert_eq!(map_key_code(KeyCode::Numpad2), EngineKeyCode::Numpad2);
    assert_eq!(map_key_code(KeyCode::Numpad3), EngineKeyCode::Numpad3);
    assert_eq!(map_key_code(KeyCode::Numpad4), EngineKeyCode::Numpad4);
    assert_eq!(map_key_code(KeyCode::Numpad5), EngineKeyCode::Numpad5);
    assert_eq!(map_key_code(KeyCode::Numpad6), EngineKeyCode::Numpad6);
    assert_eq!(map_key_code(KeyCode::Numpad7), EngineKeyCode::Numpad7);
    assert_eq!(map_key_code(KeyCode::Numpad8), EngineKeyCode::Numpad8);
    assert_eq!(map_key_code(KeyCode::Numpad9), EngineKeyCode::Numpad9);
    assert_eq!(map_key_code(KeyCode::NumpadAdd), EngineKeyCode::NumpadAdd);
    assert_eq!(
        map_key_code(KeyCode::NumpadBackspace),
        EngineKeyCode::NumpadBackspace
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadClear),
        EngineKeyCode::NumpadClear
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadClearEntry),
        EngineKeyCode::NumpadClearEntry
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadComma),
        EngineKeyCode::NumpadComma
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadDecimal),
        EngineKeyCode::NumpadDecimal
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadDivide),
        EngineKeyCode::NumpadDivide
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadEnter),
        EngineKeyCode::NumpadEnter
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadEqual),
        EngineKeyCode::NumpadEqual
    );
    assert_eq!(map_key_code(KeyCode::NumpadHash), EngineKeyCode::NumpadHash);
    assert_eq!(
        map_key_code(KeyCode::NumpadMemoryAdd),
        EngineKeyCode::NumpadMemoryAdd
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadMemoryClear),
        EngineKeyCode::NumpadMemoryClear
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadMemoryRecall),
        EngineKeyCode::NumpadMemoryRecall
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadMemoryStore),
        EngineKeyCode::NumpadMemoryStore
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadMemorySubtract),
        EngineKeyCode::NumpadMemorySubtract
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadMultiply),
        EngineKeyCode::NumpadMultiply
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadParenLeft),
        EngineKeyCode::NumpadParenLeft
    );
    assert_eq!(
        map_key_code(KeyCode::NumpadParenRight),
        EngineKeyCode::NumpadParenRight
    );
    assert_eq!(map_key_code(KeyCode::NumpadStar), EngineKeyCode::NumpadStar);
    assert_eq!(
        map_key_code(KeyCode::NumpadSubtract),
        EngineKeyCode::NumpadSubtract
    );
    assert_eq!(map_key_code(KeyCode::Escape), EngineKeyCode::Escape);
    assert_eq!(map_key_code(KeyCode::Fn), EngineKeyCode::Fn);
    assert_eq!(map_key_code(KeyCode::FnLock), EngineKeyCode::FnLock);
    assert_eq!(
        map_key_code(KeyCode::PrintScreen),
        EngineKeyCode::PrintScreen
    );
    assert_eq!(map_key_code(KeyCode::ScrollLock), EngineKeyCode::ScrollLock);
    assert_eq!(map_key_code(KeyCode::Pause), EngineKeyCode::Pause);
    assert_eq!(
        map_key_code(KeyCode::BrowserBack),
        EngineKeyCode::BrowserBack
    );
    assert_eq!(
        map_key_code(KeyCode::BrowserFavorites),
        EngineKeyCode::BrowserFavorites
    );
    assert_eq!(
        map_key_code(KeyCode::BrowserForward),
        EngineKeyCode::BrowserForward
    );
    assert_eq!(
        map_key_code(KeyCode::BrowserHome),
        EngineKeyCode::BrowserHome
    );
    assert_eq!(
        map_key_code(KeyCode::BrowserRefresh),
        EngineKeyCode::BrowserRefresh
    );
    assert_eq!(
        map_key_code(KeyCode::BrowserSearch),
        EngineKeyCode::BrowserSearch
    );
    assert_eq!(
        map_key_code(KeyCode::BrowserStop),
        EngineKeyCode::BrowserStop
    );
    assert_eq!(map_key_code(KeyCode::Eject), EngineKeyCode::Eject);
    assert_eq!(map_key_code(KeyCode::LaunchApp1), EngineKeyCode::LaunchApp1);
    assert_eq!(map_key_code(KeyCode::LaunchApp2), EngineKeyCode::LaunchApp2);
    assert_eq!(map_key_code(KeyCode::LaunchMail), EngineKeyCode::LaunchMail);
    assert_eq!(
        map_key_code(KeyCode::MediaPlayPause),
        EngineKeyCode::MediaPlayPause
    );
    assert_eq!(
        map_key_code(KeyCode::MediaSelect),
        EngineKeyCode::MediaSelect
    );
    assert_eq!(map_key_code(KeyCode::MediaStop), EngineKeyCode::MediaStop);
    assert_eq!(
        map_key_code(KeyCode::MediaTrackNext),
        EngineKeyCode::MediaTrackNext
    );
    assert_eq!(
        map_key_code(KeyCode::MediaTrackPrevious),
        EngineKeyCode::MediaTrackPrevious
    );
    assert_eq!(map_key_code(KeyCode::Power), EngineKeyCode::Power);
    assert_eq!(map_key_code(KeyCode::Sleep), EngineKeyCode::Sleep);
    assert_eq!(
        map_key_code(KeyCode::AudioVolumeDown),
        EngineKeyCode::AudioVolumeDown
    );
    assert_eq!(
        map_key_code(KeyCode::AudioVolumeMute),
        EngineKeyCode::AudioVolumeMute
    );
    assert_eq!(
        map_key_code(KeyCode::AudioVolumeUp),
        EngineKeyCode::AudioVolumeUp
    );
    assert_eq!(map_key_code(KeyCode::WakeUp), EngineKeyCode::WakeUp);
    assert_eq!(map_key_code(KeyCode::Meta), EngineKeyCode::Meta);
    assert_eq!(map_key_code(KeyCode::Hyper), EngineKeyCode::Hyper);
    assert_eq!(map_key_code(KeyCode::Turbo), EngineKeyCode::Turbo);
    assert_eq!(map_key_code(KeyCode::Abort), EngineKeyCode::Abort);
    assert_eq!(map_key_code(KeyCode::Resume), EngineKeyCode::Resume);
    assert_eq!(map_key_code(KeyCode::Suspend), EngineKeyCode::Suspend);
    assert_eq!(map_key_code(KeyCode::Again), EngineKeyCode::Again);
    assert_eq!(map_key_code(KeyCode::Copy), EngineKeyCode::Copy);
    assert_eq!(map_key_code(KeyCode::Cut), EngineKeyCode::Cut);
    assert_eq!(map_key_code(KeyCode::Find), EngineKeyCode::Find);
    assert_eq!(map_key_code(KeyCode::Open), EngineKeyCode::Open);
    assert_eq!(map_key_code(KeyCode::Paste), EngineKeyCode::Paste);
    assert_eq!(map_key_code(KeyCode::Props), EngineKeyCode::Props);
    assert_eq!(map_key_code(KeyCode::Select), EngineKeyCode::Select);
    assert_eq!(map_key_code(KeyCode::Undo), EngineKeyCode::Undo);
    assert_eq!(map_key_code(KeyCode::Hiragana), EngineKeyCode::Hiragana);
    assert_eq!(map_key_code(KeyCode::Katakana), EngineKeyCode::Katakana);
    assert_eq!(map_key_code(KeyCode::F1), EngineKeyCode::F1);
    assert_eq!(map_key_code(KeyCode::F2), EngineKeyCode::F2);
    assert_eq!(map_key_code(KeyCode::F3), EngineKeyCode::F3);
    assert_eq!(map_key_code(KeyCode::F4), EngineKeyCode::F4);
    assert_eq!(map_key_code(KeyCode::F5), EngineKeyCode::F5);
    assert_eq!(map_key_code(KeyCode::F6), EngineKeyCode::F6);
    assert_eq!(map_key_code(KeyCode::F7), EngineKeyCode::F7);
    assert_eq!(map_key_code(KeyCode::F8), EngineKeyCode::F8);
    assert_eq!(map_key_code(KeyCode::F9), EngineKeyCode::F9);
    assert_eq!(map_key_code(KeyCode::F10), EngineKeyCode::F10);
    assert_eq!(map_key_code(KeyCode::F11), EngineKeyCode::F11);
    assert_eq!(map_key_code(KeyCode::F12), EngineKeyCode::F12);
    assert_eq!(map_key_code(KeyCode::F13), EngineKeyCode::F13);
    assert_eq!(map_key_code(KeyCode::F14), EngineKeyCode::F14);
    assert_eq!(map_key_code(KeyCode::F15), EngineKeyCode::F15);
    assert_eq!(map_key_code(KeyCode::F16), EngineKeyCode::F16);
    assert_eq!(map_key_code(KeyCode::F17), EngineKeyCode::F17);
    assert_eq!(map_key_code(KeyCode::F18), EngineKeyCode::F18);
    assert_eq!(map_key_code(KeyCode::F19), EngineKeyCode::F19);
    assert_eq!(map_key_code(KeyCode::F20), EngineKeyCode::F20);
    assert_eq!(map_key_code(KeyCode::F21), EngineKeyCode::F21);
    assert_eq!(map_key_code(KeyCode::F22), EngineKeyCode::F22);
    assert_eq!(map_key_code(KeyCode::F23), EngineKeyCode::F23);
    assert_eq!(map_key_code(KeyCode::F24), EngineKeyCode::F24);
    assert_eq!(map_key_code(KeyCode::F25), EngineKeyCode::F25);
    assert_eq!(map_key_code(KeyCode::F26), EngineKeyCode::F26);
    assert_eq!(map_key_code(KeyCode::F27), EngineKeyCode::F27);
    assert_eq!(map_key_code(KeyCode::F28), EngineKeyCode::F28);
    assert_eq!(map_key_code(KeyCode::F29), EngineKeyCode::F29);
    assert_eq!(map_key_code(KeyCode::F30), EngineKeyCode::F30);
    assert_eq!(map_key_code(KeyCode::F31), EngineKeyCode::F31);
    assert_eq!(map_key_code(KeyCode::F32), EngineKeyCode::F32);
    assert_eq!(map_key_code(KeyCode::F33), EngineKeyCode::F33);
    assert_eq!(map_key_code(KeyCode::F34), EngineKeyCode::F34);
    assert_eq!(map_key_code(KeyCode::F35), EngineKeyCode::F35);
}
