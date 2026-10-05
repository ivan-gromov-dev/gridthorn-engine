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
punctuation. Milestone 4 now adds [font assets and multilingual text](TEXT.md);
higher-level [UI composition and controls](UI.md) are implemented provisionally
in Milestone 4 through explicit control commands.
Mouse-driven runtime buttons now consume the engine-owned frame input boundary,
report hover and held visuals, and emit one activation only after an inside
press completes with an inside release. Focus loss or dragging outside cancels
the pending activation; keyboard focus remains planned. Milestone 4 now supplies
reusable higher-level layout.
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

## Milestone 3 — Gridthorn specialization (complete)

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
- [x] Extend the CLI with scenario and headless simulation commands.
- [x] Complete an isometric tycoon vertical slice.

**Result:** a finished showcase validates Gridthorn's specialization.

Completed on 2026-10-02 after the maintainer confirmed that Timber Harbor is finished and working. APIs remain provisional.

The playable Timber Harbor `tycoon_slice` showcase now integrates all implemented
items with original art/audio, housed worker roles, production and transport,
resource-specific ports, a pause/save/load menu, snapshots and diagnostics. [Completion evidence](milestone-3-showcase.md) records
supported scope, verification, maintainer acceptance and explicit deferrals.

## Milestone 4 — Complete input, multilingual text and runtime UI APIs

Started on 2026-10-02. The first desktop input increment is implemented with
engine-owned keyboard/wheel events and native capture feedback; see [INPUT.md](INPUT.md).
Unicode/IME text sessions, plain-text clipboard and focus/suspend cancellation
are implemented provisionally on 2026-10-03, with domain/runtime tests.
Native validation limits are in [INPUT.md](INPUT.md).
Font assets, script fallback, multilingual shaping/bidi, measurement, wrapping and
DPI-specific rendering are implemented provisionally on 2026-10-03; see [TEXT.md](TEXT.md)
for tested Cyrillic/Arabic/Japanese coverage and explicit platform/performance deferrals.
Localization runtime APIs are implemented provisionally on 2026-10-03;
see [LOCALIZATION.md](LOCALIZATION.md) for validated Fluent catalogs, explicit
fallback, plural/select, decimal formatting and the four-language headless example.
UI composition, sizing, layout, anchoring, clipping, scrolling, styling and six
reusable controls are implemented provisionally on 2026-10-03; see [UI.md](UI.md)
for explicit value commands, domain/facade/renderer tests and the public
composed-controls headless/native smoke example. Event routing, clipped hit testing,
focus/navigation hooks, grapheme-aware text editing/selection, local pointer capture
and explicit world-input consumption are implemented provisionally on 2026-10-03;
see [UI.md](UI.md) for the supported editor/navigation subset and validation limits.
Presentation-property tweens and interruptible transitions are implemented
provisionally on 2026-10-03, with explicit unscaled frame duration independent of
simulation pause/speed; see [UI.md](UI.md) for property bounds, lifecycle policy,
domain/facade tests and the composed-controls animation workflow.
This milestone extends the narrow Milestone 1/2 subsets;
it does not change their completion status. Implement in the order below, with
each increment independently buildable, tested and documented. Detailed engine
and game ownership is defined in [RUNTIME_APIS.md](RUNTIME_APIS.md).

- [x] Complete desktop keyboard coverage: physical keys, logical keys, modifiers,
      repeat and ordered events; mouse wheel and pointer capture semantics.
- [x] Unicode text input, IME composition/commit/cancellation, clipboard access
      and focus-loss handling, separate from physical gameplay shortcuts.
- [x] Font assets, fallback, Unicode shaping, bidirectional text, measurement,
      wrapping and DPI-aware rendering, including Cyrillic and non-Latin scripts.
- [x] Localization runtime API: locale selection, message IDs, validated catalog
      assets, fallback, parameters, plural/select rules and locale-aware formatting.
- [x] UI composition, sizing, layout, anchoring, clipping, scrolling, styling and
      reusable controls: labels, buttons, toggles, sliders, lists and text fields.
- [x] UI event routing, hit testing, keyboard/controller navigation hooks, focus,
      text editing/selection and pointer capture with explicit world-input consumption.
- [x] Context menus, popups and dialogs: ordered layers, modal scopes, focus
      restoration, configurable Escape/outside-click dismissal and input blocking.
