use std::time::Duration;

use super::{FixedStepConfig, FrameTiming, SimulationControl, SimulationSpeed, TimeError};

/// Accumulates host-frame time into deterministic fixed-step assignments.
pub struct FixedStepClock {
    config: FixedStepConfig,
    accumulated: Duration,
    completed_ticks: u64,
    speed: SimulationSpeed,
    fractional_nanos: u128,
}

impl FixedStepClock {
    /// Create a clock with validated fixed-step configuration.
    #[must_use]
    pub fn new(config: FixedStepConfig) -> Self {
        Self {
            config,
            accumulated: Duration::ZERO,
            completed_ticks: 0,
            speed: SimulationSpeed::NORMAL,
            fractional_nanos: 0,
        }
    }

    /// Reconstruct an explicit-tick clock at a snapshot boundary with no host-time backlog.
    /// The next assigned tick has index `completed_ticks`; overflow remains a typed advancement error.
    #[must_use]
    pub fn at_tick(config: FixedStepConfig, completed_ticks: u64) -> Self {
        Self {
            completed_ticks,
            ..Self::new(config)
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
        self.advance_controlled(elapsed, SimulationControl::default())
    }

    /// Scale host time with integer arithmetic and retain fractional nanoseconds.
    /// A speed change discards only the old sub-nanosecond remainder.
    /// Pause freezes backlog and ignores elapsed host time.
    ///
    /// # Errors
    /// Arithmetic failures leave the clock unchanged.
    pub fn advance_controlled(
        &mut self,
        elapsed: Duration,
        control: SimulationControl,
    ) -> Result<FrameTiming, TimeError> {
        if control.is_paused() {
            return Ok(FrameTiming::new(
                elapsed,
                0,
                self.completed_ticks,
                self.completed_ticks,
                false,
                self.accumulated,
            ));
        }
        let speed = control.speed();
        let remainder = if speed == self.speed {
            self.fractional_nanos
        } else {
            0
        };
        let scaled = elapsed.as_nanos() * u128::from(speed.numerator()) + remainder;
        let nanos = scaled / u128::from(speed.denominator());
        let seconds = u64::try_from(nanos / 1_000_000_000)
            .map_err(|_| TimeError::ElapsedArithmeticOverflow)?;
        let subsec = u32::try_from(nanos % 1_000_000_000)
            .map_err(|_| TimeError::ElapsedArithmeticOverflow)?;
        let simulation_elapsed = Duration::new(seconds, subsec);
        let accumulated = self
            .accumulated
            .checked_add(simulation_elapsed)
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
            elapsed,
            fixed_steps,
            self.completed_ticks,
            completed_ticks,
            available_steps_wide > u128::from(self.config.max_catch_up_steps()),
            accumulated_lag,
        );
        self.speed = speed;
        self.fractional_nanos = scaled % u128::from(speed.denominator());
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
            Duration::ZERO,
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
