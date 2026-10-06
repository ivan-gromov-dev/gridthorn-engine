use super::{UiCompositionError, UiLayout, UiNavigation, UiRoute, UiRouter, UiTree};
use gridthorn_input::{
    ButtonState, InputEvent,
    controller::{ControllerAxis, ControllerButton, ControllerEvent, ControllerId},
};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Default)]
pub(super) struct ControllerRouting {
    pub(super) enabled: bool,
    buttons: BTreeSet<(ControllerId, ControllerButton)>,
    axes: BTreeMap<(ControllerId, ControllerAxis), i8>,
}
impl UiRouter {
    /// Opt into South/East, D-pad, shoulders and left-stick UI navigation.
    /// Disabling clears held ownership. Axis entry is 0.6, neutral rearm is 0.3;
    /// there is no automatic repeat. Unmapped controls remain available to gameplay.
    pub fn set_controller_navigation(&mut self, enabled: bool) {
        self.controllers = ControllerRouting {
            enabled,
            ..ControllerRouting::default()
        };
    }
    pub(super) fn controller_event(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        event: &InputEvent,
        result: &mut UiRoute,
    ) -> Result<bool, UiCompositionError> {
        if matches!(event, InputEvent::FocusLost) {
            self.controllers.buttons.clear();
            self.controllers.axes.clear();
        }
        let InputEvent::Controller(event) = event else {
            return Ok(false);
        };
        if let ControllerEvent::Disconnected(id) = event {
            self.controllers.buttons.retain(|(owner, _)| owner != id);
            self.controllers.axes.retain(|(owner, _), _| owner != id);
            return Ok(false);
        }
        if !self.controllers.enabled {
            return Ok(false);
        }
        let navigation = match *event {
            ControllerEvent::Button { id, button, state } => {
                let command = match button {
                    ControllerButton::South => UiNavigation::Activate,
                    ControllerButton::East => UiNavigation::Cancel,
                    ControllerButton::DPadUp => UiNavigation::Up,
                    ControllerButton::DPadDown => UiNavigation::Down,
                    ControllerButton::DPadLeft => UiNavigation::Left,
                    ControllerButton::DPadRight => UiNavigation::Right,
                    ControllerButton::LeftShoulder => UiNavigation::Previous,
                    ControllerButton::RightShoulder => UiNavigation::Next,
                    _ => return Ok(false),
                };
                if state == ButtonState::Released {
                    return Ok(self.controllers.buttons.remove(&(id, button)));
                }
                if !self.controllers.buttons.insert((id, button)) {
                    return Ok(true);
                }
                Some(command)
            }
            ControllerEvent::Axis {
                id,
                axis: axis @ (ControllerAxis::LeftStickX | ControllerAxis::LeftStickY),
                value,
            } => {
                if !value.is_finite() {
                    return Ok(false);
                }
                let sign = if value >= 0.6 {
                    1
                } else if value <= -0.6 {
                    -1
                } else {
                    0
                };
                let previous = self.controllers.axes.entry((id, axis)).or_default();
                let command = if sign != 0 && sign != *previous {
                    *previous = sign;
                    Some(match (axis, sign) {
                        (ControllerAxis::LeftStickX, 1) => UiNavigation::Right,
                        (ControllerAxis::LeftStickX, _) => UiNavigation::Left,
                        (_, 1) => UiNavigation::Up,
                        _ => UiNavigation::Down,
                    })
                } else {
                    None
                };
                if value.abs() <= 0.3 {
                    *previous = 0;
                }
                command
            }
            _ => return Ok(false),
        };
        if let Some(command) = navigation {
            let routed = self.navigate(tree, layout, command)?;
            result.effects.extend(routed.effects);
            result.dismissed.extend(routed.dismissed);
            result.platform.extend(routed.platform);
        }
        Ok(true)
    }
}
