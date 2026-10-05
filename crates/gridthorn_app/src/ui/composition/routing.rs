use super::editing::Editor;
use super::{
    UiCommand, UiCompositionError, UiControl, UiEffect, UiLayout, UiNodeId, UiSelection, UiTree,
    UiVisualState,
};
use gridthorn_input::{
    ClipboardRequest, ClipboardResponse, InputEvent, InputState, Modifiers, PhysicalKey,
    TextInputRequest,
};
use std::collections::BTreeSet;

mod keyboard;
mod layers;
mod navigation;
mod pointer;
mod presentation;
mod scopes;
pub use layers::UiLayer;

/// Device-independent hooks. Controller adapters map buttons/axes to these commands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiNavigation {
    /// Next enabled, visible control in declaration order, wrapping at the end.
    Next,
    /// Previous control, wrapping at the start.
    Previous,
    /// Move spatially or adjust a slider/list.
    Left,
    /// Move spatially or adjust a slider/list.
    Right,
    /// Move spatially or adjust a slider/list.
    Up,
    /// Move spatially or adjust a slider/list.
    Down,
    /// Activate the focused button/toggle.
    Activate,
    /// Clear focus, composition and capture.
    Cancel,
}

/// Explicit operations to forward to runtime text/clipboard resources after routing.
#[derive(Clone, Debug, PartialEq)]
pub enum UiPlatformRequest {
    /// Open/update or close the focused field's native text session.
    Text(TextInputRequest),
    /// Ordered clipboard operation with a router-owned correlation ID.
    Clipboard(ClipboardRequest),
}

/// One ordered routing result. Games explicitly map remaining input to world commands.
#[derive(Clone, Debug, Default)]
pub struct UiRoute {
    /// Layer roots dismissed in event order.
    pub dismissed: Vec<UiNodeId>,
    /// Events that were not consumed, preserving arrival order.
    pub world_events: Vec<InputEvent>,
    /// Indices consumed from the supplied event stream.
    pub consumed: Vec<usize>,
    /// Ordered value changes and activations with their owner.
    pub effects: Vec<(UiNodeId, UiEffect)>,
    /// Suppress continuous world keyboard bindings while UI owns focus/held keys.
    pub keyboard_blocked: bool,
    /// Suppress continuous world pointer bindings while hovered/captured/held by UI.
    pub pointer_blocked: bool,
    /// Operations for the platform adapter; routing itself has no native side effects.
    pub platform: Vec<UiPlatformRequest>,
    /// Correlated clipboard feedback, including native failures.
    pub clipboard: Vec<ClipboardResponse>,
}

#[derive(Clone, Debug)]
struct PendingClipboard {
    id: u64,
    node: UiNodeId,
    value: String,
    selection: UiSelection,
    cut: bool,
}

/// Presentation-only event owner, focus, capture and grapheme-aware field editor.
///
/// Route during `Input` before gameplay mapping. Layout is a detached geometry
/// snapshot: recompute after scroll/value/viewport changes. Logical pointer geometry
/// is converted using the layout's DPI. UI capture is independent of native cursor
/// confinement/locking. Reserve unique clipboard IDs for this router.
#[derive(Clone, Debug)]
pub struct UiRouter {
    performance: Option<std::sync::Arc<super::layout_performance::LayoutPerformance>>,
    layers: Vec<layers::OpenLayer>,
    layer_roots: BTreeSet<UiNodeId>,
    layer_hovered: bool,
    focus: Option<UiNodeId>,
    capture: Option<UiNodeId>,
    hovered: Option<UiNodeId>,
    cursor: Option<[f32; 2]>,
    cursor_physical: Option<[f64; 2]>,
    modifiers: Modifiers,
    owned_keys: BTreeSet<PhysicalKey>,
    pointer_owned: bool,
    other_buttons: BTreeSet<gridthorn_input::MouseButton>,
    text_owner: Option<UiNodeId>,
    await_text_stop: bool,
    editor: Editor,
    next_clipboard: u64,
    pending: Option<PendingClipboard>,
}

impl UiRouter {
    /// Construct with the first caller-reserved clipboard correlation ID.
    #[must_use]
    pub fn new(first_clipboard_id: u64) -> Self {
        Self {
            performance: super::layout_performance::LayoutPerformance::new(),
            layers: Vec::new(),
            layer_roots: BTreeSet::new(),
            layer_hovered: false,
            focus: None,
            capture: None,
            hovered: None,
            cursor: None,
            cursor_physical: None,
            modifiers: Modifiers::default(),
            owned_keys: BTreeSet::new(),
            pointer_owned: false,
            other_buttons: BTreeSet::new(),
            text_owner: None,
            await_text_stop: false,
            editor: Editor::default(),
            next_clipboard: first_clipboard_id,
            pending: None,
        }
    }

