//! Explicit frame-duration interpolation; no simulation clock or native lifecycle ownership.
mod errors;
mod transition;
mod tween;

pub use errors::UiAnimationError;
pub use transition::{UiProperty, UiTransition};
pub use tween::{UiEasing, UiTween};

#[cfg(test)]
mod test;
