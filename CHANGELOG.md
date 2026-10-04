# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Asset reload reuses committed source allocations for unchanged dependents,
  retaining dependency-first invalidation and atomic decode/publication rollback.
- World-save loading checks compatibility without cloning the live root and moves
  the decoded root into restoration. Public borrowed snapshot restoration retains
  its independent clone semantics.
- Asset/scene/save performance probes separate first-service loads, branching and
  error paths, file I/O, texture/game codecs, TOML and phase/workflow heap peaks;
  DHAT is a test-only dependency.
- Input buffers queue owned events and transfer their ordered queue into independent
  frame snapshots, avoiding full queue/payload copies. Replacement queues reserve
  the preceding frame's event count; held state, preedit and focus cancellation
  retain their existing semantics. Release burst measurements and domain regressions
  cover the change.
- Opt-in native window diagnostics now retain whole-process CPU accounting and
  wall intervals between completed redraws, including work on background threads.
  OS clock failures break pairing rather than merging frames; retention is bounded.
- Renderer textures are reused across frames and nonadjacent batches sharing a
  decoded asset. Assets absent from the next rendered frame are evicted, and
  unchanged textured geometry retains its GPU vertex buffers.
- UI layout reuses bounded shaped text across arrangement, editing geometry and
  paint within one pass, avoiding repeated shaping under large field workloads.
- Text raster snapshots share their construction vector without copying it;
  spare vector capacity remains owned by the immutable snapshot.
- Text services reuse bounded tinted glyph spans across raster calls, including
  short labels. Tint changes and `clear_raster_cache` release this cache; scratch
  storage is released before publishing each immutable raster snapshot.

- Text layout-cache line retention rises from 1024 to 4096 to admit measured
  narrow Japanese composition working sets, removing repeated warm shaping while
  preserving the existing entry/key/glyph limits and bounded LRU eviction.

- UI asset-font labels and preedit omit offscreen glyph draw spans using the new
  `TextSystem::rasterize_clipped` path, reducing measured long-field paint costs
  while preserving visible clipping and the full raster work safety limit.

- Asset-font UI field geometry now locates glyph cluster boundaries by binary
  search instead of scanning and copying all grapheme boundaries per glyph,
  preserving caret/selection positions while reducing measured long-field costs.

- The sibling multilingual workbench adds a manual long-field preedit/commit
  CPU probe for four scripts and synthetic DPI 1/2, reporting oversized layouts.

- Opt-in native window diagnostics now pair preparation callbacks with the next
  redraw and report bounded callback CPU sums and preparation counts; event-loop
  waiting and actual display intervals remain outside this measurement.

- Opt-in native window performance diagnostics report initial window DPI/extent
  and available monitor/refresh metadata for reference performance acquisitions.

- The sibling multilingual workbench adds isolated selection and injected preedit
  native smoke, preserving committed text with DPI tests and repeated measurements.

- The sibling multilingual workbench adds isolated static-window and animated
  window smoke cycles with layer/control tests and repeated native measurements.

- The sibling multilingual workbench adds isolated short-viewport wheel-scroll
  native smoke, with DPI/overflow tests and repeated four-locale measurements.

- The sibling multilingual workbench adds isolated catalog-switch native smoke
  with caption/editor regression coverage and repeated first-cycle/warm timings.

- The sibling multilingual workbench adds isolated slider-drag native smoke,
  with pointer capture/DPI regression coverage and repeated four-locale timings.

- The sibling classic_2d example loads complete assets beside its executable
  before using source assets, with headless/native relocation checks.

- Performance review inventories direct Windows DLL imports and staged package
  bytes, recording classic_2d's compile-time source asset-path portability limit.

- Performance review records independent empty-target classic_2d release builds,
  unchanged warm repeats, headless/native smoke and external asset footprint.

- Performance review separates generated-project empty-target release compilation
  from unchanged warm repeats and verifies the resulting headless executable.

- Performance review extends isolated release rebuild measurements to generic
  world, renderer and additive facade API edits, with compilation chains recorded.

- Performance review compares empty-target SDK release builds with default,
  four and two Cargo jobs; measured limits increase build time on the test machine.

- Performance review attributes cold Cargo unit durations and measures isolated
  release rebuilds after game-model and simulation implementation edits.

- Performance review records Windows release build baselines for CLI/SDK,
  generated-project and example warm builds, dependency closures and binary sizes.

- Performance review adds fixed-clock/exact-tick schedule, typed snapshot and
  direct/named RNG release probes with deterministic continuation assertions.

- Placement cell occupancy uses hash lookups while preserving ordered object
  iteration and deterministic conflict/transaction behavior; performance review
  adds placement and narrow-phase/all-pairs collision scaling probes.