- [x] Presentation animation primitives for UI properties and transitions,
      independent of authoritative simulation time.
- [ ] Validate public APIs through sibling examples of multilingual text editing,
      composed controls and nested modal/context windows; include headless behavior
      tests and native rendering/IME checks with a documented language/platform matrix.
- [ ] Complete Milestone 4.5 performance review and native workbench acceptance
      before closing this milestone.

**Result:** game code can compose a multilingual management-game interface using
public APIs without implementing text shaping, input routing or window stacking.
Game-specific screens, research trees, settings policies and translation authoring
workflows remain game-owned. No translation-management desktop service is required.

## Milestone 4.5 — Engine performance review and optimization

Started on 2026-10-03 with a reproducible workbench CPU workload and initial
UI/text preparation review; see [the checkpoint](work-in-progress/milestone-4-5.md).
The first focused optimizations remove discarded router paint/hidden-field
geometry and intermediate clipped-renderer geometry copies.
Further work retains unchanged colored/UI GPU buffers and reuses CPU vertex
capacity, with uploaded-byte and host present-call cadence diagnostics. Release CPU
before/after samples and opt-in native renderer diagnostics are recorded in the
checkpoint. Native CPU/GPU/display review is accepted by the maintainer with
documented measurement limits; the remaining domain reviews are outstanding.
The review follows poor native multilingual-workbench performance,
including after example-side layout reuse. It is the immediate target and a
required gate before Milestone 4 closes or Milestone 5 starts. Review all implemented
engine domains; fix measured bottlenecks in the order below. Measurement protocol,
workloads, ownership and acceptance criteria are in
[the performance review plan](PERFORMANCE_REVIEW.md).

Parent items remain open until all their substeps are complete. Checked substeps
record implemented and verified increments; measurements and limitations are in
[the checkpoint](work-in-progress/milestone-4-5.md).

- [ ] Establish reproducible release/debug baselines, reference hardware, workload
      sizes and budgets; distinguish cold/warm work and CPU/GPU/present costs.
  - [x] Add the workbench CPU workload for four locales, DPI 1/2 and 1000×800
        logical pixels; record debug and repeated release samples with warmups,
        percentiles and separate construction/first-prepare observations.
  - [x] Record compiler/target, CPU, GPU/backend/driver, native surface extent
        and configured present mode on the available Windows host.
  - [x] Add opt-in native window/current-monitor metadata; record repeated DPI 1
        display extent/refresh and power-plan observations on the Windows host.
  - [ ] Complete reference metadata with native DPI, display refresh and power
        conditions; establish remaining domain workloads and budgets.
  - [x] Accept recorded native CPU/GPU/display baselines with documented coverage
        and clock-precision limits (maintainer disposition, 2026-10-04).
- [x] Complete the native workbench performance review by maintainer acceptance
      of recorded measurements and manual interaction/DPI/clipboard/OS IME testing
      (2026-10-04); this is not verification of the entire quantitative matrix.
  - [x] Add bounded opt-in native renderer CPU timings, vertex/upload counters,
        retained vertex-capacity counters and host present-call cadence.
  - [x] Measure repeated native release smoke runs combining locale changes,
        nested windows and animation; distinguish cache hits and misses.
  - [x] Add optional asynchronous render-pass GPU timestamps with availability,
        skipped/error/pending reporting and bounded collection.
  - [x] Add isolated idle and bounded injected text-editing smoke modes; measure
        all four locales at native DPI 1 with separate CPU/GPU-pass samples.
    - [x] Add bounded native preparation/redraw CPU callback diagnostics, including
          runtime schedules and render extraction; keep input/waiting exclusions explicit.
    - [x] Pair completed preparations with the next redraw; test bounded accumulation
          and record repeated Japanese idle callback sums with timing exclusions.
    - [x] Measure paired callback sums for all nine native isolation modes across
          four locales at DPI 1, twice each; retain blocking/display exclusions.
  - [x] Add isolated injected slider-drag smoke with capture-release/DPI tests;
        measure two native DPI 1 release runs for each locale.
  - [x] Add isolated catalog-switch smoke with editor-preservation tests; measure
        repeated native DPI 1 cycles and separate first-cycle layout observations.
  - [x] Add isolated short-viewport wheel-scroll smoke with DPI/overflow tests;
        measure repeated native DPI 1 runs for all four locales.
  - [x] Add matching static-window and host-time-animation smoke scripts with
        layer/preservation tests; measure two native DPI 1 runs per mode/locale.
  - [x] Add isolated selection and injected preedit smoke with preservation/DPI
        tests; measure two native DPI 1 runs per mode/locale.
  - [x] Accept recorded isolated interaction workloads and maintainer manual
        testing at DPI 1/2; unmeasured matrix cells remain documented limitations.
  - [x] Close CPU/GPU/display review by maintainer decision with existing GPU/display
        evidence and coarse whole-process CPU accounting. Precise per-frame CPU
        attribution and the complete quantitative matrix are unverified follow-ups,
        not blockers for this accepted gate.
