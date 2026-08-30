use std::time::Duration;

/// Authoritative time assigned to one fixed update.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedTime {
    tick_index: u64,
    fixed_step: Duration,
}

impl FixedTime {
    /// Create authoritative time for one scheduled fixed tick.
    #[must_use]
    pub fn new(tick_index: u64, fixed_step: Duration) -> Self {
        Self {
            tick_index,
            fixed_step,
        }
    }

    /// Zero-based index of the current authoritative tick.
    #[must_use]
    pub fn tick_index(self) -> u64 {
        self.tick_index
    }

    /// Immutable duration represented by the current tick.
    #[must_use]
    pub fn fixed_step(self) -> Duration {
        self.fixed_step
    }
}
