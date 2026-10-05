use super::{UiCompositionError, UiControl, UiFlow, UiLength, UiNode, UiNodeId, UiTheme, UiTree};
use gridthorn_render::TextSystem;

/// Logical-pixel box. Zero extent represents empty content and produces no paint.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiBounds {
    /// Top-left logical origin.
    pub position: [f32; 2],
    /// Width and height.
    pub size: [f32; 2],
}

impl UiBounds {
    /// Half-open point containment for caller-owned picking.
    #[must_use]
    pub fn contains(self, point: [f32; 2]) -> bool {
        (0..2).all(|axis| {
            point[axis] >= self.position[axis]
                && point[axis] < self.position[axis] + self.size[axis]
        })
    }

    pub(super) fn intersection(self, other: Self) -> Self {
        let position = std::array::from_fn(|axis| self.position[axis].max(other.position[axis]));
        let size = std::array::from_fn(|axis| {
            (self.position[axis] + self.size[axis]).min(other.position[axis] + other.size[axis])
                - position[axis]
        });
        Self {
            position,
            size: size.map(|value| value.max(0.0)),
        }
    }
}

/// One immutable node placement, in depth-first painter order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UiPlacement {
    /// Source node identity.
    pub id: UiNodeId,
    /// Border box before clipping.
    pub bounds: UiBounds,
    /// Inner box after padding.
    pub content: UiBounds,
    /// Effective ancestor clip; scrolling/clipping also constrain control content.
    pub clip: UiBounds,
    /// Content extent before applying scroll offsets.
    pub content_extent: [f32; 2],
    /// Effective scroll offset after clamping.
    pub scroll_offset: [f32; 2],
}

/// Detached layout diagnostics and prepared render primitives.
#[derive(Clone, Debug)]
pub struct UiLayout {
    pub(super) scale: f32,
    pub(super) text_geometry:
        std::sync::Arc<std::collections::BTreeMap<UiNodeId, super::text_geometry::TextGeometry>>,
    pub(super) placements: Vec<UiPlacement>,
    pub(super) primitives: Vec<gridthorn_render::UiPrimitive>,
}

impl UiLayout {
    /// Ordered node placements.
    #[must_use]
    pub fn placements(&self) -> &[UiPlacement] {
        &self.placements
    }

    /// Resolve one stable identity.
    #[must_use]
    pub fn placement(&self, id: UiNodeId) -> Option<&UiPlacement> {
        self.placements.iter().find(|placement| placement.id == id)
    }

    /// Prepared physical-pixel primitives, ready for `RenderFrame::with_ui`.
    #[must_use]
    pub fn primitives(&self) -> &[gridthorn_render::UiPrimitive] {
        &self.primitives
    }

    /// Transfer prepared primitives without cloning raster snapshots.
    #[must_use]
    pub fn into_primitives(self) -> Vec<gridthorn_render::UiPrimitive> {
        self.primitives
    }
}

impl UiTree {
    /// Measure, arrange and paint against a logical viewport at the current DPI.
    ///
    /// Root sizing resolves against the viewport. Fraction sizes resolve against
    /// available parent content, including auto parents (no cyclic percentage solve).
    /// Flow overflow is retained for clipping/scrolling; fill shares remaining space.
    /// Supply a font service when the theme selects asset-font text. Layout remains
    /// immutable and usable after subsequent tree edits; re-layout explicitly.
    ///
    /// # Errors
    /// Rejects invalid viewport/DPI, excessive geometry or font/primitive failures.
    pub fn layout(
        &self,
        viewport: [f32; 2],
        scale: f32,
        text: Option<&mut TextSystem>,
    ) -> Result<UiLayout, UiCompositionError> {
        let mut text = super::prepared_text::PreparedText::new(text);
        let mut layout = self.arrange_layout(viewport, scale, &mut text)?;
        layout.primitives =
            super::paint::paint(self, &layout.placements, scale, &mut text, None, None, None)?;
        layout.text_geometry =
            super::text_geometry::prepare(self, &layout.placements, &mut text)?.into();
        Ok(layout)
    }

    pub(super) fn arrange_layout(
        &self,
        viewport: [f32; 2],
        scale: f32,
        text: &mut super::prepared_text::PreparedText<'_>,
    ) -> Result<UiLayout, UiCompositionError> {
        if !viewport.into_iter().all(super::tree::metric)
            || !scale.is_finite()
            || scale <= 0.0
            || scale > 8.0
        {
            return Err(UiCompositionError::InvalidMetrics("viewport/DPI"));
        }
        let mut placements = Vec::new();
        let mut measurements = super::text_measurement::TextMeasurements::new(&self.theme, text);
        arrange(
            self.root(),
            UiBounds {
                position: [0.0; 2],
                size: viewport,
            },
            UiBounds {
                position: [0.0; 2],
                size: viewport,
            },
            &mut measurements,
            &mut placements,
            None,
        )?;
        Ok(UiLayout {
            scale,
            text_geometry: std::sync::Arc::default(),
            placements,
            primitives: Vec::new(),
        })
    }
}

