use super::ControllerError;
/// Validated rescaling dead zone. Samples outside the zone map continuously to full scale.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DeadZone(f32);
impl DeadZone {
    /// Construct a finite threshold in `[0, 1)`.
    /// # Errors
    /// Rejects nonfinite or out-of-range thresholds.
    pub fn new(threshold: f32) -> Result<Self, ControllerError> {
        if threshold.is_finite() && (0.0..1.0).contains(&threshold) {
            Ok(Self(threshold))
        } else {
            Err(ControllerError::InvalidParameters)
        }
    }
    /// Filter a signed scalar. Nonfinite samples become neutral.
    #[must_use]
    pub fn axial(self, value: f32) -> f32 {
        if !value.is_finite() {
            return 0.0;
        }
        value.signum() * ((value.abs().min(1.0) - self.0) / (1.0 - self.0)).max(0.0)
    }
    /// Filter a stick radially, preserving direction and clamping its magnitude.
    #[must_use]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "normalized results are bounded to [-1, 1]"
    )]
    pub fn radial(self, value: [f32; 2]) -> [f32; 2] {
        if value.iter().any(|v| !v.is_finite()) {
            return [0.0; 2];
        }
        let value = value.map(f64::from);
        let length = value[0].hypot(value[1]);
        let threshold = f64::from(self.0);
        if length <= threshold || length == 0.0 {
            return [0.0; 2];
        }
        let magnitude = (length.min(1.0) - threshold) / (1.0 - threshold);
        let scale = magnitude / length;
        [(value[0] * scale) as f32, (value[1] * scale) as f32]
    }
}
