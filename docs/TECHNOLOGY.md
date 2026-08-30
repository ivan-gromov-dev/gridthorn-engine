# Tools and Libraries

This document is the dependency policy and initial technology baseline. Once
the Cargo workspace exists, exact versions will live in its manifests and
`Cargo.lock`; this document should not duplicate version numbers that become
stale.

The statuses below describe architectural intent, not implementation status. A
selected tool is not present in the repository until its milestone work is
completed and verified.

Statuses used below:

- **Selected:** part of the current architectural baseline.
- **Provisional:** intended for use, but must pass a focused milestone spike.
- **Candidate:** deferred until the feature that needs it is scheduled.
- **Build:** a tool used to develop Gridthorn rather than linked into games.

## Selection principles

- Build engine-specific behavior ourselves; reuse high-quality platform and
  infrastructure crates.
- Keep third-party types behind Gridthorn APIs where replacement cost matters.
- Add dependencies when a milestone needs them, not in anticipation of every
  future feature.
- Prefer actively maintained, cross-platform Rust crates with compatible open
  source licenses.
- Audit feature flags, transitive dependencies, compile time, binary size, and
  platform support before adoption.
- Record difficult-to-reverse decisions in
  [ADRs](adr/README.md) and validate them with small executable spikes.

## Milestone 0/1 baseline

| Area            | Choice                           | Status      | Purpose and boundary                                                                                      |
| --------------- | -------------------------------- | ----------- | --------------------------------------------------------------------------------------------------------- |
| Language        | Rust stable                      | Selected    | MSRV is recorded in the workspace manifest and verified in CI.                                            |
| Build           | Cargo workspace                  | Selected    | Implemented foundation for crates, feature flags, tests, and releases.                                    |
| CLI             | `clap`                           | Selected    | Implemented typed commands, help, and validation; completions are deferred.                               |
| Window/events   | `winit`                          | Selected    | Cross-platform windows and OS event loop behind `gridthorn_app`.                                          |
| Graphics        | `wgpu` + WGSL                    | Selected    | Cross-platform GPU API behind `gridthorn_render`.                                                         |
| Math            | `glam`                           | Selected    | Vectors and matrices; exposed only through deliberate SDK aliases or types.                               |
| GPU data        | `bytemuck`                       | Selected    | Checked conversion of explicitly POD GPU structs.                                                         |
| Image decode    | `image`                          | Selected    | Development-time texture decoding behind the asset pipeline.                                              |
| ECS/schedules   | `bevy_ecs` standalone            | Provisional | World, queries, resources, schedules, and parallel execution. Wrapped by Gridthorn lifecycle and prelude. |
| Serialization   | `serde`                          | Selected    | Common serialization traits for manifests and engine data.                                                |
| Project config  | `toml`                           | Selected    | Human-readable project manifest.                                                                          |
| Scene data      | `ron`                            | Provisional | Human-readable authored scenes and data; validate diagnostics and migrations.                             |
| Errors          | `thiserror`                      | Selected    | Typed errors in library crates.                                                                           |
| CLI errors      | `anyhow`                         | Selected    | Context-rich errors at executable boundaries only.                                                        |
| Diagnostics     | `tracing` + `tracing-subscriber` | Selected    | Structured logs and spans shared by runtime and CLI.                                                      |
| GPU init bridge | `pollster`                       | Selected    | Minimal synchronous bridge during early native renderer initialization.                                   |

`bevy_ecs` is selected for the first implementation spike because its standalone
crate includes a mature world/query model and configurable schedules, including
parallel execution. It is still marked provisional: Milestone 0 must validate
compile cost, diagnostics, change detection, schedule control, headless use, and
our ability to avoid leaking Bevy-specific policy into the public SDK.

We are building Gridthorn on reusable low-level crates, not wrapping the Bevy
engine. Gridthorn owns application lifecycle, rendering, assets, scenes,
simulation semantics, plugins, CLI, and tools.

## Development tooling

| Tool            | Status    | Use                                                             |
| --------------- | --------- | --------------------------------------------------------------- |
| `rustfmt`       | Build     | Canonical Rust formatting.                                      |
| Clippy          | Build     | Workspace linting with an explicitly documented lint policy.    |
| `cargo test`    | Build     | Unit, integration, and documentation tests.                     |
| `cargo-nextest` | Candidate | Faster CI test execution once the suite justifies it.           |
| `cargo-deny`    | Candidate | License, advisory, duplicate, and source policy checks.         |
| `cargo-machete` | Candidate | Detection of unused dependencies.                               |
| `cargo-bloat`   | Candidate | Binary-size investigations, not a default CI gate.              |
| Criterion       | Candidate | Repeatable microbenchmarks for proven hot paths.                |
| GitHub Actions  | Selected  | Formatting, linting, tests, and target build checks.            |
| `mdbook`        | Candidate | User guide when the documentation outgrows repository Markdown. |