- [x] Review and resolve measured CPU text/UI bottlenecks within recorded workload limits: shaping/rasterization, invalidation,
      layout/paint duplication, hidden-layer work, routing and allocations.
  - [x] Remove discarded router paint and closed-layer editing geometry;
        preserve sizing/arrangement and verify clipping, DPI and text-session recovery.
  - [x] Repeat release CPU measurements after the single-paint optimization.
    - [x] Add bounded native workbench input-phase timings to distinguish routing,
          localized-preview effects and cached/dirty layout preparation.
    - [x] Separate router arrangement/text-geometry/paint costs and reuse repeated
          asset-font measurements within a bounded single arrangement pass.
    - [x] Reuse prepared field geometry for focused caret/selection painting;
          preserve preedit preparation and native text-anchor behavior.
    - [x] Add bounded text-service layout/rasterize diagnostics with independent
          sample limits, successful-call totals and input/output size counters.
    - [x] Reuse shaped layouts within each font service using a bounded text/style
          LRU; verify style changes, font-owner isolation, eviction and live snapshots.
    - [x] Add cache hit/miss/eviction and peak-retention diagnostics; verify that
          unowned backend buffers are released on eviction.
    - [x] Reuse bounded request-local glyph raster spans for large layouts; preserve exact pixel output/tint/fractional DPI and record repeated before/after samples.
  - [x] Attribute expanded font preparation to repeated shaping, field geometry
        and paint; share bounded prepared text across a UI pass and tinted glyph
        spans across raster calls, and remove raster snapshot construction copies.
  - [x] Remeasure long-field preedit/commit, overlapping routing, closed-layer sizing,
        Japanese primary/fallback shaping and raster storage/release; record supported
        limits and concrete backend/incremental-layout follow-ups in the review.
        Native whole-frame/DPI/OS-IME acceptance remains in the gates below.
- [x] Review and optimize measured rendering/presentation bottlenecks: text/sprite
      geometry, clipping, batching, GPU uploads, resource lifetime and frame pacing.
  - [x] Remove intermediate nested-clip geometry copies; verify sibling isolation
        and painter order, and record repeated native CPU before/after samples.
  - [x] Retain unchanged colored/UI geometry and GPU buffers, reuse CPU vertex
        capacity on invalidation, and verify camera/sprite/UI/overlay/resize changes.
  - [x] Measure cache hits/misses, uploaded bytes and retained-capacity tradeoffs
        in repeated native release smoke runs.
  - [x] Review sprite/text batching, dirty-frame uploads, textured-resource lifetime
        and GPU execution across workbench, Crystal Trail and Timber Harbor.
  - [x] Measure recorded Windows/DPI 1 frame-pacing/presentation workloads with
        repeated native GPU/display capture and longer warmed confirmation runs;
        retain the existing presentation policy after an inconclusive queue-depth
        experiment. Native DPI 2 and complete interaction acceptance remain in
        the baseline, workload and final acceptance gates above/below.
