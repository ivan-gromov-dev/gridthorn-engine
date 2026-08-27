# Gridthorn Engine

Gridthorn is a modular, code-first game engine and Rust SDK for building 2D
games. It is suitable for traditional 2D genres while providing a particularly
strong foundation for tile-based, isometric, management, tycoon, and
simulation-heavy games.

The project is in its first implementation milestone. It now has a reproducible
Cargo workspace, a minimal SDK facade, and a CLI foundation; the runtime itself
is still under construction. A visual editor will later be built on the same
public APIs and development protocol.

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

The Milestone 0 foundation is partially implemented. Commands and Rust snippets
outside the section below still describe target developer experience unless
they are explicitly marked as implemented.

| Area | Status |
| --- | --- |
| Product and architecture direction | Design baseline |
| Cargo workspace and CI | Implemented foundation |
| CLI | `new`, `run`, `check`, and `--version` implemented |
| Public SDK facade | Version API only; runtime APIs planned |
| Runtime and public SDK | Planned for Milestone 1 and later |
| Stable release or API | Not available |

The first implementation target is a foundation and runtime vertical slice with
a usable CLI, window, game loop, world, sprite, input, fixed-step simulation,
diagnostics, and a documented example.

## Try the implemented foundation

From this source checkout, generate a project against the local SDK and run it:

```console
cargo run -p gridthorn_cli -- new ../hello-gridthorn --engine-path ./crates/gridthorn
cargo run -p gridthorn_cli -- check ../hello-gridthorn
cargo run -p gridthorn_cli -- run ../hello-gridthorn
```

The `--engine-path` option is only needed while Gridthorn is used from an
unpublished checkout. Installed releases generate a registry dependency by
default.

## Repository workflows

Codex discovers project workflows under `.agents/skills`:

- `$implement` implements a feature under scoped `AGENTS.md` guidance, adds
  domain-owned tests, and runs the complete existing verification suite.
- `$release` prepares the next patch release, reconciles versioned project
  documentation and optimized agent guidance, verifies the candidate, and
  generates copy-ready tag and release text.

## License

See [LICENSE](LICENSE).
