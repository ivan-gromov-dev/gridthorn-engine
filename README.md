# Gridthorn Engine

Gridthorn is a modular, code-first game engine and Rust SDK for building 2D
games. It is suitable for traditional 2D genres while providing a particularly
strong foundation for tile-based, isometric, management, tycoon, and
simulation-heavy games.

The project has completed its Milestone 1 runtime vertical slice and started
Milestone 2 with provisional frame-boundary game states and scene switching. It has a
reproducible Cargo workspace, a working CLI, native window and input handling,
fixed-step world updates, sprite presentation, texture loading, and runtime
timing diagnostics. A visual editor will later be built on the same public APIs
and development protocol.

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

The Milestone 1 runtime vertical slice is complete. Commands and Rust snippets
outside the section below still describe target developer experience unless
they are explicitly marked as implemented.

| Area                               | Status                                                                 |
| ---------------------------------- | ---------------------------------------------------------------------- |
| Product and architecture direction | Design baseline                                                        |
| Milestone 0 foundation             | Complete; runtime deferrals recorded                                   |
| Cargo workspace and CI             | Implemented foundation                                                 |
| CLI                                | Commands plus cross-manifest engine compatibility checks implemented   |
| Public SDK facade                  | Version API plus provisional lifecycle and schedule APIs               |
| App and renderer internals         | Window/surface lifecycle and basic colored-sprite pipeline implemented |
| Keyboard and mouse input           | Provisional engine-owned frame state implemented                       |
| World and schedules                | ECS lifecycle schedule spike implemented                               |
| Game commands and world control    | Ordered commands and controllable entity example implemented           |
| Texture assets                     | PNG/PNM decoding and textured-sprite presentation implemented           |
| Headless schedule execution        | Internal architecture spike implemented                                |
| Cross-layer diagnostics            | CLI/app/renderer tracing spike implemented                             |
| Runtime timing overlay             | Frame time, fixed work, lag, and overload bars implemented             |
| Deterministic fixed-step replay    | Seeded RNG and state fingerprint spike implemented                     |
| Runtime and public SDK             | Milestone 1 vertical slice complete; APIs remain provisional           |
| Scenes and game states             | Provisional state stack and atomic scene-owned entity switching         |
| Sprite batching and animation      | Provisional ordered texture batches and sprite-sheet playback            |
| Text and runtime UI                | Provisional bitmap text, screen panels, and mouse buttons                |
| Audio                              | Provisional PCM16 WAV assets, queue, and opt-in Kira output adapter      |
| Current source release             | `0.2.0`                                                                |
| Stable API                         | Not available; APIs remain pre-1.0 and provisional                     |

The first implementation target is a foundation and runtime vertical slice with
a usable CLI, window, game loop, world, sprite, input, fixed-step simulation,
diagnostics, and a documented example.

## Try the implemented foundation

From this source checkout, generate a controllable sprite project against the
local SDK and run it:

```console
cargo run -p gridthorn_cli -- new ../hello-gridthorn --engine-path ./crates/gridthorn
cargo run -p gridthorn_cli -- check ../hello-gridthorn
cargo run -p gridthorn_cli -- run ../hello-gridthorn
```

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

## Repository workflows

Codex discovers project workflows under `.agents/skills`:

- `$implement` implements a feature under scoped `AGENTS.md` guidance, adds
  domain-owned tests, and runs the complete existing verification suite.
- `$release` prepares the next patch release, reconciles versioned project
  documentation and optimized agent guidance, verifies the candidate, and
  generates copy-ready tag and release text.

## License

See [LICENSE](LICENSE).