pub(super) fn caption(control: &UiControl) -> Option<&str> {
    match control {
        UiControl::Label(text)
        | UiControl::Button(text)
        | UiControl::Toggle { label: text, .. } => Some(text),
        UiControl::TextField { value, placeholder } => {
            Some(if value.is_empty() { placeholder } else { value })
        }
        _ => None,
    }
}

#[expect(
    clippy::cast_precision_loss,
    reason = "bounded UI text and row counts become presentation dimensions"
)]
pub(super) fn text_size(
    value: &str,
    width: f32,
    theme: &UiTheme,
    text: &mut super::prepared_text::PreparedText<'_>,
) -> Result<[f32; 2], UiCompositionError> {
    if let Some(style) = &theme.text {
        let mut style = style.clone();
        style.width = Some(width.max(1.0));
        let measurement = text.layout(value, &style)?.measurement();
        Ok([measurement.width, measurement.height])
    } else {
        let lines: Vec<_> = value.split('\n').collect();
        Ok([
            lines
                .iter()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(0) as f32
                * 6.0
                * theme.bitmap_scale,
            lines.len() as f32 * 8.0 * theme.bitmap_scale,
        ])
    }
}

fn resolve(length: UiLength, natural: f32, available: f32, min: f32, max: f32) -> f32 {
    let size = match length {
        UiLength::Auto => natural,
        UiLength::Pixels(value) => value,
        UiLength::Fraction(value) => available * value,
        UiLength::Fill => available,
    };
    size.clamp(min, max)
}

fn insets(node: &UiNode) -> [f32; 2] {
    [
        node.style.padding[0] + node.style.padding[2],
        node.style.padding[1] + node.style.padding[3],
    ]
}

#[expect(
    clippy::cast_precision_loss,
    reason = "bounded list counts become logical row extents"
)]
fn measure(
    node: &UiNode,
    available: [f32; 2],
    measurements: &mut super::text_measurement::TextMeasurements<'_, '_>,
) -> Result<[f32; 2], UiCompositionError> {
    let theme = measurements.theme;
    let padding = insets(node);
    let width = match node.style.size[0] {
        UiLength::Pixels(value) => value,
        UiLength::Fraction(value) => available[0] * value,
        _ => available[0],
    }
    .clamp(node.style.min_size[0], node.style.max_size[0]);
    let inner = [
        (width - padding[0]).max(0.0),
        (available[1] - padding[1]).max(0.0),
    ];
    let mut natural = [0.0; 2];
    if let Some(value) = caption(&node.control) {
        natural = measurements.size(value, inner[0])?;
        if matches!(node.control, UiControl::Toggle { .. }) {
            natural[0] += theme.row_height;
        }
    }
    match &node.control {
        UiControl::Slider { .. } => natural = [120.0, theme.row_height],
        UiControl::List { items, .. } => {
            for value in items {
                natural[0] = natural[0].max(measurements.size(value, inner[0])?[0]);
            }
            natural[1] = items.len() as f32 * theme.row_height;
        }
        _ => {}
    }
    for (index, child) in node.children.iter().enumerate() {
        let size = measure(child, inner, measurements)?;
        match node.style.flow {
            UiFlow::Overlay => {
                for axis in 0..2 {
                    natural[axis] = natural[axis].max(size[axis]);
                }
            }
            UiFlow::Row => {
                natural[0] += size[0] + if index == 0 { 0.0 } else { node.style.gap };
                natural[1] = natural[1].max(size[1]);
            }
            UiFlow::Column => {
                natural[1] += size[1] + if index == 0 { 0.0 } else { node.style.gap };
                natural[0] = natural[0].max(size[0]);
            }
        }
    }
    let mut size = std::array::from_fn(|axis| {
        resolve(
            node.style.size[axis],
            natural[axis] + padding[axis],
            available[axis],
            node.style.min_size[axis],
            node.style.max_size[axis],
        )
    });
    if node.style.size[1] == UiLength::Auto
        && let Some(value) = caption(&node.control)
    {
        let indicator = if matches!(node.control, UiControl::Toggle { .. }) {
            theme.row_height
        } else {
            0.0
        };
        let actual_width = (size[0] - padding[0] - indicator).max(0.0);
        size[1] = (measurements.size(value, actual_width)?[1] + padding[1])
            .clamp(node.style.min_size[1], node.style.max_size[1]);
    }
    Ok(size)
}

