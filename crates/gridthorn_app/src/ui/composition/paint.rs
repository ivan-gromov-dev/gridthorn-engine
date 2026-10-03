use super::{UiBounds, UiCompositionError, UiControl, UiNode, UiPlacement, UiTree, UiVisualState};
use gridthorn_render::{Color, TextLabel, TextSystem, UiPrimitive, UiRect};

pub(super) fn paint(
    tree: &UiTree,
    placements: &[UiPlacement],
    scale: f32,
    text: &mut Option<&mut TextSystem>,
    router: Option<&super::UiRouter>,
) -> Result<Vec<UiPrimitive>, UiCompositionError> {
    let mut output = Vec::new();
    for placement in placements {
        let node = tree
            .node(placement.id)
            .ok_or(UiCompositionError::UnknownNode(placement.id))?;
        if placement.clip.size.contains(&0.0) || placement.bounds.size.contains(&0.0) {
            continue;
        }
        let mut primitives = Vec::new();
        let theme = &tree.theme;
        let background = node.style.background.or({
            if matches!(node.control, UiControl::Panel | UiControl::Label(_)) {
                None
            } else {
                Some(match node.visual {
                    UiVisualState::Normal => theme.surface,
                    UiVisualState::Hovered => theme.hovered,
                    UiVisualState::Pressed => theme.pressed,
                    UiVisualState::Disabled => theme.disabled,
                })
            }
        });
        if let Some(color) = background {
            rect(&mut primitives, placement.bounds, color, scale)?;
        }
        let mut content = Vec::new();
        control(tree, node, placement, scale, text, &mut content)?;
        let clip = if node.style.clip || node.style.scroll {
            placement.clip.intersection(placement.content)
        } else {
            placement.clip
        };
        append_clipped(&mut primitives, content, clip, scale)?;
        if let Some(router) = router {
            router.paint_field(tree, placement, scale, text, &mut primitives)?;
        }
        append_clipped(&mut output, primitives, placement.clip, scale)?;
    }
    Ok(output)
}

pub(super) fn append_clipped(
    output: &mut Vec<UiPrimitive>,
    children: Vec<UiPrimitive>,
    bounds: UiBounds,
    scale: f32,
) -> Result<(), UiCompositionError> {
    if !children.is_empty() && bounds.size.into_iter().all(|value| value > 0.0) {
        output.push(UiPrimitive::Clipped {
            bounds: UiRect::new(
                bounds.position.map(|value| value * scale),
                bounds.size.map(|value| value * scale),
                Color::default(),
            )?,
            children,
        });
    }
    Ok(())
}

pub(super) fn rect(
    output: &mut Vec<UiPrimitive>,
    bounds: UiBounds,
    color: Color,
    scale: f32,
) -> Result<(), UiCompositionError> {
    if bounds.size.into_iter().all(|value| value > 0.0) {
        output.push(
            UiRect::new(
                bounds.position.map(|value| value * scale),
                bounds.size.map(|value| value * scale),
                color,
            )?
            .into(),
        );
    }
    Ok(())
}

pub(super) fn label(
    tree: &UiTree,
    value: &str,
    bounds: UiBounds,
    color: Color,
    scale: f32,
    text: &mut Option<&mut TextSystem>,
    output: &mut Vec<UiPrimitive>,
) -> Result<(), UiCompositionError> {
    if bounds.size[0] <= 0.0 || bounds.size[1] <= 0.0 || value.is_empty() {
        return Ok(());
    }
    if let Some(style) = &tree.theme.text {
        let service = text
            .as_deref_mut()
            .ok_or(UiCompositionError::InvalidMetrics(
                "asset font theme requires TextSystem",
            ))?;
        let mut style = style.clone();
        style.width = Some(bounds.size[0].max(1.0));
        let layout = service.layout(value, &style)?;
        output.push(
            service
                .rasterize(&layout, scale, color)?
                .at(bounds.position)?
                .into(),
        );
    } else {
        output.push(
            TextLabel::new(
                value,
                bounds.position.map(|value| value * scale),
                tree.theme.bitmap_scale * scale,
                color,
            )?
            .into(),
        );
    }
    Ok(())
}

#[expect(
    clippy::cast_precision_loss,
    reason = "bounded list indices become logical row positions"
)]
fn control(
    tree: &UiTree,
    node: &UiNode,
    placement: &UiPlacement,
    scale: f32,
    text: &mut Option<&mut TextSystem>,
    output: &mut Vec<UiPrimitive>,
) -> Result<(), UiCompositionError> {
    let color = node.style.foreground.unwrap_or(tree.theme.foreground);
    let origin = std::array::from_fn(|axis| {
        placement.content.position[axis] - placement.scroll_offset[axis]
    });
    let mut bounds = UiBounds {
        position: origin,
        size: placement.content.size,
    };
    match &node.control {
        UiControl::Panel => {}
        UiControl::Label(value) | UiControl::Button(value) => {
            label(tree, value, bounds, color, scale, text, output)?;
        }
        UiControl::TextField { value, placeholder } => label(
            tree,
            if value.is_empty() { placeholder } else { value },
            bounds,
            color,
            scale,
            text,
            output,
        )?,
        UiControl::Toggle {
            label: value,
            checked,
        } => {
            let side = tree.theme.row_height.min(bounds.size[1]);
            rect(
                output,
                UiBounds {
                    position: origin,
                    size: [side, side],
                },
                if *checked {
                    tree.theme.accent
                } else {
                    tree.theme.disabled
                },
                scale,
            )?;
            bounds.position[0] += tree.theme.row_height;
            bounds.size[0] = (bounds.size[0] - tree.theme.row_height).max(0.0);
            label(tree, value, bounds, color, scale, text, output)?;
        }
        UiControl::Slider { min, max, value } => {
            let ratio = (*value - *min) / (*max - *min);
            rect(
                output,
                UiBounds {
                    position: origin,
                    size: [bounds.size[0] * ratio, bounds.size[1]],
                },
                tree.theme.accent,
                scale,
            )?;
            let thumb = bounds.size[0].min(6.0);
            rect(
                output,
                UiBounds {
                    position: [origin[0] + (bounds.size[0] - thumb) * ratio, origin[1]],
                    size: [thumb, bounds.size[1]],
                },
                color,
                scale,
            )?;
        }
        UiControl::List { items, selected } => {
            for (index, value) in items.iter().enumerate() {
                let row = UiBounds {
                    position: [origin[0], origin[1] + index as f32 * tree.theme.row_height],
                    size: [bounds.size[0], tree.theme.row_height],
                };
                if !row
                    .intersection(placement.content)
                    .size
                    .into_iter()
                    .all(|value| value > 0.0)
                    && (node.style.clip || node.style.scroll)
                {
                    continue;
                }
                if *selected == Some(index) {
                    rect(output, row, tree.theme.accent, scale)?;
                }
                let mut row_content = Vec::new();
                label(tree, value, row, color, scale, text, &mut row_content)?;
                append_clipped(output, row_content, row, scale)?;
            }
        }
    }
    Ok(())
}
