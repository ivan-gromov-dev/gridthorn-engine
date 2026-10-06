use super::{UiCompositionError, UiControl, UiLayout, UiNodeId, UiRoute, UiRouter, UiTree};
use gridthorn_input::{ButtonState, InputEvent, KeyCode, LogicalKey, NamedKey, PhysicalKey};
use std::{borrow::Cow, collections::BTreeSet};

/// Policy for a context menu, popup or dialog rooted in a direct child panel.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct UiLayer {
    /// Block input and focus traversal beneath this layer, including empty space.
    pub modal: bool,
    /// Dismiss on a nonrepeat Escape press after IME cancellation.
    pub dismiss_escape: bool,
    /// Dismiss on any pointer button press outside the clipped root box.
    pub dismiss_outside: bool,
}

#[derive(Clone, Debug)]
pub(super) struct OpenLayer {
    pub options: UiLayer,
    pub(super) root: UiNodeId,
    restore: Option<UiNodeId>,
}

impl UiRouter {
    /// Register a closed direct-child panel before its first presentation.
    ///
    /// # Errors
    /// Rejects missing roots or controls other than panels.
    pub fn register_layer(
        &mut self,
        tree: &UiTree,
        root: UiNodeId,
    ) -> Result<(), UiCompositionError> {
        let node = tree
            .root()
            .children
            .iter()
            .find(|node| node.id == root)
            .ok_or(UiCompositionError::UnknownNode(root))?;
        if node.control != UiControl::Panel {
            return Err(UiCompositionError::WrongControl(root));
        }
        self.layer_roots.insert(root);
        Ok(())
    }

    /// Open a direct child panel above existing layers and focus its first control.
    /// Registered roots remain hidden after closing; use router layout for painting.
    /// Supply a fresh tree layout so closed roots have geometry for initial focus.
    ///
    /// # Errors
    /// Rejects absent, non-panel or already open roots without changing state.
    pub fn open_layer(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        root: UiNodeId,
        options: UiLayer,
    ) -> Result<UiRoute, UiCompositionError> {
        let node = tree
            .root()
            .children
            .iter()
            .find(|node| node.id == root)
            .ok_or(UiCompositionError::UnknownNode(root))?;
        if node.control != UiControl::Panel || self.layers.iter().any(|layer| layer.root == root) {
            return Err(UiCompositionError::WrongControl(root));
        }
        let ids = Self::all_ids(node);
        let mut result = UiRoute::default();
        self.layer_roots.insert(root);
        self.layers.push(OpenLayer {
            options,
            root,
            restore: self.focus,
        });
        self.capture = None;
        self.hovered = None;
        self.set_focus(
            tree,
            ids.into_iter().find(|id| Self::visible(tree, layout, *id)),
            &mut result,
        );
        self.request_text(tree, layout, &mut result);
        self.layer_hovered = self.layer_at_cursor(tree, layout).is_some();
        self.finish(&mut result);
        Ok(result)
    }

    /// Close the top layer and restore enabled visible focus in the surviving scope.
    /// Empty stacks are a no-op. Platform requests must be forwarded as with routing.
    pub fn close_layer(&mut self, tree: &UiTree, layout: &UiLayout) -> UiRoute {
        let mut result = UiRoute::default();
        self.pop_layer(tree, layout, &mut result);
        self.request_text(tree, layout, &mut result);
        self.finish(&mut result);
        result
    }

    /// Open roots in bottom-to-top painter order.
    #[must_use]
    pub fn open_layers(&self) -> Vec<UiNodeId> {
        self.layers.iter().map(|layer| layer.root).collect()
    }

    /// Pick an enabled control in logical pixels, respecting modal scopes and
    /// layer backgrounds. Unlike the single-tree `hit_test`, this uses the stack.
    #[must_use]
    pub fn hit_test_layers(
        &self,
        tree: &UiTree,
        layout: &UiLayout,
        point: [f32; 2],
    ) -> Option<UiNodeId> {
        let mut router = self.clone();
        router.capture = None;
        let event = InputEvent::CursorMoved(gridthorn_input::CursorPosition {
            x: f64::from(point[0]) * f64::from(layout.scale),
            y: f64::from(point[1]) * f64::from(layout.scale),
        });
        let mut prepared = super::scopes::RoutingScopes::new(&router, tree, layout);
        let scoped = prepared.event_layout(&mut router, tree, layout, &event);
        Self::hit_test(tree, scoped, point)
    }

    pub(super) fn pop_layer(&mut self, tree: &UiTree, layout: &UiLayout, result: &mut UiRoute) {
        if let Some(layer) = self.layers.pop() {
            self.capture = None;
            self.hovered = None;
            self.layer_hovered = false;
            let scoped = self.input_layout(tree, layout);
            let focus = layer.restore.filter(|id| Self::visible(tree, &scoped, *id));
            self.set_focus(tree, focus, result);
            result.dismissed.push(layer.root);
            self.layer_hovered = self.layer_at_cursor(tree, layout).is_some();
        }
    }

