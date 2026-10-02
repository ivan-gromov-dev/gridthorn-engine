# Provisional scenarios, snapshots, and controlled randomness

Implemented on 2026-10-02 through the public SDK. `Scenario`, `ScenarioState`,
`ScenarioRuntime`, `SimulationSnapshot`, `RandomStreams`, and their typed errors
are provisional. No dependencies or crate edges are added. The orchestration
belongs to the application crate; named random primitives remain independent
of world storage and presentation in the simulation crate.

## Authoritative root and scenario execution

`Scenario::new(name, revision, initial)` defines reusable initial state with an
explicit nonempty, unpadded name and positive project-owned revision. A
`ScenarioState<S, C>` groups game data `S`, the ordered `GameCommandQueue<C>`,
and named `RandomStreams`. Games use owned `Clone + Send + Sync + 'static` data
and commands. Clone must produce independent authoritative state. Shared mutable
handles, runtime entity IDs, device objects, ambient randomness, and external
mutable state must not enter the root.

`ScenarioRuntime::new(schedules, config, scenario)` installs a clone of the
initial root before running Startup exactly once. Systems access the root
through the ordinary public WorldAccess resource API. Startup may initialize
it. Missing roots produce ScenarioError; systems must not remove the root during
execution. `run_ticks` delegates exact work to the [headless runner](SIMULATION.md),
including per-tick Input, state/scene transitions, fixed time, and exit semantics.
Authoritative game data changes only during FixedUpdate or explicit initialization/
reset boundaries. Commands are drained by game code during FixedUpdate. Queue insertion between
requests explicitly assigns them to the next tick; automatic timestamped command
routing, recording, and replay files are not provided.

All authoritative game data must reside in ScenarioState. Other resources,
entities, scene controllers/state stacks, system-local mutable state, and closures
are outside snapshot capture and must remain non-authoritative in this subset.
A game using those as authoritative owners needs a future world snapshot adapter.
Schedules and callbacks must be identical for continuation; changing game rules
or root/command meaning requires a scenario revision change. The API cannot
verify that user callbacks obey these ownership and determinism contracts.

## Capture and restore

`snapshot()` clones the entire root, including queued commands, master seed,
registered stream states, simulation controls, ExitRequest, configured fixed
step/catch-up limit, next tick index, scenario name/revision, and exact SDK release.
Snapshots are opaque, typed, cloneable in-memory values; there is no serialized
format or filesystem I/O. Capture is valid between tick requests, including at
zero ticks and after exit or Shutdown for inspection.

`restore(&snapshot)` checks scenario identity/revision, exact engine release,
and full fixed configuration before cloning and changing live state. Shutdown
rejects restoration. All recoverable failures preserve the root and clock.
Successful restoration replaces the root, controls, exit state, and explicit
clock together without running schedules or repeating Startup. Old FixedTime
and FrameTiming resources are removed and republished on the next actual tick.
The next tick has the snapshot's completed-tick index. Host-time lag is zero
because ScenarioRuntime uses only explicit ticks. The saved ExitRequest is
preserved, so restoring an exited snapshot remains stopped until explicitly
cleared by game code. A snapshot can restore into a fresh compatible runner
after that runner's Startup has initialized its non-authoritative services.

World data outside the root is untouched. The transaction excludes allocation
failure, panics, and side effects in user Clone/destructor implementations.
Snapshots are trusted captures, not untrusted document inputs; invalid externally
edited snapshot documents and migrations belong to later persistence work.

## Named randomness

`RandomStreams::new(master_seed)` starts an empty registry. `register(name)`
requires an explicit nonempty, unpadded UTF-8 name and rejects duplicates instead
of reseeding. Stream seeds use FNV-1a over little-endian u64 master seed,
little-endian u64 UTF-8 byte length, then the name bytes. Each registered stream
uses the existing SplitMix64 algorithm. Names are ordered lexicographically
for state inspection. Registration order and consumption of unrelated streams
do not affect a stream's output. Unknown stream consumption returns a contextual
error without altering any RNG. Cloning preserves the master seed and every
stream position, including deterministic seeding for registrations after restore.

For master seed 42 and name `economy`, the derived state is
`21b82d29fc053710`; the first outputs are `3ee96a6fef43c8f3`,
`474729adeb100b19`, and `b776a43f6915ee7e`. These vectors pin algorithm/encoding
compatibility. Seed derivation is not cryptographic and does not promise
collision-free names. Distribution/range helpers and cryptographic randomness
are outside this API; game code must not silently introduce modulo bias when
it needs a uniform bounded distribution.

`StateFingerprint` is also exposed through the facade. Games supply explicit
canonical byte/integer encodings and iteration order for authoritative hashes.
No generic hash for arbitrary S or ECS entities is inferred. FNV fingerprints
are diagnostic comparisons, not security/integrity proofs.

## Evidence and scope

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_scenarios_snapshots
```

The example compares full integer state and RNG after 100 ticks, snapshot
continuation in a fresh runner, and replay from the initial snapshot. A queued
command captured at tick 25 is consumed exactly once. It prints a game-encoded
fingerprint (`d255a15d9a986482` for this example's initial version).
Domain tests cover fixed vectors, registration order, independent streams,
clone continuation, invalid names/access, compatibility rollback, commands,
controls, exit, zero ticks, missing roots, Shutdown, and tick overflow.

Reproducibility requires the same engine, target, features, game rules,
configuration, initial state, command sequence, and seeds. No stronger
cross-target bit-identical guarantee is made. In-memory snapshots have the same
compiled S/C types and process/target; portable serialized metadata is deferred.
This completes the Milestone 3 item for the typed authoritative-root subset.
Arbitrary ECS/world saving/loading, scenario/snapshot files, migrations,
interactive-clock snapshots, tooling restoration, CLI scenario commands,
large-state clone costs/memory limits, binary size, and cross-platform/performance
measurements are deferred. The rationale is recorded in the
[proposed snapshot ADR](adr/0002-typed-simulation-snapshots.md).
