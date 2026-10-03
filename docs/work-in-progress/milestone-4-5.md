# Milestone 4.5 checkpoint

Started 2026-10-03. Scope of the first increment: reproducible workbench CPU
baseline and code review of UI/text preparation. No engine optimization yet.
The complete engine-wide review and native acceptance remain outstanding.

## Starting state

- Engine HEAD: `16baf0eb9fc8f41c70fb37e7fd83296f60ae9649`.
- Examples HEAD: `c89adb9a5317007b3469782c1c8da9d8b4b1b04a`.
- Engine already had README, ROADMAP and UI edits and untracked PERFORMANCE_REVIEW.
- Examples already had workspace/lockfile/README edits and untracked workbench.
- Baselines therefore represent these working trees, not pristine commits.
- Rust 1.99.0, LLVM 23.1.1, x86_64-pc-windows-msvc. Registry reports AMD Ryzen
  5 5600X 6-Core Processor and Windows build 26200 / 25H2 (legacy ProductName
  reports Windows 10 Pro). CPU/GPU/driver queries via CIM were denied in sandbox.
  GPU/driver, power conditions and display details are unknown.

## Review evidence

`UiRouter::layout` in composition/routing/presentation.rs calls `UiTree::layout`,
which prepares paint and field geometry for all placements. Router then orders
and filters layers and paints again. This verifies discarded first paint and
work on closed layers; its isolated timing contribution remains unmeasured.
`TextSystem::layout` constructs and shapes a new Buffer on each call.
Raster glyph images are cached, but RasterText span collection is rebuilt.
Renderer geometry expands every visible raster span to six vertices per frame.
These are optimization candidates, not proof of GPU cost or measured speedups.

## Baseline workload

The sibling workbench now has `--performance`: four locales, DPI 1/2, 1000×800
logical pixels, ten warmups and 100 samples per operation. It reports nearest-rank
median/p95/p99/max microseconds for router layout, unchanged prepare, empty route,
render-input clone and direct mixed-script field invalidation. Construction and
first prepare are single observations. Operations share warmed fonts; editing
excludes command dispatch, localization refresh and native event processing.
Temporary result destruction is timed. Counters exclude GPU, present, geometry,
allocations and peak memory. The primitive count is top-level, not span/vertex count.

Changed example files: main.rs, interface/mod.rs, interface/performance.rs, README.
No engine source, dependency or public API changed.

## Next action

Retain raw debug/release CSV in ignored engine target output; record results and
checks here. Then instrument geometry/upload and native CPU/present separately,
including focused editing, IME, scrolling, modal animation and cold samples.
Establish hardware/power metadata before calling this a reference baseline.
Optimize duplicate paint only with regression coverage and before/after evidence.

## Debug results

One complete debug run succeeded, 100 warm samples per operation. Microseconds
below are p95. Raw output: `target/workbench-performance-debug.csv` (ignored).

| Locale | DPI | Router layout | Field invalidation | Idle prepare |
| --- | --- | --- | --- | --- |
| en-US | 1 | 136796 | 135221 | 7 |
| ru | 1 | 121986 | 130202 | 7 |
| ar-EG | 1 | 165442 | 163272 | 7 |
| ja | 1 | 352725 | 327294 | 7 |
| en-US | 2 | 156851 | 147998 | 6 |
| ru | 2 | 157882 | 138861 | 7 |
| ar-EG | 2 | 176242 | 167114 | 7 |
| ja | 2 | 350230 | 344890 | 6 |

This confirms expensive CPU preparation on invalidation in debug, not native
frame intervals or release acceptance. Geometry, GPU and presentation remain
unmeasured. Direct edits are unfocused and exclude localized preview refresh.

Domain disposition: UI/text has confirmed duplicate paint and measured debug
cost; rendering has span expansion requiring measurement; runtime/world/input/
localization scaling, assets/scenes/saves, simulation/grids/collision/snapshots,
audio/platform and build footprint are still pending, not accepted by this review.

## Release results

Two release runs succeeded with the same warm workload. The first run's microseconds are p95;
raw output is `target/workbench-performance-release.csv`. This is a first-host
sample, not a complete repeatability/hardware baseline or a measured optimization.

| Locale | DPI | Router layout | Field invalidation | Idle prepare |
| --- | --- | --- | --- | --- |
| en-US | 1 | 10858 | 7958 | 1 |
| ru | 1 | 6589 | 6165 | 1 |
| ar-EG | 1 | 10698 | 10034 | 2 |
| ja | 1 | 17808 | 16601 | 2 |
| en-US | 2 | 11819 | 10494 | 2 |
| ru | 2 | 11200 | 10970 | 2 |
| ar-EG | 2 | 11845 | 11485 | 2 |
| ja | 2 | 20279 | 18456 | 2 |

Japanese full layout exceeds 16.67 ms at both DPIs; direct field invalidation
exceeds it at DPI 2, before renderer geometry/upload or native input work.
The repeat (`target/workbench-performance-release-repeat.csv`) reproduces the
Japanese bottleneck: layout p95 19.491/19.851 ms at DPI 1/2 and field invalidation
p95 16.785/19.110 ms. Repeat Japanese DPI 1 layout p99 was 31.603 ms. Other
locales' layout p95 ranged from 8.003 to 14.069 ms in this repeat. Do not pool
these distributions or attribute run-to-run variance to an implementation change.
The other locales' preparation leaves only a fraction of the engine CPU budget;
passing this isolated timing is not whole-frame acceptance.
First release build took 2m15s including previously absent dependency artifacts;
this is not a controlled cold-build comparison.

## Checks

Example formatting and package Clippy (all targets, warnings denied) passed.
Debug and release `--performance` both completed all eight configurations.
Engine docs-only verification passed before the result tables were added.
Package tests passed: 5 tests, including public workflow and layout reuse.
Full `./scripts/verify.ps1` passed: formatting, check, Clippy, tests, dependency
boundaries and diff whitespace. Logs: `target/workbench-tests.log` and
`target/milestone-4-5-verify.log`. No dependency edge changed.
