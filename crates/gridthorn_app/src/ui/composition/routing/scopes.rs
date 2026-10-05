use std::{borrow::Cow, collections::BTreeSet};

use super::{UiLayout, UiNodeId, UiRouter, UiTree};
use gridthorn_input::InputEvent;

/// Batch-local immutable scopes; routing commands preserve topology and only pop layers.
pub(super) struct RoutingScopes<'layout> {
    base: Cow<'layout, UiLayout>,
    layers: usize,
    pointer: Option<(usize, Option<UiNodeId>, UiLayout)>,
}

impl<'layout> RoutingScopes<'layout> {
    pub(super) fn new(router: &UiRouter, tree: &UiTree, layout: &'layout UiLayout) -> Self {
        Self {
            base: router.input_layout(tree, layout),
            layers: router.layers.len(),
            pointer: None,
        }
    }

    pub(super) fn base(&self) -> &UiLayout {
        &self.base
    }

    pub(super) fn event_layout(
        &mut self,
        router: &mut UiRouter,
        tree: &UiTree,
        layout: &'layout UiLayout,
        event: &InputEvent,
    ) -> &UiLayout {
        if self.layers != router.layers.len() {
            *self = Self::new(router, tree, layout);
        }
        if let InputEvent::CursorMoved(position) = event {
            router.cursor_physical = Some([position.x, position.y]);
            router.refresh_cursor(tree, &self.base);
        }
        let hovered = router.layer_at_cursor(tree, layout);
        router.layer_hovered = hovered.is_some();
        if matches!(
            event,
            InputEvent::CursorMoved(_)
                | InputEvent::MouseButton { .. }
                | InputEvent::MouseWheel { .. }
                | InputEvent::PointerMotion { .. }
        ) && let Some(index) = hovered
        {
            if !self
                .pointer
                .as_ref()
                .is_some_and(|(layer, capture, _)| *layer == index && *capture == router.capture)
            {
                let ids: BTreeSet<_> = router.layers[index..]
                    .iter()
                    .filter_map(|layer| tree.node(layer.root))
                    .flat_map(UiRouter::all_ids)
                    .collect();
                let mut scoped = self.base.as_ref().clone();
                scoped
                    .placements
                    .retain(|p| ids.contains(&p.id) || router.capture == Some(p.id));
                self.pointer = Some((index, router.capture, scoped));
            }
            &self.pointer.as_ref().unwrap().2
        } else {
            &self.base
        }
    }
}
