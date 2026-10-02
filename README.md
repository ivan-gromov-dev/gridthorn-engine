# Gridthorn Engine

Gridthorn is a modular, code-first game engine and Rust SDK for building 2D
games. It is suitable for traditional 2D genres while providing a particularly
strong foundation for tile-based, isometric, management, tycoon, and
simulation-heavy games.

The project has completed Milestone 2, the general-purpose 2D SDK, with
provisional APIs validated by a small playable game. It has a
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
- [Project terminology](docs/GLOSSARY.md)
- [Architecture Decision Records](docs/adr/README.md)
- [Contributor guide](CONTRIBUTING.md)

## Status

The Milestone 2 general-purpose 2D SDK is complete for its documented subsets. Commands and Rust snippets
outside the section below still describe target developer experience unless
they are explicitly marked as implemented.

| Area                               | Status                                                                                     |
| ---------------------------------- | ------------------------------------------------------------------------------------------ |
| Product and architecture direction | Design baseline                                                                            |
| Milestone 0 foundation             | Complete; runtime deferrals recorded                                                       |
| Cargo workspace and CI             | Implemented foundation                                                                     |
| CLI                                | Project commands, compatibility checks, source watch and release builds                    |
| Public SDK facade                  | Version API plus provisional lifecycle and schedule APIs                                   |
| App and renderer internals         | Window/surface lifecycle and basic colored-sprite pipeline implemented                     |
| Keyboard and mouse input           | Provisional engine-owned frame state implemented                                           |
| World and schedules                | ECS lifecycle schedule spike implemented                                                   |
| Game commands and world control    | Ordered commands and controllable entity example implemented                               |
| Texture assets                     | PNG/PNM decoding and textured-sprite presentation implemented                              |
| Asset dependencies and hot reload  | Implemented provisional path IDs, dependencies, background reload and atomic frame commits |
| Headless simulation                | Provisional exact-tick SDK runner, lifecycle, exit and repeatability implemented                                                    |
| Cross-layer diagnostics            | CLI/app/renderer tracing spike implemented                                                 |
| Runtime timing overlay             | Frame time, fixed work, lag, and overload bars implemented                                 |
| Deterministic fixed-step replay    | Named seeded RNG and explicit state fingerprint API; replay files planned                                         |
| Runtime and public SDK             | Milestone 2 complete; APIs remain provisional                                              |
| Scenes and game states             | Provisional state stack and atomic scene-owned entity switching                            |
| Sprite batching and animation      | Provisional ordered texture batches and sprite-sheet playback                              |
| Text and runtime UI                | Provisional bitmap text, screen panels, and mouse buttons                                  |
| Audio                              | Provisional PCM16 WAV assets, queue, and opt-in Kira output adapter                        |
| Component/resource reflection      | Provisional explicit registration, scalar metadata and read-only snapshots                 |
| Versioned scene serialization      | Provisional schema 1 TOML, validated prepared loading and migration hooks                  |
| Basic 2D collision                 | Provisional circle/AABB overlap and minimum-translation queries                            |
| Grid coordinates | Provisional opt-in square/isometric projection and inverse conversion |
| Tilemaps | Provisional sparse layers/chunks and camera-aware occupied-cell picking |
| Grid-based object placement | Provisional integer footprints, exclusive occupancy, atomic moves and removal |
| Pathfinding and diagnostics | Provisional deterministic four-neighbor weighted search, budgets, visited/frontier data and SVG example |
| Scenarios and snapshots | Provisional typed authoritative roots, exact-tick restoration and compatibility checks |
| Simulation clock | Provisional rational speed, pause/resume, preserved backlog and explicit stepping |
| Current source release             | `0.2.0`                                                                                    |
| Stable API                         | Not available; APIs remain pre-1.0 and provisional                                         |

Milestone 3 is underway: provisional square and isometric coordinates are
implemented behind the `grid` feature; see [the grid contract](docs/GRIDS.md).
Sparse tilemaps, layers, chunks, picking, and grid-based object placement are
also implemented, along with bounded weighted pathfinding and SVG diagnostics.
Simulation clock, pause, and speed control are implemented; see the
[simulation contract](docs/SIMULATION.md). Headless simulation is implemented through `HeadlessSimulation`. Typed scenarios, in-memory snapshots, and named seeded RNG streams are implemented;
see the [scenario contract](docs/SCENARIOS.md). The next target is world saving and loading.

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

Run the provisional window and GPU surface lifecycle example:

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
- `$release` prepares the next patch release, reconciles versioned project
  documentation and optimized agent guidance, verifies the candidate, and
  generates copy-ready tag and release text.

## License

See [LICENSE](LICENSE).

Component/resource reflection is available through the public SDK; see the
[provisional reflection contract](docs/REFLECTION.md) and the sibling
reflection-basics example.