- Pathfinding uses hash lookups for tentative costs and predecessors while retaining
  deterministic ordered frontier expansion, routes and diagnostics.

- The sibling classic_2d audio worker preserves the latest pause/resume state
  under effect-queue saturation through a coalescing atomic mailbox.
- Performance review records example-owned audio worker handoff and shutdown
  measurements, including bounded-queue loss under overload.
- Audio output reuses shared clip conversions within bounded command batches and
  removes completed voice handles when processing queues, including empty queues.

- Performance review adds isolated release audio control-side and runtime lifecycle
  callback scaling probes, with explicit mock backend and native-device limits.

- Asset reload replaces repeated graph scans with reverse-edge propagation and
  an ordered dependency-count queue, preserving deterministic dependency-first
  publication and failed-batch rollback.

- Large text raster requests reuse bounded glyph spans within one call, preserving
  font selection, ordered pixels, tint and DPI; short requests retain direct sampling.

- Performance review adds a public-example warm asset-font editing and overlapping
  layer probe across four scripts and DPI 1/2, with explicit workload limits.

- UI routing reuses batch-local input and pointer scopes, rebuilding after layer
  dismissal or pointer-layer/capture changes instead of filtering every event.

- UI layout clones and registered-layer input scopes share immutable prepared
  text geometry instead of copying field strings and caret/selection geometry.

- Registered-layer UI routing avoids copying render primitives and uses ordered
  ID sets for placement filtering; prepared text geometry and painter order remain intact.

- UI trees index node IDs through immutable child paths shared by clones,
  rebuilding on validated replacement to avoid repeated whole-tree searches.

- Plain-tree UI routing borrows unchanged immutable layout scopes instead of
  copying them per event; registered layers retain filtered input scopes.

- Opt-in runtime diagnostics record bounded schedule execution durations, including
  Input state/scene transitions and aggregate fixed updates.

- Opt-in text diagnostics report shaped-layout cache hits, misses, evictions and
  peak retained entry/key/glyph/line counts.

- Text services reuse shaped layouts through a bounded service-local LRU keyed
  by text and complete style; immutable clones share shaping and diagnostic data.

- Optional bounded text-service diagnostics distinguish shaping/layout and
  immutable raster-snapshot preparation, with operation counts and output sizes.

- Focused UI field painting reuses the layout's prepared text geometry for caret
  and selection decoration instead of shaping the field a second time.

- UI arrangement reuses identical asset-font text measurements within one layout
  pass, with bounded storage and no cache retained across tree/theme/font changes.
  Optional router layout diagnostics separate arrangement, text geometry and paint.

- Opt-in window diagnostics record bounded preparation, redraw and render-frame
  extraction callback CPU timings separately from event-loop waiting.

- Opt-in renderer diagnostics now report asynchronous GPU render-pass timestamps
  where supported, with bounded nonblocking readback and explicit sampling gaps.

- Renderer retains colored/UI geometry and its immutable GPU vertex buffer for
  unchanged snapshots, and reuses CPU vertex capacity when snapshots change.
  Surface reconfiguration recreates the cache; textured resources remain per-frame.

- Renderer UI geometry now appends nested clipped primitives into one vertex
  vector, preserving painter order without intermediate clip-subtree copies.
  Opt-in bounded native CPU diagnostics separate acquisition, geometry, resource
  preparation, encoding, submission and presentation calls.

- UI router layout prepares paint once after filtering and ordering managed
  layers, and prepares editing geometry only for visible layer placements.
  Closed layers still participate in sizing and arrangement.

### Added

- Provisional presentation `UiTween`, easing and interruptible `UiTransition`
  APIs for logical offset/size, linear RGBA colors and requested scrolling.
  Explicit unscaled frame time, local pause/resume, atomic application errors,
  public lifecycle tests and composed-controls animation workflows.

- Provisional UI popup/dialog layers with ordered painting, modal input/focus
  scopes, focus restoration and configurable Escape/outside-click dismissal.

- Provisional retained UI event router with clipped painter-order hit testing,
  keyboard/controller navigation hooks, focus and pointer capture, explicit
  event/continuous world-input consumption, Unicode grapheme selection/editing,
  caret/preedit paint and text-session/clipboard requests. Atomic routing and
  cancellation tests, public facade lifecycle tests and composed-controls workflow.

- Provisional retained UI composition with logical sizing, row/column/overlay
  layout, anchors, padding, constraints, nested rectangular clipping, clamped
  scrolling, themes and reusable labels/buttons/toggles/sliders/lists/text fields.
  Explicit validated control commands, immutable DPI-specific presentation,
  contextual errors, domain/facade tests and public composed-controls example.

