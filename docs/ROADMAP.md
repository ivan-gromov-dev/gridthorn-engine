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

## Milestone 2 — General-purpose 2D SDK

Started on 2026-08-30. The implemented provisional lifecycle foundation now
includes engine-owned game-state identifiers, an ordered state stack, scene
identifiers, and scene-owned entities. State changes requested by lifecycle
systems are applied after `Input`, giving fixed updates and presentation a
consistent active state for the entire host frame. Active-scene replacement
removes the exited scene's entities and runs `SceneTransition` construction
systems exactly once before fixed updates. Scene serialization remains planned.
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
