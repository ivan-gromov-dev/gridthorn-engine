use std::collections::BTreeSet;

use crate::{ButtonState, CursorPosition, InputEvent, KeyCode, MouseButton};

/// Immutable keyboard and mouse state for one host frame.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InputState {
    keys_down: BTreeSet<KeyCode>,
    keys_pressed: BTreeSet<KeyCode>,
    keys_released: BTreeSet<KeyCode>,
    mouse_buttons_down: BTreeSet<MouseButton>,
    mouse_buttons_pressed: BTreeSet<MouseButton>,
    mouse_buttons_released: BTreeSet<MouseButton>,
    cursor_position: Option<CursorPosition>,
}

impl InputState {
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
        match event {
            InputEvent::Keyboard { key, state } => self.apply_key(key, state),
            InputEvent::MouseButton { button, state } => self.apply_mouse_button(button, state),
            InputEvent::CursorMoved(position) => self.state.cursor_position = Some(position),
            InputEvent::CursorLeft => self.state.cursor_position = None,
            InputEvent::FocusLost => self.release_all(),
        }
    }

    /// Return the current frame state and clear only edge transitions.
    #[must_use]
    pub fn snapshot(&mut self) -> InputState {
        let snapshot = self.state.clone();
        self.state.keys_pressed.clear();
        self.state.keys_released.clear();
        self.state.mouse_buttons_pressed.clear();
        self.state.mouse_buttons_released.clear();
        snapshot
    }

    fn apply_key(&mut self, key: KeyCode, state: ButtonState) {
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