#[expect(
    clippy::too_many_lines,
    reason = "one arrangement pass resolves flow allocation, scroll extent and child placement"
)]
#[expect(
    clippy::cast_precision_loss,
    reason = "bounded child counts become logical gaps and fill shares"
)]
fn arrange(
    node: &UiNode,
    parent: UiBounds,
    ancestor_clip: UiBounds,
    measurements: &mut super::text_measurement::TextMeasurements<'_, '_>,
    placements: &mut Vec<UiPlacement>,
    forced: Option<UiBounds>,
) -> Result<(), UiCompositionError> {
    let bounds = if let Some(bounds) = forced {
        bounds
    } else {
        let size = measure(node, parent.size, measurements)?;
        UiBounds {
            size,
            position: std::array::from_fn(|axis| {
                parent.position[axis]
                    + (parent.size[axis] - size[axis]) * node.style.anchor[axis].factor()
                    + node.style.offset[axis]
            }),
        }
    };
    if !bounds
        .position
        .into_iter()
        .all(|value| value.is_finite() && value.abs() <= 1_000_000.0)
        || !bounds.size.into_iter().all(super::tree::metric)
    {
        return Err(UiCompositionError::InvalidMetrics(
            "computed layout exceeds geometry budget",
        ));
    }
    let theme = measurements.theme;
    let padding = insets(node);
    let content = UiBounds {
        position: [
            bounds.position[0] + node.style.padding[0],
            bounds.position[1] + node.style.padding[1],
        ],
        size: std::array::from_fn(|axis| (bounds.size[axis] - padding[axis]).max(0.0)),
    };
    let child_clip = if node.style.clip || node.style.scroll {
        ancestor_clip.intersection(content)
    } else {
        ancestor_clip
    };
    let mut sizes = node
        .children
        .iter()
        .map(|child| measure(child, content.size, measurements))
        .collect::<Result<Vec<_>, _>>()?;
    let main = match node.style.flow {
        UiFlow::Overlay => None,
        UiFlow::Row => Some(0),
        UiFlow::Column => Some(1),
    };
    if let Some(axis) = main {
        let count = node
            .children
            .iter()
            .filter(|child| child.style.size[axis] == UiLength::Fill)
            .count();
        if count > 0 {
            let occupied = sizes
                .iter()
                .zip(&node.children)
                .filter(|(_, child)| child.style.size[axis] != UiLength::Fill)
                .map(|(size, _)| size[axis])
                .sum::<f32>();
            let gaps = node.style.gap * node.children.len().saturating_sub(1) as f32;
            let share = ((content.size[axis] - occupied - gaps) / count as f32).max(0.0);
            for (size, child) in sizes.iter_mut().zip(&node.children) {
                if child.style.size[axis] == UiLength::Fill {
                    size[axis] =
                        share.clamp(child.style.min_size[axis], child.style.max_size[axis]);
                }
                if axis == 0 && child.style.size[1] == UiLength::Auto {
                    size[1] = measure(child, [size[0], content.size[1]], measurements)?[1];
                }
            }
        }
    }
    let mut extent = content.size;
    let mut cursor = 0.0;
    let mut child_bounds = Vec::new();
    for (child, size) in node.children.iter().zip(sizes) {
        let mut position = std::array::from_fn(|axis| {
            (content.size[axis] - size[axis]) * child.style.anchor[axis].factor()
                + child.style.offset[axis]
        });
        if let Some(axis) = main {
            position[axis] = cursor + child.style.offset[axis];
            cursor += size[axis] + node.style.gap;
        }
        for axis in 0..2 {
            extent[axis] = extent[axis].max(position[axis] + size[axis]);
        }
        child_bounds.push(UiBounds { position, size });
    }
    if let UiControl::List { items, .. } = &node.control {
        extent[1] = extent[1].max(items.len() as f32 * theme.row_height);
    }
    if let Some(value) = caption(&node.control) {
        let natural = measurements.size(value, content.size[0])?;
        for axis in 0..2 {
            extent[axis] = extent[axis].max(natural[axis]);
        }
    }
    if !extent.into_iter().all(super::tree::metric) {
        return Err(UiCompositionError::InvalidMetrics("content extent"));
    }
    let scroll_offset = if node.style.scroll {
        std::array::from_fn(|axis| {
            node.scroll_offset[axis].min((extent[axis] - content.size[axis]).max(0.0))
        })
    } else {
        [0.0; 2]
    };
    placements.push(UiPlacement {
        id: node.id,
        bounds,
        content,
        clip: ancestor_clip,
        content_extent: extent,
        scroll_offset,
    });
    for (child, mut bounds) in node.children.iter().zip(child_bounds) {
        for (axis, offset) in scroll_offset.iter().enumerate() {
            bounds.position[axis] += content.position[axis] - offset;
        }
        arrange(
            child,
            content,
            child_clip,
            measurements,
            placements,
            Some(bounds),
        )?;
    }
    Ok(())
}
