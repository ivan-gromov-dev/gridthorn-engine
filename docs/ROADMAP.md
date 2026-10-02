# Implementation Roadmap

This roadmap describes risk-reduction order rather than promised dates. Every
phase ends in a working, documented result.

## Milestone 0 — Project and CLI foundation (complete)

- [x] Set and document the minimum supported Rust version (MSRV).
- [x] Create the Cargo workspace and dependency-boundary rules.
- [x] Create `gridthorn_cli` with `new`, `run`, `check`, and `--version`.
- [x] Add an embedded minimal project template and generation tests.
- [x] Configure formatting, lints, tests, and CI.
- [x] Add an ADR process for architectural decisions.
- [x] Define public API, SemVer, and changelog policies.
- [x] Define runtime, determinism, plugin, and serialization contracts.
- [x] Complete the dependency spikes listed in
      [TECHNOLOGY.md](TECHNOLOGY.md).
- [x] Provide a contributor guide and common development commands.

**Result:** a reproducible SDK foundation and a real CLI that creates, checks,
and launches the minimal generated project.

Completed on 2026-08-28. The evidence, measurements, and explicit runtime
deferrals are recorded in the
[Milestone 0 review](spikes/milestone-0-review.md).

## Milestone 1 — Runtime vertical slice (complete)

Started and completed on 2026-08-30. The implemented slice exposes the provisional
application lifecycle, ordered frame schedules, and fixed-time accumulation
through the SDK facade. The timed runtime is now integrated with the native
window loop, including suspend/resume and shutdown behavior. Native keyboard
and mouse events cross an engine-owned boundary
into frame-scoped `InputState`; command mapping and a controllable example
now drive a world entity through continuous tick input and ordered one-shot
`GameCommand` values. The same example now presents that entity through an
engine-owned orthographic camera, colored sprite frame, and GPU pipeline.
Texture files now decode into engine-owned RGBA assets and can be presented by
the same example as textured sprites. That example also maps host-frame time,
fixed work, accumulated lag, and overload state into a compact screen-space
timing overlay.
`gridthorn check` now rejects drift between `gridthorn.toml`, the Cargo package
name, the Cargo dependency requirement, and the running CLI/SDK version before
delegating compilation to Cargo.
The generated minimal project composes these capabilities into a controllable
sprite and is built and executed through `gridthorn new`, `check`, and `run`.

- [x] Application lifecycle and schedules.
- [x] Time, `Update`, and fixed-step `FixedUpdate`.
- [x] Window and keyboard/mouse input.
- [x] Minimal world model through the Gridthorn API.
- [x] Basic 2D renderer, camera, and sprite.
- [x] Texture loading as an asset.
- [x] Diagnostic timing overlay.
- [x] Extend CLI checks with engine/project compatibility validation.
- [x] Build and run one documented example through the CLI.

**Result:** `gridthorn new` to a controllable sprite and observable fixed tick in
one cohesive workflow.

Evidence, limitations, and explicit performance deferrals are recorded in the
[Milestone 1 review](milestone-1-review.md).

## Milestone 2 — General-purpose 2D SDK (complete)

The sibling `classic_2d` Crystal Trail game now exercises scenes/game states,
shared-atlas sprites and animation, playable menu/pause/victory UI, and native
audio through an example-owned worker. Headless tests cover collision pickups,
pause, victory cleanup, and restart. Completed on 2026-10-02 after the maintainer
confirmed the playable result and corrected vertical input. Supported subsets,
verification evidence, and explicit deferrals are recorded in the
[Milestone 2 review](milestone-2-review.md).

Started on 2026-08-30. The implemented provisional lifecycle foundation now
includes engine-owned game-state identifiers, an ordered state stack, scene
identifiers, and scene-owned entities. State changes requested by lifecycle
systems are applied after `Input`, giving fixed updates and presentation a
consistent active state for the entire host frame. Active-scene replacement
removes the exited scene's entities and runs `SceneTransition` construction
systems exactly once before fixed updates. The separate scalar scene persistence
service now provides explicit versioned documents and prepared loading.
The renderer now submits adjacent textured sprites sharing one cloned texture
asset as a single ordered GPU batch. Normalized sprite-sheet regions feed that
same batch path, while provisional uniform-frame animation clips support
looping, one-shot completion, pause, resume, and restart in presentation time.
The provisional runtime UI path now renders ordered colored rectangles and
5x7 bitmap text in top-left-origin screen pixels through the same colored GPU
batch. The built-in font covers Latin letters, digits, and common diagnostics
punctuation; richer font assets, layout, and interactive widgets remain planned.
Mouse-driven runtime buttons now consume the engine-owned frame input boundary,
report hover and held visuals, and emit one activation only after an inside
press completes with an inside release. Focus loss or dragging outside cancels
the pending activation; keyboard focus and higher-level layout remain planned.
The provisional audio foundation decodes interleaved PCM16 WAV clips without an
output device and exposes an ordered queue for play, stop, and normalized volume
commands with stable voice identifiers. An opt-in `native-output` feature now
provides a Kira-backed service that mixes mono and stereo clips, applies queued
voice controls, and supports suspend/resume; the same command path is tested
through Kira's mock backend without native audio development libraries.
Crystal Trail integrates playback, game pause/resume, and shutdown in its
application schedules through an example-owned worker. Automatic native platform
suspend/resume, streaming, device recovery, latency validation, and broader
formats remain deferred.
The provisional basic 2D collision boundary now validates engine-owned circles
and axis-aligned boxes and provides deterministic-order overlap and minimum
translation queries for box-box, circle-circle, and circle-box pairs. It is a
narrow-phase query API rather than a rigid-body simulation; broad-phase spatial
indexing, collision layers, continuous collision, and physics response remain
planned for proven game requirements.

