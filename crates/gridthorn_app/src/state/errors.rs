use thiserror::Error;

/// Failure to construct or change a game-state stack.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum GameStateError {
    /// State identifiers must contain at least one non-whitespace character.
    #[error("game-state identifier must not be empty")]
    EmptyIdentifier,
    /// The root state cannot be removed because a running game always has one active state.
    #[error("cannot pop the root game state")]
    CannotPopRoot,
}
