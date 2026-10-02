use std::collections::BTreeSet;

use crate::{ButtonState, CursorPosition, InputEvent, KeyCode, MouseButton};

/// Immutable keyboard and mouse state for one host frame.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InputState {
    events: Vec<InputEvent>,
    physical_keys_down: BTreeSet<crate::PhysicalKey>,
    modifiers: crate::Modifiers,
    focused: bool,
    capture: crate::PointerCaptureMode,
    keys_down: BTreeSet<KeyCode>,
    keys_pressed: BTreeSet<KeyCode>,
    keys_released: BTreeSet<KeyCode>,
    mouse_buttons_down: BTreeSet<MouseButton>,
    mouse_buttons_pressed: BTreeSet<MouseButton>,
    mouse_buttons_released: BTreeSet<MouseButton>,
    cursor_position: Option<CursorPosition>,
}

impl InputState {
    /// Events in arrival order, including repeats and focus cancellation.
    #[must_use]
    pub fn events(&self) -> &[InputEvent] {
        &self.events
    }

    /// Held physical identity, including native unidentified keys.
    #[must_use]
    pub fn physical_key_down(&self, key: &crate::PhysicalKey) -> bool {
        self.physical_keys_down.contains(key)
    }

    /// Latest effective modifiers.
    #[must_use]
    pub const fn modifiers(&self) -> crate::Modifiers {
        self.modifiers
    }

    /// Latest native pointer capture mode.
    #[must_use]
    pub const fn pointer_capture(&self) -> crate::PointerCaptureMode {
        self.capture
    }

    /// Whether the platform has reported focus gained.
    #[must_use]
    pub const fn focused(&self) -> bool {
        self.focused
    }
    /// Whether a key is held during this frame.
    #[must_use]
    pub fn key_down(&self, key: KeyCode) -> bool {
        self.keys_down.contains(&key)
    }

    /// Whether a key changed from released to pressed during this frame.
    #[must_use]
    pub fn key_just_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    /// Whether a key changed from pressed to released during this frame.
    #[must_use]
    pub fn key_just_released(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&key)
    }

    /// Whether a mouse button is held during this frame.
    #[must_use]
    pub fn mouse_button_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons_down.contains(&button)
    }

    /// Whether a mouse button became pressed during this frame.
    #[must_use]
    pub fn mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    /// Whether a mouse button became released during this frame.
    #[must_use]
    pub fn mouse_button_just_released(&self, button: MouseButton) -> bool {
        self.mouse_buttons_released.contains(&button)
    }

    /// Latest cursor position, or `None` while the cursor is outside.
    #[must_use]
    pub fn cursor_position(&self) -> Option<CursorPosition> {
        self.cursor_position
    }
}

/// Collects platform events and emits frame-scoped input snapshots.
#[derive(Default)]
pub struct InputBuffer {
    state: InputState,
}

impl InputBuffer {
    /// Create an empty input buffer.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Apply one engine-owned platform event in arrival order.
    pub fn push(&mut self, event: InputEvent) {
        self.state.events.push(event.clone());
        match event {
            InputEvent::Key(key) => {
                if let crate::PhysicalKey::Code(code) = key.physical_key {
                    if key.repeat && key.state == ButtonState::Pressed {
                        self.state.keys_down.insert(code);
                    } else {
                        self.apply_key(code, key.state);
                    }
                }
                match key.state {
                    ButtonState::Pressed => {
                        self.state.physical_keys_down.insert(key.physical_key);
                    }
                    ButtonState::Released => {
                        self.state.physical_keys_down.remove(&key.physical_key);
                    }
                }
            }
            InputEvent::ModifiersChanged(modifiers) => self.state.modifiers = modifiers,
            InputEvent::FocusGained => self.state.focused = true,
            InputEvent::PointerCaptureChanged(status) => self.state.capture = status.effective,
            InputEvent::MouseWheel { .. } | InputEvent::PointerMotion { .. } => {}
            InputEvent::Keyboard { key, state } => self.apply_key(key, state),
            InputEvent::MouseButton { button, state } => self.apply_mouse_button(button, state),
            InputEvent::CursorMoved(position) => self.state.cursor_position = Some(position),
            InputEvent::CursorLeft => {
                if self.state.capture == crate::PointerCaptureMode::None {
                    self.state.cursor_position = None;
                }
            }
            InputEvent::FocusLost => self.release_all(),
        }
    }

    /// Return the current frame state and clear only edge transitions.
    #[must_use]
    pub fn snapshot(&mut self) -> InputState {
        let snapshot = self.state.clone();
        self.state.events.clear();
        self.state.keys_pressed.clear();
        self.state.keys_released.clear();
        self.state.mouse_buttons_pressed.clear();
        self.state.mouse_buttons_released.clear();
        snapshot
    }

    fn apply_key(&mut self, key: KeyCode, state: ButtonState) {
        match state {
            ButtonState::Pressed => {
                self.state
                    .physical_keys_down
                    .insert(crate::PhysicalKey::Code(key));
            }
            ButtonState::Released => {
                self.state
                    .physical_keys_down
                    .remove(&crate::PhysicalKey::Code(key));
            }
        }
        match state {
            ButtonState::Pressed => {
                if self.state.keys_down.insert(key) {
                    self.state.keys_pressed.insert(key);
                }
            }
            ButtonState::Released => {
                if self.state.keys_down.remove(&key) {
                    self.state.keys_released.insert(key);
                }
            }
        }
    }

    fn apply_mouse_button(&mut self, button: MouseButton, state: ButtonState) {
        match state {
            ButtonState::Pressed => {
                if self.state.mouse_buttons_down.insert(button) {
                    self.state.mouse_buttons_pressed.insert(button);
                }
            }
            ButtonState::Released => {
                if self.state.mouse_buttons_down.remove(&button) {
                    self.state.mouse_buttons_released.insert(button);
                }
            }
        }
    }

    fn release_all(&mut self) {
        self.state.physical_keys_down.clear();
        self.state.modifiers = crate::Modifiers::default();
        self.state.focused = false;
        self.state.capture = crate::PointerCaptureMode::None;
        self.state.cursor_position = None;
        self.state
            .keys_released
            .extend(self.state.keys_down.iter().copied());
        self.state.keys_down.clear();
        self.state
            .mouse_buttons_released
            .extend(self.state.mouse_buttons_down.iter().copied());
        self.state.mouse_buttons_down.clear();
    }
}

#[cfg(test)]
mod test;