The provisional asset store now uses stable relative-path identities for raw
sources and decoded textures, validates explicit dependency edges, and reports
transitive invalidation in deterministic dependency-first order. Synchronous
content polling commits a complete reload batch only after all reads and texture
decodes succeed; existing snapshots and last-good data survive failed edits.
The `AssetReloader` service now performs polling and decoding on one background
worker, bounds outstanding work to one request/result, and publishes only when
the caller polls at a frame boundary. Failed batches preserve last-good data;
shutdown joins the worker and discards unapplied results. The sibling
`asset-reload` example requests a scan every 250 ms, publishes at `PollEvents`,
and resolves fresh textures for rendering. Its headless smoke covers file edits,
dependency propagation, rollback, recovery, and shutdown. The asset dependency
and hot-reload milestone item is complete for raw sources and PNG/PNM textures.
Native file watching, custom derived-asset loaders, dynamic registration,
unloading, and audio reload are deferred extensions. Large-project performance,
memory, and cross-platform watcher measurements remain explicitly deferred.
This is a development presentation service, not authoritative game-data reload.

The provisional [reflection contract](REFLECTION.md) provides explicit type
registration, deterministic scalar field metadata, and validated read-only
component/resource snapshots. Nested data, editing, derive macros, and
performance measurements remain deferred.

The provisional [scene persistence contract](SCENES.md) now provides strict schema 1
TOML documents, engine compatibility requirements, registered scalar components
and resource overlays, explicit migrations, and off-world validation/construction
before atomic load-boundary application. A public headless example covers
round-trip, reconstruction of transient fields, rejected edits, and legacy field
migration. Entity references, nested data, filesystem/user-save durability,
automatic scene-loop integration, and large-scene measurements remain deferred.

- [x] Scenes and game states.
- [x] Sprite batching and 2D animation.
- [x] Text and basic runtime UI.
- [x] Audio.
- [x] Basic 2D collision.
- [x] Asset dependencies and hot reload.
- [x] Reflection for components and resources.
- [x] Versioned scene serialization.
- [x] Extend the CLI with development watching and release builds.
- [x] Complete a small traditional 2D game.

**Result:** the SDK can build a small, complete traditional 2D game.

## Milestone 3 — Gridthorn specialization

Started on 2026-10-02 with the provisional opt-in square and isometric
[coordinate contract](GRIDS.md), signed cell identities, validated presentation
projection/inverse conversion, focused tests, and the sibling `grid-coordinates`
public SDK example. The subsequent provisional tilemap increment now provides
sparse generic tile data, deterministic ordered layers/chunks, signed chunk
boundaries, empty-chunk reclamation, and square/isometric occupied-cell picking
through validated orthographic cursor conversion. The sibling `tilemap-basics`
example exercises the public facade without a GPU; focused tests cover storage,
layer pass-through, camera pan/zoom, viewport edges, and invalid queries.
The supported subset and explicit performance/presentation deferrals are
recorded in [GRIDS.md](GRIDS.md).

The earlier Milestone 0 headless spike only validates the dependency boundary.
This milestone turns that prototype into a supported public simulation API with
documented behavior, diagnostics, and tests.

- [x] Square and isometric coordinate systems.
- [x] Tilemaps, layers, chunks, and picking.
- [x] Grid-based object placement.
- [x] Pathfinding and diagnostic visualization.
- [x] Simulation clock, pause, and speed control.
- [x] Headless simulation.
- [x] Scenarios, snapshots, and controlled random-number generation.
- [x] World saving and loading.
- [ ] Extend the CLI with scenario and headless simulation commands.
- [ ] Complete an isometric tycoon vertical slice.