- Provisional headless localization API with canonical locale/message IDs,
  validated Fluent catalog assets, explicit ordered fallback and atomic replacement,
  typed parameters, cardinal/ordinal/select rules, ICU decimal formatting,
  contextual diagnostics and a public four-language example.

- Repository review and diagnosis skills, domain context routing, conditional
  long-task checkpoints, and a guarded Markdown-only verification mode.
- Provisional validated font assets, isolated game-owned fallback fonts, advanced
  Unicode shaping and bidi, logical measurement/wrapping/alignment, antialiased
  DPI-specific text snapshots in ordered runtime UI, and native `WindowScaleFactor`.
  Fixture tests and the public multilingual example cover Cyrillic, Arabic and Japanese.
- Provisional Unicode text sessions independent of physical shortcuts, ordered
  commits and IME preedit/lifecycle/cancellation, UTF-8 cursor offsets, physical
  candidate-window anchors, and focus/suspend cancellation without automatic restart.
- Ordered plain-text clipboard read/write requests with caller correlation,
  Unicode results, typed focus/content/platform failures, and lazy native access.

### Fixed

- Generated-project CLI lifecycle verification uses cached workspace dependencies
  offline, avoiding registry timeout failures after the workspace checks pass.
- Release guidance now uses the full release range, the correct patch increment,
  compatibility-aware version selection, and selective lockfile inspection.
- Defer GPU surface configuration until a renderable frame, coalescing resize
  events and recovery requests. This avoids the reproduced NVIDIA/Vulkan native
  exception during shutdown before the first presentation.

## [0.4.0] - 2026-10-02

### Added

- Expanded the sibling Timber Harbor showcase with housed lumberjack/carpenter/porter
  jobs, fair warehouse dispatch, player-selected port resources, construction/hiring
  costs, pause/save/load menu, half speed and generated workforce/UI artwork.

- Provisional `WindowViewport` resource with actual physical creation/resize
  extents for windowed camera picking and screen-space UI.

- Provisional CLI scenario listing and dedicated headless simulation launch with
  explicit ticks/seeds, project configuration, release-profile selection,
  contextual diagnostics, Cargo command tests, and a public SDK example.

- Provisional versioned world saves over typed scenario roots, game-owned codecs,
  exact tick/command/RNG continuation, validated loading, atomic file replacement,
  typed diagnostics, and a public headless example.

- Provisional bounded deterministic weighted grid pathfinding, typed errors,
  expansion budgets, visited/frontier diagnostics, and a public SVG example.

- Provisional integer object footprints, deterministic exclusive grid occupancy,
  read-only placement preview, atomic placement/relocation, removal, typed
  diagnostics, and a headless public SDK example behind `grid`.

- Provisional sparse `TileMap<T>` with ordered layers, signed chunks, empty-chunk
  reclamation, square/isometric occupied-cell picking, validated orthographic
  cursor conversion, and a headless public SDK example behind `grid`.
- Provisional opt-in `gridthorn::grid` square/isometric coordinate projection,
  signed cell identities, checked inverse conversion, and a headless SDK example.

### Fixed

- Draw screen-space UI and timing diagnostics above textured world sprites.

## [0.3.0] - 2026-10-02

### Changed

- Opt-in native window diagnostics now pair preparation callbacks with the next
  redraw and report bounded callback CPU sums and preparation counts; event-loop
  waiting and actual display intervals remain outside this measurement.

- Opt-in native window performance diagnostics report initial window DPI/extent
  and available monitor/refresh metadata for reference performance acquisitions.

- The sibling multilingual workbench adds isolated selection and injected preedit
  native smoke, preserving committed text with DPI tests and repeated measurements.

- The sibling multilingual workbench adds isolated static-window and animated
  window smoke cycles with layer/control tests and repeated native measurements.

- The sibling multilingual workbench adds isolated short-viewport wheel-scroll
  native smoke, with DPI/overflow tests and repeated four-locale measurements.

- The sibling multilingual workbench adds isolated catalog-switch native smoke
  with caption/editor regression coverage and repeated first-cycle/warm timings.

- The sibling multilingual workbench adds isolated slider-drag native smoke,
  with pointer capture/DPI regression coverage and repeated four-locale timings.

- The sibling classic_2d example loads complete assets beside its executable
  before using source assets, with headless/native relocation checks.

- Performance review inventories direct Windows DLL imports and staged package
  bytes, recording classic_2d's compile-time source asset-path portability limit.

- Performance review records independent empty-target classic_2d release builds,
  unchanged warm repeats, headless/native smoke and external asset footprint.

