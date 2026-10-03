use super::UiAnimationError;
use std::time::Duration;

/// Bounded monotonic easing with exact zero/one endpoints and no overshoot.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum UiEasing {
    /// Constant speed.
    #[default]
    Linear,
    /// Quadratic acceleration.
    EaseIn,
    /// Quadratic deceleration.
    EaseOut,
    /// Smooth cubic acceleration and deceleration.
    SmoothStep,
}

impl UiEasing {
    fn sample(self, t: f64) -> f64 {
        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => t * (2.0 - t),
            Self::SmoothStep => t * t * (3.0 - 2.0 * t),
        }
    }
}

/// Provisional interpolation of finite scalar/vector channels using caller-supplied frame time.
///
/// Use one channel for a scalar and four for linear RGBA. This state is presentation-only;
/// pass unscaled frame duration during `Update`, never fixed-tick or simulation-scaled time.
#[derive(Clone, Debug)]
pub struct UiTween<const N: usize> {
    from: [f32; N],
    to: [f32; N],
    duration: Duration,
    elapsed: Duration,
    easing: UiEasing,
    paused: bool,
}

impl<const N: usize> UiTween<N> {
    /// Start a tween; zero duration immediately samples the exact destination.
    ///
    /// # Errors
    /// Rejects nonfinite endpoints without constructing state.
    pub fn new(
        from: [f32; N],
        to: [f32; N],
        duration: Duration,
        easing: UiEasing,
    ) -> Result<Self, UiAnimationError> {
        if !from.into_iter().chain(to).all(f32::is_finite) {
            return Err(UiAnimationError::InvalidValue("nonfinite tween endpoint"));
        }
        Ok(Self {
            from,
            to,
            duration,
            elapsed: Duration::ZERO,
            easing,
            paused: false,
        })
    }

    /// Current interpolated channels; construction does not consume time.
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "convex interpolation of finite f32 endpoints is bounded; f64 avoids intermediate overflow"
    )]
    pub fn value(&self) -> [f32; N] {
        if self.is_finished() {
            return self.to;
        }
        if self.elapsed.is_zero() {
            return self.from;
        }
        let t = self
            .easing
            .sample(self.elapsed.as_secs_f64() / self.duration.as_secs_f64());
        std::array::from_fn(|i| {
            ((1.0 - t) * f64::from(self.from[i]) + t * f64::from(self.to[i])) as f32
        })
    }

    /// Advance by an unscaled frame delta, clamping large deltas to the endpoint.
    pub fn advance(&mut self, delta: Duration) -> [f32; N] {
        if !self.paused {
            self.elapsed = self.elapsed.saturating_add(delta).min(self.duration);
        }
        self.value()
    }

    /// Interrupt smoothly from the current sample, restarting duration and easing.
    /// The explicit pause state is preserved.
    ///
    /// # Errors
    /// Invalid destinations leave the tween unchanged.
    pub fn retarget(
        &mut self,
        to: [f32; N],
        duration: Duration,
        easing: UiEasing,
    ) -> Result<(), UiAnimationError> {
        let mut next = Self::new(self.value(), to, duration, easing)?;
        next.paused = self.paused;
        *self = next;
        Ok(())
    }

    /// Pause only this presentation animation; unrelated simulation controls have no effect.
    pub fn pause(&mut self) {
        self.paused = true;
    }

    /// Resume this animation without accumulating paused deltas.
    pub fn resume(&mut self) {
        self.paused = false;
    }

    /// Whether this tween is explicitly paused.
    #[must_use]
    pub const fn is_paused(&self) -> bool {
        self.paused
    }

    /// Whether the destination has been reached, including zero-duration tweens.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.elapsed >= self.duration
    }
}
