# Architecture Direction

This document records architectural direction rather than final crate names or
every dependency choice. Decisions with long-term consequences should receive
an [Architecture Decision Record](adr/README.md) after a focused prototype.

Normative terms used below are defined in [GLOSSARY.md](GLOSSARY.md). Statements
using **must** or **must not** are architecture invariants. Statements using
**should** are defaults that may be changed by an accepted ADR.

## Layers

```text
Game code and game data
          ↓
Public Gridthorn SDK and plugins
          ↓
World | Simulation | Assets | Presentation
          ↓
Renderer | Audio | Input | Platform
```

Genre-specific systems do not belong in the minimal core. Isometric projection,
tilemaps, navigation, simulation helpers, and UI are standard plugins.

## Proposed workspace shape

```text
crates/
├── gridthorn              # SDK facade and prelude
├── gridthorn_app          # lifecycle, schedules, and states
├── gridthorn_world        # engine-facing ECS facade and world services
├── gridthorn_reflect      # type metadata and inspector API
├── gridthorn_render       # renderer abstraction and 2D pipeline
├── gridthorn_assets       # loading, handles, dependencies, and hot reload
├── gridthorn_input
├── gridthorn_audio
├── gridthorn_scene        # scenes, serialization, and migrations
├── gridthorn_simulation   # fixed ticks, clock, and headless execution
├── gridthorn_debug        # inspection, profiling, and diagnostics
└── gridthorn_cli

plugins/
├── sprite
├── tilemap
├── isometric
├── ui
├── physics2d
├── navigation
└── save

examples/
├── classic_2d
└── tycoon_slice
```

This layout will evolve. A crate boundary is justified only by a clear
responsibility, dependency boundary, or public API boundary.

## Dependency and ownership rules

The workspace must enforce allowed dependency edges rather than relying only on
convention:

| Source | Allowed project dependencies |
| --- | --- |
| Game and standard plugins | `gridthorn` facade and deliberately public subsystem APIs |
| `gridthorn` facade | Application and deliberately re-exported subsystem crates |
| Application orchestration | World, simulation, assets, input, audio, render, and diagnostics through narrow services |
| Simulation | World and deterministic core utilities |
| Scene and save | World, reflection, and stable asset identifiers |
| Renderer | Stable asset identifiers and the platform graphics adapter |
| World and reflection | Foundation utilities; no presentation or platform subsystem |
| Platform adapters | Engine-owned core types and their selected external backends |

- The `gridthorn` facade may re-export public APIs but must not contain hidden
  runtime behavior.
- Foundation crates must not depend on the facade or on game code.
- Authoritative world and simulation crates must not depend on rendering,
  audio, windowing, editor UI, or a GPU.
- Platform adapters deliver events and services through engine-owned types.
  Third-party implementation types cross a public boundary only through an
  explicit interoperability decision.
- Application orchestration owns lifecycle execution. Subsystems own their
  internal resources and expose narrow services rather than reaching into each
  other's storage.
- Scene and save code serializes registered authoritative data. It must not
  serialize renderer handles, caches, operating-system objects, or live tasks.
- Standard plugins depend on public SDK contracts. They must not gain privileged
  access unavailable to third-party plugins.

Allowed and forbidden crate dependencies will be checked in CI once the
workspace exists. Any exception requires an ADR that records the replacement
boundary and its consequences.

## Application lifecycle

The fundamental stages distinguish frame-based presentation from fixed-step
simulation:

```text
Startup
   ↓
PollEvents → Input → FixedUpdate (zero or more times)
                         ↓
             Update → PostUpdate → Render

Platform lifecycle events → Suspend / Resume / Shutdown
```

- `Startup` registers completed application state and runs once before the first
  frame. A failed startup returns a contextual error without entering the loop.
- `PollEvents` collects platform events without changing authoritative state.
- `Input` maps the collected events and continuous device state into immutable
  `GameCommand` values before any fixed tick for that host frame.
- A command is assigned to the next fixed tick that has not started. If the
  frame executes no fixed tick, the command remains queued. With multiple ticks,
  an edge-triggered command is consumed once; continuous input is represented by
  an explicit tick input state rather than repeated platform events.
- `FixedUpdate` advances the integer tick index and handles economy, AI, and
  other authoritative rules. Authoritative simulation data may only change at a
  fixed-tick boundary or during explicit load/reset operations.
- `Update` handles frame-based cameras, animation, effects, and non-authoritative
  UI. UI actions that affect simulation enqueue commands for a future fixed
  tick.
- `PostUpdate` completes presentation extraction, resolves deferred
  presentation work, and prepares immutable render input.
- `Render` consumes presentation state. It must not modify authoritative state.
- Pausing or changing simulation speed must not pause development UI.
- A headless runtime executes the world and simulation without a renderer.
- `Suspend` and `Resume` rebuild platform resources when necessary without
  advancing simulation time. `Shutdown` stops accepting work, flushes permitted
  persistent operations, and releases subsystem resources in reverse dependency
  order.

The fixed duration, maximum interactive catch-up ticks, and overload policy are
explicit configuration. Fixed tick duration never stretches to hide a slow
frame. Headless tests do not silently drop ticks. Interactive applications may
cap catch-up work, but must emit a diagnostic and preserve the authoritative
tick sequence.

## Data flow

```text
Platform input, tests, automation, or tools
      ↓
ordered GameCommand queue
      ↓
Authoritative simulation state
      ↓
Presentation state
      ↓
Rendering, audio, and UI
```

Rendered output is never the source of truth for gameplay. `GameCommand` values
provide a common boundary for input, tests, replay, and automation.

