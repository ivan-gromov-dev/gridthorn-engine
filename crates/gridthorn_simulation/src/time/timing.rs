use std::time::Duration;

/// Observable timing result for one host frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameTiming {
    frame_elapsed: Duration,
    fixed_steps: u32,
    first_tick_index: u64,
    completed_ticks: u64,
    overloaded: bool,
    accumulated_lag: Duration,
}

impl FrameTiming {
    pub(crate) fn new(
        frame_elapsed: Duration,
        fixed_steps: u32,
        first_tick_index: u64,
        completed_ticks: u64,
        overloaded: bool,
        accumulated_lag: Duration,
    ) -> Self {
        Self {
            frame_elapsed,
            fixed_steps,
            first_tick_index,
            completed_ticks,
            overloaded,
            accumulated_lag,
        }
    }

    /// Host time accumulated for this frame.
    #[must_use]
    pub fn frame_elapsed(self) -> Duration {
        self.frame_elapsed
    }

    /// Number of fixed updates assigned to this host frame.
    #[must_use]
    pub fn fixed_steps(self) -> u32 {
        self.fixed_steps
    }

    /// Index assigned to the first fixed update in this frame.
    #[must_use]
    pub fn first_tick_index(self) -> u64 {
        self.first_tick_index
    }

    /// Total number of fixed ticks assigned since clock creation.
    #[must_use]
    pub fn completed_ticks(self) -> u64 {
        self.completed_ticks
    }

    /// Whether available work exceeded the configured per-frame limit.
    #[must_use]
    pub fn overloaded(self) -> bool {
        self.overloaded
    }

    /// Host time still waiting to be converted into fixed ticks.
    #[must_use]
    pub fn accumulated_lag(self) -> Duration {
        self.accumulated_lag
    }
}
