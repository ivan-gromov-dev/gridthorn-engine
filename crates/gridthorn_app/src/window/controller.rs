use gilrs::ff::{BaseEffect, BaseEffectType, EffectBuilder, Repeat, Replay, Ticks};
use gilrs::{Axis, Button, EventType, GamepadId};
use gridthorn_input::{
    ButtonState, InputEvent,
    controller::{
        ControllerAxis, ControllerButton, ControllerError, ControllerEvent, ControllerId,
        ControllerInfo, RumbleRequest,
    },
};
use std::collections::BTreeMap;

const BUTTONS: &[(Button, ControllerButton)] = &[
    (Button::South, ControllerButton::South),
    (Button::East, ControllerButton::East),
    (Button::North, ControllerButton::North),
    (Button::West, ControllerButton::West),
    (Button::LeftTrigger, ControllerButton::LeftShoulder),
    (Button::RightTrigger, ControllerButton::RightShoulder),
    (Button::LeftTrigger2, ControllerButton::LeftTrigger),
    (Button::RightTrigger2, ControllerButton::RightTrigger),
    (Button::Select, ControllerButton::Select),
    (Button::Start, ControllerButton::Start),
    (Button::Mode, ControllerButton::Mode),
    (Button::LeftThumb, ControllerButton::LeftStick),
    (Button::RightThumb, ControllerButton::RightStick),
    (Button::DPadUp, ControllerButton::DPadUp),
    (Button::DPadDown, ControllerButton::DPadDown),
    (Button::DPadLeft, ControllerButton::DPadLeft),
    (Button::DPadRight, ControllerButton::DPadRight),
];
const AXES: &[(Axis, ControllerAxis)] = &[
    (Axis::LeftStickX, ControllerAxis::LeftStickX),
    (Axis::LeftStickY, ControllerAxis::LeftStickY),
    (Axis::RightStickX, ControllerAxis::RightStickX),
    (Axis::RightStickY, ControllerAxis::RightStickY),
    (Axis::LeftZ, ControllerAxis::LeftTrigger),
    (Axis::RightZ, ControllerAxis::RightTrigger),
];

