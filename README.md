# Gridthorn Engine

Gridthorn is a modular, code-first game engine and Rust SDK for building 2D
games. It is suitable for traditional 2D genres while providing a particularly
strong foundation for tile-based, isometric, management, tycoon, and
simulation-heavy games.

The project has completed Milestones 1–3: the runtime vertical slice,
general-purpose 2D SDK and Gridthorn specialization, with provisional APIs
validated by Crystal Trail and Timber Harbor. It has a
reproducible Cargo workspace, a working CLI, native window and input handling,
fixed-step world updates, sprite presentation, texture loading, and runtime
timing diagnostics. A visual editor will later be built on the same public APIs
and development protocol.

[Milestone 2 review](docs/milestone-2-review.md) records completion evidence and
deferred platform and performance validation.

## Principles

- A clear and stable Rust API for game developers.
- A modular core with no genre-specific game logic.
- First-class frame-based and fixed-step updates.
- A deterministic simulation layer separated from presentation.
- Data-driven content and asset hot reload.
- Headless simulation and automated testing.
- Debugging, inspection, and profiling tools as part of the SDK.
- A future editor that uses the same APIs as third-party tools and games.

## Project documentation

- [Product vision and scope](docs/VISION.md)
- [Architecture direction](docs/ARCHITECTURE.md)
- [Game development workflow](docs/DEVELOPMENT_WORKFLOW.md)
- [Tools and libraries](docs/TECHNOLOGY.md)
- [Implementation roadmap](docs/ROADMAP.md)
- [Workspace dependency boundaries](docs/DEPENDENCY_BOUNDARIES.md)
- [Public API and release policy](docs/PUBLIC_API_POLICY.md)
- [UI composition and controls](docs/UI.md)
- [Project terminology](docs/GLOSSARY.md)
- [Architecture Decision Records](docs/adr/README.md)
- [Contributor guide](CONTRIBUTING.md)

## Status

Milestones 1–3 are complete for their documented supported subsets. Commands and Rust snippets
outside the section below still describe target developer experience unless
they are explicitly marked as implemented.

| Area                               | Status                                                                                                     |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Product and architecture direction | Design baseline                                                                                            |
| Milestone 0 foundation             | Complete; runtime deferrals recorded                                                                       |
| Cargo workspace and CI             | Implemented foundation                                                                                     |
| CLI                                | Project commands, compatibility checks, source watch, release builds, scenario listing and headless launch |
| Public SDK facade                  | Version API plus provisional lifecycle and schedule APIs                                                   |
| App and renderer internals         | Window/surface lifecycle and basic colored-sprite pipeline implemented                                     |
| Keyboard and mouse input           | Provisional [desktop input](docs/INPUT.md): physical/logical keys, modifiers, repeat, ordered events, wheel, capture, Unicode/IME text sessions and plain-text clipboard |
| World and schedules                | ECS lifecycle schedule spike implemented                                                                   |
| Game commands and world control    | Ordered commands and controllable entity example implemented                                               |
| Texture assets                     | PNG/PNM decoding and textured-sprite presentation implemented                                              |
| Asset dependencies and hot reload  | Implemented provisional path IDs, dependencies, background reload and atomic frame commits                 |
| Headless simulation                | Provisional exact-tick SDK runner, lifecycle, exit and repeatability implemented                           |
| Cross-layer diagnostics            | CLI/app/renderer tracing spike implemented                                                                 |
| Runtime timing overlay             | Frame time, fixed work, lag, and overload bars implemented                                                 |
| Deterministic fixed-step replay    | Named seeded RNG and explicit state fingerprint API; replay files planned                                  |
| Runtime and public SDK             | Milestones 1–3 complete; APIs remain provisional                                                              |
| Scenes and game states             | Provisional state stack and atomic scene-owned entity switching                                            |
| Sprite batching and animation      | Provisional ordered texture batches and sprite-sheet playback                                              |
| UI property animation              | Provisional frame-time tweens and interruptible offset, size, color and scroll transitions; independent of simulation pause/speed |
| Text and runtime UI                | Provisional [multilingual text](docs/TEXT.md) and [UI composition/controls](docs/UI.md): sizing, layout, anchors, clipping, scrolling, themes and six reusable controls |
| Audio                              | Provisional PCM16 WAV assets, queue, and opt-in Kira output adapter                                        |
| Component/resource reflection      | Provisional explicit registration, scalar metadata and read-only snapshots                                 |
| Versioned scene serialization      | Provisional schema 1 TOML, validated prepared loading and migration hooks                                  |
| Basic 2D collision                 | Provisional circle/AABB overlap and minimum-translation queries                                            |
| Grid coordinates                   | Provisional opt-in square/isometric projection and inverse conversion                                      |
| Tilemaps                           | Provisional sparse layers/chunks and camera-aware occupied-cell picking                                    |
| Grid-based object placement        | Provisional integer footprints, exclusive occupancy, atomic moves and removal                              |
| Pathfinding and diagnostics        | Provisional deterministic four-neighbor weighted search, budgets, visited/frontier data and SVG example    |
| Scenarios and snapshots            | Provisional typed authoritative roots, exact-tick restoration and compatibility checks                     |
| World saving and loading           | Provisional versioned typed-root saves, game codecs, validated loading and atomic file replacement         |
| Simulation clock                   | Provisional rational speed, pause/resume, preserved backlog and explicit stepping                          |
| Current source release             | `0.2.0`                                                                                                    |
| Stable API                         | Not available; APIs remain pre-1.0 and provisional                                                         |