    pub(super) fn order_layers(&self, tree: &UiTree, layout: &mut UiLayout) {
        let original = layout.placements.clone();
        let managed: BTreeSet<_> = self
            .layer_roots
            .iter()
            .filter_map(|id| tree.node(*id))
            .flat_map(Self::all_ids)
            .collect();
        layout.placements.retain(|p| !managed.contains(&p.id));
        for layer in &self.layers {
            if let Some(node) = tree.node(layer.root) {
                let ids: BTreeSet<_> = Self::all_ids(node).into_iter().collect();
                layout
                    .placements
                    .extend(original.iter().filter(|p| ids.contains(&p.id)).copied());
            }
        }
    }

    pub(super) fn input_layout<'layout>(
        &self,
        tree: &UiTree,
        layout: &'layout UiLayout,
    ) -> Cow<'layout, UiLayout> {
        if self.layer_roots.is_empty() {
            return Cow::Borrowed(layout);
        }
        let mut scoped = UiLayout {
            scale: layout.scale,
            text_geometry: layout.text_geometry.clone(),
            placements: layout.placements.clone(),
            primitives: Vec::new(),
        };
        self.order_layers(tree, &mut scoped);
        if let Some(index) = self.layers.iter().rposition(|layer| layer.options.modal) {
            let ids: BTreeSet<_> = self.layers[index..]
                .iter()
                .filter_map(|layer| tree.node(layer.root))
                .flat_map(Self::all_ids)
                .collect();
            scoped.placements.retain(|p| ids.contains(&p.id));
        }
        Cow::Owned(scoped)
    }

    pub(super) fn layer_at_cursor(&self, tree: &UiTree, layout: &UiLayout) -> Option<usize> {
        self.cursor.and_then(|point| {
            self.layers.iter().rposition(|layer| {
                tree.node(layer.root).is_some()
                    && layout
                        .placement(layer.root)
                        .is_some_and(|p| p.bounds.contains(point) && p.clip.contains(point))
            })
        })
    }

    pub(super) fn layer_event(
        &mut self,
        tree: &UiTree,
        layout: &UiLayout,
        event: &InputEvent,
        result: &mut UiRoute,
    ) -> bool {
        let Some(layer) = self.layers.last() else {
            return false;
        };
        let escape = match event {
            InputEvent::Keyboard {
                key: KeyCode::Escape,
                state: ButtonState::Pressed,
            } => Some((PhysicalKey::Code(KeyCode::Escape), true)),
            InputEvent::Key(key)
                if key.state == ButtonState::Pressed
                    && key.logical_key == LogicalKey::Named(NamedKey::Escape) =>
            {
                Some((key.physical_key.clone(), !key.repeat && !key.synthetic))
            }
            _ => None,
        };
        if let Some((key, dismiss)) = escape
            && self.editor.preedit.is_empty()
        {
            self.owned_keys.insert(key);
            if dismiss && layer.options.dismiss_escape {
                self.pop_layer(tree, layout, result);
            }
            return true;
        }
        if let InputEvent::MouseButton {
            button,
            state: ButtonState::Pressed,
        } = event
        {
            let inside = self.cursor.is_some_and(|point| {
                layout
                    .placement(layer.root)
                    .is_some_and(|p| p.bounds.contains(point) && p.clip.contains(point))
            });
            if !inside && layer.options.dismiss_outside {
                if *button == gridthorn_input::MouseButton::Left {
                    self.pointer_owned = true;
                } else {
                    self.other_buttons.insert(*button);
                }
                self.pop_layer(tree, layout, result);
                result.pointer_blocked = true;
                return true;
            }
        }
        false
    }

    pub(super) fn block_modal_event(&mut self, event: &InputEvent) -> bool {
        let pointer = matches!(
            event,
            InputEvent::MouseButton { .. }
                | InputEvent::MouseWheel { .. }
                | InputEvent::CursorMoved(_)
                | InputEvent::PointerMotion { .. }
        );
        let blocked = (self.layers.iter().any(|layer| layer.options.modal)
            || (self.layer_hovered && pointer))
            && matches!(
                event,
                InputEvent::Key(_)
                    | InputEvent::Keyboard { .. }
                    | InputEvent::Text(_)
                    | InputEvent::MouseButton { .. }
                    | InputEvent::MouseWheel { .. }
                    | InputEvent::CursorMoved(_)
                    | InputEvent::CursorLeft
                    | InputEvent::PointerMotion { .. }
                    | InputEvent::ModifiersChanged(_)
                    | InputEvent::Controller(
                        gridthorn_input::controller::ControllerEvent::Button { .. }
                            | gridthorn_input::controller::ControllerEvent::Axis { .. }
                    )
            );
        if blocked {
            match event {
                InputEvent::Keyboard {
                    key,
                    state: ButtonState::Pressed,
                } => {
                    self.owned_keys.insert(PhysicalKey::Code(*key));
                }
                InputEvent::Key(key) if key.state == ButtonState::Pressed => {
                    self.owned_keys.insert(key.physical_key.clone());
                }
                InputEvent::MouseButton {
                    button,
                    state: ButtonState::Pressed,
                } => {
                    if *button == gridthorn_input::MouseButton::Left {
                        self.pointer_owned = true;
                    } else {
                        self.other_buttons.insert(*button);
                    }
                }
                _ => {}
            }
        }
        blocked
    }
}
