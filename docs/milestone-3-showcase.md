# Milestone 3 integrated showcase candidate

Implemented on 2026-10-02 in the sibling `tycoon_slice` example. Timber Harbor is
a playable isometric production/harbor game consuming public SDK APIs. It combines
all implemented Milestone 3 areas with Milestone 2 presentation, UI, audio,
reflection, scalar scene persistence and development asset reload.

The game has construction costs, connected production, routed deliveries, rent
and upkeep, seeded market prices, a concrete win condition, restart, and a sandbox.
Its generated RGBA art, procedural PCM16 music/effects and source provenance live
with the example. The [example guide](../../gridthorn-examples/tycoon_slice/README.md)
maps each milestone capability to gameplay and describes every control.

## Integration decisions

The complete integer authoritative world resides in one typed scenario root.
A nested ScenarioRuntime executes one exact tick from each outer FixedUpdate.
The outer runtime remains a non-authoritative owner of window input, camera,
UI, presentation, audio and the host frame accumulator. This uses the current
supported snapshot/save subset without claiming arbitrary ECS or interactive
clock restoration.

Two integration gaps were corrected in the engine: screen-space UI/timing now
draws after textured world sprites, and WindowViewport publishes actual physical
creation/resize/zero extents to frame systems for correct camera picking. Neither
change introduces dependencies or exposes platform/backend types. Domain tests
cover geometry partition and viewport publication; the game tests resized picking.

## Evidence

Focused game tests cover a winnable contract, rejected placement/move rollback,
road-break recovery, deterministic request partitioning, fresh-runner snapshot
and save continuation, queued commands/RNG, invalid-load rollback, atomic file
replacement and reconstructed maps, menu/pause/step, resized input, and validated
camera scene round-trip.

The local verification includes engine `scripts/verify.ps1`, examples workspace
format/Clippy/tests, the no-device integration smoke, the native window/GPU smoke,
and CLI project check, scenario listing, headless launch and graphical launch.
Detailed check results and the native smoke output belong to the implementation
handoff; remote CI and a clean-checkout run have not been independently observed
in this session.

Local results: the full engine verifier passed with `CARGO_NET_OFFLINE=true`
after the existing generated-project test attempted inaccessible crates.io.
Examples workspace Clippy and tests passed. All 11 focused tycoon tests passed,
along with both smoke modes and the four CLI workflows above. The harbor CLI run
at seed 42/tick 1000 reported 99 shipments, 1,287 coins and fingerprint
`288d555e869c5503`; this default scenario needs player-built cottages to win.

## Remaining acceptance and deferrals

This is an implemented showcase candidate. Final Milestone 3 completion remains
pending the maintainer's playable review and milestone signoff; the roadmap must
not label that acceptance complete prematurely.

Supported showcase scale is 12×12 cells, single-cell buildings, two tile layers,
64 concurrent couriers, and a small economic contract. API stability, large-world
frame time/memory/compile-size benchmarks, cross-platform native output, wider
footprints in this particular game, arbitrary ECS snapshots, cross-release save
migrations, replay files, responsive UI layout and distributable packaging remain
explicitly deferred. The square toggle is a diagnostic view with isometric art.
