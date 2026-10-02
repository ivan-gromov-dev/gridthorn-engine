# Provisional simulation clock

Implemented on 2026-10-02. The SDK exposes `SimulationControl`,
`SimulationSpeed`, `FixedStepConfig`, `FixedTime`, and `FrameTiming`.
These APIs remain provisional and introduce no dependencies.

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
boundary does not implement the planned headless simulation service.

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
replay recording, remote tooling, a dedicated headless runner, scenarios,
snapshots, and world persistence belong to later increments.
