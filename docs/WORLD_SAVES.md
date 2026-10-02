# Provisional world saving and loading

Implemented on 2026-10-02 for the typed authoritative-root subset of
[scenarios](SCENARIOS.md). `ScenarioRuntime::save_document`, `load_document`,
`save_file`, `load_file`, `WorldSaveCodec`, and `WorldSaveError` are public
provisional SDK APIs. This completes the Milestone 3 item for games that keep
their complete authoritative world in `ScenarioState<S, C>`.

## Saved state and compatibility

Schema 1 is a strict TOML envelope identified by `gridthorn-world-save`. It stores
the exact SDK release, scenario identity/revision, fixed duration and catch-up
limit, completed tick count, pause/speed controls, exit request, master seed,
ordered named RNG positions, and game payload. Full-width u64 values use decimal
strings to avoid TOML's signed integer limit. Unknown envelope fields, invalid
numbers, invalid/duplicate stream names, unsupported schemas, and mismatched
compatibility metadata are errors. Empty RNG registries are supported.

The game implements `WorldSaveCodec<S, C>` to encode nested data and pending
commands into a UTF-8 string, and decode them into independent owned state.
This keeps serialization libraries and game schemas out of the public engine
boundary. Encoders must preserve command order and normalize unordered data.
Decoders must reject unknown required types and validate game invariants before
returning. Games own payload versions, durable object identities/references,
explicit deterministic migrations within a supported scenario revision, and
reconstruction of transient fields. Scenario revision changes are rejected;
there is no implicit engine-envelope migration or lossy field fallback.

All authoritative state must reside in the root. Arbitrary ECS components,
resources outside the root, scene/state controllers, callback-local state, and
runtime entity IDs are not captured. A game may persist its own durable IDs and
rebuild presentation from the loaded root. GPU handles, caches, tasks, and OS
objects must never enter the payload. The format does not infer authoritative
ownership or validate game-codec correctness.

## Load boundary and rollback

Call save/load between tick requests. Loading parses and checks engine/scenario/
fixed configuration before calling the decoder, validates controls and RNG,
then uses snapshot restoration to replace root, clock, controls, and exit state
together. No schedules run and Startup is not repeated. The next tick index is
the saved completed count; stale timing resources are removed. Loading into a
fresh runner happens after its Startup initializes non-authoritative services.
Shutdown rejects loading. Saved exit remains requested until game code replaces
the ExitRequest resource. Controls are stored even though explicit headless ticks
ignore pause/speed.

Every recoverable failure leaves live state intact. The transaction excludes
allocation failure, panics, and side effects in user codecs/Clone/destructors.
Codecs must perform no externally observable mutation while decoding.
Reproducibility still requires identical target, features, game rules, and command
meaning; the envelope checks SDK, scenario, and fixed configuration, but cannot
discover those application build conditions automatically.

## Filesystem contract

`save_file` encodes fully before touching the destination. It creates a unique
sibling with create-new semantics, writes and syncs it, closes it, then renames
over the destination. This provides atomic replacement where the filesystem
supports same-directory replacement. Failures before replacement preserve the
old file and attempt temporary-file cleanup. Readers see an old or new complete
document; concurrent writers have last-successful-replacement behavior.
Parent directories must already exist. File loading reads at most 16 MiB plus
one detection byte; oversized files and invalid UTF-8 are rejected. File saving
enforces the same limit. In-memory document APIs have no size limit.

Directory synchronization and power-loss durability, filesystem permissions
preservation, backups, multi-process locking, async I/O, hostile-input sandboxing,
checksums/authentication, compression, arbitrary ECS adapters, interactive-clock
restoration, and cross-release migrations remain deferred. Save/load is synchronous
and clones the root; large-world latency/memory and binary-size measurements,
network filesystems, and non-Windows replacement behavior need future validation.

## Evidence

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_world_saving
```

The headless example replaces an existing save at tick 25 with one queued command,
loads into a fresh runtime, and compares full canonical saves at tick 100.
Domain tests cover exact continuation, full-width seed/RNG state, controls/exit,
deterministic output, invalid schemas/metadata/payloads, duplicate RNG names,
shutdown rollback, file replacement/cleanup, codec failure preserving an old save,
read failure, invalid UTF-8, and the file limit. Windows replacement is exercised
through the real filesystem. The rationale is recorded in
[ADR 0003](adr/0003-world-save-envelope.md).
