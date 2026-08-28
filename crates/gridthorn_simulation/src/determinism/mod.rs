//! Reproducible random streams and authoritative-state fingerprints.

mod fingerprint;
mod rng;

pub use fingerprint::StateFingerprint;
pub use rng::DeterministicRng;

#[cfg(test)]
mod test;
