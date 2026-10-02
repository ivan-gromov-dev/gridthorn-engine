/// Engine-owned `NamedKey` values for desktop keyboard input.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum NamedKey {
    /// A future unmapped backend key.
    Unidentified,
    /// The `Alt` key.
    Alt,
    /// The `AltGraph` key.
    AltGraph,
    /// The `CapsLock` key.
    CapsLock,
    /// The `Control` key.
    Control,
    /// The `Fn` key.
    Fn,
    /// The `FnLock` key.
    FnLock,
    /// The `NumLock` key.
    NumLock,
    /// The `ScrollLock` key.
    ScrollLock,
    /// The `Shift` key.
    Shift,
    /// The `Symbol` key.
    Symbol,
    /// The `SymbolLock` key.
    SymbolLock,
    /// The `Meta` key.
    Meta,
    /// The `Hyper` key.
    Hyper,
    /// The `Super` key.
    Super,
    /// The `Enter` key.
    Enter,
    /// The `Tab` key.
    Tab,
    /// The `Space` key.
    Space,
    /// The `ArrowDown` key.
    ArrowDown,
    /// The `ArrowLeft` key.
    ArrowLeft,
    /// The `ArrowRight` key.
    ArrowRight,
    /// The `ArrowUp` key.
    ArrowUp,
    /// The `End` key.
    End,
    /// The `Home` key.
    Home,
    /// The `PageDown` key.
    PageDown,
    /// The `PageUp` key.
    PageUp,
    /// The `Backspace` key.
    Backspace,
    /// The `Clear` key.
    Clear,
    /// The `Copy` key.
    Copy,
    /// The `CrSel` key.
    CrSel,
    /// The `Cut` key.
    Cut,
    /// The `Delete` key.
    Delete,
    /// The `EraseEof` key.
    EraseEof,
    /// The `ExSel` key.
    ExSel,
    /// The `Insert` key.
    Insert,
    /// The `Paste` key.
    Paste,
    /// The `Redo` key.
    Redo,
    /// The `Undo` key.
    Undo,
    /// The `Accept` key.
    Accept,
    /// The `Again` key.
    Again,
    /// The `Attn` key.
    Attn,
    /// The `Cancel` key.
    Cancel,
    /// The `ContextMenu` key.
    ContextMenu,
    /// The `Escape` key.
    Escape,
    /// The `Execute` key.
    Execute,
    /// The `Find` key.
    Find,
    /// The `Help` key.
    Help,
    /// The `Pause` key.
    Pause,
    /// The `Play` key.
    Play,
    /// The `Props` key.
    Props,
    /// The `Select` key.
    Select,
    /// The `ZoomIn` key.
    ZoomIn,
    /// The `ZoomOut` key.
    ZoomOut,
    /// The `BrightnessDown` key.
    BrightnessDown,
    /// The `BrightnessUp` key.
    BrightnessUp,
    /// The `Eject` key.
    Eject,
    /// The `LogOff` key.
    LogOff,
    /// The `Power` key.
    Power,
    /// The `PowerOff` key.
    PowerOff,
    /// The `PrintScreen` key.
    PrintScreen,
    /// The `Hibernate` key.
    Hibernate,
    /// The `Standby` key.
    Standby,
    /// The `WakeUp` key.
    WakeUp,
    /// The `AllCandidates` key.
    AllCandidates,
    /// The `Alphanumeric` key.
    Alphanumeric,
    /// The `CodeInput` key.
    CodeInput,
    /// The `Compose` key.
    Compose,
    /// The `Convert` key.
    Convert,
    /// The `FinalMode` key.
    FinalMode,
    /// The `GroupFirst` key.
    GroupFirst,
    /// The `GroupLast` key.
    GroupLast,
    /// The `GroupNext` key.
    GroupNext,
    /// The `GroupPrevious` key.
    GroupPrevious,
    /// The `ModeChange` key.
    ModeChange,
    /// The `NextCandidate` key.
    NextCandidate,
    /// The `NonConvert` key.
    NonConvert,
    /// The `PreviousCandidate` key.
    PreviousCandidate,
    /// The `Process` key.
    Process,
    /// The `SingleCandidate` key.
    SingleCandidate,
    /// The `HangulMode` key.
    HangulMode,
    /// The `HanjaMode` key.
    HanjaMode,
    /// The `JunjaMode` key.
    JunjaMode,
    /// The `Eisu` key.
    Eisu,
    /// The `Hankaku` key.
    Hankaku,
    /// The `Hiragana` key.
    Hiragana,
    /// The `HiraganaKatakana` key.
    HiraganaKatakana,
    /// The `KanaMode` key.
    KanaMode,
    /// The `KanjiMode` key.
    KanjiMode,
    /// The `Katakana` key.
    Katakana,
    /// The `Romaji` key.
    Romaji,
    /// The `Zenkaku` key.
    Zenkaku,
    /// The `ZenkakuHankaku` key.
    ZenkakuHankaku,
    /// The `Soft1` key.
    Soft1,
    /// The `Soft2` key.
    Soft2,
    /// The `Soft3` key.
    Soft3,
    /// The `Soft4` key.
    Soft4,
    /// The `ChannelDown` key.
    ChannelDown,
    /// The `ChannelUp` key.
    ChannelUp,
    /// The `Close` key.
    Close,
    /// The `MailForward` key.
    MailForward,
    /// The `MailReply` key.
    MailReply,
    /// The `MailSend` key.
    MailSend,
    /// The `MediaClose` key.
    MediaClose,
    /// The `MediaFastForward` key.
    MediaFastForward,
    /// The `MediaPause` key.
    MediaPause,
    /// The `MediaPlay` key.
    MediaPlay,
    /// The `MediaPlayPause` key.
    MediaPlayPause,
    /// The `MediaRecord` key.
    MediaRecord,
    /// The `MediaRewind` key.
    MediaRewind,
    /// The `MediaStop` key.
    MediaStop,
    /// The `MediaTrackNext` key.
    MediaTrackNext,
    /// The `MediaTrackPrevious` key.
    MediaTrackPrevious,
    /// The `New` key.
    New,
    /// The `Open` key.
    Open,
    /// The `Print` key.
    Print,
    /// The `Save` key.
    Save,
    /// The `SpellCheck` key.
    SpellCheck,
    /// The `Key11` key.
    Key11,
    /// The `Key12` key.
    Key12,
    /// The `AudioBalanceLeft` key.
    AudioBalanceLeft,
    /// The `AudioBalanceRight` key.
    AudioBalanceRight,
    /// The `AudioBassBoostDown` key.
    AudioBassBoostDown,
    /// The `AudioBassBoostToggle` key.
    AudioBassBoostToggle,
    /// The `AudioBassBoostUp` key.
    AudioBassBoostUp,
    /// The `AudioFaderFront` key.
    AudioFaderFront,
    /// The `AudioFaderRear` key.
    AudioFaderRear,
    /// The `AudioSurroundModeNext` key.
    AudioSurroundModeNext,
    /// The `AudioTrebleDown` key.
    AudioTrebleDown,
    /// The `AudioTrebleUp` key.
    AudioTrebleUp,
    /// The `AudioVolumeDown` key.
    AudioVolumeDown,
    /// The `AudioVolumeUp` key.
    AudioVolumeUp,
    /// The `AudioVolumeMute` key.
    AudioVolumeMute,
    /// The `MicrophoneToggle` key.
    MicrophoneToggle,
    /// The `MicrophoneVolumeDown` key.
    MicrophoneVolumeDown,
    /// The `MicrophoneVolumeUp` key.
    MicrophoneVolumeUp,
    /// The `MicrophoneVolumeMute` key.
    MicrophoneVolumeMute,
    /// The `SpeechCorrectionList` key.
    SpeechCorrectionList,
    /// The `SpeechInputToggle` key.
    SpeechInputToggle,
    /// The `LaunchApplication1` key.
    LaunchApplication1,
    /// The `LaunchApplication2` key.
    LaunchApplication2,
    /// The `LaunchCalendar` key.
    LaunchCalendar,
    /// The `LaunchContacts` key.
    LaunchContacts,
    /// The `LaunchMail` key.
    LaunchMail,
    /// The `LaunchMediaPlayer` key.
    LaunchMediaPlayer,
    /// The `LaunchMusicPlayer` key.
    LaunchMusicPlayer,
    /// The `LaunchPhone` key.
    LaunchPhone,
    /// The `LaunchScreenSaver` key.
    LaunchScreenSaver,
    /// The `LaunchSpreadsheet` key.
    LaunchSpreadsheet,
    /// The `LaunchWebBrowser` key.
    LaunchWebBrowser,
    /// The `LaunchWebCam` key.
    LaunchWebCam,
    /// The `LaunchWordProcessor` key.
    LaunchWordProcessor,
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
    /// The `AppSwitch` key.
    AppSwitch,
    /// The `Call` key.
    Call,
    /// The `Camera` key.
    Camera,
    /// The `CameraFocus` key.
    CameraFocus,
    /// The `EndCall` key.
    EndCall,
    /// The `GoBack` key.
    GoBack,
    /// The `GoHome` key.
    GoHome,
    /// The `HeadsetHook` key.
    HeadsetHook,
    /// The `LastNumberRedial` key.
    LastNumberRedial,
    /// The `Notification` key.
    Notification,
    /// The `MannerMode` key.
    MannerMode,
    /// The `VoiceDial` key.
    VoiceDial,
    /// The `TV` key.
    TV,
    /// The `TV3DMode` key.
    TV3DMode,
    /// The `TVAntennaCable` key.
    TVAntennaCable,
    /// The `TVAudioDescription` key.
    TVAudioDescription,
    /// The `TVAudioDescriptionMixDown` key.
    TVAudioDescriptionMixDown,
    /// The `TVAudioDescriptionMixUp` key.
    TVAudioDescriptionMixUp,
    /// The `TVContentsMenu` key.
    TVContentsMenu,
    /// The `TVDataService` key.
    TVDataService,
    /// The `TVInput` key.
    TVInput,
    /// The `TVInputComponent1` key.
    TVInputComponent1,
    /// The `TVInputComponent2` key.
    TVInputComponent2,
    /// The `TVInputComposite1` key.
    TVInputComposite1,
    /// The `TVInputComposite2` key.
    TVInputComposite2,
    /// The `TVInputHDMI1` key.
    TVInputHDMI1,
    /// The `TVInputHDMI2` key.
    TVInputHDMI2,
    /// The `TVInputHDMI3` key.
    TVInputHDMI3,
    /// The `TVInputHDMI4` key.
    TVInputHDMI4,
    /// The `TVInputVGA1` key.
    TVInputVGA1,
    /// The `TVMediaContext` key.
    TVMediaContext,
    /// The `TVNetwork` key.
    TVNetwork,
    /// The `TVNumberEntry` key.
    TVNumberEntry,
    /// The `TVPower` key.
    TVPower,
    /// The `TVRadioService` key.
    TVRadioService,
    /// The `TVSatellite` key.
    TVSatellite,
    /// The `TVSatelliteBS` key.
    TVSatelliteBS,
    /// The `TVSatelliteCS` key.
    TVSatelliteCS,
    /// The `TVSatelliteToggle` key.
    TVSatelliteToggle,
    /// The `TVTerrestrialAnalog` key.
    TVTerrestrialAnalog,
    /// The `TVTerrestrialDigital` key.
    TVTerrestrialDigital,
    /// The `TVTimer` key.
    TVTimer,
    /// The `AVRInput` key.
    AVRInput,
    /// The `AVRPower` key.
    AVRPower,
    /// The `ColorF0Red` key.
    ColorF0Red,
    /// The `ColorF1Green` key.
    ColorF1Green,
    /// The `ColorF2Yellow` key.
    ColorF2Yellow,
    /// The `ColorF3Blue` key.
    ColorF3Blue,
    /// The `ColorF4Grey` key.
    ColorF4Grey,
    /// The `ColorF5Brown` key.
    ColorF5Brown,
    /// The `ClosedCaptionToggle` key.
    ClosedCaptionToggle,
    /// The `Dimmer` key.
    Dimmer,
    /// The `DisplaySwap` key.
    DisplaySwap,
    /// The `DVR` key.
    DVR,
    /// The `Exit` key.
    Exit,
    /// The `FavoriteClear0` key.
    FavoriteClear0,
    /// The `FavoriteClear1` key.
    FavoriteClear1,
    /// The `FavoriteClear2` key.
    FavoriteClear2,
    /// The `FavoriteClear3` key.
    FavoriteClear3,
    /// The `FavoriteRecall0` key.
    FavoriteRecall0,
    /// The `FavoriteRecall1` key.
    FavoriteRecall1,
    /// The `FavoriteRecall2` key.
    FavoriteRecall2,
    /// The `FavoriteRecall3` key.
    FavoriteRecall3,
    /// The `FavoriteStore0` key.
    FavoriteStore0,
    /// The `FavoriteStore1` key.
    FavoriteStore1,
    /// The `FavoriteStore2` key.
    FavoriteStore2,
    /// The `FavoriteStore3` key.
    FavoriteStore3,
    /// The `Guide` key.
    Guide,
    /// The `GuideNextDay` key.
    GuideNextDay,
    /// The `GuidePreviousDay` key.
    GuidePreviousDay,
    /// The `Info` key.
    Info,
    /// The `InstantReplay` key.
    InstantReplay,
    /// The `Link` key.
    Link,
    /// The `ListProgram` key.
    ListProgram,
    /// The `LiveContent` key.
    LiveContent,
    /// The `Lock` key.
    Lock,
    /// The `MediaApps` key.
    MediaApps,
    /// The `MediaAudioTrack` key.
    MediaAudioTrack,
    /// The `MediaLast` key.
    MediaLast,
    /// The `MediaSkipBackward` key.
    MediaSkipBackward,
    /// The `MediaSkipForward` key.
    MediaSkipForward,
    /// The `MediaStepBackward` key.
    MediaStepBackward,
    /// The `MediaStepForward` key.
    MediaStepForward,
    /// The `MediaTopMenu` key.
    MediaTopMenu,
    /// The `NavigateIn` key.
    NavigateIn,
    /// The `NavigateNext` key.
    NavigateNext,
    /// The `NavigateOut` key.
    NavigateOut,
    /// The `NavigatePrevious` key.
    NavigatePrevious,
    /// The `NextFavoriteChannel` key.
    NextFavoriteChannel,
    /// The `NextUserProfile` key.
    NextUserProfile,
    /// The `OnDemand` key.
    OnDemand,
    /// The `Pairing` key.
    Pairing,
    /// The `PinPDown` key.
    PinPDown,
    /// The `PinPMove` key.
    PinPMove,
    /// The `PinPToggle` key.
    PinPToggle,
    /// The `PinPUp` key.
    PinPUp,
    /// The `PlaySpeedDown` key.
    PlaySpeedDown,
    /// The `PlaySpeedReset` key.
    PlaySpeedReset,
    /// The `PlaySpeedUp` key.
    PlaySpeedUp,
    /// The `RandomToggle` key.
    RandomToggle,
    /// The `RcLowBattery` key.
    RcLowBattery,
    /// The `RecordSpeedNext` key.
    RecordSpeedNext,
    /// The `RfBypass` key.
    RfBypass,
    /// The `ScanChannelsToggle` key.
    ScanChannelsToggle,
    /// The `ScreenModeNext` key.
    ScreenModeNext,
    /// The `Settings` key.
    Settings,
    /// The `SplitScreenToggle` key.
    SplitScreenToggle,
    /// The `STBInput` key.
    STBInput,
    /// The `STBPower` key.
    STBPower,
    /// The `Subtitle` key.
    Subtitle,
    /// The `Teletext` key.
    Teletext,
    /// The `VideoModeNext` key.
    VideoModeNext,
    /// The `Wink` key.
    Wink,
    /// The `ZoomToggle` key.
    ZoomToggle,
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
