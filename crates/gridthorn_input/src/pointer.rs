/// Native pointer capture mode, independent from future UI routing ownership.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PointerCaptureMode {
    /// Free pointer movement.
    #[default]
    None,
    /// Restrict the cursor to the window.
    Confined,
    /// Lock the cursor position; movement arrives as relative motion.
    Locked,
}

/// Native capture feedback; unsupported requests retain the previous effective mode.
#[derive(Clone, Debug, PartialEq)]
pub struct PointerCaptureStatus {
    /// Mode requested by the caller.
    pub requested: PointerCaptureMode,
    /// Mode actually applied by the platform.
    pub effective: PointerCaptureMode,
    /// Contextual platform failure, when present.
    pub error: Option<crate::PointerCaptureError>,
}

/// Runtime resource for one-shot capture requests, processed after the current frame.
#[derive(Debug, Default)]
pub struct PointerCapture {
    pending: Option<PointerCaptureMode>,
}

impl PointerCapture {
    /// Queue a native capture request; the last request in a frame wins.
    pub fn request(&mut self, mode: PointerCaptureMode) {
        self.pending = Some(mode);
    }

    /// Take the pending request for a platform adapter.
    pub fn take_request(&mut self) -> Option<PointerCaptureMode> {
        self.pending.take()
    }
}

/// Wheel units are preserved rather than mixing lines and physical pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WheelDelta {
    /// Horizontal and vertical line counts.
    Lines { x: f32, y: f32 },
    /// Horizontal and vertical physical pixels.
    Pixels { x: f64, y: f64 },
}

/// Platform wheel gesture phase.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ScrollPhase {
    /// Gesture began.
    Started,
    /// Gesture continued.
    Moved,
    /// Gesture ended.
    Ended,
    /// Gesture was cancelled.
    Cancelled,
}
