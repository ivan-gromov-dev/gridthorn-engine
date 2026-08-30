use std::time::Duration;

use super::{FixedStepConfig, FrameTiming, TimeError};

/// Accumulates host-frame time into deterministic fixed-step assignments.
pub struct FixedStepClock {
    config: FixedStepConfig,
    accumulated: Duration,
    completed_ticks: u64,
}

impl FixedStepClock {
    /// Create a clock with validated fixed-step configuration.
    #[must_use]
    pub fn new(config: FixedStepConfig) -> Self {
        Self {
            config,
            accumulated: Duration::ZERO,
            completed_ticks: 0,
        }
    }

    /// Fixed duration represented by every assigned tick.
    #[must_use]
    pub fn fixed_step(&self) -> Duration {
        self.config.fixed_step()
    }

    /// Accumulate one host-frame duration and assign bounded fixed work.
    ///
    /// Excess catch-up work remains accumulated and is reported as overload.
    ///
    /// # Errors
    ///
    /// Returns an error without changing the clock if elapsed time or the tick
    /// index exceeds its supported range.
    pub fn advance(&mut self, elapsed: Duration) -> Result<FrameTiming, TimeError> {
        let accumulated = self
            .accumulated
            .checked_add(elapsed)
            .ok_or(TimeError::ElapsedArithmeticOverflow)?;
        let available_steps_wide = accumulated.as_nanos() / self.config.fixed_step().as_nanos();
        let available_steps = u32::try_from(available_steps_wide).unwrap_or(u32::MAX);
        let fixed_steps = available_steps.min(self.config.max_catch_up_steps());
        let completed_ticks = self
            .completed_ticks
            .checked_add(u64::from(fixed_steps))
            .ok_or(TimeError::TickIndexOverflow)?;
        let consumed = self.config.fixed_step() * fixed_steps;
        let accumulated_lag = accumulated
            .checked_sub(consumed)
            .ok_or(TimeError::ElapsedArithmeticOverflow)?;
        let timing = FrameTiming::new(
            fixed_steps,
            self.completed_ticks,
            completed_ticks,
            available_steps_wide > u128::from(self.config.max_catch_up_steps()),
            accumulated_lag,
        );
        self.accumulated = accumulated_lag;
        self.completed_ticks = completed_ticks;
        Ok(timing)
    }

    /// Assign an explicit number of fixed ticks without consuming host time.
    ///
    /// This path is intended for controlled tests and headless execution.
    ///
    /// # Errors
    ///
    /// Returns an error without changing the clock if the tick index exceeds
    /// its supported range.
    pub fn advance_steps(&mut self, fixed_steps: u32) -> Result<FrameTiming, TimeError> {
        let completed_ticks = self
            .completed_ticks
            .checked_add(u64::from(fixed_steps))
            .ok_or(TimeError::TickIndexOverflow)?;
        let timing = FrameTiming::new(
            fixed_steps,
            self.completed_ticks,
            completed_ticks,
            false,
            self.accumulated,
        );
        self.completed_ticks = completed_ticks;
        Ok(timing)
    }
}