The repository will provide task commands or scripts for common checks so local
development and CI execute the same operations.

## Planned feature dependencies

These choices are intentionally not Milestone 0 dependencies.

| Area                   | Current direction                      | Status      | Decision point                                                                                             |
| ---------------------- | -------------------------------------- | ----------- | ---------------------------------------------------------------------------------------------------------- |
| Debug/editor UI        | `egui`                                 | Provisional | Integrate the first runtime debug overlay without coupling game UI to it.                                  |
| Text shaping/rendering | `glyphon` / `cosmic-text`              | Candidate   | Validate version alignment with `wgpu`, font fallback, and atlas behavior.                                 |
| Audio                  | `kira`                                 | Provisional | Opt-in native adapter implemented; validate streaming, latency, and cross-platform recovery.                |
| File watching          | `notify`                               | Candidate   | Add with asset hot reload and CLI development mode.                                                        |
| Cargo integration      | `cargo_metadata`                       | Candidate   | Add when CLI project discovery requires structured Cargo metadata.                                         |
| Reflection             | `bevy_reflect` or a Gridthorn registry | Candidate   | Decide after the ECS spike and before scene/inspector work; keep reflection out of Milestone 0 public API. |
| Gamepad input          | `gilrs`                                | Candidate   | Validate platform support when controller input enters the roadmap.                                        |
| Parallel jobs          | `rayon`                                | Candidate   | Add only for work not adequately scheduled through the ECS or renderer.                                    |
| 2D physics             | `rapier2d` behind a plugin             | Candidate   | Evaluate against the collision needs of both showcase games.                                               |
| Compression/archive    | Undecided                              | Candidate   | Choose when packaged asset bundles are designed.                                                           |
| IPC/dev protocol       | Undecided                              | Candidate   | Design protocol and transport during the professional debugging milestone.                                 |

## Systems Gridthorn owns

The following are product-defining systems and will not be delegated wholesale
to a general game framework:

- public application and plugin APIs;
- update stages, fixed-step semantics, and simulation clock;
- the 2D renderer, batching policy, cameras, and render layers built on `wgpu`;
- assets, handles, dependency tracking, and packaging;
- scenes, versioning, and migrations;
- square and isometric grids, tilemaps, chunks, and picking;
- scenarios, snapshots, replay, and development protocol;
- CLI workflows, debug tools, and editor integration;
- the eventual runtime game UI abstraction.

`egui` is intended for developer tooling. It is not automatically the public
game UI system because game UI has different styling, layout, animation, input,
serialization, and performance requirements.

## Required Milestone 0 spikes

Before the baseline is considered stable, create and document small tests for:

1. [Implemented as a Windows-validated prototype](spikes/window-surface.md): a
   `winit` application that owns a `wgpu` surface and survives resize, minimize,
   restore, and close events. macOS and Linux runtime validation remains open.
2. [Implemented as a windowed prototype](spikes/ecs-schedule-loop.md): a
   `bevy_ecs` world running explicit `Startup`, `FixedUpdate`, and `Update`
   schedules under Gridthorn's application loop. Headless reuse, change
   detection, and parallel-system policy remain separate validation work.
3. [Implemented as an internal architecture prototype](spikes/headless-schedule.md):
   a headless run of the same fixed-update schedule without application,
   windowing, GPU, or renderer dependencies.
4. [Implemented and process-validated](spikes/cli-project-lifecycle.md): a
   minimal `gridthorn` CLI that creates, checks, and runs a generated project.
5. [Implemented as a cross-layer prototype](spikes/structured-diagnostics.md):
   structured error and tracing output spanning CLI, app, and renderer layers.
6. [Implemented as a deterministic replay prototype](spikes/deterministic-fixed-step.md):
   repeatable fixed-step runs that produce the same state fingerprint for the
   same commands and seed under the determinism scope defined in
   [ARCHITECTURE.md](ARCHITECTURE.md#determinism-contract).

The Milestone 0 headless run is an internal architecture spike. It proves that
the runtime is not structurally coupled to a window or GPU; it is not yet the
supported public headless simulation API planned for Milestone 3.

The outcome of a failed spike is an ADR changing the selection, not an adapter
that hides an unsuitable dependency indefinitely.
