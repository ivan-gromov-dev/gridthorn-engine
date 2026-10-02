# ADR 0003: Versioned world saves over typed authoritative roots

- Status: Proposed
- Date: 2026-10-02
- Owners: project maintainers
- Supersedes: none
- Superseded by: none

## Context

Typed scenario snapshots already capture exact headless continuation. Scalar
scene documents cannot represent complete nested game worlds, pending commands,
or random positions. Filesystem persistence needs explicit compatibility and
transactional loading without serializing live runtime handles.

## Decision

Use a strict schema 1 TOML engine envelope over the existing ScenarioState root.
The application service coordinates capture and restoration; a game-owned codec
encodes/validates nested data and commands. The simulation service reconstructs
named RNG positions through its validated public API. No new project dependency
edge or public third-party serialization type is introduced.

Require exact engine release, scenario revision, and fixed configuration. Reject
unknown fields/future schemas. Decode off-world before snapshot commit. Save via
a synced unique sibling and same-directory rename; bound file I/O to 16 MiB.
See [WORLD_SAVES.md](../WORLD_SAVES.md) for the testable contract.

## Alternatives and consequences

Generic ECS serialization would require authority registration, durable entity
references, nested reflection, and scheduler-state rules before game validation.
A root-only game payload without an engine envelope would lose exact tick/RNG/
command compatibility. The selected boundary supports nested game data today,
while leaving arbitrary ECS capture, cross-release migrations, interactive-clock
restoration, and stronger crash durability explicit future work.

The game must enforce authoritative ownership, stable identities, payload
validation, deterministic encoding, and side-effect-free decoding. Synchronous
encoding and root clones need large-world measurements. The decision remains
Proposed until tycoon integration and broader platform/filesystem validation.

## Validation

Domain tests and the public world-saving example exercise rollback, real Windows
replacement, fresh-runner continuation, queued commands, controls, and RNG.
