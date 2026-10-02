# Provisional CLI scenarios and headless simulation

Implemented on 2026-10-02. The CLI is a process adapter over a game-owned
headless binary using the public [scenario API](SCENARIOS.md). It adds no SDK
dependencies, privileged world access, or development protocol.

## Project configuration

Add this optional section to the existing schema 1 `gridthorn.toml`:

```toml
[simulation]
binary = "scenario-headless"
scenarios = ["economy", "traffic-jam"]
```

`binary` names a Cargo binary target in the project package. Use ASCII letters,
digits, hyphens, or underscores. Declare the target using `[[bin]]` or
`src/bin/<name>.rs`. Scenario names are unique, nonempty, unpadded strings without
control characters. At least one name is required. Unknown configuration fields
are rejected. Existing projects without this section still support the existing
commands; scenario/simulation commands report how to configure the capability.
All project commands validate an optional simulation section when present.

```console
gridthorn scenario list ./my-game
gridthorn simulate ./my-game --scenario economy --ticks 100 --seed 42
gridthorn simulate ./my-game --scenario economy --ticks 100 --seed 42 --release
```

The project path defaults to `.`. Listing checks manifest/engine compatibility
and prints one declared name per line in lexicographic order without compiling
or executing game code. These are project declarations, not runtime discovery.
An unknown selected name is rejected before Cargo runs.

## Game launch contract

Simulation validates the project and invokes:

```console
cargo run --manifest-path <absolute-project>/Cargo.toml --bin <binary> [--release] -- --scenario <name> --ticks <u64> --seed <u64>
```

The child working directory is the project directory. Arguments are separate
process arguments, never shell commands. Tick count and master seed are required
explicit unsigned 64-bit integers. Zero ticks requests initialization only.
There are no implicit seed, clock, or scenario defaults. Cargo's normal target
directory and feature configuration apply; `--release` selects its optimized
profile. A distinct target avoids accidentally choosing the interactive game
when a package has multiple binaries. A missing target, compile error, or game
failure is reported with subprocess context and a nonzero CLI exit status.
The CLI reports failure but does not preserve the child's exact numeric exit code.
Child stdout/stderr are inherited; result formatting belongs to the game.

The binary must validate its arguments, map the name to a game-owned `Scenario`,
initialize `RandomStreams` with the supplied seed, and run `ScenarioRuntime::run_ticks`
or `HeadlessSimulation::run_ticks`. It must avoid initializing a window, GPU, or
audio device. It should report completed ticks, including an early `ExitRequest`,
and a canonical authoritative fingerprint when useful. Shutdown remains the game's
responsibility. The CLI cannot inspect generic game types or enforce a binary's
tick, randomness, or device behavior. Compatibility and determinism scope are those
of the underlying SDK and game rules.

## Evidence and deferred work

The sibling `scenarios-snapshots` example supplies a dedicated `scenario-headless`
target and project manifest:

```console
cargo run -p gridthorn_cli -- scenario list ../gridthorn-examples/scenarios-snapshots
cargo run -p gridthorn_cli -- simulate ../gridthorn-examples/scenarios-snapshots --scenario economy --ticks 100 --seed 42
```

Command tests exercise actual Cargo binary selection, exact argument forwarding,
project working directory, initialization at zero ticks, maximum seed, debug and
release profiles, process failure, missing binaries, unknown scenarios, malformed
configuration, and projects without simulation support. Parser tests reject missing
inputs, negative tick counts, and integer overflow. The public example executes
the real SDK and permits repeated-process fingerprint comparison.

Scenario files, runtime discovery, snapshots/save restoration from the CLI,
timestamped command streams, replay, feature-selection flags, structured result
transport, execution timeouts, and a development connection remain deferred.
Large simulation workloads, process startup/binary-size measurements, and
cross-platform launch validation remain unmeasured. This completes the Milestone 3
CLI item for the provisional explicit game-process contract.