- [x] Review runtime/world/input/localization overhead and scaling with increasing
      entity/control/event counts; record measurements and each domain's disposition.
  - [x] Add bounded runtime schedule diagnostics and record repeated native Japanese
        idle/editing samples with explicit wall-clock and phase-index limits.
  - [x] Measure empty-runtime dispatch with 0/1/32 no-op systems per stage and
        0/1/8 fixed ticks; record isolated native render-frame extraction timings.
  - [x] Measure homogeneous resident-world/fixed-traversal scaling through 100,000
        entities and synthetic input publication through 16,384 events per frame.
  - [x] Measure flat bitmap-button layout/routing through 1024 controls and warm
        four-locale formatting with catalogs through 4096 messages.
  - [x] Borrow unchanged plain-tree routing scopes; verify layer transitions and
        record repeated before/after pointer routing samples.
  - [x] Index UI node reads/commands by validated child paths; verify replacement,
        clone isolation and depth/node limits, and measure construction tradeoffs.
    - [x] Measure registered closed/open/modal text-field routing through 1024 fields;
          omit input-scope paint copies and replace linear layer membership scans,
          with repeated before/after samples and detached-layout regression coverage.
    - [x] Share immutable prepared field geometry across layout clones/input scopes;
          verify detached snapshot lifetime/release and repeat layered routing measurements.
    - [x] Reuse batch-local input/pointer scopes with layer-dismissal and capture
          invalidation; verify transitions/atomic rejection and remeasure layered routing.
    - [x] Measure warm asset-font preedit/commit paint and alternating pointer scopes
          across three overlapping layers, four scripts and DPI 1/2 through public APIs.
    - [x] Measure mixed single-component populations/scene churn and fresh-service localization publication; record repeated samples and workload limits.
    - [x] Measure repeated long/unique text and layout-cache pressure, attribute Japanese primary/fallback costs, and measure expanded bitmap layers through 64 roots/1024 fields.
    - [x] Measure long-field asset-font preedit/commit cycles for four content scripts
          at synthetic DPI 1/2; record repeated costs and rejected largest layouts.
    - [x] Attribute long-field UI/text phases and replace per-glyph full-boundary
          scans with indexed ranges; verify cluster/RTL offsets and remeasure.
    - [x] Cull offscreen asset-font glyph draw spans using actual ink bounds and
          effective clips; preserve raster limits/visible output and remeasure.
    - [x] Attribute narrow Japanese composition misses to line-budget bypass;
          calibrate bounded line retention, verify eviction/release and remeasure.
    - [x] Measure retained text-cache diagnostic capacity and separate-process
          private/resident memory through narrow layouts, pressure and release.
    - [x] Separate fresh-service Japanese layout, warm-font unique misses, hits,
          first/repeated raster calls and retained clipped/full output storage.
    - [x] Measure raster construction-vector and glyph-span capacities; release
          request-local glyph spans before snapshot construction and verify output.
    - [x] Measure fresh-service complex localization validation/publication with
          reference chains, nested selectors/numbers and rejected cyclic candidates.
    - [x] Measure expanded asset-font layout/paint through64 overlapping layers
          and1024 unique fields, four scripts and synthetic DPI1/2.
    - [x] Attribute long Japanese fallback shaping, raster output/span/image storage,
          long-field editing and expanded font layout/routing; fix duplicate work,
          verify output/lifetimes and record measured limits.
    - [x] Measure broader mixed ECS/component churn and attribute larger input bursts.
  - [x] Record runtime, world, input and localization dispositions and remeasure fixes.
