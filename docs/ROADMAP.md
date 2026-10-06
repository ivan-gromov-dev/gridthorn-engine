# Implementation Roadmap

This roadmap follows risk-reduction order rather than promised dates. Completed
capabilities remain provisional under [PUBLIC_API_POLICY.md](PUBLIC_API_POLICY.md).
Domain documents define the supported subsets and exclusions.

## Milestone 0 — Project and CLI foundation (complete)

- [x] Cargo workspace, MSRV, dependency boundaries, verification and CI.
- [x] Project generation, check/run commands and compatibility validation.
- [x] Architecture, runtime, determinism and release contracts.

## Milestone 1 — Runtime vertical slice (complete)

- [x] Application lifecycle, ordered schedules and fixed updates.
- [x] Native window/input, world access, camera, sprites and textures.
- [x] Generated project and timing overlay.

## Milestone 2 — General-purpose 2D SDK (complete)

- [x] Scenes/states, sprite batching/animation, runtime UI, audio and collision.
- [x] Asset dependencies/reload, reflection and scalar scene persistence.
- [x] CLI source watching/release builds and the Crystal Trail example.

See [RUNTIME_APIS.md](RUNTIME_APIS.md), [ASSETS.md](ASSETS.md),
[REFLECTION.md](REFLECTION.md) and [SCENES.md](SCENES.md).

## Milestone 3 — Gridthorn specialization (complete)

- [x] Square/isometric grids, tilemaps, placement and bounded pathfinding.
- [x] Simulation clock/control, headless scenarios, snapshots and named RNG.
- [x] Typed-root world saves, CLI simulation and the Timber Harbor example.

See [GRIDS.md](GRIDS.md), [SIMULATION.md](SIMULATION.md),
[SCENARIOS.md](SCENARIOS.md), [WORLD_SAVES.md](WORLD_SAVES.md)
and [CLI_SIMULATION.md](CLI_SIMULATION.md).

## Milestone 4 — Input, multilingual text and runtime UI (complete)

- [x] Desktop keys/pointer, Unicode/IME sessions and plain-text clipboard.
- [x] Explicit font assets, shaping/fallback/bidi, DPI rendering and localization.
- [x] Composition/controls, event routing, editing/focus, modal layers and transitions.
- [x] Public examples, headless tests and Windows native/manual acceptance.

See [INPUT.md](INPUT.md), [TEXT.md](TEXT.md), [LOCALIZATION.md](LOCALIZATION.md)
and [UI.md](UI.md). Linux/macOS native validation and broader device support
remain explicit limitations; no stable or universal platform guarantee is made.

## Milestone 4.5 — Performance review and optimization (complete)

Completed on 2026-10-05 for the available Windows workload matrix.

- [x] Review implemented domains and build/delivery footprint; apply focused fixes.
- [x] Preserve behavior, deterministic continuation and resource-lifetime contracts.
- [x] Record workload envelopes and remaining limits in [PERFORMANCE.md](PERFORMANCE.md).
- [x] Retain repeatable probes and compare CPU performance with the previous revision in CI.

Native acceptance includes measurement limitations. It does not establish
whole-engine frame-budget compliance, native DPI-2 timings or cross-platform performance.

## Milestone 5 — Desktop platform, devices and presentation controls

In progress; follows completed Milestones 4 and 4.5 and depends on their input and UI contracts. Platform
capabilities are explicit, optional and reported through engine-owned APIs.

- [x] Enumerate monitors, display modes, resolutions, refresh rates and DPI;
      report changes and disconnection with documented identifier lifetimes.
      Provisional OS DPI scaling and explicit-query connection changes are documented in
      [DISPLAYS.md](DISPLAYS.md); physical DPI and Linux/macOS native acceptance
      remain explicit limitations.
- [x] Window controls: size, resizing policy, placement, monitor selection,
      windowed/borderless/exclusive fullscreen where supported and applied-state feedback.
      Explicit requests, native confirmation, windowed/borderless controls and
      capability-gated exclusive are implemented. Windows exclusive resolution/
      refresh changes and monitor transfer are supported on the happy path;
      Linux/macOS native acceptance is deferred. See [WINDOWS.md](WINDOWS.md).
- [ ] Recover Windows exclusive-fullscreen mode-switch failures without panic,
      report only applied state, and restore desktop modes on transfer, exit and
      disconnection. Integrate compatible upstream winit recovery and verify
      rejected activation/restoration and queued teardown. Until then, native
      rejection may panic; exclusive fullscreen supports the happy path only.
- [x] Enumerate compatible graphics adapters, select an adapter at initialization
      or through an explicit restart/recreation contract, and report incompatibility.
      Provisional initialization selection, surface-specific inventory and typed
      diagnostics are implemented; see [GRAPHICS_ADAPTERS.md](GRAPHICS_ADAPTERS.md)
      for the restart contract and native coverage limits.
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

Milestone 5: explicit desktop platform/device capabilities and presentation
controls. Implement each increment with its public contract, supported-platform
limits, domain tests and an affected sibling example. Profiling/editor tooling
remains in Milestones 6 and 7.
