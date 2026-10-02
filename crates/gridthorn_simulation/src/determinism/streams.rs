use std::collections::BTreeMap;

use super::{DeterministicRng, RandomStreamError, StateFingerprint};

/// Cloneable named `SplitMix64` streams with deterministic, registration-order-independent seeds.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RandomStreams {
    seed: u64,
    streams: BTreeMap<String, DeterministicRng>,
}

impl RandomStreams {
    /// Reconstruct persisted stream positions without consuming random values.
    ///
    /// # Errors
    /// Rejects invalid or duplicate stream names.
    pub fn from_states(
        seed: u64,
        states: impl IntoIterator<Item = (String, u64)>,
    ) -> Result<Self, RandomStreamError> {
        let mut result = Self::new(seed);
        for (name, state) in states {
            result.register(&name)?;
            result.streams.insert(name, DeterministicRng::new(state));
        }
        Ok(result)
    }
    /// Create an empty registry with an explicit master seed.
    #[must_use]
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            streams: BTreeMap::new(),
        }
    }

    /// Register a stream using FNV-1a over the master seed, name length, and UTF-8 name.
    /// Integers use little-endian u64 encoding. This provisional encoding is versioned with the SDK.
    ///
    /// # Errors
    /// Rejects empty/padded names and duplicate registrations without mutation.
    pub fn register(&mut self, name: &str) -> Result<(), RandomStreamError> {
        if name.is_empty() || name.trim() != name {
            return Err(RandomStreamError::InvalidName);
        }
        if self.streams.contains_key(name) {
            return Err(RandomStreamError::Duplicate(name.to_owned()));
        }
        let mut fingerprint = StateFingerprint::new();
        fingerprint.write_u64(self.seed);
        fingerprint.write_u64(name.len() as u64);
        fingerprint.write_bytes(name.as_bytes());
        self.streams
            .insert(name.to_owned(), DeterministicRng::new(fingerprint.finish()));
        Ok(())
    }

    /// Consume one value from an explicitly registered stream.
    ///
    /// # Errors
    /// Returns a contextual error for an unknown stream without changing any stream.
    pub fn next_u64(&mut self, name: &str) -> Result<u64, RandomStreamError> {
        self.streams
            .get_mut(name)
            .map(DeterministicRng::next_u64)
            .ok_or_else(|| RandomStreamError::Missing(name.to_owned()))
    }

    /// Inspect stream states in lexicographic name order without consuming values.
    pub fn states(&self) -> impl Iterator<Item = (&str, u64)> {
        self.streams
            .iter()
            .map(|(name, rng)| (name.as_str(), rng.state()))
    }

    /// Master seed used for future registrations.
    #[must_use]
    pub fn seed(&self) -> u64 {
        self.seed
    }
}
