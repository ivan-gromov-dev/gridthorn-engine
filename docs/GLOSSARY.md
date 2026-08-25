# Project Terminology

This glossary gives architectural terms one meaning across design documents,
public APIs, diagnostics, and future tooling.

## Authoritative state

Gameplay state whose values determine simulation outcomes and are eligible for
saves, snapshots, replay verification, or state hashing. Render resources,
caches, interpolation state, and transient effects are not authoritative.

## Command-line command

A CLI operation such as `gridthorn run`. It invokes SDK services but is not a
simulation command or a world-edit command.

## Fixed tick

One discrete execution of authoritative simulation at a configured duration and
integer tick index. A rendered frame may execute zero or more fixed ticks.

## GameCommand

An immutable, serializable request to authoritative simulation, assigned to a
specific fixed tick. Player input, automation, tests, and replay use the same
ordering and validation rules for these commands.

## Plugin

An opt-in unit that registers public engine capabilities during application
construction. A plugin may add systems, resources, asset types, diagnostics, or
other plugins through the contract in [ARCHITECTURE.md](ARCHITECTURE.md#plugin-contract).

## Presentation state

Derived, non-authoritative state used by rendering, audio, UI, animation, and
debug visualization. It may be reconstructed from authoritative state.

## Replay

Compatibility metadata, an initial authoritative state or scenario, controlled
random seeds, and an ordered stream of `GameCommand` values.

## Scenario

A small, named starting state intended for development, automated tests, and
reproduction of a particular situation.

## Snapshot

A capture of authoritative state at one fixed tick for short-term development,
testing, or replay workflows. Long-lived user saves have a stricter migration
and compatibility contract.

## Stage

A named lifecycle boundary such as `Startup`, `PollEvents`, `Input`,
`FixedUpdate`, `Update`, `PostUpdate`, `Render`, `Suspend`, `Resume`, or
`Shutdown`. Stages define ordering; systems within a stage require explicit
ordering whenever their observable results could conflict.

## WorldEditCommand

A validated tooling operation that inspects or modifies world state and carries
the metadata required for undo, redo, diagnostics, and authorization. A
`WorldEditCommand` that changes authoritative state is converted into one or
more `GameCommand` values at a fixed-tick boundary rather than mutating that
state at an arbitrary point in a frame.
