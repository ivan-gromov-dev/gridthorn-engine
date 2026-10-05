use super::{UiCommand, UiCompositionError, UiControl, UiEffect, UiStyle, UiTheme, UiVisualState};
use std::collections::BTreeSet;
use std::sync::Arc;

use super::node_index::NodeIndex;

/// Caller-chosen identity, unique within one tree and stable across layouts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UiNodeId(pub u64);

/// Owned composition node. Construct freely, then validate through `UiTree::new`.
#[derive(Clone, Debug)]
pub struct UiNode {
    /// Stable identity.
    pub id: UiNodeId,
    /// Reusable content.
    pub control: UiControl,
    /// Placement and colors.
    pub style: UiStyle,
    /// Ordered descendants; only panels may own children.
    pub children: Vec<UiNode>,
    /// Explicit interaction visuals.
    pub visual: UiVisualState,
    /// Requested scroll offset, clamped by each layout.
    pub scroll_offset: [f32; 2],
}

impl UiNode {
    /// Create a node with intrinsic sizing and normal visuals.
    #[must_use]
    pub fn new(id: UiNodeId, control: UiControl) -> Self {
        Self {
            id,
            control,
            style: UiStyle::default(),
            children: Vec::new(),
            visual: UiVisualState::Normal,
            scroll_offset: [0.0; 2],
        }
    }
}

/// Validated presentation tree. No world state, native handles or event-loop policy.
#[derive(Clone)]
pub struct UiTree {
    root: UiNode,
    pub(super) theme: UiTheme,
    index: Arc<NodeIndex>,
}

#[allow(
    clippy::missing_fields_in_debug,
    reason = "Preserve the existing public Debug output without internal lookup metadata"
)]
impl std::fmt::Debug for UiTree {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("UiTree")
            .field("root", &self.root)
            .field("theme", &self.theme)
            .finish()
    }
}

impl UiTree {
    /// Validate IDs, bounded geometry, depth and control data before construction.
    ///
    /// # Errors
    /// Returns contextual errors for duplicate IDs, invalid controls/style or oversized trees.
    pub fn new(root: UiNode, theme: UiTheme) -> Result<Self, UiCompositionError> {
        validate_theme(&theme)?;
        validate_node(&root, 0, &mut BTreeSet::new())?;
        let index = Arc::new(NodeIndex::new(&root));
        Ok(Self { root, theme, index })
    }

    /// Read the immutable composition root.
    #[must_use]
    pub const fn root(&self) -> &UiNode {
        &self.root
    }

    /// Read an identified control and its presentation state.
    #[must_use]
    pub fn node(&self, id: UiNodeId) -> Option<&UiNode> {
        let mut node = &self.root;
        for &position in self.index.path(id)? {
            node = node.children.get(position)?;
        }
        Some(node)
    }

    /// Replace composition atomically, preserving the theme.
    ///
    /// # Errors
    /// Invalid replacement leaves the original tree intact.
    pub fn replace(&mut self, root: UiNode) -> Result<(), UiCompositionError> {
        validate_node(&root, 0, &mut BTreeSet::new())?;
        let index = Arc::new(NodeIndex::new(&root));
        self.root = root;
        self.index = index;
        Ok(())
    }

    /// Replace the validated theme.
    ///
    /// # Errors
    /// Rejects invalid metrics without changing the theme.
    pub fn set_theme(&mut self, theme: UiTheme) -> Result<(), UiCompositionError> {
        validate_theme(&theme)?;
        self.theme = theme;
        Ok(())
    }

    /// Replace one node's style atomically; recompute layout before painting or routing.
    ///
    /// # Errors
    /// Rejects unknown nodes and invalid geometry without changing the tree.
    pub fn set_style(&mut self, id: UiNodeId, style: UiStyle) -> Result<(), UiCompositionError> {
        validate_style(&style)?;
        let node = self
            .node_mut(id)
            .ok_or(UiCompositionError::UnknownNode(id))?;
        node.style = style;
        Ok(())
    }