#[derive(Default)]
pub(super) struct NativeControllers {
    initialized: bool,
    discard_before: Option<std::time::SystemTime>,
    backend: Option<gilrs::Gilrs>,
    connections: BTreeMap<usize, (GamepadId, ControllerId)>,
    next_id: u64,
    effects: BTreeMap<ControllerId, (gilrs::ff::Effect, std::time::Instant)>,
}
impl NativeControllers {
    fn connect(&mut self, native: GamepadId) -> ControllerEvent {
        self.next_id += 1;
        let id = ControllerId(self.next_id);
        self.connections.insert(native.into(), (native, id));
        let pad = self
            .backend
            .as_ref()
            .expect("initialized backend")
            .gamepad(native);
        ControllerEvent::Connected(ControllerInfo {
            id,
            name: pad.name().into(),
            model_uuid: pad.uuid(),
            vendor_id: pad.vendor_id(),
            product_id: pad.product_id(),
            buttons: BUTTONS
                .iter()
                .filter(|(button, _)| {
                    pad.button_code(*button).is_some()
                        || match button {
                            Button::DPadLeft | Button::DPadRight => {
                                pad.axis_code(Axis::DPadX).is_some()
                            }
                            Button::DPadUp | Button::DPadDown => {
                                pad.axis_code(Axis::DPadY).is_some()
                            }
                            _ => false,
                        }
                })
                .map(|(_, button)| *button)
                .collect(),
            axes: AXES
                .iter()
                .filter(|(axis, _)| {
                    pad.axis_code(*axis).is_some()
                        || match axis {
                            Axis::LeftZ => pad.button_code(Button::LeftTrigger2).is_some(),
                            Axis::RightZ => pad.button_code(Button::RightTrigger2).is_some(),
                            _ => false,
                        }
                })
                .map(|(_, axis)| *axis)
                .collect(),
            rumble_supported: pad.is_ff_supported(),
        })
    }
    pub(super) fn poll(&mut self) -> Vec<InputEvent> {
        let mut events = Vec::new();
        self.effects
            .retain(|_, (_, deadline)| *deadline > std::time::Instant::now());
        if !self.initialized {
            self.initialized = true;
            match gilrs::GilrsBuilder::new()
                .with_default_filters(false)
                .set_update_state(false)
                .build()
            {
                Ok(backend) => {
                    events.push(InputEvent::Controller(ControllerEvent::Ready));
                    let ids: Vec<_> = backend.gamepads().map(|(id, _)| id).collect();
                    self.backend = Some(backend);
                    for id in ids {
                        events.push(InputEvent::Controller(self.connect(id)));
                    }
                }
                Err(error) => events.push(InputEvent::Controller(ControllerEvent::Unavailable(
                    ControllerError::Platform(error.to_string()),
                ))),
            }
        }
        while let Some(event) = self.backend.as_mut().and_then(|backend| {
            use gilrs::Filter;
            let event = backend
                .next_event()
                .filter_ev(&gilrs::ev::filter::axis_dpad_to_button, backend);
            if let Some(event) = &event {
                backend.update(event);
            }
            event
        }) {
            if self
                .discard_before
                .is_some_and(|cutoff| event.time < cutoff)
                && !matches!(event.event, EventType::Connected | EventType::Disconnected)
            {
                continue;
            }
            if matches!(event.event, EventType::Connected) {
                if !self.connections.contains_key(&usize::from(event.id)) {
                    events.push(InputEvent::Controller(self.connect(event.id)));
                }
                continue;
            }
            let Some((_, id)) = self.connections.get(&usize::from(event.id)).copied() else {
                continue;
            };
            let mapped = match event.event {
                EventType::Disconnected => {
                    self.connections.remove(&usize::from(event.id));
                    self.effects.remove(&id);
                    Some(ControllerEvent::Disconnected(id))
                }
                EventType::ButtonPressed(button, _) | EventType::ButtonReleased(button, _) => {
                    BUTTONS
                        .iter()
                        .find(|(native, _)| *native == button)
                        .map(|(_, button)| ControllerEvent::Button {
                            id,
                            button: *button,
                            state: if matches!(event.event, EventType::ButtonPressed(..)) {
                                ButtonState::Pressed
                            } else {
                                ButtonState::Released
                            },
                        })
                }
                EventType::AxisChanged(axis, value, _) => AXES
                    .iter()
                    .find(|(native, _)| *native == axis)
                    .map(|(_, axis)| ControllerEvent::Axis {
                        id,
                        axis: *axis,
                        value,
                    }),
                EventType::ButtonChanged(button, value, _) => match button {
                    Button::LeftTrigger2 => Some(ControllerEvent::Axis {
                        id,
                        axis: ControllerAxis::LeftTrigger,
                        value,
                    }),
                    Button::RightTrigger2 => Some(ControllerEvent::Axis {
                        id,
                        axis: ControllerAxis::RightTrigger,
                        value,
                    }),
                    _ => None,
                },
                _ => None,
            };
            if let Some(event) = mapped {
                events.push(InputEvent::Controller(event));
            }
        }
        events
    }
    pub(super) fn stop(&mut self) {
        self.effects.clear();
    }
    pub(super) fn cancel_input(&mut self) {
        self.stop();
        self.discard_before = Some(std::time::SystemTime::now());
    }
    pub(super) fn feedback_focused(&mut self, request: RumbleRequest, focused: bool) -> InputEvent {
        if focused {
            self.feedback(request)
        } else {
            InputEvent::Controller(ControllerEvent::Feedback {
                request: request.request,
                id: request.id,
                result: Err(ControllerError::Unfocused),
            })
        }
    }
    pub(super) fn feedback(&mut self, request: RumbleRequest) -> InputEvent {
        let result = self.rumble(request);
        InputEvent::Controller(ControllerEvent::Feedback {
            request: request.request,
            id: request.id,
            result,
        })
    }
    fn rumble(&mut self, request: RumbleRequest) -> Result<(), ControllerError> {
        request.validate()?;
        let native = self
            .connections
            .values()
            .find(|(_, id)| *id == request.id)
            .map(|(native, _)| *native)
            .ok_or(ControllerError::Disconnected)?;
        let backend = self.backend.as_mut().ok_or(ControllerError::Disconnected)?;
        if !backend.gamepad(native).is_ff_supported() {
            return Err(ControllerError::Unsupported);
        }
        if request.duration_ms == 0 {
            self.effects.remove(&request.id);
            return Ok(());
        }

        let duration = Ticks::from_ms(request.duration_ms);
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let magnitude = |value: f32| (value * f32::from(u16::MAX)).round() as u16;
        let effect = EffectBuilder::new()
            .add_effect(BaseEffect {
                kind: BaseEffectType::Strong {
                    magnitude: magnitude(request.strong),
                },
                scheduling: Replay {
                    play_for: duration,
                    ..Replay::default()
                },
                ..BaseEffect::default()
            })
            .add_effect(BaseEffect {
                kind: BaseEffectType::Weak {
                    magnitude: magnitude(request.weak),
                },
                scheduling: Replay {
                    play_for: duration,
                    ..Replay::default()
                },
                ..BaseEffect::default()
            })
            .repeat(Repeat::For(duration))
            .gamepads(&[native])
            .finish(backend)
            .map_err(|error| ControllerError::Platform(error.to_string()))?;
        effect
            .play()
            .map_err(|error| ControllerError::Platform(error.to_string()))?;
        self.effects.insert(
            request.id,
            (
                effect,
                std::time::Instant::now()
                    + std::time::Duration::from_millis(u64::from(request.duration_ms)),
            ),
        );
        Ok(())
    }
}

#[cfg(test)]
mod test;
