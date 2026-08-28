# Headless Schedule Spike

## Status

Implemented as an internal Milestone 0 architecture prototype. It validates
that the ECS runtime and fixed-update schedule can execute in a process with no
application, window, GPU, or renderer dependency. This is not yet the supported
public headless simulation API planned for Milestone 3.

## Question

Can the same `ScheduleRuntime` and `FixedUpdate` boundary used by a windowed
application execute a requested number of steps without linking presentation or
platform subsystems?

## Result

- `gridthorn_example_headless_schedule` depends only on `gridthorn_world`.
- The scenario runs `Startup` once and then exactly the requested number of
  `FixedUpdate` steps.
- Zero-step and multi-step runs are covered by focused scenario tests.
- Command parsing reports contextual failures for missing, invalid, or unknown
  arguments.
- The package dependency tree contains no `gridthorn_app`, `gridthorn_render`,
  `winit`, or `wgpu` dependency.

The test scenario produces a compact observable snapshot after execution. A
stable state hash and repeated-run determinism contract remain the dedicated
final dependency spike rather than an implied guarantee of this prototype.

## Run

From the engine repository:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_headless_schedule -- --ticks 10
```

The command prints the number of startup runs, completed fixed steps, and final
scenario value. Omitting `--ticks` uses a small default run.

## Evidence

The executable completed a ten-step run with one startup execution. Automated
tests cover default and explicit arguments, invalid tick counts, zero steps,
and multiple fixed steps. `cargo tree` confirmed the absence of application and
presentation packages.

Compile-time, binary-size, and dependency-growth comparisons remain deferred
until every Milestone 0 dependency spike is present so their costs can be
measured consistently.
