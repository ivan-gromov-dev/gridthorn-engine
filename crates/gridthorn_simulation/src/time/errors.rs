use thiserror::Error;

/// Invalid fixed-step clock configuration.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FixedStepConfigError {
    /// A zero duration cannot advance authoritative time.
    #[error("fixed-step duration must be greater than zero")]
    ZeroDuration,
    /// A zero catch-up limit would prevent all fixed updates.
    #[error("maximum catch-up steps must be greater than zero")]
    ZeroCatchUpLimit,
}

/// Failure while advancing fixed-step time.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum TimeError {
    /// Host-frame time arithmetic exceeded [`std::time::Duration`] capacity.
    #[error("fixed-step elapsed-time arithmetic exceeded its supported range")]
    ElapsedArithmeticOverflow,
    /// The authoritative fixed tick index exceeded its supported range.
    #[error("authoritative fixed tick index overflowed")]
    TickIndexOverflow,
}
