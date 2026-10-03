use super::{
    PendingClipboard, UiCommand, UiCompositionError, UiControl, UiLayout, UiNavigation,
    UiPlatformRequest, UiRoute, UiRouter, UiSelection, UiTree,
};
use gridthorn_input::{
    ButtonState, ClipboardOperation, ClipboardRequest, InputEvent, KeyCode, LogicalKey, NamedKey,
    PhysicalKey, TextInputEvent,
};

impl UiRouter {
    pub(super) fn keyboard_event(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        event: &InputEvent,
        result: &mut UiRoute,
    ) -> Result<bool, UiCompositionError> {
        match event {
            InputEvent::Key(key) => {
                if key.synthetic {
                    return Ok(self.focus.is_some());
                }
                self.key(
                    tree,
                    layout,
                    &key.physical_key,
                    &key.logical_key,
                    key.state,
                    key.repeat,
                    result,
                )
            }
            InputEvent::Keyboard { key, state } => {
                let named = match key {
                    KeyCode::Tab => NamedKey::Tab,
                    KeyCode::Enter | KeyCode::NumpadEnter => NamedKey::Enter,
                    KeyCode::Space => NamedKey::Space,
                    KeyCode::Escape => NamedKey::Escape,
                    KeyCode::ArrowLeft => NamedKey::ArrowLeft,
                    KeyCode::ArrowRight => NamedKey::ArrowRight,
                    KeyCode::ArrowUp => NamedKey::ArrowUp,
                    KeyCode::ArrowDown => NamedKey::ArrowDown,
                    KeyCode::Home => NamedKey::Home,
                    KeyCode::End => NamedKey::End,
                    KeyCode::Backspace => NamedKey::Backspace,
                    KeyCode::Delete => NamedKey::Delete,
                    _ => {
                        return self.key(
                            tree,
                            layout,
                            &PhysicalKey::Code(*key),
                            &LogicalKey::Unidentified(gridthorn_input::NativeKey::Unidentified),
                            *state,
                            false,
                            result,
                        );
                    }
                };
                self.key(
                    tree,
                    layout,
                    &PhysicalKey::Code(*key),
                    &LogicalKey::Named(named),
                    *state,
                    false,
                    result,
                )
            }
            InputEvent::Text(text) => self.text_event(tree, text, result),
            InputEvent::TextInputChanged { active: false, .. } => {
                self.await_text_stop = false;
                self.editor.cancel_composition();
                Ok(false)
            }
            InputEvent::Clipboard(response) => {
                if self
                    .pending
                    .as_ref()
                    .is_none_or(|pending| pending.id != response.id)
                {
                    return Ok(false);
                }
                let pending = self.pending.take().expect("matched request");
                result.clipboard.push(response.clone());
                if self.focus == Some(pending.node)
                    && self.editor.value == pending.value
                    && self.editor.selection == pending.selection
                    && self.editor.preedit.is_empty()
                {
                    let insertion = if pending.cut && response.result == Ok(None) {
                        Some("")
                    } else if !pending.cut {
                        response
                            .result
                            .as_ref()
                            .ok()
                            .and_then(|value| value.as_deref())
                    } else {
                        None
                    };
                    if let Some(text) = insertion {
                        let next = self.editor.replace(text)?;
                        self.write_text(tree, next, result)?;
                    }
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    fn text_event(
        &mut self,
        tree: &mut UiTree,
        text: &TextInputEvent,
        result: &mut UiRoute,
    ) -> Result<bool, UiCompositionError> {
        if !self.text_focused(tree) {
            return Ok(false);
        }
        if self.await_text_stop {
            return Ok(true);
        }
        match text {
            TextInputEvent::Commit(value) => {
                let next = self.editor.replace(value)?;
                self.write_text(tree, next, result)?;
            }
            TextInputEvent::Composition { text, cursor } => {
                if text.len() > 65536 {
                    return Err(UiCompositionError::InvalidMetrics("preedit length"));
                }
                self.editor.preedit.clone_from(text);
                self.editor.preedit_cursor =
                    cursor.filter(|(a, b)| text.is_char_boundary(*a) && text.is_char_boundary(*b));
            }
            TextInputEvent::CompositionCancelled | TextInputEvent::ImeDisabled => {
                self.editor.cancel_composition();
            }
            TextInputEvent::ImeEnabled => {}
        }
        Ok(true)
    }

    fn text_focused(&self, tree: &UiTree) -> bool {
        self.focus
            .and_then(|id| tree.node(id))
            .is_some_and(|node| matches!(node.control, UiControl::TextField { .. }))
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "physical ownership and logical shortcut identity are distinct keyboard contracts"
    )]
    fn key(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        physical: &PhysicalKey,
        logical: &LogicalKey,
        state: ButtonState,
        repeat: bool,
        result: &mut UiRoute,
    ) -> Result<bool, UiCompositionError> {
        if state == ButtonState::Released {
            return Ok(self.owned_keys.remove(physical) || self.focus.is_some());
        }
        let tab = *logical == LogicalKey::Named(NamedKey::Tab);
        if self.focus.is_none()
            && (!tab
                || !layout
                    .placements()
                    .iter()
                    .any(|p| Self::visible(tree, layout, p.id)))
        {
            return Ok(false);
        }
        self.owned_keys.insert(physical.clone());
        if self.text_focused(tree) && !self.editor.preedit.is_empty() {
            if *logical == LogicalKey::Named(NamedKey::Escape) {
                self.editor.cancel_composition();
                self.await_text_stop = true;
                result.platform.push(UiPlatformRequest::Text(
                    gridthorn_input::TextInputRequest::Stop,
                ));
            }
            return Ok(true);
        }
        if tab {
            if !repeat {
                self.navigation(
                    tree,
                    layout,
                    if self.modifiers.shift {
                        UiNavigation::Previous
                    } else {
                        UiNavigation::Next
                    },
                    result,
                )?;
            }
            return Ok(true);
        }
        if self.text_focused(tree) {
            self.edit_key(tree, layout, logical, result)?;
            return Ok(true);
        }
        let navigation = match logical {
            LogicalKey::Named(NamedKey::ArrowLeft) => Some(UiNavigation::Left),
            LogicalKey::Named(NamedKey::ArrowRight) => Some(UiNavigation::Right),
            LogicalKey::Named(NamedKey::ArrowUp) => Some(UiNavigation::Up),
            LogicalKey::Named(NamedKey::ArrowDown) => Some(UiNavigation::Down),
            LogicalKey::Named(NamedKey::Enter | NamedKey::Space) if !repeat => {
                Some(UiNavigation::Activate)
            }
            LogicalKey::Named(NamedKey::Escape) if !repeat => Some(UiNavigation::Cancel),
            _ => None,
        };
        if let Some(navigation) = navigation {
            self.navigation(tree, layout, navigation, result)?;
        }
        Ok(true)
    }

    fn edit_key(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        logical: &LogicalKey,
        result: &mut UiRoute,
    ) -> Result<(), UiCompositionError> {
        if (self.modifiers.control || self.modifiers.super_key) && !self.modifiers.alt {
            if let LogicalKey::Character(character) = logical {
                match character.to_lowercase().as_str() {
                    "a" => {
                        self.editor.selection = UiSelection {
                            anchor: 0,
                            caret: self.editor.value.len(),
                        }
                    }
                    "c" | "x" | "v" => self.clipboard(character.to_lowercase().as_str(), result)?,
                    _ => {}
                }
            }
            return Ok(());
        }
        let LogicalKey::Named(key) = logical else {
            return Ok(());
        };
        match key {
            NamedKey::ArrowLeft => self.editor.step(false, self.modifiers.shift),
            NamedKey::ArrowRight => self.editor.step(true, self.modifiers.shift),
            NamedKey::ArrowUp | NamedKey::ArrowDown => {
                if let Some(geometry) = self.focus.and_then(|id| layout.text_geometry.get(&id))
                    && geometry.value == self.editor.value
                {
                    let mut point = geometry.point(self.editor.selection.caret);
                    point[1] += geometry.height
                        * if *key == NamedKey::ArrowDown {
                            1.0
                        } else {
                            -1.0
                        };
                    self.editor
                        .move_to(geometry.hit(point), self.modifiers.shift);
                }
            }
            NamedKey::Home => self.editor.move_to(0, self.modifiers.shift),
            NamedKey::End => self
                .editor
                .move_to(self.editor.value.len(), self.modifiers.shift),
            NamedKey::Backspace | NamedKey::Delete => {
                let next = self.editor.delete(*key == NamedKey::Delete)?;
                self.write_text(tree, next, result)?;
            }
            NamedKey::Escape => self.set_focus(tree, None, result),
            _ => {}
        }
        Ok(())
    }

    fn clipboard(
        &mut self,
        shortcut: &str,
        result: &mut UiRoute,
    ) -> Result<(), UiCompositionError> {
        let id = self.next_clipboard;
        self.next_clipboard = id.checked_add(1).ok_or(UiCompositionError::InvalidMetrics(
            "clipboard IDs exhausted",
        ))?;
        let operation = if shortcut == "v" {
            ClipboardOperation::Read
        } else {
            ClipboardOperation::Write(self.editor.value[self.editor.selection.range()].to_owned())
        };
        if shortcut != "c" {
            self.pending = Some(PendingClipboard {
                id,
                node: self.focus.expect("text focus"),
                value: self.editor.value.clone(),
                selection: self.editor.selection,
                cut: shortcut == "x",
            });
        }
        result
            .platform
            .push(UiPlatformRequest::Clipboard(ClipboardRequest {
                id,
                operation,
            }));
        Ok(())
    }

    fn write_text(
        &self,
        tree: &mut UiTree,
        value: String,
        result: &mut UiRoute,
    ) -> Result<(), UiCompositionError> {
        Self::command(
            tree,
            self.focus.expect("text focus"),
            UiCommand::SetText(value),
            result,
        )
    }
}
