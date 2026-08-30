# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Provisional public application runtime with ordered `Startup`, `PollEvents`,
  `Input`, `FixedUpdate`, `Update`, `PostUpdate`, `Render`, and `Shutdown`
  schedules exposed through the `gridthorn` facade.
- Configurable fixed-step time accumulation with integer tick indices, bounded
  per-frame catch-up, observable overload, and preserved backlog.
- A provisional `WindowedApplication` that drives timed runtime frames from the
  native event loop, excludes suspended time, and guarantees runtime shutdown.

## [0.1.0] - 2026-08-28

### Added

- Cargo workspace with Rust 1.97.1 as the MSRV.
- Initial `gridthorn` SDK facade and version API.
- `gridthorn` CLI with `new`, `run`, `check`, and `--version`.
- Embedded minimal project template and generation/build tests.
- Formatting, lint, test, dependency-boundary, and CI checks.
- Scoped `AGENTS.md` guidance and domain-owned source/test organization.
- Matching Windows and macOS/Linux verification scripts plus three-platform CI
  with an aggregate `CI Success` gate.
- Repository-scoped `$implement` and `$release` skills for tested feature work
  and consistent release preparation.
- Provisional `gridthorn_app` and `gridthorn_render` crates with a documented
  window, GPU surface, resize, minimize, restore, and close lifecycle example.
- Provisional `gridthorn_world` ECS storage and explicit `Startup`,
  `FixedUpdate`, and `Update` schedules behind Gridthorn-owned APIs.
- A windowed `schedule-loop` example that drives ECS schedules through the
  Gridthorn application lifecycle.
- A `headless-schedule` example that runs the same fixed-update boundary without
  application, windowing, GPU, or renderer dependencies.
- Process-level and public-parser coverage for the complete CLI-generated
  project lifecycle, including contextual invalid-project diagnostics.
- Structured `component` fields across CLI, application, and renderer tracing,
  plus a CLI-driven diagnostics-flow example and renderer-to-app error test.
- A sibling `gridthorn-examples` repository convention with one independently
  owned directory per executable example.
- Engine-owned SplitMix64 streams and stable FNV-1a state fingerprint encoding
  in the provisional `gridthorn_simulation` crate.
- A deterministic-replay example covering repeatable fixed-step runs and
  changed-seed and changed-command controls.
- A completed Milestone 0 foundation review with clean-build, binary-size, and
  dependency-growth baselines plus explicit runtime deferrals.

### Changed

- Raised the project MSRV to Rust 1.97.1 so current `bevy_ecs` and `wgpu`
  releases can be validated.
- Updated `bevy_ecs`, `pollster`, `thiserror`, `toml`, and `wgpu` to their
  current releases.