- [x] Review assets/reload, scenes/saves, audio and platform lifecycle for I/O,
      memory peaks, worker/publication costs and frame stalls.
  - [x] Measure asset reload, scene/world-save I/O, publication and memory peaks.
    - [x] Measure warm Windows reload/scalar-scene/typed-save scaling and separate process-resident peaks; optimize reverse-chain invalidation/order and repeat affected samples.
    - [x] Complete first-service/fresh-process, branching/error-path workloads,
          phase/workflow heap peaks and codec/serializer attribution; fix source/root
          copies, remeasure and record Windows envelopes and physical-cold-storage limits.
  - [x] Measure audio command/worker costs and platform suspend/resume/shutdown.
    - [x] Measure mock audio queue/output control and synthetic runtime lifecycle
          callbacks across clip, voice and shutdown-system counts on Windows.
    - [x] Attribute repeated PCM conversion; bound batch-local frame reuse, release
          completed handles and remeasure the affected mock workloads.
    - [x] Measure example worker handoff, native device/mixer latency, long-lived
          output memory and platform lifecycle costs.
      - [x] Measure classic_2d headless worker submission/shutdown and bounded
            transport overload; record lost pause requests as a reliability follow-up.
      - [x] Preserve the latest pause/resume state under full example queues;
            verify saturation, idle wake and stale notifications, and remeasure.
      - [x] Measure native mixer/control and software loopback latency, repeated-output memory and
            deferred release; verify explicit audio sleep/wake and native window
            shutdown on Windows (2026-10-05). Acoustic latency, hours-long sessions
            and automatic power/device integration have concrete follow-ups in
            [the review](PERFORMANCE_REVIEW.md#native-audio-and-windows-lifecycle-disposition--2026-10-05).
  - [x] Record each domain's disposition and remeasure any fixes.
    - [x] Record assets/scenes/saves dispositions and repeat affected CPU/heap
          measurements; preserve rollback, publication and durable replacement.
- [ ] Review fixed simulation, grids, pathfinding/placement, collision and
      snapshots for scaling, allocation costs and preserved determinism.
  - [ ] Measure fixed ticks, grid/pathfinding/placement and collision scaling.
    - [x] Measure fixed-clock normal/catch-up/paused arithmetic and exact-tick
          orchestration through 256 scalar fixed systems on Windows release.
    - [x] Measure open/weighted/unreachable/budgeted pathfinding through 512x512;
          optimize cost/predecessor lookups and verify deterministic diagnostics.
    - [x] Measure sparse placement through 65536 objects and narrow-phase/all-pairs
          collision batches; optimize cell lookups while preserving transactions.
  - [ ] Measure snapshots/RNG and allocation costs; verify determinism after fixes.
    - [x] Measure typed Vec-root capture/clone/restore through 262144 values and
          named/direct RNG through 1024 streams; verify continuation and stream states.
  - [ ] Record each domain's disposition and supported workload limits.
- [ ] Measure cold/warm build time, dependency footprint and binary size; address
      measured regressions without introducing unrelated subsystems.
  - [ ] Run controlled cold/warm engine, examples and generated-project builds.
    - [x] Record empty-target CLI/SDK release builds and warm repeats, plus
          dependency-primed generated-project and existing-cache example builds.
    - [x] Measure a generated project with an independent empty build target and
          two unchanged warm repeats; verify the resulting headless smoke.
    - [x] Measure classic_2d with an independent empty target and two unchanged
          warm repeats; verify headless/native smoke and record external asset bytes.
    - [x] Inspect cold Cargo unit timings and measure game-model/simulation edits
          in isolated source copies with dependency invalidation recorded.
    - [x] Extend isolated release edit measurements to generic world, renderer and
          additive facade API changes; record compilation chains and restore sources.
    - [x] Compare default/4/2-job empty-target SDK release builds and retain
          existing jobs policy based on the measured Windows results.
  - [ ] Record dependency footprint/binary sizes and resolve measured regressions.
    - [x] Audit direct Windows DLL imports and staged executable/asset bytes;
          verify local runs and record the example's source-path relocation limit.
    - [x] Resolve classic_2d assets beside the executable with source fallback;
          verify headless/native relocation while the copied source path is absent.
    - [x] Record Windows CLI/default-SDK dependency closures and CLI/template/
          classic_2d release executable sizes without changing release profiles.
- [ ] Add repeatable performance workloads/regression checks, rerun the complete
      matrix and repository verification, and document before/after results and limits.
  - [x] Add workbench CPU/native measurement entrypoints and behavior regression
        tests for completed UI/renderer fixes; record before/after samples and limits.
  - [x] Run full repository verification and affected native smoke after each
        completed optimization increment.
  - [ ] Complete workloads/regression checks for the remaining domains and rerun
        the entire performance matrix and final repository verification.
- [ ] Complete the Milestone 4.5 review before resuming Milestone 4 closure;
      native performance acceptance is recorded below with measurement limits.
  - [x] Accept the native workbench performance gate with recorded CPU/GPU/display
        evidence and explicit measurement limitations (maintainer decision,
        2026-10-04); full-matrix budget compliance is not established.
  - [x] Obtain maintainer manual acceptance of the 1000×800 logical-pixel
        window criterion at DPI 1/2 (2026-10-04, maintainer-reported testing).
        Manual acceptance is recorded separately from performance measurements.
  - [x] Obtain maintainer manual acceptance of native clipboard and OS IME
        behavior (2026-10-04, maintainer-confirmed prior manual testing).
  - [x] Obtain maintainer native acceptance with the documented measurement limits.
  - [ ] Publish the Milestone 4.5 completion review after the remaining domain gates.

**Result:** implemented engine capabilities have measured performance envelopes,
the multilingual UI is usable within its recorded frame budget, and remaining
limits have concrete evidence. The professional profiling/editor tooling remains
in Milestone 6; this milestone owns measurement and focused engine improvements.

## Milestone 5 — Desktop platform, devices and presentation controls

Planned; follows Milestone 4 closure, including its Milestone 4.5 performance gate,
and depends on the input and UI contracts from Milestone 4. Platform
capabilities are explicit, optional and reported through engine-owned APIs.

- [ ] Enumerate monitors, display modes, resolutions, refresh rates and DPI;
      report changes and disconnection with documented identifier lifetimes.
- [ ] Window controls: size, resizing policy, placement, monitor selection,
      windowed/borderless/exclusive fullscreen where supported and applied-state feedback.
- [ ] Enumerate compatible graphics adapters, select an adapter at initialization
      or through an explicit restart/recreation contract, and report incompatibility.
- [ ] Presentation controls: supported VSync/present modes and frame-rate caps,
      independent of fixed simulation ticks and monitor refresh rate.
- [ ] Controller discovery, buttons/axes, connection changes, dead-zone primitives,
      supported feedback and device identity; integrate generic UI navigation.
- [ ] Input-device discovery/selection where the OS supports it, capability
      reporting for aggregate keyboard/mouse input and configurable action bindings.
- [ ] Enumerate/select audio output and capture devices; opt-in capture API,
      output switching, default-device changes and device-loss recovery.
- [ ] Master/category audio buses, mute and gain controls without game-specific
      category names or settings-screen policies.
- [ ] Platform user-data/config/cache directory discovery and clipboard integration
      with contextual errors; preserve the existing game-owned save-codec boundary.
- [ ] Validate APIs through a sibling settings/device example and Windows/Linux/
      macOS capability matrix covering unavailable devices, unsupported modes,
      DPI/monitor changes and safe recovery. Measure pacing separately from simulation.

**Result:** a game can build its own settings UI using public capability queries
and validated platform operations. Profiles, save-slot management, settings
persistence, confirmation countdowns and fallback preferences remain game logic.
Unsupported platform features must produce explicit capability results or typed
errors rather than an implied universal hardware guarantee.

## Milestone 6 — Professional debugging workflow

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

## Milestone 7 — Minimal Gridthorn Editor

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

Repository AI workflows and context routing are documented in
[AI_WORKFLOW.md](AI_WORKFLOW.md). Token usage and workflow latency improvements
require equivalent-task measurements; no savings are assumed from instruction
size alone.

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
CLI scenario and headless simulation commands are complete on 2026-10-02 for the
provisional game-process subset documented in [CLI_SIMULATION.md](CLI_SIMULATION.md).
Project declarations, dedicated binary routing, explicit ticks/seeds, and failure
diagnostics are covered by command tests and the public scenarios-snapshots example.
Milestone 3 is complete following maintainer acceptance on 2026-10-02; see
[the completion review](milestone-3-showcase.md). In Milestone 4,
desktop keyboard/pointer and Unicode/IME/clipboard input are implemented;
font assets, fallback and multilingual shaping/rendering are implemented provisionally.
Localization runtime APIs are implemented provisionally with validated catalogs,
explicit fallback and locale-aware decimal formatting. UI composition and controls
are implemented provisionally with explicit value commands, ordered event routing,
hit testing, focus/navigation hooks, text editing/selection, local capture and
explicit world-input consumption. Context menus, popups and modal dialogs are
implemented provisionally with ordered layers, modal scopes, focus restoration
and configurable dismissal. Presentation transitions are implemented provisionally.
The integrated sibling multilingual-workbench example combines localization, editing,
controls and nested windows; native interactive IME behavior has maintainer manual acceptance.
The immediate target remains Milestone 4.5. Native workbench acceptance, text/UI,
rendering, runtime/world/input/localization and assets/scenes/saves reviews are
complete within their recorded limits. Audio/platform lifecycle review is also
complete on 2026-10-05 for native control/mixer progress, repeated-output memory,
deferred release, explicit manual sleep/wake and orderly native shutdown;
acoustic latency and automatic power/device integration remain explicit follow-ups.
Next are simulation/grid/collision/snapshot domain closure and the other domain gates
in the order above. Milestone 4 remains open until the full performance review
closes; native language/IME acceptance is already recorded. See
[PERFORMANCE_REVIEW.md](PERFORMANCE_REVIEW.md).
Milestone 5 adds desktop platform/device controls. Professional debugging and
the editor are deferred to Milestones 6 and 7 respectively.
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
