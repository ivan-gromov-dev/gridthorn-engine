mod document;
mod errors;
mod files;

pub use errors::WorldSaveError;

use gridthorn_simulation::GameCommandQueue;

/// Game-owned encoding and validation for nested authoritative data and ordered commands.
/// Implementations must exclude runtime handles and use deterministic ordering.
/// Decode must validate all game invariants and reconstruct transient fields without side effects.
pub trait WorldSaveCodec<S, C> {
    /// Encode game data and pending commands into a UTF-8 payload.
    ///
    /// # Errors
    /// Return a contextual diagnostic when the state cannot be persisted.
    fn encode(&self, data: &S, commands: &GameCommandQueue<C>) -> Result<String, String>;

    /// Decode and validate the complete game payload before live state can change.
    ///
    /// # Errors
    /// Reject malformed data, unknown required types, and invalid game state.
    fn decode(&self, payload: &str) -> Result<(S, GameCommandQueue<C>), String>;
}

#[cfg(test)]
mod test;
