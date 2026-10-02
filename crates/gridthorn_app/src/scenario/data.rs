use gridthorn_simulation::{GameCommandQueue, RandomStreams};

use super::ScenarioError;

/// Complete game-owned authoritative root, ordered pending commands, and named RNG states.
/// Snapshottable games must keep all authoritative state in this root, using owned cloneable data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScenarioState<S, C> {
    /// Game data; caches, runtime entity IDs, and shared mutable handles must be excluded.
    pub data: S,
    /// Commands waiting for game code to drain at a fixed boundary.
    pub commands: GameCommandQueue<C>,
    /// Explicitly registered authoritative streams.
    pub random: RandomStreams,
}

/// Reusable initial state with an explicit project-owned compatibility identity.
#[derive(Clone, Debug)]
pub struct Scenario<S, C> {
    pub(super) name: String,
    pub(super) revision: u32,
    pub(super) initial: ScenarioState<S, C>,
}

impl<S, C> Scenario<S, C> {
    /// Define a scenario; clones of the initial state start independent runs.
    ///
    /// # Errors
    /// Rejects empty/padded names and revision zero.
    pub fn new(
        name: &str,
        revision: u32,
        initial: ScenarioState<S, C>,
    ) -> Result<Self, ScenarioError> {
        if name.is_empty() || name.trim() != name || revision == 0 {
            return Err(ScenarioError::InvalidIdentity);
        }
        Ok(Self {
            name: name.to_owned(),
            revision,
            initial,
        })
    }
}
