# ADR 0002: Typed authoritative roots for provisional simulation snapshots

- Status: Proposed
- Date: 2026-10-02
- Owners: project maintainers
- Supersedes: none
- Superseded by: none

## Context

Headless execution needs reusable initial conditions, controlled random streams,
and exact continuation without adopting a premature arbitrary-world save format.
The scene persistence contract captures registered scalar scene data, but cannot
preserve arbitrary nested authoritative data, pending commands, or simulation RNG.

## Decision drivers

- Restore exact tick and command/RNG continuation with recoverable error rollback.
- Keep lifecycle in app and random primitives independent of ECS/platform.
- Avoid pretending that cloning a subset captures the entire ECS world.
- Keep persistence formats and durable entity identities in the next increment.

## Considered options

### Generic ECS capture

Requires a registration/serialization contract for every authoritative component,
resource, entity reference, scheduler state, command type, and RNG stream.
This overlaps the separate world saving/loading milestone prematurely.

### Game-owned typed root

An explicit owned cloneable root contains authoritative data, ordered commands,
and named random streams. App-owned snapshots add exact clock and controls.
Games must obey a narrow ownership contract; arbitrary ECS games need adapters.

## Decision

The provisional implementation selects typed roots and opaque in-memory snapshots.
Same scenario identity/revision, exact SDK release, and fixed configuration are
required. Recovery validates and clones before committing; no schedules run during
restore. Named stream seeds use specified FNV encoding and SplitMix64 pinned by
fixed vectors. See [SCENARIOS.md](../SCENARIOS.md) for the complete contract.
This record remains Proposed until game integration and platform validation.

## Consequences

### Positive

- No new dependency edges, reflection requirements, or serialized format.
- Independent stream consumption and exact snapshot continuation are testable.
- World persistence can establish its own identity and migration boundary later.

### Negative

- Clone correctness and authoritative ownership are application contracts.
- Arbitrary ECS data, scene controllers, and system-local state are not captured.
- Large root cloning needs workload measurements before broad use.

## Validation

Domain tests cover vectors, restoration, queued commands, clock overflow, and
compatibility rollback. The public scenarios-snapshots example compares full
state and game-encoded fingerprints on Windows. Cross-platform validation and
large-root measurements remain required before accepting a stronger guarantee.

## Follow-up

Define durable world IDs, serialized scenario/snapshot envelopes, migrations,
interactive clock restoration, and tooling integrations when those milestones
require them.