Milestone 3 is complete: provisional square and isometric coordinates are
implemented behind the `grid` feature; see [the grid contract](docs/GRIDS.md).
Sparse tilemaps, layers, chunks, picking, and grid-based object placement are
also implemented, along with bounded weighted pathfinding and SVG diagnostics.
Simulation clock, pause, and speed control are implemented; see the
[simulation contract](docs/SIMULATION.md). Headless simulation is implemented through `HeadlessSimulation`. Typed scenarios, in-memory snapshots, and named seeded RNG streams are implemented;
see the [scenario contract](docs/SCENARIOS.md). World saving and loading are implemented
for the typed authoritative-root subset; see [the save contract](docs/WORLD_SAVES.md).
CLI scenario listing and headless simulation launch are implemented through a
dedicated game-owned binary; see [the CLI simulation contract](docs/CLI_SIMULATION.md).
The playable Timber Harbor `tycoon_slice` now integrates these features with
Milestone 2 presentation, UI, audio, reflection, scene persistence and asset reload.
Its workforce economy includes housing, multi-tick production, resource-specific
ports and a pause/save/load menu with generated UI artwork.
The [Milestone 3 completion review](docs/milestone-3-showcase.md) records scope,
verification, deferrals and maintainer acceptance on 2026-10-02. The next phase
is Milestone 4.5: an [engine performance review](docs/PERFORMANCE_REVIEW.md)
and measured optimization of implemented capabilities, starting with multilingual
UI/text and rendering. Opt-in bounded renderer/GPU-pass and window
preparation/redraw diagnostics are documented in the performance review,
with measured workload evidence and limits.
Milestone 4 remains open until this performance gate and
native language/IME acceptance finish, followed by Milestone 5 desktop
platform/device controls. The debugging workflow and
editor move to Milestones 6 and 7. Desktop input, Unicode/IME/clipboard and
multilingual font rendering and [runtime localization](docs/LOCALIZATION.md) are
implemented provisionally. [UI composition and controls](docs/UI.md) are implemented
with explicit value commands, ordered event routing, focus/navigation, grapheme-aware
editing/selection and pointer capture with explicit world-input consumption.
Ordered popup/dialog layers, modal scopes, focus restoration and configurable
Escape/outside-click dismissal are implemented provisionally.
Remaining runtime UI/device additions are planned;
see [the runtime API scope](docs/RUNTIME_APIS.md) and [roadmap](docs/ROADMAP.md).

Run the public composed UI example (use `--headless` for resize/DPI/control checks):

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_composed_controls --locked
```

Run the public headless localization example (English, Russian, Arabic and Japanese):

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_localization --locked
```

Run the public multilingual text example (Cyrillic, Arabic bidi and Japanese):

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_multilingual_text
```

```console
cargo run -p gridthorn_cli -- scenario list ../gridthorn-examples/scenarios-snapshots
cargo run -p gridthorn_cli -- simulate ../gridthorn-examples/scenarios-snapshots --scenario economy --ticks 100 --seed 42
```

Run the public world save/load continuation and rollback example:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_world_saving
```

