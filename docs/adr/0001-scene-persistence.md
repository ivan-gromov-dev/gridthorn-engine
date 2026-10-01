# ADR 0001: Explicit scalar scene persistence with prepared loading

- Status: Proposed
- Date: 2026-10-01
- Owners: project maintainers
- Supersedes: none
- Superseded by: none

## Context

Milestone 2 needs versioned scenes built on existing scalar reflection. World
storage must remain independent of persistence libraries and platform services.
Loading invalid or incompatible data must preserve the live world. The first
scene schema has no historical formats requiring automatic migration yet.

## Decision drivers

- Explicit authoritative-data registration through the same SDK as game code.
- Stable type/field identities independent of backend ECS implementation.
- Contextual incompatibility/domain failures and no partial world application.
- Deterministic document ordering and migrations without ambient inputs.
- Reuse existing serde/TOML dependencies; avoid a derive-macro subsystem.

## Considered options

### Backend ECS snapshot serialization

Direct backend snapshots would couple persisted identities and construction to
ECS internals and risk including runtime storage. They do not satisfy the current
facade and authoritative-data ownership requirements.

### Explicit scalar adapters with prepared loading

Reflection provides named snapshots. A separate opt-in constructor validates
persisted values and reconstructs omitted fields off-world. Engine-owned adapters
then commit fully prepared values at one explicit load/reset boundary.

## Decision

Prototype `gridthorn_scene` as an independent world-dependent service with strict
TOML schema 1, explicit engine requirements, stable named fields, and pure ordered
migration hooks. Default engine compatibility is the exact source release.
Persist only scene-owned entities and explicitly registered global resource
overlays. Missing resources are preserved, and resources have no scene ownership.
Entity references/durable identities are outside schema 1. Encode u64 as canonical
decimal text and require finite floats; this does not authorize stronger
cross-platform deterministic floating-point simulation claims.

Keep this decision Proposed while the scalar boundary is provisional. Application
scene transitions remain separate from data loading. Filesystem/user-save atomic
replacement is not part of this prototype.

## Consequences

### Positive

- Invalid data cannot partially mutate live world storage.
- No platform, rendering, or backend ECS types cross persistence APIs.
- Projects validate domain values and rebuild transient state explicitly.
- No new third-party library family is needed for the first schema.

### Negative

- Manual reflection and constructor implementation is required.
- All prepared values coexist with old world data before commit.
- Runtime allocation order can change re-captured entity array order.
- Nested data, references, automatic scene orchestration, and durable user saves
  require subsequent increments rather than implicit extension of schema 1.

## Validation

- Domain tests cover every scalar kind, full integer ranges, Unicode, deterministic
  unchanged capture, compatibility failures, scope preservation, late constructor
  failure, and pure field migration before commit.
- The sibling `scene-serialization` example exercises the public facade headlessly.
- Existing verification and dependency-boundary checks must pass before handoff.
- Acceptance requires a concrete game scene workload and cross-platform CI;
  latency, allocation, and binary-size thresholds remain to be measured.

## Follow-up

- Evaluate durable entity identities and references against the traditional 2D game.
- Define crash-safe file replacement before exposing user save APIs.
- Measure scene preparation/capture memory and latency before optimization.
