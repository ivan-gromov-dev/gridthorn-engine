# Gridthorn Vision

## Vision

Gridthorn will be a complete, modular 2D game engine and Rust SDK. A third-party
developer should be able to create, debug, test, and package a game without
working directly with low-level platform or graphics APIs.

The engine remains general-purpose: platformers, RPGs, puzzle games, arcade
games, and strategy games are all normal use cases. Its defining specialization
is tile-based and isometric worlds, deterministic simulation, large populations
of game objects, and tooling for management and tycoon games.

> Gridthorn is a modular, code-first 2D game engine and Rust SDK designed for
> tile-based, isometric, simulation, and management games.

## Audience

- Rust developers who want a cohesive 2D SDK rather than disconnected crates.
- Independent developers and small teams.
- Authors of games built around grids, large worlds, economies, and
  reproducible simulation.
- Developers of traditional 2D games who prefer a code-first workflow.

## The product

Gridthorn is more than a runtime. The complete product consists of:

1. **Engine runtime:** application lifecycle, world, rendering, input, audio,
   and platform integration.
2. **SDK:** public crates, APIs, plugins, data formats, and documentation.
3. **CLI:** project creation, development, validation, testing, building, and
   packaging.
4. **Development tools:** inspector, profiler, debug overlay, scenarios,
   snapshots, and replay.
5. **Editor:** a future visual frontend for existing SDK capabilities.
6. **Examples and showcases:** a small traditional 2D game and an isometric
   tycoon vertical slice that validate the public API.

## Product goals

- A new project displays its first result within five to ten minutes.
- A simple game does not require knowledge of the renderer or platform layer.
- Games only enable the modules they need.
- Simulation can run without a window or GPU.
- Bugs can be reproduced with a scenario, snapshot, or replay.
- Standard CLI commands cover the path from project creation to distribution.
- The public API is sufficient for games and official development tools.

## Early non-goals

Before the SDK has a stable vertical slice, we will not build:

- a Unity- or Unreal-scale feature set;
- visual scripting;
- a source-code editor or Git client;
- a marketplace or collaborative editing;
- general-purpose shader or animation graphs;
- a substantial 3D subsystem;
- game-code hot reload based on an unstable dynamic Rust ABI.

These limits protect the primary objective: a complete, approachable 2D SDK
validated by real games.

## Criteria for the first stable release

- A documented, versioned public API.
- A working create, run, debug, test, build, and package workflow.
- Supported Windows, Linux, and macOS desktop targets.
- Scenes, sprites, cameras, input, audio, UI, tilemaps, and basic 2D collision.
- Fixed-step and headless simulation.
- Validated assets with development-time hot reload.
- Inspection, profiling, and diagnostic visualization.
- Versioned scene formats and an explicit migration strategy.
- A CLI, practical guide, and focused examples.
- At least two complete showcases from different genres.
