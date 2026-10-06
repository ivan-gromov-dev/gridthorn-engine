use super::{ControllerAxis, ControllerButton, ControllerEvent, ControllerInfo};
use std::collections::{BTreeMap, BTreeSet};
/// Connected controller state for one immutable host frame.
#[derive(Clone, Debug, PartialEq)]
pub struct ControllerState {
    info: ControllerInfo,
    down: BTreeSet<ControllerButton>,
    pressed: BTreeSet<ControllerButton>,
    released: BTreeSet<ControllerButton>,
    axes: BTreeMap<ControllerAxis, f32>,
}
impl ControllerState {
    pub(crate) fn new(info: ControllerInfo) -> Self {
        Self {
            info,
            down: BTreeSet::new(),
            pressed: BTreeSet::new(),
            released: BTreeSet::new(),
            axes: BTreeMap::new(),
        }
    }
    /// Identity and capabilities for this connection.
    #[must_use]
    pub fn info(&self) -> &ControllerInfo {
        &self.info
    }
    /// Whether a button is held.
    #[must_use]
    pub fn button_down(&self, button: ControllerButton) -> bool {
        self.down.contains(&button)
    }
    /// Whether a button became pressed this frame.
    #[must_use]
    pub fn button_just_pressed(&self, button: ControllerButton) -> bool {
        self.pressed.contains(&button)
    }
    /// Whether a button became released this frame.
    #[must_use]
    pub fn button_just_released(&self, button: ControllerButton) -> bool {
        self.released.contains(&button)
    }
    /// Latest axis value; absent axes are neutral.
    #[must_use]
    pub fn axis(&self, axis: ControllerAxis) -> f32 {
        self.axes.get(&axis).copied().unwrap_or(0.0)
    }
    pub(crate) fn clear_edges(&mut self) {
        self.pressed.clear();
        self.released.clear();
    }
    pub(crate) fn cancel(&mut self) {
        self.released.extend(std::mem::take(&mut self.down));
        self.axes.clear();
    }
    pub(crate) fn apply(&mut self, event: &ControllerEvent) {
        match *event {
            ControllerEvent::Button {
                button,
                state: crate::ButtonState::Pressed,
                ..
            } => {
                if self.down.insert(button) {
                    self.pressed.insert(button);
                }
            }
            ControllerEvent::Button {
                button,
                state: crate::ButtonState::Released,
                ..
            } => {
                if self.down.remove(&button) {
                    self.released.insert(button);
                }
            }
            ControllerEvent::Axis { axis, value, .. } if value.is_finite() => {
                let minimum = if matches!(
                    axis,
                    ControllerAxis::LeftTrigger | ControllerAxis::RightTrigger
                ) {
                    0.0
                } else {
                    -1.0
                };
                self.axes.insert(axis, value.clamp(minimum, 1.0));
            }
            _ => {}
        }
    }
}