Run the public scenario and snapshot continuation example:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_scenarios_snapshots
```

Run the public headless simulation example (no window or GPU initialization):

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_headless_simulation
```

Run the public SDK clock example:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_simulation_clock
```

Run the headless placement example through the public SDK:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_grid_placement
```

Run the headless tilemap example through the public SDK:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_tilemap_basics
```

Local development and the MSRV use Rust 1.99.0; rustup selects the pinned
toolchain from `rust-toolchain.toml`.

## Try the implemented foundation

From this source checkout, generate a controllable sprite project against the
local SDK and run it:

```console
cargo run -p gridthorn_cli -- new ../hello-gridthorn --engine-path ./crates/gridthorn
cargo run -p gridthorn_cli -- check ../hello-gridthorn
cargo run -p gridthorn_cli -- run ../hello-gridthorn
```

Watch compilation inputs or produce an optimized build:

```console
cargo run -p gridthorn_cli -- check ../hello-gridthorn --watch
cargo run -p gridthorn_cli -- build ../hello-gridthorn --release
```

Watch retries after source/configuration edits and remains active after failed
checks. It checks compilation inputs; game-process restart remains planned.
See the [CLI workflow contract](docs/DEVELOPMENT_WORKFLOW.md) for polling scope,
profile behavior, and deferred packaging.

Use WASD or arrows to move, Space to reset, and Escape to exit. The generated
project includes fixed updates and the three-bar timing overlay.

The `--engine-path` option selects this source checkout. Without it, the CLI
generates a registry dependency; publishing the crates is separate from this
source release.

Run the provisional window and GPU surface lifecycle example. Surface configuration
is deferred until rendering, coalescing startup/resize events and supporting
shutdown before the first presented frame:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_window_surface
```

Runnable examples live in their own directories in the sibling
`gridthorn-examples` repository.

Run the provisional ECS schedule example:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_schedule_loop
```

Run the provisional keyboard-controlled textured-sprite example (WASD or
arrows; Space resets and Escape exits):

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_runtime_input
```

The three bars in the upper-left show host-frame duration, fixed updates, and
remaining fixed-step lag. The lag bar turns red while catch-up is overloaded.

Run the same fixed-update boundary without a window or renderer:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_headless_schedule -- --ticks 10
```

Run the self-closing structured diagnostics flow through the CLI:

```console
cargo run -p gridthorn_cli -- run ../gridthorn-examples/diagnostics-flow
```

Run a fixed-step command stream twice and compare its authoritative state
fingerprint:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_deterministic_replay
```

Run the texture hot-reload example, then edit its plain-text
`asset-reload/assets/sprite.ppm` fixture while the window is open:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_asset_reload
```

Add `-- --smoke` for a headless file-edit/recovery check. The provisional
`AssetReloader` service prepares changes in a background thread and publishes
them at an explicit frame boundary, reports transitive dependents, and keeps
the last good data on errors. The example requests a scan every 250 ms. See the
[asset reload contract](docs/ASSETS.md) for polling costs and current limits.

Run the headless basic collision query example:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_collision_basics
```

Run the headless versioned scene round-trip, rollback, and migration example:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_scene_serialization
```

The [provisional scene contract](docs/SCENES.md) covers registered scalar data,
compatibility checks, and prepared loading at an explicit load/reset boundary.

## Repository workflows

Codex discovers project workflows under `.agents/skills`:

- `$implement` implements a feature under scoped `AGENTS.md` guidance, adds
  domain-owned tests, and runs the complete existing verification suite.
- `$release` establishes the release range, selects a compatible version,
  reconciles release documentation, verifies the candidate, and generates
  copy-ready tag and release text without publishing.
- `$review` reviews a defined diff for behavioral, architectural, and
  compatibility defects without applying fixes.
- `$diagnose` reproduces and localizes failures, applying a focused fix when
  requested.

[AI workflow](docs/AI_WORKFLOW.md) routes domain context, verification, optional
long-task checkpoints, and measurements of token usage and execution time.
Markdown-only prose and instruction changes support explicit docs-only
verification; code, configuration, scripts, and releases retain full checks.

## License

See [LICENSE](LICENSE).

Component/resource reflection is available through the public SDK; see the
[provisional reflection contract](docs/REFLECTION.md) and the sibling
reflection-basics example.
