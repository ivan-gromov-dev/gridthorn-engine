mod errors;

use std::collections::VecDeque;
use std::fmt;

pub use errors::GameStateError;

/// Stable, engine-owned identifier for one game state.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GameStateId(String);

impl GameStateId {
    /// Construct a state identifier.
    ///
    /// # Errors
    ///
    /// Returns [`GameStateError::EmptyIdentifier`] for empty or whitespace-only input.
    pub fn new(identifier: impl Into<String>) -> Result<Self, GameStateError> {
        let identifier = identifier.into();
        if identifier.trim().is_empty() {
            return Err(GameStateError::EmptyIdentifier);
        }
        Ok(Self(identifier))
    }

    /// Borrow the identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for GameStateId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// A state-stack change applied at a host-frame boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameStateChange {
    /// Replace the active state without changing the stack depth.
    Set {
        /// State that stopped being active.
        exited: GameStateId,
        /// State that became active.
        entered: GameStateId,
    },
    /// Suspend the previous state and make a new state active.
    Push {
        /// State that was active before the push.
        suspended: GameStateId,
        /// State that became active.
        entered: GameStateId,
    },
    /// Remove the active state and resume its parent.
    Pop {
        /// State removed from the stack.
        exited: GameStateId,
        /// State that became active again.
        resumed: GameStateId,
    },
}

/// Ordered game-state stack with deferred transitions.
///
/// Systems may queue transitions during any lifecycle stage. The application
/// runtime applies them in request order after `Input` and before the next
/// `FixedUpdate`, so every fixed tick and presentation stage in a host frame
/// observe the same active state. Transitions requested later in the frame are
/// visible on the following frame.
pub struct GameStateStack {
    states: Vec<GameStateId>,
    pending: VecDeque<GameStateTransition>,
    changes: Vec<GameStateChange>,
}

impl GameStateStack {
    /// Create a stack with one active root state.
    #[must_use]
    pub fn new(initial: GameStateId) -> Self {
        Self {
            states: vec![initial],
            pending: VecDeque::new(),
            changes: Vec::new(),
        }
    }

    /// Return the currently active state.
    ///
    /// # Panics
    ///
    /// Panics only if the stack's internal root-state invariant is violated.
    /// Public transitions cannot remove the root state.
    #[must_use]
    pub fn current(&self) -> &GameStateId {
        self.states
            .last()
            .expect("a game-state stack always retains its root state")
    }

    /// Return the active stack from root to current state.
    #[must_use]
    pub fn states(&self) -> &[GameStateId] {
        &self.states
    }

    /// Return changes applied at the most recent transition boundary.
    #[must_use]
    pub fn changes(&self) -> &[GameStateChange] {
        &self.changes
    }

    /// Queue replacement of the active state.
    pub fn request_set(&mut self, state: GameStateId) {
        self.pending.push_back(GameStateTransition::Set(state));
    }

    /// Queue a state push.
    pub fn request_push(&mut self, state: GameStateId) {
        self.pending.push_back(GameStateTransition::Push(state));
    }

    /// Queue removal of the active state.
    ///
    /// # Errors
    ///
    /// Returns [`GameStateError::CannotPopRoot`] if earlier queued transitions
    /// would leave only the root state at the next transition boundary.
    pub fn request_pop(&mut self) -> Result<(), GameStateError> {
        let projected_depth = self
            .pending
            .iter()
            .fold(self.states.len(), |depth, transition| match transition {
                GameStateTransition::Set(_) => depth,
                GameStateTransition::Push(_) => depth + 1,
                GameStateTransition::Pop => depth - 1,
            });
        if projected_depth == 1 {
            return Err(GameStateError::CannotPopRoot);
        }
        self.pending.push_back(GameStateTransition::Pop);
        Ok(())
    }

    pub(crate) fn apply_pending(&mut self) {
        self.changes.clear();
        while let Some(transition) = self.pending.pop_front() {
            match transition {
                GameStateTransition::Set(entered) => {
                    let exited = std::mem::replace(
                        self.states
                            .last_mut()
                            .expect("a game-state stack always retains its root state"),
                        entered.clone(),
                    );
                    self.changes.push(GameStateChange::Set { exited, entered });
                }
                GameStateTransition::Push(entered) => {
                    let suspended = self.current().clone();
                    self.states.push(entered.clone());
                    self.changes
                        .push(GameStateChange::Push { suspended, entered });
                }
                GameStateTransition::Pop => {
                    let exited = self
                        .states
                        .pop()
                        .expect("validated transition cannot remove a missing state");
                    let resumed = self.current().clone();
                    self.changes.push(GameStateChange::Pop { exited, resumed });
                }
            }
        }
    }
}

enum GameStateTransition {
    Set(GameStateId),
    Push(GameStateId),
    Pop,
}

#[cfg(test)]
mod test;
