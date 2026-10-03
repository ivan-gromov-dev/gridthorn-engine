use super::{UiAnimationError, UiEasing, UiTween};
use crate::composition::{UiCommand, UiLength, UiNodeId, UiTree};
use gridthorn_render::Color;
use std::time::Duration;

/// Explicit endpoints in logical pixels or linear RGBA; unrelated style fields are preserved.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UiProperty {
    /// Offset after anchoring; each coordinate is within ±65536 logical pixels.
    Offset([f32; 2]),
    /// Fixed pixel width and height, replacing the node's sizing policies.
    Size([f32; 2]),
    /// Explicit foreground override; all RGBA channels are in zero through one.
    Foreground([f32; 4]),
    /// Explicit background override; alpha affects this color, not descendant opacity.
    Background([f32; 4]),
    /// Requested scroll offset; layout clamps it to the current content extent.
    Scroll([f32; 2]),
}

impl UiProperty {
    fn channels(self) -> Result<[f32; 4], UiAnimationError> {
        let (channels, valid) = match self {
            Self::Offset(v) => (
                [v[0], v[1], 0.0, 0.0],
                v.into_iter().all(|n| n.is_finite() && n.abs() <= 65536.0),
            ),
            Self::Size(v) | Self::Scroll(v) => (
                [v[0], v[1], 0.0, 0.0],
                v.into_iter()
                    .all(|n| n.is_finite() && (0.0..=65536.0).contains(&n)),
            ),
            Self::Foreground(v) | Self::Background(v) => (
                v,
                v.into_iter()
                    .all(|n| n.is_finite() && (0.0..=1.0).contains(&n)),
            ),
        };
        if valid {
            Ok(channels)
        } else {
            Err(UiAnimationError::InvalidValue(
                "property endpoint outside finite logical-pixel/RGBA bounds",
            ))
        }
    }

    fn with_channels(self, v: [f32; 4]) -> Self {
        match self {
            Self::Offset(_) => Self::Offset([v[0], v[1]]),
            Self::Size(_) => Self::Size([v[0], v[1]]),
            Self::Scroll(_) => Self::Scroll([v[0], v[1]]),
            Self::Foreground(_) => Self::Foreground(v),
            Self::Background(_) => Self::Background(v),
        }
    }

    fn apply(self, tree: &mut UiTree, node: UiNodeId) -> Result<(), UiAnimationError> {
        let result = (|| {
            if let Self::Scroll(v) = self {
                return tree.command(node, UiCommand::ScrollTo(v)).map(|_| ());
            }
            let mut style = tree
                .node(node)
                .ok_or(crate::composition::UiCompositionError::UnknownNode(node))?
                .style
                .clone();
            match self {
                Self::Offset(v) => style.offset = v,
                Self::Size(v) => style.size = v.map(UiLength::Pixels),
                Self::Foreground(v) => style.foreground = Some(Color::rgba(v[0], v[1], v[2], v[3])),
                Self::Background(v) => style.background = Some(Color::rgba(v[0], v[1], v[2], v[3])),
                Self::Scroll(_) => unreachable!(),
            }
            tree.set_style(node, style)
        })();
        result.map_err(|source| UiAnimationError::Destination { node, source })
    }
}

/// One explicit UI property transition. Dropping it cancels and retains the last applied value.
///
/// Endpoints are caller-owned; construction does not edit the tree. Apply with zero delta
/// to install the start (or immediate destination). Retarget uses the last sampled value.
/// Multiple transitions may run together; callers own ordering and conflicting writes.
#[derive(Clone, Debug)]
pub struct UiTransition {
    node: UiNodeId,
    property: UiProperty,
    tween: UiTween<4>,
}

impl UiTransition {
    /// Construct a transition with explicit matching endpoints.
    ///
    /// # Errors
    /// Rejects property-kind mismatches and invalid endpoints. Destination checks occur on apply.
    pub fn new(
        node: UiNodeId,
        from: UiProperty,
        to: UiProperty,
        duration: Duration,
        easing: UiEasing,
    ) -> Result<Self, UiAnimationError> {
        if std::mem::discriminant(&from) != std::mem::discriminant(&to) {
            return Err(UiAnimationError::PropertyMismatch);
        }
        Ok(Self {
            node,
            property: from,
            tween: UiTween::new(from.channels()?, to.channels()?, duration, easing)?,
        })
    }

    /// Advance and atomically apply one sample; recompute layout before routing or rendering.
    /// Returns whether this transition has finished. Finished transitions reapply their endpoint.
    ///
    /// # Errors
    /// Unknown nodes or unsupported scroll targets preserve both tree and transition time.
    pub fn advance(
        &mut self,
        tree: &mut UiTree,
        delta: Duration,
    ) -> Result<bool, UiAnimationError> {
        let mut next = self.tween.clone();
        self.property
            .with_channels(next.advance(delta))
            .apply(tree, self.node)?;
        self.tween = next;
        Ok(self.is_finished())
    }

    /// Interrupt smoothly from the current sample. This does not read externally edited style.
    ///
    /// # Errors
    /// Rejects mismatched kinds/invalid endpoints without changing the transition.
    pub fn retarget(
        &mut self,
        to: UiProperty,
        duration: Duration,
        easing: UiEasing,
    ) -> Result<(), UiAnimationError> {
        if std::mem::discriminant(&self.property) != std::mem::discriminant(&to) {
            return Err(UiAnimationError::PropertyMismatch);
        }
        self.tween.retarget(to.channels()?, duration, easing)
    }

    /// Current sample, including values not yet installed into the tree.
    #[must_use]
    pub fn value(&self) -> UiProperty {
        self.property.with_channels(self.tween.value())
    }

    /// Pause only this transition.
    pub fn pause(&mut self) {
        self.tween.pause();
    }

    /// Resume without accumulating paused frame time.
    pub fn resume(&mut self) {
        self.tween.resume();
    }

    /// Whether the transition has reached its endpoint.
    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.tween.is_finished()
    }
}
