use std::time::Duration;

/// Frame timing sample rendered as a compact screen-space diagnostic overlay.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimingOverlay {
    frame_elapsed: Duration,
    fixed_steps: u32,
    accumulated_lag: Duration,
    overloaded: bool,
}

impl TimingOverlay {
    /// Create a timing sample for one presented host frame.
    #[must_use]
    pub fn new(
        frame_elapsed: Duration,
        fixed_steps: u32,
        accumulated_lag: Duration,
        overloaded: bool,
    ) -> Self {
        Self {
            frame_elapsed,
            fixed_steps,
            accumulated_lag,
            overloaded,
        }
    }

    /// Host duration represented by this frame.
    #[must_use]
    pub fn frame_elapsed(self) -> Duration {
        self.frame_elapsed
    }

    /// Fixed updates executed in this frame.
    #[must_use]
    pub fn fixed_steps(self) -> u32 {
        self.fixed_steps
    }

    /// Fixed-step backlog remaining after this frame.
    #[must_use]
    pub fn accumulated_lag(self) -> Duration {
        self.accumulated_lag
    }

    /// Whether fixed work exceeded the configured catch-up limit.
    #[must_use]
    pub fn overloaded(self) -> bool {
        self.overloaded
    }
}
