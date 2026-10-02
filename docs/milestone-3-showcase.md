# Milestone 3 integrated showcase candidate

Implemented on 2026-10-02 in the sibling `tycoon_slice` example. Timber Harbor
combines all implemented Milestone 3 areas with Milestone 2 rendering, input,
UI, audio, reflection, scalar scenes and development asset reload through
public SDK APIs.

The current playable revision has a continuous workforce economy: renewable
forest plots, log/plank warehouses, sawmills, ports, four-person houses,
individually assigned lumberjacks/carpenters/porters and physical road transport.
Chopping/processing take multiple authoritative ticks. Each port sells exactly
one player-selected resource; the initial port sells logs. Sales earn gold,
construction/hiring cost gold and warehouse dispatch rotates fairly among
waiting porters. The starting state includes 120 gold and four housed workers.

A generated textured UI provides resource/site/worker counts, a game timer,
pause and 0.5×/1×/2×/4× speeds, selection/staff inspection and a menu toggled by
button or Escape. Save/load, world/camera snapshots, restart, sandbox and quit
are menu actions. Menu entry pauses simulation and resume preserves the player's
previous speed/manual pause. Three RGBA atlases and original PCM16 music/effects
are included with provenance. The [example guide](../../gridthorn-examples/tycoon_slice/README.md)
contains rules, costs, controls and the feature integration matrix.

## Integration decisions

The integer authoritative world belongs to one typed scenario root. A nested
ScenarioRuntime executes one exact tick from each outer FixedUpdate. The outer
runtime owns input, UI, camera, presentation, audio and the host accumulator.
No device/worker/presentation clock is serialized into the world save.

The original candidate corrected two engine integration gaps: screen-space UI
and timing draw after textured world sprites; WindowViewport publishes physical
creation/resize/zero extents for picking. The workforce revision adds no engine
crate dependency edges. Textured UI frames use public sprites anchored to the
viewport, composed after world artwork and before screen-space text. Nine-slice
frames, labels and button bounds scale together. Building roof picking and UI
interaction exclusion use the real camera/viewport.

Scenario revision 2 and `harbor-v2` save worker homes/jobs/positions, reserved and
carried resources, work counters, stocks, dispatch cursors and port resource
settings. Invalid jobs, cargo phases, overfull houses, stock counters, scenario
mismatches and missing market RNG are rejected without replacing the live world.
Revision-1 automatic-workshop saves are explicitly incompatible. In-memory
snapshots include a separate scalar camera bookmark; disk saves do not restore
camera state or the outer host frame accumulator.

## Local evidence

All 22 focused game tests passed. Coverage includes initial staffing, housing and
costs, protected edits, multi-tick processing, resource-specific ports, shared
porters, starvation-free dispatch, broken-road recovery, exact partition/snapshot/
save continuation, pending commands and RNG, malformed jobs/cargo/stocks,
atomic file replacement, reconstructed maps, load rollback, every speed option,
Escape/menu transitions, save/load during work, roof selection, blocked map
input, resized picking and camera scene validation.

Engine `scripts/verify.ps1` passed with `CARGO_NET_OFFLINE=true`, including the
boundary checker. Examples workspace format, Clippy and tests passed. The
no-device integration smoke and real window/GPU/audio smoke exited successfully.
CLI project check, scenario listing and headless launch passed. At seed 42/tick
1000, the default harbor produced 14 sales, 219 gold and canonical fingerprint
`a06206981f3ebecd`. The expanded two-port test hires multiple plank porters while
preserving the original raw-log port and sells both resource streams.

Remote CI, clean-checkout verification, cross-platform native output and a full
manual playthrough have not been independently observed in this session.

## Remaining acceptance and deferrals

This is an implemented showcase candidate. Final Milestone 3 completion awaits
the maintainer's playable review and milestone signoff; its roadmap checkbox
remains open.

Scope is a finite 12×12 map, one-cell buildings, two tile layers, two economic
scenarios and up to 64 workers. Square projection is a diagnostic view using
isometric building art. API stability, large-world frame/memory/compile-size
benchmarks, cross-platform output, arbitrary ECS snapshots, replay files,
cross-release migrations and distributable packaging remain deferred.