    /// Current enabled focus owner, reconciled on routing/navigation.
    #[must_use]
    pub const fn focused(&self) -> Option<UiNodeId> {
        self.focus
    }

    /// Current primary-pointer capture owner.
    #[must_use]
    pub const fn captured(&self) -> Option<UiNodeId> {
        self.capture
    }

    /// Focused text selection, if focus owns a text field.
    #[must_use]
    pub fn selection(&self, tree: &UiTree) -> Option<UiSelection> {
        self.focus
            .filter(|id| {
                matches!(
                    tree.node(*id).map(|node| &node.control),
                    Some(UiControl::TextField { .. })
                )
            })
            .map(|_| self.editor.selection)
    }

    /// Pending IME preedit; committed tree values remain unchanged until commit.
    #[must_use]
    pub fn preedit(&self) -> &str {
        &self.editor.preedit
    }

    /// Native IME preedit caret/selection endpoints in UTF-8 bytes.
    #[must_use]
    pub const fn composition_cursor(&self) -> Option<(usize, usize)> {
        self.editor.preedit_cursor
    }

    /// Set a grapheme-valid selection in the focused enabled text field.
    ///
    /// # Errors
    /// Rejects missing/non-text focus or invalid endpoints without mutation.
    pub fn select(
        &mut self,
        tree: &UiTree,
        selection: UiSelection,
    ) -> Result<(), UiCompositionError> {
        let id = self
            .focus
            .ok_or(UiCompositionError::InvalidMetrics("no text focus"))?;
        let node = tree.node(id).ok_or(UiCompositionError::UnknownNode(id))?;
        let UiControl::TextField { value, .. } = &node.control else {
            return Err(UiCompositionError::WrongControl(id));
        };
        selection.validate(value)?;
        if node.visual == UiVisualState::Disabled {
            return Err(UiCompositionError::WrongControl(id));
        }
        self.editor.sync(value);
        self.editor.selection = selection;
        Ok(())
    }

    /// Topmost enabled interactive border box within its ancestor clip.
    /// Panels/labels pass through. Disabled controls block pointer input but cannot be hit.
    #[must_use]
    pub fn hit_test(tree: &UiTree, layout: &UiLayout, point: [f32; 2]) -> Option<UiNodeId> {
        Self::pointer_target(tree, layout, point).filter(|id| Self::enabled(tree, *id))
    }

    /// Route the snapshot's ordered events atomically, retaining raw input unchanged.
    ///
    /// # Errors
    /// Invalid text/geometry/commands leave both router and tree unchanged.
    pub fn route(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        input: &InputState,
    ) -> Result<UiRoute, UiCompositionError> {
        self.route_events(tree, layout, input.events())
    }

    /// Route injected engine events atomically without a native window or input buffer.
    ///
    /// # Errors
    /// Invalid text/commands preserve router/tree and discard platform requests.
    pub fn route_events(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        events: &[InputEvent],
    ) -> Result<UiRoute, UiCompositionError> {
        let mut router = self.clone();
        let mut next = tree.clone();
        let mut result = UiRoute::default();
        while router
            .layers
            .last()
            .is_some_and(|layer| next.node(layer.root).is_none())
        {
            router.pop_layer(&next, layout, &mut result);
        }
        let mut prepared = scopes::RoutingScopes::new(&router, &next, layout);
        router.reconcile(&next, prepared.base(), &mut result);
        for (index, event) in events.iter().enumerate() {
            let scoped = prepared.event_layout(&mut router, &next, layout, event);
            if router.layer_event(&next, layout, event, &mut result)
                || router.event(&mut next, scoped, event, &mut result)?
                || router.block_modal_event(event)
            {
                result.consumed.push(index);
            } else {
                result.world_events.push(event.clone());
            }
        }
        router.refresh_visuals(&mut next)?;
        router.layer_hovered = router.layer_at_cursor(&next, layout).is_some();
        router.request_text(&next, layout, &mut result);
        router.finish(&mut result);
        *self = router;
        *tree = next;
        Ok(result)
    }

