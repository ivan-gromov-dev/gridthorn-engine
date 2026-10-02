use super::SimulationSpeedError;

/// Positive rational host-time multiplier; tick duration stays fixed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimulationSpeed {
    numerator: u32,
    denominator: u32,
}

impl SimulationSpeed {
    /// Normal playback.
    pub const NORMAL: Self = Self {
        numerator: 1,
        denominator: 1,
    };

    /// Construct a positive ratio such as 1/2 or 4/1.
    ///
    /// # Errors
    /// Rejects zero terms. Use pause to stop simulation.
    pub fn new(numerator: u32, denominator: u32) -> Result<Self, SimulationSpeedError> {
        if numerator == 0 || denominator == 0 {
            return Err(SimulationSpeedError::ZeroRatioTerm);
        }
        let (mut a, mut b) = (numerator, denominator);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        Ok(Self {
            numerator: numerator / a,
            denominator: denominator / a,
        })
    }
    /// Reduced numerator.
    #[must_use]
    pub fn numerator(self) -> u32 {
        self.numerator
    }
    /// Reduced denominator.
    #[must_use]
    pub fn denominator(self) -> u32 {
        self.denominator
    }
}

/// Controls sampled after Input. Presentation continues during pause.
/// Explicit tick execution ignores these controls.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SimulationControl {
    paused: bool,
    speed: SimulationSpeed,
}

impl Default for SimulationControl {
    fn default() -> Self {
        Self {
            paused: false,
            speed: SimulationSpeed::NORMAL,
        }
    }
}
impl SimulationControl {
    /// Freeze timed ticks and preserve accumulated lag.
    pub fn pause(&mut self) {
        self.paused = true;
    }
    /// Resume without accumulating paused host time.
    pub fn resume(&mut self) {
        self.paused = false;
    }
    /// Whether timed simulation is paused.
    #[must_use]
    pub fn is_paused(self) -> bool {
        self.paused
    }
    /// Select speed independently of pause.
    pub fn set_speed(&mut self, speed: SimulationSpeed) {
        self.speed = speed;
    }
    /// Current multiplier.
    #[must_use]
    pub fn speed(self) -> SimulationSpeed {
        self.speed
    }
}
