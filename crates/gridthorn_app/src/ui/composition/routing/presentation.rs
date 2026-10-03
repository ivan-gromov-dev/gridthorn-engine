use super::super::{UiBounds, UiPlacement};
use super::{
    UiCompositionError, UiControl, UiLayout, UiPlatformRequest, UiRoute, UiRouter, UiTree,
};
use gridthorn_input::{ImeCursorArea, TextInputRequest};
use gridthorn_render::{Color, TextSystem, UiPrimitive};

impl UiRouter {
    /// Prepare a fresh layout including focus, caret, selection and IME preedit paint.
    ///
    /// # Errors
    /// Returns normal layout/font errors. Native session requests are returned by routing.
    pub fn layout(
        &self,
        tree: &UiTree,
        viewport: [f32; 2],
        scale: f32,
        mut text: Option<&mut TextSystem>,
    ) -> Result<UiLayout, UiCompositionError> {
        let mut layout = tree.layout(viewport, scale, text.as_deref_mut())?;
        self.order_layers(tree, &mut layout);
        layout.primitives =
            super::super::paint::paint(tree, &layout.placements, scale, &mut text, Some(self))?;
        Ok(layout)
    }

    pub(super) fn request_text(&mut self, tree: &UiTree, layout: &UiLayout, result: &mut UiRoute) {
        if self.await_text_stop {
            return;
        }
        let field = self
            .focus
            .filter(|id| {
                matches!(
                    tree.node(*id).map(|node| &node.control),
                    Some(UiControl::TextField { .. })
                )
            })
            .and_then(|id| layout.text_geometry.get(&id).map(|geometry| (id, geometry)));
        let Some((id, geometry)) = field else {
            if self.text_owner.take().is_some() {
                result
                    .platform
                    .push(UiPlatformRequest::Text(TextInputRequest::Stop));
                self.editor.cancel_composition();
            }
            return;
        };
        self.text_owner = Some(id);
        if geometry.value != self.editor.value {
            return;
        }
        let caret = geometry.caret(self.editor.selection.caret);
        let scale = f64::from(layout.scale);
        result
            .platform
            .push(UiPlatformRequest::Text(TextInputRequest::Start(
                ImeCursorArea {
                    x: f64::from(caret.position[0]) * scale,
                    y: f64::from(caret.position[1]) * scale,
                    width: scale,
                    height: f64::from(caret.size[1]) * scale,
                },
            )));
    }

    pub(in crate::ui::composition) fn paint_field(
        &self,
        tree: &UiTree,
        placement: &UiPlacement,
        scale: f32,
        text: &mut Option<&mut TextSystem>,
        output: &mut Vec<UiPrimitive>,
    ) -> Result<(), UiCompositionError> {
        if self.focus != Some(placement.id) {
            return Ok(());
        }
        let Some(node) = tree.node(placement.id) else {
            return Ok(());
        };
        let UiControl::TextField { value, .. } = &node.control else {
            return Ok(());
        };
        let geometry = super::super::text_geometry::prepare_field(tree, value, placement, text)?;
        let selection = if value == &self.editor.value {
            self.editor.selection
        } else {
            super::UiSelection {
                anchor: value.len(),
                caret: value.len(),
            }
        };
        let mut paint = Vec::new();
        let range = selection.range();
        for (segment, bounds) in &geometry.segments {
            if segment.start < range.start || segment.end > range.end {
                continue;
            }
            super::super::paint::rect(
                &mut paint,
                *bounds,
                Color::rgba(0.2, 0.65, 0.9, 0.3),
                scale,
            )?;
        }
        let caret = geometry.caret(selection.caret);
        super::super::paint::rect(&mut paint, caret, tree.theme.foreground, scale)?;
        self.paint_preedit(tree, placement, caret, scale, text, &mut paint)?;
        super::super::paint::append_clipped(
            output,
            paint,
            placement.clip.intersection(placement.content),
            scale,
        )
    }
    fn paint_preedit(
        &self,
        tree: &UiTree,
        placement: &UiPlacement,
        caret: UiBounds,
        scale: f32,
        text: &mut Option<&mut TextSystem>,
        paint: &mut Vec<UiPrimitive>,
    ) -> Result<(), UiCompositionError> {
        if !self.editor.preedit.is_empty() {
            let bounds = UiBounds {
                position: caret.position,
                size: [
                    (placement.content.position[0] + placement.content.size[0] - caret.position[0])
                        .max(1.0),
                    caret.size[1],
                ],
            };
            super::super::paint::rect(paint, bounds, tree.theme.surface, scale)?;
            super::super::paint::label(
                tree,
                &self.editor.preedit,
                bounds,
                tree.theme.foreground,
                scale,
                text,
                paint,
            )?;
            if let Some((anchor, caret)) = self.editor.preedit_cursor {
                let mut preedit_placement = *placement;
                preedit_placement.content = bounds;
                preedit_placement.scroll_offset = [0.0; 2];
                let preedit = super::super::text_geometry::prepare_field(
                    tree,
                    &self.editor.preedit,
                    &preedit_placement,
                    text,
                )?;
                for (range, bounds) in &preedit.segments {
                    if range.start < anchor.max(caret) && range.end > anchor.min(caret) {
                        super::super::paint::rect(
                            paint,
                            *bounds,
                            Color::rgba(0.2, 0.65, 0.9, 0.3),
                            scale,
                        )?;
                    }
                }
                let caret = super::super::editing::boundaries(&self.editor.preedit)
                    .into_iter()
                    .find(|byte| *byte >= caret)
                    .unwrap_or(self.editor.preedit.len());
                super::super::paint::rect(
                    paint,
                    preedit.caret(caret),
                    tree.theme.foreground,
                    scale,
                )?;
            }
            super::super::paint::rect(
                paint,
                UiBounds {
                    position: [bounds.position[0], bounds.position[1] + caret.size[1] - 1.0],
                    size: [bounds.size[0], 1.0],
                },
                tree.theme.accent,
                scale,
            )?;
        }
        Ok(())
    }
}