    /// Apply one controller/navigation hook atomically through the same focus contract.
    ///
    /// # Errors
    /// Invalid control commands preserve router/tree.
    pub fn navigate(
        &mut self,
        tree: &mut UiTree,
        layout: &UiLayout,
        navigation: UiNavigation,
    ) -> Result<UiRoute, UiCompositionError> {
        let mut router = self.clone();
        let mut next = tree.clone();
        let mut result = UiRoute::default();
        router.reconcile(&next, layout, &mut result);
        let scoped = router.input_layout(&next, layout);
        if navigation == UiNavigation::Cancel
            && !router.layers.is_empty()
            && !router.editor.preedit.is_empty()
        {
            router.editor.cancel_composition();
            router.await_text_stop = true;
            result
                .platform
                .push(UiPlatformRequest::Text(TextInputRequest::Stop));
        } else if navigation == UiNavigation::Cancel && !router.layers.is_empty() {
            if router
                .layers
                .last()
                .is_some_and(|layer| layer.options.dismiss_escape)
            {
                router.pop_layer(&next, layout, &mut result);
            }
        } else {
            router.navigation(&mut next, &scoped, navigation, &mut result)?;
        }
        router.refresh_visuals(&mut next)?;
        router.request_text(&next, layout, &mut result);
        router.finish(&mut result);
        *self = router;
        *tree = next;
        Ok(result)
    }

    fn enabled(tree: &UiTree, id: UiNodeId) -> bool {
        tree.node(id).is_some_and(|node| {
            node.visual != UiVisualState::Disabled
                && !matches!(node.control, UiControl::Panel | UiControl::Label(_))
        })
    }

    fn reconcile(&mut self, tree: &UiTree, layout: &UiLayout, result: &mut UiRoute) {
        self.refresh_cursor(tree, layout);
        if self.text_owner.is_some_and(|id| {
            !matches!(
                tree.node(id).map(|node| &node.control),
                Some(UiControl::TextField { .. })
            )
        }) {
            self.set_focus(tree, None, result);
        }
        if self
            .focus
            .is_some_and(|id| !Self::visible(tree, layout, id))
        {
            self.set_focus(tree, None, result);
        }
        if self
            .capture
            .is_some_and(|id| !Self::visible(tree, layout, id))
        {
            self.capture = None;
        }
        if let Some(UiControl::TextField { value, .. }) = self
            .focus
            .and_then(|id| tree.node(id))
            .map(|node| &node.control)
        {
            self.editor.sync(value);
        }
    }

    fn set_focus(&mut self, tree: &UiTree, focus: Option<UiNodeId>, result: &mut UiRoute) {
        if self.focus == focus {
            return;
        }
        self.focus = focus;
        self.pending = None;
        self.await_text_stop |= self.text_owner.take().is_some();
        self.editor = Editor::default();
        if let Some(UiControl::TextField { value, .. }) =
            focus.and_then(|id| tree.node(id)).map(|node| &node.control)
        {
            self.editor.sync(value);
        }
        result
            .platform
            .push(UiPlatformRequest::Text(TextInputRequest::Stop));
    }

    fn command(
        tree: &mut UiTree,
        id: UiNodeId,
        command: UiCommand,
        result: &mut UiRoute,
    ) -> Result<(), UiCompositionError> {
        let effect = tree.command(id, command)?;
        if effect != UiEffect::None {
            result.effects.push((id, effect));
        }
        Ok(())
    }

    fn finish(&self, result: &mut UiRoute) {
        let modal = self.layers.iter().any(|layer| layer.options.modal);
        result.keyboard_blocked |= modal || self.focus.is_some() || !self.owned_keys.is_empty();
        result.pointer_blocked |= modal
            || self.layer_hovered
            || self.hovered.is_some()
            || self.capture.is_some()
            || self.pointer_owned
            || !self.other_buttons.is_empty();
    }

    fn refresh_visuals(&self, tree: &mut UiTree) -> Result<(), UiCompositionError> {
        let ids: Vec<_> = Self::all_ids(tree.root());
        for id in ids {
            if !Self::enabled(tree, id) {
                continue;
            }
            let visual = if self.capture == Some(id) {
                UiVisualState::Pressed
            } else if self.hovered == Some(id) || self.focus == Some(id) {
                UiVisualState::Hovered
            } else {
                UiVisualState::Normal
            };
            tree.command(id, UiCommand::Visual(visual))?;
        }
        Ok(())
    }

    fn all_ids(node: &super::UiNode) -> Vec<UiNodeId> {
        std::iter::once(node.id)
            .chain(node.children.iter().flat_map(Self::all_ids))
            .collect()
    }
}