**Result:** a finished showcase validates Gridthorn's specialization.

## Milestone 4 — Professional debugging workflow

- [ ] Entity hierarchy and universal inspector.
- [ ] Command API and runtime state editing.
- [ ] System, renderer, and memory profiling.
- [ ] Game-command replay.
- [ ] Local development protocol.
- [ ] Automatic restart after code changes.
- [ ] Scenario and snapshot restoration.
- [ ] Asset packaging and distributable builds.
- [ ] Extend the CLI with testing, profiling, replay, and packaging commands.
- [ ] Publish a practical SDK guide.

**Result:** a complete code-first workflow from project creation to diagnosis and
release packaging.

## Milestone 5 — Minimal Gridthorn Editor

- [ ] Separate desktop application.
- [ ] Project browser and dockable layout.
- [ ] Scene viewport, hierarchy, inspector, and asset browser.
- [ ] Play, Pause, Step, and Stop through the development protocol.
- [ ] Object selection and transform manipulation.
- [ ] Undo and redo through the command API.
- [ ] Tilemap painting.
- [ ] Console, profiler, and build panels.
- [ ] Crash recovery and layout persistence.

**Result:** an editor-like development loop without attempting to reproduce the
full breadth of Unity or Unreal.

## Cross-cutting requirements

Every milestone monitors:

- compile times, binary size, and dependency growth;
- public API clarity and stability;
- automated tests and examples;
- contextual, actionable diagnostics;
- absence of editor-only dependencies in release games;
- headless execution where applicable;
- documentation for new public capabilities.

## Definition of done

A milestone result is complete only when:

- its user-visible behavior and architectural contracts match the
  implementation;
- normal local checks and the same checks in CI pass from a clean checkout;
- each public capability has API documentation and at least one compiled usage
  example;
- relevant failure paths have contextual diagnostics and automated tests;
- performance, binary-size, platform, and determinism expectations have either
  measured results or an explicitly recorded deferral;
- irreversible decisions and accepted exceptions are recorded as ADRs;
- README and roadmap status distinguish planned, prototyped, implemented, and
  stable behavior.

## Immediate target

Grid-based object placement is complete on 2026-10-02 for the provisional
integer-footprint and exclusive-occupancy subset documented in [GRIDS.md](GRIDS.md).
The public `grid-placement` example and domain tests validate preview, picking,
atomic placement/movement, removal, and rejected edits.
Pathfinding and diagnostic visualization are complete on 2026-10-02 for the
bounded four-neighbor weighted-search subset documented in [GRIDS.md](GRIDS.md).
The public `pathfinding` example generates square/isometric SVG diagnostics.
Simulation clock, pause, and speed control are complete on 2026-10-02 for the
provisional rational-speed and frame-boundary control subset documented in
[SIMULATION.md](SIMULATION.md). The public simulation-clock example and domain
tests validate pause, resume, backlog, speed scaling, explicit stepping, and
continued presentation.
Headless simulation is complete on 2026-10-02 for the provisional exact-tick
runner documented in [SIMULATION.md](SIMULATION.md). The public headless-simulation
example validates ordered command consumption and repeatable state across request
partitions without presentation.
Scenarios, snapshots, and controlled random-number generation are complete on
2026-10-02 for the provisional typed authoritative-root and in-memory snapshot
subset documented in [SCENARIOS.md](SCENARIOS.md). The public scenarios-snapshots
example and domain tests validate named stream independence, fixed RNG vectors,
queued command capture, exact tick continuation, and compatibility rollback.
World saving and loading are complete on 2026-10-02 for the provisional typed-root
subset documented in [WORLD_SAVES.md](WORLD_SAVES.md): strict versioned documents,
game-owned nested-data codecs, exact continuation, validated atomic loading, and
synced temporary-file replacement. The public world-saving example and domain
tests validate fresh-runner continuation, queued commands/RNG, rejected edits,
filesystem replacement, and rollback. Arbitrary ECS capture remains deferred.
The immediate target is Milestone 3: CLI scenario and headless simulation commands.
Milestone 2 is complete for the supported subsets documented in the
[completion review](milestone-2-review.md); its APIs remain provisional.

```text
Square and isometric coordinate systems
→ tilemaps, layers, chunks, and picking
→ grid-based object placement
→ pathfinding and diagnostic visualization
→ simulation clock, pause, and speed control
→ headless simulation
→ scenarios, snapshots, and controlled random-number generation
→ world saving and loading
```

Native development watching, external dependency discovery, automatic restart,
packaging, and large-project scan/memory measurements remain deferred; see
[DEVELOPMENT_WORKFLOW.md](DEVELOPMENT_WORKFLOW.md).
