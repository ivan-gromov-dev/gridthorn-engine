# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

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

- Raised the project MSRV to Rust 1.97.1 so current `bevy_ecs` and `wgpu`
  releases can be validated.
- Updated `bevy_ecs`, `pollster`, `thiserror`, `toml`, and `wgpu` to their
  current releases.
