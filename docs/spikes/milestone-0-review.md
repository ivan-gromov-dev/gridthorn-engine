# Milestone 0 Review

## Decision

Milestone 0 was completed on 2026-08-28. Its intended result is a reproducible
SDK foundation and a real CLI, not a stable or cohesive game runtime. All six
required dependency spikes have executable examples or process-level coverage,
focused automated tests, documented boundaries, and explicit limitations.

The repository verification script passed formatting, workspace checks,
Clippy, 23 engine tests, documentation tests, and dependency-boundary checks.
The sibling examples workspace passed formatting, Clippy, and 13 tests. The
deterministic replay executable also completed end to end.

## Windows baseline

Measurements used Rust 1.97.1, the release profile, an empty temporary target
directory, and the locked dependency graph. They are observational baselines,
not CI performance gates:

| Measurement                            |          Result |
| -------------------------------------- | --------------: |
| Engine workspace clean release build   |   81.88 seconds |
| Examples workspace clean release build |   79.43 seconds |
| Unique normal dependency-tree lines    |             196 |
| CLI executable                         | 1,997,824 bytes |
| Deterministic replay example           |   873,984 bytes |
| Headless schedule example              |   890,368 bytes |
| Diagnostics flow example               | 7,630,848 bytes |
| Window surface example                 | 7,635,456 bytes |
| Windowed schedule example              | 8,336,896 bytes |

The two headless executables remain below one megabyte in this baseline. The
windowed executables include the application and GPU stacks and are roughly
7.6–8.3 MB. Future milestones should compare like-for-like builds on the same
toolchain and host rather than treating these values as cross-platform limits.

## Accepted deferrals

- Display-backed resize, minimize, restore, and close validation on macOS and
  Linux moves to Milestone 1. The CI matrix still compiles and tests on Windows,
  macOS, and Linux.
- ECS change-detection semantics and the parallel-system ordering policy move
  to Milestone 1. The prototype deliberately executes authoritative callbacks
  sequentially in registration order.
- Named random-stream registration, replay metadata, snapshots, cross-target
  deterministic fixtures, and a supported headless API remain later runtime or
  specialization work as already scoped by the roadmap.
- The public SDK remains intentionally minimal and unstable. The subsystem
  APIs exercised by spikes are prototypes, not SemVer-stable capabilities.

These deferrals do not weaken the Milestone 0 result. They constrain what may
be claimed until the runtime vertical slice validates and exposes those
capabilities.
