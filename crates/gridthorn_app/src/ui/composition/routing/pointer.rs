use super::{
    UiCommand, UiCompositionError, UiControl, UiLayout, UiNavigation, UiNodeId, UiRoute, UiRouter,
    UiTree,
};
use gridthorn_input::{ButtonState, InputEvent, MouseButton, ScrollPhase, WheelDelta};

impl UiRouter {
    pub(super) fn pointer_target(
        tree: &UiTree,
        layout: &UiLayout,
        point: [f32; 2],
    ) -> Option<UiNodeId> {
        layout
            .placements()
            .iter()
            .rev()
            .find(|p| {
                p.bounds.contains(point)
                    && p.clip.contains(point)
                    && tree.node(p.id).is_some_and(|node| {
                        !matches!(node.control, UiControl::Panel | UiControl::Label(_))
                    })
            })
            .map(|p| p.id)
    }

    pub(super) fn event(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        event: &InputEvent,
        result: &mut UiRoute,
    ) -> Result<bool, UiCompositionError> {
        match event {
            InputEvent::CursorMoved(position) => {
                self.cursor_physical = Some([position.x, position.y]);
                self.refresh_cursor(tree, layout);
                if let Some(id) = self.capture
                    && let Some(point) = self.cursor
                {
                    self.pointer_value(tree, layout, id, point, true, result)?;
                }
                Ok(self.capture.is_some() || self.hovered.is_some())
            }
            InputEvent::CursorLeft => {
                self.hovered = None;
                self.cursor = None;
                self.cursor_physical = None;
                Ok(self.capture.is_some())
            }
            InputEvent::MouseButton {
                button: MouseButton::Left,
                state,
            } => {
                let target = self
                    .cursor
                    .and_then(|point| Self::pointer_target(tree, layout, point));
                if *state == ButtonState::Pressed {
                    self.pointer_owned = target.is_some();
                    self.capture = target.filter(|id| Self::enabled(tree, *id));
                    self.set_focus(tree, self.capture, result);
                    if let Some(id) = self.capture
                        && let Some(point) = self.cursor
                    {
                        self.pointer_value(tree, layout, id, point, false, result)?;
                    }
                    Ok(target.is_some())
                } else {
                    let consumed = self.pointer_owned;
                    if let Some(id) = self.capture.take()
                        && target == Some(id)
                        && Self::enabled(tree, id)
                        && matches!(
                            tree.node(id).map(|node| &node.control),
                            Some(UiControl::Button(_) | UiControl::Toggle { .. })
                        )
                    {
                        Self::command(tree, id, UiCommand::Activate, result)?;
                    }
                    self.pointer_owned = false;
                    Ok(consumed)
                }
            }
            InputEvent::MouseButton { button, state } => {
                if *state == ButtonState::Released {
                    Ok(self.other_buttons.remove(button))
                } else {
                    let consumed = self.capture.is_some() || self.hovered.is_some();
                    if consumed {
                        self.other_buttons.insert(*button);
                    }
                    Ok(consumed)
                }
            }
            InputEvent::PointerMotion { .. } => {
                Ok(self.capture.is_some() || self.hovered.is_some())
            }
            InputEvent::MouseWheel { delta, phase } => {
                self.wheel(tree, layout, *delta, *phase, result)
            }
            InputEvent::FocusLost => {
                self.navigation(tree, layout, UiNavigation::Cancel, result)?;
                self.hovered = None;
                self.cursor = None;
                self.cursor_physical = None;
                self.owned_keys.clear();
                self.pointer_owned = false;
                self.other_buttons.clear();
                self.await_text_stop = false;
                self.modifiers = gridthorn_input::Modifiers::default();
                Ok(false)
            }
            InputEvent::ModifiersChanged(modifiers) => {
                self.modifiers = *modifiers;
                Ok(self.focus.is_some())
            }
            _ => self.keyboard_event(tree, layout, event, result),
        }
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "physical coordinates become logical presentation pixels"
    )]
    pub(super) fn refresh_cursor(&mut self, tree: &UiTree, layout: &UiLayout) {
        self.cursor = self
            .cursor_physical
            .filter(|point| point.iter().all(|value| value.is_finite()))
            .map(|point| point.map(|value| (value / f64::from(layout.scale)) as f32));
        self.hovered = self
            .cursor
            .and_then(|point| Self::pointer_target(tree, layout, point));
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "physical wheel deltas become logical pixels"
    )]
    fn wheel(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        delta: WheelDelta,
        phase: ScrollPhase,
        result: &mut UiRoute,
    ) -> Result<bool, UiCompositionError> {
        if phase == ScrollPhase::Cancelled {
            return Ok(self.capture.is_some() || self.hovered.is_some());
        }
        let scroll = self.cursor.and_then(|point| {
            layout.placements().iter().rev().find(|p| {
                p.bounds.contains(point)
                    && p.clip.contains(point)
                    && tree.node(p.id).is_some_and(|node| {
                        node.style.scroll
                            && node.visual != super::super::UiVisualState::Disabled
                            && self
                                .capture
                                .or(self.hovered)
                                .is_none_or(|target| Self::all_ids(node).contains(&target))
                    })
            })
        });
        if let Some(p) = scroll {
            let delta = match delta {
                WheelDelta::Lines { x, y } => {
                    [x * tree.theme.row_height, y * tree.theme.row_height]
                }
                WheelDelta::Pixels { x, y } => [x as f32 / layout.scale, y as f32 / layout.scale],
            };
            let offset = std::array::from_fn(|axis| {
                let maximum = (p.content_extent[axis] - p.content.size[axis]).max(0.0);
                let current = tree.node(p.id).map_or(p.scroll_offset[axis], |node| {
                    node.scroll_offset[axis].min(maximum)
                });
                (current - delta[axis]).clamp(
                    0.0,
                    (p.content_extent[axis] - p.content.size[axis]).max(0.0),
                )
            });
            Self::command(tree, p.id, UiCommand::ScrollTo(offset), result)?;
            Ok(true)
        } else {
            Ok(self.capture.is_some() || self.hovered.is_some())
        }
    }

    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "clamped logical list row coordinates become bounded indices"
    )]
    fn pointer_value(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        id: UiNodeId,
        point: [f32; 2],
        dragging: bool,
        result: &mut UiRoute,
    ) -> Result<(), UiCompositionError> {
        let Some(p) = layout.placement(id) else {
            return Ok(());
        };
        match tree.node(id).map(|node| node.control.clone()) {
            Some(UiControl::Slider { min, max, .. }) => {
                let ratio = if p.content.size[0] > 0.0 {
                    ((point[0] - p.content.position[0]) / p.content.size[0]).clamp(0.0, 1.0)
                } else {
                    0.0
                };
                Self::command(
                    tree,
                    id,
                    UiCommand::SetValue(min + (max - min) * ratio),
                    result,
                )?;
            }
            Some(UiControl::List { items, .. }) if !dragging && p.content.contains(point) => {
                let row = ((point[1] - p.content.position[1] + p.scroll_offset[1])
                    / tree.theme.row_height)
                    .floor()
                    .max(0.0) as usize;
                if row < items.len() {
                    Self::command(tree, id, UiCommand::Select(Some(row)), result)?;
                }
            }
            Some(UiControl::TextField { value, .. }) => {
                let byte = layout
                    .text_geometry
                    .get(&id)
                    .filter(|geometry| geometry.value == value)
                    .map_or(value.len(), |geometry| geometry.hit(point));
                self.editor.move_to(byte, dragging || self.modifiers.shift);
                self.editor.cancel_composition();
            }
            _ => {}
        }
        Ok(())
    }
}
