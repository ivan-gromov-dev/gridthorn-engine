use std::time::Duration;

use super::FixedStepConfigError;

/// Configuration for fixed-step simulation catch-up.
///
/// The default uses a 16,666,667 ns fixed step and at most eight catch-up
/// updates per host frame.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixedStepConfig {
    fixed_step: Duration,
    max_catch_up_steps: u32,
}

impl FixedStepConfig {
    /// Create validated fixed-step configuration.
    ///
    /// # Errors
    ///
    /// Returns an error when the duration or catch-up limit is zero.
    pub fn new(
        fixed_step: Duration,
        max_catch_up_steps: u32,
    ) -> Result<Self, FixedStepConfigError> {
        if fixed_step.is_zero() {
            return Err(FixedStepConfigError::ZeroDuration);
        }
        if max_catch_up_steps == 0 {
            return Err(FixedStepConfigError::ZeroCatchUpLimit);
        }
        Ok(Self {
            fixed_step,
            max_catch_up_steps,
        })
    }

    /// Duration represented by one authoritative tick.
    #[must_use]
    pub fn fixed_step(self) -> Duration {
        self.fixed_step
    }

    /// Maximum fixed ticks executed during one interactive frame.
    #[must_use]
    pub fn max_catch_up_steps(self) -> u32 {
        self.max_catch_up_steps
    }
}

impl Default for FixedStepConfig {
    fn default() -> Self {
        Self {
            fixed_step: Duration::from_nanos(16_666_667),
            max_catch_up_steps: 8,
        }
    }
}