Tooling uses a separate `WorldEditCommand` contract because inspection, undo,
redo, and authorization have different requirements. A world edit that changes
authoritative state is validated and converted into ordered `GameCommand`
values at a fixed-tick boundary. Read-only inspection does not enter the
simulation command stream.

## Determinism contract

The initial guarantee is reproducibility for the same Gridthorn version, target
triple, enabled features, initial state, fixed-step configuration, command
stream, and random seeds. Bit-identical results across different CPU
architectures or operating systems are not promised until a milestone validates
and documents that stronger scope.

Within the supported scope:

- authoritative time is the integer fixed-tick index and configured tick
  duration, never wall-clock time;
- authoritative randomness comes from engine-managed, explicitly seeded named
  streams whose consumption order is stable;
- systems with observable conflicts have explicit ordering; scheduler
  parallelism must not change results;
- iteration order from hash maps, parallel queries, filesystem enumeration, or
  asset discovery must not become authoritative unless it is normalized;
- deferred entity, event, and command changes become visible only at documented
  schedule boundaries and in stable order;
- a replay records compatibility metadata and is rejected with an actionable
  error when its required contract is unsupported;
- automated tests run identical scenarios more than once and compare an
  authoritative state hash after selected ticks.

Floating-point values may be used in presentation. Authoritative floating-point
use requires a focused decision that documents platform scope, normalization,
and comparison behavior. Systems that cannot satisfy the determinism contract
must be explicitly classified as non-authoritative.

## Foundations required by tooling

### Reflection

Components and resources may expose field metadata. Reflection supports the
inspector, serialization, and diagnostics, while custom types may provide custom
editor representations.

### Versioned serialization

Project, scene, and save formats carry explicit versions. Runtime-only data such
as GPU handles, caches, and transient events is reconstructed rather than stored
as authoritative state.

Each persisted document must identify its format, schema version, and required
engine compatibility. Migrations are ordered, testable transformations that do
not depend on ambient time, filesystem order, or implicit randomness. Loading
must either produce a fully validated state or leave the previous state intact;
partial application is not allowed.

Unknown required types or unsupported future versions produce contextual errors
rather than silent data loss. User saves use atomic replacement where the
platform permits it. Snapshots and replays may have a narrower compatibility
window than user saves, but that window must be stated in their metadata and
public documentation.

### Command API

Tools modify a world through `WorldEditCommand` values. This is the basis for
undo and redo, action recording, remote inspection, and the future editor.
Simulation tests and replay use `GameCommand` values instead. The two command
types may share validation infrastructure, but are not interchangeable APIs.

### Plugin contract

Plugins are registered during application construction before `Startup`. A
plugin may register types, resources, systems, asset loaders, diagnostics, and
other plugins through public SDK services.

- Every plugin has a stable identifier and is registered at most once.
- Required plugin dependencies and ordering constraints are declared before
  schedule construction. Missing dependencies and cycles are startup errors.
- System ordering uses stable labels rather than Rust type names or incidental
  registration order.
- Plugins receive only documented capabilities and may not access subsystem
  internals through downcasting or hidden global state.
- Dynamic plugin unloading is not part of the initial SDK contract. Runtime
  state changes use commands and lifecycle services instead.
- Disabling a plugin must avoid linking its optional heavy dependencies unless a
  shared enabled capability requires them.

The first plugin spike must validate diagnostics for missing dependencies,
duplicate registration, ordering cycles, and incompatible feature sets.

### Development protocol

The CLI and future editor manage a running game through a constrained local
protocol: pause, resume, step, inspect, modify, snapshot, and profile. The
initial direction is a separate game process, so a rebuild or game crash does
not terminate development tools.

The initial protocol is local-only, version-negotiated, and capability-based.
It must reject incompatible clients and must not expose arbitrary memory access
or code execution. Transport and authentication details require an ADR before
the protocol is implemented.

## API-first rule

Official tooling receives no hidden privileged access to the game world. When a
tool lacks a capability, we first design an appropriate SDK API. This keeps the
engine extensible for third-party tools and game-specific editors.

Third-party implementation types should not leak through Gridthorn's public API
unless exposing them is a deliberate interoperability decision. In particular,
the SDK will provide facades around the ECS, renderer, audio, and windowing
libraries selected in [TECHNOLOGY.md](TECHNOLOGY.md).

### Public API contract

- The `gridthorn` facade is the default application-facing entry point. Direct
  subsystem crates are public only when independent use is an intentional and
  tested contract.
- A prelude is curated for common, low-conflict names. Adding a broadly named
  item to it requires the same compatibility review as changing a public API.
- Recoverable failures return typed, contextual errors. Public library code must
  not panic for invalid user data, unavailable platform capabilities, or
  ordinary runtime failure.
- Thread-affinity and `Send`/`Sync` expectations are documented for public
  handles and callbacks. Callers must not infer them from a provisional backend.
- Experimental APIs are visibly marked and isolated behind opt-in features.
  Stable documentation must not present them as compatibility guarantees.
- Public behavior includes defaults, ordering, diagnostics, serialization, and
  side effects, not only Rust type signatures.

SemVer and deprecation rules will be finalized before the first public release.
Until then, examples and status tables must identify provisional API sketches so
they cannot be mistaken for supported interfaces.

## Validation

Each major abstraction must be proven in real use:

- `classic_2d` validates simplicity and general-purpose design.
- `tycoon_slice` validates grids, isometric projection, fixed ticks, world
  scale, saves, and diagnostics.

A subsystem is not complete merely because it compiles. It requires a usage
example, relevant tests, and public API documentation.
