# ECS Schedule Loop Spike

## Status

Implemented as a provisional Milestone 0 prototype. Automated tests validate
world storage, typed resources, schedule ordering, one-time startup, repeated
fixed updates, and frame updates. The sibling examples repository contains the
windowed end-to-end path.

This result completes the explicit windowed schedule-loop question, but does
not make the ECS selection stable. Headless reuse is the next spike. Change
detection, parallel execution policy, diagnostics, and dependency-cost
measurements remain open.

## Question

Can Gridthorn run a `bevy_ecs` world through engine-owned `Startup`,
`FixedUpdate`, and `Update` stages without exposing Bevy world, entity,
component, resource, query, or schedule types to application code?

## Result

- `gridthorn_world` owns the backend ECS world and executable schedules.
- `ScheduleStage`, `ScheduleBuilder`, `ScheduleRuntime`, `WorldAccess`, and
  `EntityId` form the provisional Gridthorn boundary.
- Domain components and resources are wrapped internally, so callers use
  ordinary Rust types without Bevy derives or trait bounds.
- Systems within a stage currently execute in registration order. This makes
  the initial lifecycle contract explicit; parallel conflict and ordering
  policy remains deferred.
- `gridthorn_example_schedule_loop` connects the schedule runtime to
  `WindowLifecycle` and proves `Startup` once followed by explicit fixed and
  frame updates.

The selected ECS release enables its multithreaded executor and builds on the
project's current Rust toolchain, but this prototype intentionally dispatches
engine-owned callbacks sequentially until deterministic ordering rules are
defined.

## Run

From the engine repository, run the interactive example:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_schedule_loop
```

Run the self-closing end-to-end path:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_schedule_loop -- --smoke
```

## Review outcome

Combined compile-time, binary-size, and dependency-growth measurements are in
the [Milestone 0 review](milestone-0-review.md). Headless reuse and deterministic
sequential execution were validated by later spikes. Change detection and a
parallel-system policy remain Milestone 1 runtime work.
