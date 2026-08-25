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

These commands define target UX; they are not implemented yet.

## CLI evolution

The first implemented CLI will intentionally be small but real:

- `gridthorn new` creates a project from an embedded minimal template.
- `gridthorn run` launches the project through Cargo.
- `gridthorn check` validates the Rust workspace and project manifest.
- `gridthorn --version` reports SDK and CLI compatibility information.

As engine capabilities become available, the CLI adds asset validation, file
watching, scenarios, tests, builds, and packaging. Business logic belongs in
reusable SDK crates; CLI subcommands remain thin adapters so the future editor
can invoke the same services.

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
