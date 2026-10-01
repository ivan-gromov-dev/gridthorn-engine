# Game Development Workflow

Gridthorn aims for a fast code-first workflow that can later be presented in a
GUI without introducing a second, incompatible development model.

## Target developer journey

```text
Create a project
→ enable plugins
→ write code and data
→ run in development mode
→ inspect the world
→ reproduce and fix a problem
→ run tests
→ package a distribution
```

The CLI will exist from the first implemented milestone and grow alongside the
SDK:

```console
gridthorn new iron-valley --template minimal
cd iron-valley
gridthorn run
gridthorn check
```

Later milestones extend the same command rather than replace it:

```console
gridthorn dev --scenario traffic-jam
gridthorn test
gridthorn build --release
gridthorn package --target windows
```

`gridthorn build --release` is now implemented. `dev`, `test`, and `package`
remain target UX.

## CLI evolution

The first implemented CLI will intentionally be small but real:

- `gridthorn new` creates a project from an embedded minimal template.
- `gridthorn run` launches the project through Cargo.
- `gridthorn check` validates the Rust workspace and project manifest.
- `gridthorn check --watch` repeats validation and Cargo checking after source
  or configuration edits, including recovery from invalid edits.
- `gridthorn build` compiles a validated project; `--release` selects Cargo's
  optimized release profile.
- `gridthorn --version` reports SDK and CLI compatibility information.

As engine capabilities become available, the CLI adds asset validation, file
watching, scenarios, tests, builds, and packaging. Business logic belongs in
reusable SDK crates; CLI subcommands remain thin adapters so the future editor
can invoke the same services.

## Implemented watching and builds

```console
gridthorn check ./my-game --watch
gridthorn check ./my-game --watch --interval-ms 250
gridthorn build ./my-game
gridthorn build ./my-game --release
```

Watch performs an initial check, then polls every 500 ms by default. The interval
must be positive and can be specified only with `--watch`. Checks are synchronous:
one Cargo subprocess runs at a time, and edits made during a check are detected
by the next complete scan. Compilation or manifest validation failures are
reported without ending the watch session; the next observed edit triggers a
retry. An unchanged failing input is not checked repeatedly. Cargo creating or
updating `Cargo.lock` can cause one additional check.

Snapshots compare complete file contents in deterministic path order. Watching
covers project-local `.rs` files, including `build.rs`, Cargo manifests/lockfiles,
`gridthorn.toml`, Rust toolchain files, and the root `.cargo/config` or
`.cargo/config.toml`. Creation, deletion, renaming, and same-size edits are
detected. The scanner skips `target`, VCS directories, symlinks, and an existing
custom `CARGO_TARGET_DIR`. A failed scan retains the previous snapshot and
reports contextual diagnostics; repeated identical scan errors are suppressed
until scanning recovers. Stop the command with the terminal's Ctrl+C behavior.

This is provisional polling for compilation inputs. External path dependencies,
Cargo configuration in ancestor directories, generated assets, and native file
watching are outside this increment. Assets use the existing runtime asset reload
service. File edits that are made and reverted between polls are not observed.
Large-project scan latency and retained snapshot memory remain explicitly deferred
until a game workload establishes practical budgets.

Build validates project/engine compatibility before starting Cargo, inherits
Cargo's target-directory/profile configuration, and reports Cargo failures with
their exit code. It produces normal Cargo artifacts and does not package assets,
launch a game, or create an installer. The CLI adds no development runtime service
to release games. Automatic restart, scenarios, and development-tool connections
remain later milestones.

## Development mode

The eventual `gridthorn dev` command coordinates:

- incremental Rust builds;
- launching and safely restarting the game process;
- hot reload of assets and game data;
- structured diagnostics;
- development-tool connections;
- restoration of a selected scenario or snapshot after restart.

Rust code initially requires a process restart. Dynamically loading game code
is not a requirement for the first stable SDK.

## Runtime debugging

An in-game debug overlay provides an editor-like loop before a separate editor
exists:

- entity hierarchy;
- component and resource inspection;
- pause, resume, single-step, and simulation speed;
- schedule and system profiling;
- console and structured events;
- grid, bounds, collision, and navigation visualization;
- snapshot creation and restoration.

## Reproducibility

Simulation-heavy games need three related mechanisms:

1. **Scenario:** a small named starting state for development or testing.
2. **Snapshot:** world state captured at a particular moment.
3. **Replay:** an initial state, random seed, and sequence of
   [`GameCommand`](GLOSSARY.md#gamecommand) values.

They turn rare behavior into a repeatable test case. Simulation therefore uses
controlled clocks and random-number generators rather than ambient time and
implicit randomness.

## Automated validation

The simulation API must support testing without a window. The following is
illustrative pseudocode, not a committed or implemented public API:

```rust,ignore
#[test]
fn sawmill_produces_planks() {
    let mut game = TestGame::new();
    game.load_scenario("sawmill_with_logs");

    game.tick_n(40);

    assert_eq!(game.resource_amount("planks"), 4);
}
```

The test strategy also includes headless integration tests, serialization and
migration tests, asset-graph validation, render smoke tests, and long-running
simulation tests with fixed seeds.

## Transition to a GUI

Gridthorn Editor will present existing SDK operations:

| SDK capability | Editor representation |
| --- | --- |
| Development process control | Play, Pause, Step, and Stop |
| World inspection | Hierarchy and Inspector |
| Asset service | Asset Browser |
| World-edit command API | Gizmos, property editing, undo, and redo |
| Scenarios and snapshots | State and test-scene panels |
| Profiler stream | Profiler window |
| Build and package services | Build window |

The first editor will not reproduce Unity or Unreal's breadth. Its job is to
bring the run, observe, modify, and reproduce loop into one application.
