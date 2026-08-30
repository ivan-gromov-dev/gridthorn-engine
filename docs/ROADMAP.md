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

## Milestone 1 — Runtime vertical slice

Started on 2026-08-30. The first implemented increment exposes the provisional
application lifecycle, ordered frame schedules, and fixed-time accumulation
through the SDK facade. The timed runtime is now integrated with the native
window loop, including suspend/resume and shutdown behavior. A documented
end-to-end runtime example remains before the lifecycle and time items are
complete.

- [ ] Application lifecycle and schedules.
- [ ] Time, `Update`, and fixed-step `FixedUpdate`.
- [ ] Window and keyboard/mouse input.
- [ ] Minimal world model through the Gridthorn API.
- [ ] Basic 2D renderer, camera, and sprite.
- [ ] Texture loading as an asset.
- [ ] Diagnostic timing overlay.
- [ ] Extend CLI checks with engine/project compatibility validation.
- [ ] Build and run one documented example through the CLI.

**Result:** `gridthorn new` to a controllable sprite and observable fixed tick in
one cohesive workflow.

## Milestone 2 — General-purpose 2D SDK

- [ ] Scenes and game states.
- [ ] Sprite batching and 2D animation.
- [ ] Text and basic runtime UI.
- [ ] Audio.
- [ ] Basic 2D collision.
- [ ] Asset dependencies and hot reload.
- [ ] Reflection for components and resources.
- [ ] Versioned scene serialization.
- [ ] Extend the CLI with development watching and release builds.
- [ ] Complete a small traditional 2D game.

**Result:** the SDK can build a small, complete traditional 2D game.

## Milestone 3 — Gridthorn specialization

The earlier Milestone 0 headless spike only validates the dependency boundary.
This milestone turns that prototype into a supported public simulation API with
documented behavior, diagnostics, and tests.

- [ ] Square and isometric coordinate systems.
- [ ] Tilemaps, layers, chunks, and picking.
- [ ] Grid-based object placement.
- [ ] Pathfinding and diagnostic visualization.
- [ ] Simulation clock, pause, and speed control.
- [ ] Headless simulation.
- [ ] Scenarios, snapshots, and controlled random-number generation.
- [ ] World saving and loading.
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

The immediate target spans Milestones 0 and 1:

```text
Install gridthorn CLI
→ create a Cargo project
→ run Game::new().run()
→ receive window and input events
→ spawn a world entity
→ render a camera and sprite
→ execute Update and FixedUpdate
→ inspect timing diagnostics
→ run the documented example through the CLI
```

Until this target is complete, a new major subsystem is added only when the
vertical slice requires it.