    /// Apply an explicit command atomically. Recompute layout after value changes.
    ///
    /// Disabled controls ignore value commands; scroll and visual commands remain available.
    /// This method does not consume native input or enqueue simulation commands.
    ///
    /// # Errors
    /// Rejects unknown nodes, wrong control kinds, invalid values and out-of-range selection.
    pub fn command(
        &mut self,
        id: UiNodeId,
        command: UiCommand,
    ) -> Result<UiEffect, UiCompositionError> {
        let node = self
            .node_mut(id)
            .ok_or(UiCompositionError::UnknownNode(id))?;
        match command {
            UiCommand::Visual(state) => {
                node.visual = state;
                return Ok(UiEffect::None);
            }
            UiCommand::ScrollTo(offset) => {
                if !node.style.scroll {
                    return Err(UiCompositionError::WrongControl(id));
                }
                if !offset.into_iter().all(metric) {
                    return Err(UiCompositionError::InvalidMetrics("scroll offset"));
                }
                node.scroll_offset = offset;
                return Ok(UiEffect::None);
            }
            _ => {}
        }
        if node.visual == UiVisualState::Disabled {
            return Ok(UiEffect::None);
        }
        let old = node.control.clone();
        match (&mut node.control, command) {
            (UiControl::Button(_), UiCommand::Activate) => return Ok(UiEffect::Activated),
            (UiControl::Toggle { checked, .. }, UiCommand::Activate) => *checked = !*checked,
            (UiControl::Toggle { checked, .. }, UiCommand::SetChecked(value)) => *checked = value,
            (UiControl::Slider { min, max, value }, UiCommand::SetValue(next)) => {
                if !next.is_finite() {
                    return Err(UiCompositionError::InvalidMetrics("slider value"));
                }
                *value = next.clamp(*min, *max);
            }
            (UiControl::List { items, selected }, UiCommand::Select(next)) => {
                if next.is_some_and(|index| index >= items.len()) {
                    return Err(UiCompositionError::InvalidMetrics("list selection"));
                }
                *selected = next;
            }
            (UiControl::TextField { value, .. }, UiCommand::SetText(next)) => {
                validate_text(&next)?;
                *value = next;
            }
            (UiControl::TextField { value, .. }, UiCommand::AppendText(next)) => {
                if value.len().saturating_add(next.len()) > 65536 {
                    return Err(UiCompositionError::InvalidMetrics("text length"));
                }
                value.push_str(&next);
            }
            (UiControl::TextField { value, .. }, UiCommand::PopText) => {
                value.pop();
            }
            _ => return Err(UiCompositionError::WrongControl(id)),
        }
        Ok(if old == node.control {
            UiEffect::None
        } else {
            UiEffect::Changed
        })
    }
    fn node_mut(&mut self, id: UiNodeId) -> Option<&mut UiNode> {
        let mut node = &mut self.root;
        for &position in self.index.path(id)? {
            node = node.children.get_mut(position)?;
        }
        Some(node)
    }
}

pub(super) fn metric(value: f32) -> bool {
    value.is_finite() && (0.0..=65536.0).contains(&value)
}

fn validate_theme(theme: &UiTheme) -> Result<(), UiCompositionError> {
    if !metric(theme.bitmap_scale)
        || theme.bitmap_scale == 0.0
        || !metric(theme.row_height)
        || theme.row_height == 0.0
    {
        return Err(UiCompositionError::InvalidMetrics("theme metrics"));
    }
    Ok(())
}

fn validate_text(text: &str) -> Result<(), UiCompositionError> {
    if text.len() > 65536 {
        Err(UiCompositionError::InvalidMetrics("text length"))
    } else {
        Ok(())
    }
}

fn validate_node(
    node: &UiNode,
    depth: usize,
    ids: &mut BTreeSet<UiNodeId>,
) -> Result<(), UiCompositionError> {
    if depth >= 64 || ids.len() >= 4096 {
        return Err(UiCompositionError::TooLarge);
    }
    if !ids.insert(node.id) {
        return Err(UiCompositionError::DuplicateNode(node.id));
    }
    validate_style(&node.style)?;
    if !node.scroll_offset.into_iter().all(metric) {
        return Err(UiCompositionError::InvalidMetrics("scroll offset"));
    }
    if !node.children.is_empty() && !matches!(node.control, UiControl::Panel) {
        return Err(UiCompositionError::WrongControl(node.id));
    }
    match &node.control {
        UiControl::Label(text)
        | UiControl::Button(text)
        | UiControl::Toggle { label: text, .. } => validate_text(text)?,
        UiControl::TextField { value, placeholder } => {
            validate_text(value)?;
            validate_text(placeholder)?;
        }
        UiControl::Slider { min, max, value } => {
            if !min.is_finite()
                || !max.is_finite()
                || !value.is_finite()
                || *min >= *max
                || !(*max - *min).is_finite()
                || *value < *min
                || *value > *max
            {
                return Err(UiCompositionError::InvalidMetrics("slider range/value"));
            }
        }
        UiControl::List { items, selected } => {
            if items.len() > 4096 || selected.is_some_and(|index| index >= items.len()) {
                return Err(UiCompositionError::InvalidMetrics("list length/selection"));
            }
            for text in items {
                validate_text(text)?;
            }
        }
        UiControl::Panel => {}
    }
    for child in &node.children {
        validate_node(child, depth + 1, ids)?;
    }
    Ok(())
}

fn validate_style(style: &UiStyle) -> Result<(), UiCompositionError> {
    for axis in 0..2 {
        let valid_length = match style.size[axis] {
            super::UiLength::Pixels(value) => metric(value),
            super::UiLength::Fraction(value) => value.is_finite() && (0.0..=1.0).contains(&value),
            _ => true,
        };
        if !valid_length
            || !metric(style.min_size[axis])
            || !metric(style.max_size[axis])
            || style.min_size[axis] > style.max_size[axis]
            || !metric(style.offset[axis].abs())
        {
            return Err(UiCompositionError::InvalidMetrics("node sizing/offset"));
        }
    }
    if !style.padding.into_iter().all(metric) || !metric(style.gap) {
        return Err(UiCompositionError::InvalidMetrics("padding/gap"));
    }
    Ok(())
}
