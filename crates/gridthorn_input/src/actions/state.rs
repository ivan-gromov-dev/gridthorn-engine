use std::collections::BTreeMap;

/// Action state for one evaluation; a short tap can have both edge flags set.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "independent frame edges and cancellation can coexist"
)]
pub struct ActionState {
    /// At least one alternative binding is currently held.
    pub down: bool,
    /// The action became active at least once, excluding duplicate repeat presses.
    pub just_pressed: bool,
    /// The last held alternative was normally released at least once.
    pub just_released: bool,
    /// Focus or connection loss ended an active action; not a completed click.
    pub cancelled: bool,
}

/// Immutable game-action results in caller-defined action order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ActionFrame<A> {
    pub(super) actions: BTreeMap<A, ActionState>,
}

impl<A: Ord> ActionFrame<A> {
    /// Read an action; absent or removed actions are neutral after their final edge.
    #[must_use]
    pub fn get(&self, action: &A) -> ActionState {
        self.actions.get(action).copied().unwrap_or_default()
    }

    /// Configured actions and final edges for removed actions, in action-key order.
    pub fn iter(&self) -> impl Iterator<Item = (&A, &ActionState)> {
        self.actions.iter()
    }
}