- Performance review separates generated-project empty-target release compilation
  from unchanged warm repeats and verifies the resulting headless executable.

- Performance review extends isolated release rebuild measurements to generic
  world, renderer and additive facade API edits, with compilation chains recorded.

- Large text raster requests reuse bounded glyph spans within one call, preserving
  font selection, ordered pixels, tint and DPI; short requests retain direct sampling.

- Raised the MSRV and pinned local engine/example toolchains to Rust 1.99.0
  to align local Clippy diagnostics with current stable CI; updated the MSRV job
  and generated project manifest.

### Added

- Provisional `gridthorn check --watch` with configurable content polling of
  project-local compilation inputs, serialized Cargo checks, failure recovery,
  and preserved snapshots after scan failures.
- `gridthorn build` and `build --release` with compatibility validation and
  Cargo debug/release profile delegation.

- Provisional `gridthorn_scene` service with schema 1 TOML, explicit engine
  compatibility, registered scalar authoritative data, prepared transactional
  scene loading, migration hooks, and a headless public SDK usage example.
- World APIs for ordered scene entity enumeration, empty scene-owned entities,
  and inserting additional components without exposing ECS backend types.

- Provisional component/resource reflection with explicit stable type registration,
  scalar field metadata, validated read-only snapshots, and a headless SDK example.

- Provisional `AssetReloader` for background file polling and texture decoding,
  one outstanding request, explicit atomic frame-boundary publication, typed
  worker failures, and joined shutdown. The asset example now polls every
  250 ms off the frame thread and includes asynchronous recovery smoke coverage.
- Provisional `AssetId` and `AssetStore` APIs for raw sources and textures,
  validated dependency edges, deterministic invalidation, and atomic content
  polling with last-good-data recovery. A sibling `asset-reload` example
  demonstrates frame-boundary texture updates and a headless smoke workflow.
- Provisional engine-owned circle and axis-aligned box colliders with overlap
  and minimum-translation contact queries through the public SDK facade.

## [0.2.0] - 2026-08-30

### Added

- Provisional public application runtime with ordered `Startup`, `PollEvents`,
  `Input`, `FixedUpdate`, `Update`, `PostUpdate`, `Render`, and `Shutdown`
  schedules exposed through the `gridthorn` facade.
- Configurable fixed-step time accumulation with integer tick indices, bounded
  per-frame catch-up, observable overload, and preserved backlog.
- A provisional `WindowedApplication` that drives timed runtime frames from the
  native event loop, excludes suspended time, and guarantees runtime shutdown.
- Engine-owned keyboard and mouse events with held and frame-edge state,
  cursor position, focus-loss release, and native window-loop delivery.
- Ordered generic `GameCommandQueue` consumption, engine-owned exit requests,
  direct component reads, and a keyboard-controlled world entity example.
- An engine-owned orthographic `Camera2d`, colored `Sprite` presentation frame,
  and basic GPU pipeline used by the keyboard-controlled example.
- A provisional `TextureAsset` loader for PNG and PNM images plus textured
  sprite presentation through the public SDK facade.
- A compact screen-space timing overlay for host-frame duration, fixed work,
  accumulated lag, and fixed-step overload state.
- A generated Milestone 1 runtime project with native input, fixed updates, a
  controllable sprite, timing diagnostics, and a headless CI smoke path.

### Changed

- Opt-in native window diagnostics now pair preparation callbacks with the next
  redraw and report bounded callback CPU sums and preparation counts; event-loop
  waiting and actual display intervals remain outside this measurement.

- Opt-in native window performance diagnostics report initial window DPI/extent
  and available monitor/refresh metadata for reference performance acquisitions.

- The sibling multilingual workbench adds isolated selection and injected preedit
  native smoke, preserving committed text with DPI tests and repeated measurements.

- The sibling multilingual workbench adds isolated static-window and animated
  window smoke cycles with layer/control tests and repeated native measurements.

- The sibling multilingual workbench adds isolated short-viewport wheel-scroll
  native smoke, with DPI/overflow tests and repeated four-locale measurements.

- The sibling multilingual workbench adds isolated catalog-switch native smoke
  with caption/editor regression coverage and repeated first-cycle/warm timings.

- The sibling multilingual workbench adds isolated slider-drag native smoke,
  with pointer capture/DPI regression coverage and repeated four-locale timings.

- The sibling classic_2d example loads complete assets beside its executable
  before using source assets, with headless/native relocation checks.

- Performance review inventories direct Windows DLL imports and staged package
  bytes, recording classic_2d's compile-time source asset-path portability limit.

