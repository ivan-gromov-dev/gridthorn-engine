/// Session-local connection identity; never persist as a hardware serial number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ControllerId(pub u64);

/// Standardized controller buttons; labels refer to physical positions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ControllerButton {
    /// Bottom face button.
    South,
    /// Right face button.
    East,
    /// Top face button.
    North,
    /// Left face button.
    West,
    /// Left shoulder.
    LeftShoulder,
    /// Right shoulder.
    RightShoulder,
    /// Left trigger digital threshold.
    LeftTrigger,
    /// Right trigger digital threshold.
    RightTrigger,
    /// Select/back.
    Select,
    /// Start/menu.
    Start,
    /// System/home.
    Mode,
    /// Left stick click.
    LeftStick,
    /// Right stick click.
    RightStick,
    /// Directional pad up.
    DPadUp,
    /// Directional pad down.
    DPadDown,
    /// Directional pad left.
    DPadLeft,
    /// Directional pad right.
    DPadRight,
}
/// Standard axes. Stick Y is positive upward; triggers range from zero to one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ControllerAxis {
    /// Left stick horizontal.
    LeftStickX,
    /// Left stick vertical.
    LeftStickY,
    /// Right stick horizontal.
    RightStickX,
    /// Right stick vertical.
    RightStickY,
    /// Left trigger pressure.
    LeftTrigger,
    /// Right trigger pressure.
    RightTrigger,
}
/// Connection capabilities and descriptive model identity; UUID need not be unique.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ControllerInfo {
    /// Session connection key.
    pub id: ControllerId,
    /// Backend mapping or OS name.
    pub name: String,
    /// Backend model UUID; not a stable per-unit serial number.
    pub model_uuid: [u8; 16],
    /// USB vendor if available.
    pub vendor_id: Option<u16>,
    /// USB product if available.
    pub product_id: Option<u16>,
    /// Mapped buttons actually exposed by this device.
    pub buttons: Vec<ControllerButton>,
    /// Mapped axes actually exposed by this device.
    pub axes: Vec<ControllerAxis>,
    /// Backend reports force-feedback support; individual requests may still fail.
    pub rumble_supported: bool,
}
/// Ordered controller lifecycle, input and operation results.
#[derive(Clone, Debug, PartialEq)]
pub enum ControllerEvent {
    /// Native backend initialized, including when no devices are connected.
    Ready,
    /// Initial discovery or a new connection.
    Connected(ControllerInfo),
    /// Connection ended; all held state is removed.
    Disconnected(ControllerId),
    /// Digital transition.
    Button {
        /// Connection key.
        id: ControllerId,
        /// Physical button.
        button: ControllerButton,
        /// New state.
        state: crate::ButtonState,
    },
    /// Analog sample, without engine dead-zone filtering.
    Axis {
        /// Connection key.
        id: ControllerId,
        /// Standard axis.
        axis: ControllerAxis,
        /// Normalized value.
        value: f32,
    },
    /// Backend initialization failed; keyboard and pointer remain operational.
    Unavailable(super::ControllerError),
    /// Correlated request submission result; success does not prove physical vibration.
    Feedback {
        /// Caller request identity.
        request: u64,
        /// Target connection.
        id: ControllerId,
        /// Native submission result.
        result: Result<(), super::ControllerError>,
    },
}
