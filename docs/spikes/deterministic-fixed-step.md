# Deterministic Fixed-Step Spike

## Status

Implemented as a Milestone 0 prototype. The executable scenario runs an ordered
command stream twice with the same seed and fixed-step count, then requires the
authoritative state fingerprints to match. Tests also establish that changing
the seed or a command changes the result.

## Proven boundary

- `gridthorn_simulation` owns an explicitly seeded SplitMix64 stream rather than
  delegating authoritative randomness to platform or ECS behavior.
- Authoritative integers are fed to an incremental FNV-1a fingerprint using
  explicit little-endian encoding.
- The example advances only through `ScheduleStage::FixedUpdate` and consumes
  commands in their declared order.
- The scenario depends on world schedules and simulation primitives without an
  application, window, or renderer dependency.

This validates the current determinism contract for one process and the same
Gridthorn version, target triple, feature set, initial state, fixed-step
configuration, command stream, and seed. Fixed-vector unit tests protect the
chosen random-stream and fingerprint algorithms from accidental changes.

## Provisional limits

The fingerprint is a fast regression identifier, not a cryptographic hash or a
serialized snapshot format. The crate API and encoded state schema remain
provisional until replay metadata, named random streams, snapshots, and
cross-target fixtures are designed in later milestones. Parallel authoritative
systems and unordered collections are outside this spike and must be normalized
before entering a future fingerprint.

## Run

From the engine repository:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_deterministic_replay
```

A successful run prints one hexadecimal fingerprint after both executions
match.
