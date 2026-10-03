use gridthorn_input::{ImeCursorArea, InputEvent, Modifiers, TextInputError, TextInputEvent};
use winit::event::{ElementState, Ime, WindowEvent};

/// Native text-session translation, isolated from physical keyboard mapping.
#[derive(Default)]
pub(super) struct NativeTextInput {
    active: bool,
    ime_enabled: bool,
    composing: bool,
    modifiers: Modifiers,
}

impl NativeTextInput {
    pub(super) fn request(
        &mut self,
        area: Option<ImeCursorArea>,
        focused: bool,
    ) -> Vec<InputEvent> {
        let error = area.and_then(|area| {
            area.validate()
                .err()
                .or_else(|| (!focused).then_some(TextInputError::Unfocused))
        });
        if let Some(error) = error {
            return vec![InputEvent::TextInputChanged {
                active: self.active,
                error: Some(error),
            }];
        }
        let mut events = Vec::new();
        if area.is_none() {
            self.cancel(&mut events);
        }
        self.active = area.is_some();
        events.push(InputEvent::TextInputChanged {
            active: self.active,
            error: None,
        });
        events
    }

    pub(super) fn active(&self) -> bool {
        self.active
    }

    pub(super) fn translate(&mut self, event: &WindowEvent) -> Vec<InputEvent> {
        let mut events = Vec::new();
        match event {
            WindowEvent::ModifiersChanged(modifiers) => {
                let state = modifiers.state();
                self.modifiers = Modifiers {
                    shift: state.shift_key(),
                    control: state.control_key(),
                    alt: state.alt_key(),
                    super_key: state.super_key(),
                };
            }
            WindowEvent::Focused(false) => {
                self.cancel(&mut events);
                self.active = false;
                self.modifiers = Modifiers::default();
                events.push(InputEvent::TextInputChanged {
                    active: false,
                    error: None,
                });
            }
            WindowEvent::Ime(ime) if self.active => match ime {
                Ime::Enabled => {
                    self.ime_enabled = true;
                    events.push(InputEvent::Text(TextInputEvent::ImeEnabled));
                }
                Ime::Disabled => {
                    self.cancel(&mut events);
                    events.push(InputEvent::Text(TextInputEvent::ImeDisabled));
                }
                Ime::Preedit(text, cursor) => {
                    if text.is_empty() {
                        if self.composing {
                            events.push(InputEvent::Text(TextInputEvent::CompositionCancelled));
                        }
                        self.composing = false;
                    } else {
                        self.composing = true;
                        let cursor = cursor.filter(|(start, end)| {
                            text.is_char_boundary(*start) && text.is_char_boundary(*end)
                        });
                        events.push(InputEvent::Text(TextInputEvent::Composition {
                            text: text.clone(),
                            cursor,
                        }));
                    }
                }
                Ime::Commit(text) => {
                    self.composing = false;
                    if !text.is_empty() {
                        events.push(InputEvent::Text(TextInputEvent::Commit(text.clone())));
                    }
                }
            },
            WindowEvent::KeyboardInput {
                event,
                is_synthetic,
                ..
            } => {
                if let Some(text) =
                    self.keyboard_commit(event.text.as_deref(), event.state, *is_synthetic)
                {
                    events.push(InputEvent::Text(TextInputEvent::Commit(text)));
                }
            }
            _ => {}
        }
        events
    }

    /// Reject shortcut chords and control characters while preserving `AltGr` and Unicode sequences.
    fn keyboard_commit(
        &self,
        text: Option<&str>,
        state: ElementState,
        synthetic: bool,
    ) -> Option<String> {
        if !self.active
            || self.ime_enabled
            || self.composing
            || synthetic
            || state != ElementState::Pressed
        {
            return None;
        }
        Self::keyboard_text(text, self.modifiers)
    }

    /// Extract printable text without normalizing Unicode or deriving it from logical keys.
    fn keyboard_text(text: Option<&str>, modifiers: Modifiers) -> Option<String> {
        if modifiers.super_key || (modifiers.control && !modifiers.alt) {
            return None;
        }
        let text: String = text?
            .chars()
            .filter(|character| !character.is_control())
            .collect();
        (!text.is_empty()).then_some(text)
    }

    fn cancel(&mut self, events: &mut Vec<InputEvent>) {
        if self.composing {
            events.push(InputEvent::Text(TextInputEvent::CompositionCancelled));
        }
        self.composing = false;
        self.ime_enabled = false;
    }
}

#[cfg(test)]
mod test;
