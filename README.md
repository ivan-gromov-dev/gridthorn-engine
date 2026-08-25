# Gridthorn Engine

Gridthorn is a modular, code-first game engine and Rust SDK for building 2D
games. It is suitable for traditional 2D genres while providing a particularly
strong foundation for tile-based, isometric, management, tycoon, and
simulation-heavy games.

The project is currently in its design stage. Our immediate goal is to build a
cohesive SDK with a fast development and debugging loop. A visual editor will
later be built on the same public APIs and development protocol.

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
- [Project terminology](docs/GLOSSARY.md)
- [Architecture Decision Records](docs/adr/README.md)

## Status

This repository currently contains design documentation only. It does not yet
contain a Cargo workspace, buildable engine code, an installable CLI, examples,
or a public API. Commands and Rust snippets in the documentation describe target
developer experience unless they are explicitly marked as implemented.

| Area | Status |
| --- | --- |
| Product and architecture direction | Design baseline |
| Cargo workspace and CI | Planned for Milestone 0 |
| CLI | Planned for Milestone 0 |
| Runtime and public SDK | Planned for Milestone 1 and later |
| Stable release or API | Not available |

The first implementation target is a foundation and runtime vertical slice with
a usable CLI, window, game loop, world, sprite, input, fixed-step simulation,
diagnostics, and a documented example.

## License

See [LICENSE](LICENSE).
