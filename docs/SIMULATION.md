# Provisional simulation clock

Implemented on 2026-10-02. The SDK exposes `SimulationControl`,
`SimulationSpeed`, `FixedStepConfig`, `FixedTime`, and `FrameTiming`.
These APIs remain provisional and introduce no dependencies.

The [performance review](PERFORMANCE_REVIEW.md#simulation-grid-collision-and-snapshot-domain-closure)
records capped ECS catch-up through 65536 agents and a paused 65536-command
burst. A catch-up cap bounds ticks per frame, not system CPU time or queued
payload memory. Games own command admission and backlog policy; the runtime
preserves accumulated lag and does not silently discard commands or ticks.

## Frame contract

`ApplicationRuntime` installs default controls (running, 1x) unless already
configured. Startup can also configure them. Timed frames run Startup once,
PollEvents, Input, and state/scene transitions before sampling controls.
Input changes affect the current frame; fixed-update and presentation changes
affect the next timed frame. Controls are orchestration settings.

Pause freezes all timed fixed work, including backlog, preserves fractional
time and accumulated lag, and ignores paused host time. Update, PostUpdate,
and Render continue, keeping development UI responsive. Game commands remain
queued until game code drains them during a fixed tick. Resume uses the current
speed without adding the paused interval.

`SimulationSpeed::new(numerator, denominator)` accepts positive u32 terms and
reduces the ratio. Zero terms return `SimulationSpeedError`; stopping uses pause.
Speed scales incoming host nanoseconds with integer arithmetic; fixed tick
duration never changes. Fractional nanoseconds persist across frames at the
same normalized speed. Changing speed discards only the previous sub-nanosecond
remainder, preserving whole-nanosecond backlog.

`FrameTiming::frame_elapsed` is unscaled host time, including paused frames.
`accumulated_lag` is scaled simulation time waiting for ticks. `completed_ticks`
counts assigned ticks; `FixedTime` carries the zero-based tick index and fixed
duration. Paused frames report zero work and no overload. Running frames cap
work at the configured catch-up limit, preserve backlog, and emit the existing
runtime overload diagnostic. No ticks are dropped or durations stretched.

`ApplicationRuntime::run_frame(n)` assigns exactly n ticks regardless of pause,
speed, or catch-up limits, without consuming lag. This allows explicit stepping
while paused and still runs input and presentation. This existing execution
boundary remains distinct from the dedicated headless service below.

Clock arithmetic errors leave clock state unchanged. Startup, input, and scene
transitions may already have executed when a runtime returns a time error;
fixed updates and presentation do not run for that failed frame.

## Usage and evidence

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_simulation_clock
```

The public SDK example asserts contiguous ticks, retained commands consumed
once, responsive presentation, slow/fast playback, and stepping during pause.
Domain tests cover input-boundary controls, invalid ratios, fractional-time
partitioning, backlog preservation, and overflow rollback. Existing window
lifecycle tests cover exclusion of platform suspension time independently of
user pause.

Determinism remains scoped to the same engine version, target, configuration,
and command/control sequence. Performance, large-backlog workloads, native UI
integration, and cross-platform measurements are deferred. Control persistence,
replay recording, remote tooling, and world persistence belong to later increments.
Typed headless scenarios and in-memory snapshots are now documented in
[SCENARIOS.md](SCENARIOS.md).

## Provisional headless simulation

Implemented on 2026-10-02. `HeadlessSimulation` and `HeadlessProgress` are
available through the SDK facade. The runner owns the existing application
orchestration and fixed clock; it introduces no dependencies. It never creates
a window, renderer, GPU, input device, or audio device. The current facade and
application manifests still compile platform/presentation dependencies; a lean
feature-gated binary/dependency graph and binary-size measurements are deferred.
Game callbacks are responsible for avoiding platform services themselves.

`HeadlessSimulation::new(schedules, config)` takes a completed schedule set and
validated fixed configuration. `run_ticks(u64)` runs Startup once, then Input,
state/scene transitions, and FixedUpdate for each requested tick. PollEvents,
Update, PostUpdate, Render, Suspend, and Resume are excluded. Tick indices are
contiguous and zero-based, with the configured duration exposed through
`FixedTime`. `FrameTiming` represents one explicit tick with zero host elapsed
time, no overload, and no lag. Catch-up limits, pause, and speed are ignored:
requests specify authoritative work directly and never silently drop ticks.
Zero ticks runs Startup only, without Input or pending transitions.

ExitRequest is checked after Startup, after Input/transitions, and after each
complete fixed tick. An exit returns partial progress without running further
ticks. Reports contain request-local executed ticks, total completed ticks,
and exit status. Requests after exit return zero work until game code explicitly
replaces the exit resource. Shutdown is explicit and idempotent, runs the
Shutdown schedule once, and rejects subsequent requests with LifecycleError.
Dropping the runner does not execute callbacks. All callbacks run synchronously
on the caller thread, with the same affinity requirements as ScheduleRuntime.

Between requests, `world()` supports initial configuration, explicit load/reset
operations, inspection, and command injection. Games drain GameCommandQueue at
a fixed boundary, exactly as in interactive execution. Requests split into
multiple batches produce the same tick/Input sequence for the same initial
state and commands. Wall-clock timing, cross-target bit identity, generic state
hashing, persistence, CLI simulation
commands, and performance/large-world measurements are deferred.
Named RNG and typed scenario snapshots are available through
[ScenarioRuntime](SCENARIOS.md).

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_headless_simulation
```

The facade example executes 100 ticks twice with differently partitioned
requests and compares integer authoritative state, including one-shot commands.
Domain tests verify contiguous ticks, catch-up bypass, pause bypass, excluded
presentation, zero work, Startup/Input exits, partial progress, and idempotent
shutdown. Existing scene/state tests continue to verify the shared transition
boundary used by both drivers.