- Performance review records independent empty-target classic_2d release builds,
  unchanged warm repeats, headless/native smoke and external asset footprint.

- Performance review separates generated-project empty-target release compilation
  from unchanged warm repeats and verifies the resulting headless executable.

- Performance review extends isolated release rebuild measurements to generic
  world, renderer and additive facade API edits, with compilation chains recorded.

- Large text raster requests reuse bounded glyph spans within one call, preserving
  font selection, ordered pixels, tint and DPI; short requests retain direct sampling.

- `gridthorn check` now validates that the Cargo package name and Gridthorn
  dependency requirement agree with `gridthorn.toml` and the running CLI.
- CI now restores Cargo dependency artifacts and shares one target directory
  with CLI-generated project checks to avoid rebuilding the engine from scratch.

## [0.1.0] - 2026-08-28

### Added

- Cargo workspace with Rust 1.97.1 as the MSRV.
- Initial `gridthorn` SDK facade and version API.
- `gridthorn` CLI with `new`, `run`, `check`, and `--version`.
- Embedded minimal project template and generation/build tests.
- Formatting, lint, test, dependency-boundary, and CI checks.
- Scoped `AGENTS.md` guidance and domain-owned source/test organization.
- Matching Windows and macOS/Linux verification scripts plus three-platform CI
  with an aggregate `CI Success` gate.
- Repository-scoped `$implement` and `$release` skills for tested feature work
  and consistent release preparation.
- Provisional `gridthorn_app` and `gridthorn_render` crates with a documented
  window, GPU surface, resize, minimize, restore, and close lifecycle example.
- Provisional `gridthorn_world` ECS storage and explicit `Startup`,
  `FixedUpdate`, and `Update` schedules behind Gridthorn-owned APIs.
- A windowed `schedule-loop` example that drives ECS schedules through the
  Gridthorn application lifecycle.
- A `headless-schedule` example that runs the same fixed-update boundary without
  application, windowing, GPU, or renderer dependencies.
- Process-level and public-parser coverage for the complete CLI-generated
  project lifecycle, including contextual invalid-project diagnostics.
- Structured `component` fields across CLI, application, and renderer tracing,
  plus a CLI-driven diagnostics-flow example and renderer-to-app error test.
- A sibling `gridthorn-examples` repository convention with one independently
  owned directory per executable example.
- Engine-owned SplitMix64 streams and stable FNV-1a state fingerprint encoding
  in the provisional `gridthorn_simulation` crate.
- A deterministic-replay example covering repeatable fixed-step runs and
  changed-seed and changed-command controls.
- A completed Milestone 0 foundation review with clean-build, binary-size, and
  dependency-growth baselines plus explicit runtime deferrals.

### Changed

- Opt-in native window diagnostics now pair preparation callbacks with the next
  redraw and report bounded callback CPU sums and preparation counts; event-loop
  waiting and actual display intervals remain outside this measurement.

- Opt-in native window performance diagnostics report initial window DPI/extent
  and available monitor/refresh metadata for reference performance acquisitions.

- The sibling multilingual workbench adds isolated selection and injected preedit
  native smoke, preserving committed text with DPI tests and repeated measurements.

- The sibling multilingual workbench adds isolated static-window and animated
  window smoke cycles with layer/control tests and repeated native measurements.

- The sibling multilingual workbench adds isolated short-viewport wheel-scroll
  native smoke, with DPI/overflow tests and repeated four-locale measurements.

- The sibling multilingual workbench adds isolated catalog-switch native smoke
  with caption/editor regression coverage and repeated first-cycle/warm timings.

- The sibling multilingual workbench adds isolated slider-drag native smoke,
  with pointer capture/DPI regression coverage and repeated four-locale timings.

- The sibling classic_2d example loads complete assets beside its executable
  before using source assets, with headless/native relocation checks.

- Performance review inventories direct Windows DLL imports and staged package
  bytes, recording classic_2d's compile-time source asset-path portability limit.

- Performance review records independent empty-target classic_2d release builds,
  unchanged warm repeats, headless/native smoke and external asset footprint.

- Performance review separates generated-project empty-target release compilation
  from unchanged warm repeats and verifies the resulting headless executable.

- Performance review extends isolated release rebuild measurements to generic
  world, renderer and additive facade API edits, with compilation chains recorded.

- Large text raster requests reuse bounded glyph spans within one call, preserving
  font selection, ordered pixels, tint and DPI; short requests retain direct sampling.

- Raised the project MSRV to Rust 1.97.1 so current `bevy_ecs` and `wgpu`
  releases can be validated.
- Updated `bevy_ecs`, `pollster`, `thiserror`, `toml`, and `wgpu` to their
  current releases.
