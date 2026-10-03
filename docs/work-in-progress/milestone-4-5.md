# Milestone 4.5 checkpoint

Started 2026-10-03. Scope of the first increment: reproducible workbench CPU
baseline and code review of UI/text preparation, followed by removal of discarded
router paint and closed-layer field geometry.
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
Duplicate paint has now been removed with regression coverage and release samples.

Native renderer CPU instrumentation and in-place clipped geometry have since been
implemented below. Next measure whole-engine frame work, GPU execution and
presented intervals, then complete the interaction/DPI/platform matrix.

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

## Single-paint increment

During this increment engine HEAD advanced to
`7ef3d8e138a1e70b4d9d70515a5cbeebc499f674`; the performance plan/checkpoint are
now tracked. Examples HEAD remains unchanged. The focused optimization remains
an uncommitted working-tree change; unrelated work was preserved.

`UiTree::arrange_layout` is private composition preparation shared by tree/router.
Public tree layout retains paint/field geometry for all authored placements.
Router orders/filters before preparing visible field geometry and one paint pass.
Hidden nodes still participate in sizing/arrangement; no cache, dependency or
public API was added. Paint/caret preparation for a focused visible field remains.

Two new domain tests check undecorated router/tree equivalence under clipping,
scrolling, zero viewport and DPI 1/2, and excluded closed-layer field geometry with
reopening/text-session recovery. The latter acknowledges native text-stop feedback
between reopen cycles, as required by the existing session lifecycle.
All 50 composition tests pass.

First post-change release run (p95 milliseconds):

| Locale | DPI | Router layout | Field invalidation |
| --- | --- | --- | --- |
| en-US | 1 | 5.441 | 5.298 |
| ru | 1 | 5.430 | 5.876 |
| ar-EG | 1 | 5.870 | 6.645 |
| ja | 1 | 10.518 | 12.105 |
| en-US | 2 | 8.994 | 6.286 |
| ru | 2 | 8.527 | 6.249 |
| ar-EG | 2 | 7.210 | 6.512 |
| ja | 2 | 12.680 | 11.212 |

Raw output: `target/workbench-performance-release-single-paint.csv`.
Japanese DPI 2 layout median drops from 16.388/16.581 ms in baseline runs to
10.734 ms; p95 drops from 20.279/19.851 to 12.680 ms. This measures the combined
removal of discarded paint and hidden-field preparation, not isolated shaping,
raster or GPU time. Native frame acceptance and focused editing remain outstanding.

Repeat output: `target/workbench-performance-release-single-paint-repeat.csv`.
Japanese p95 layout is 10.727/14.289 ms at DPI 1/2, and field invalidation is
10.763/12.561 ms. DPI 2 layout medians in the two post-change runs are
10.734/11.237 ms versus 16.388/16.581 ms before the change (about 32–35% lower).
The combined optimization is consistently beneficial across these samples;
power/display state remains uncontrolled. Worst repeat Japanese DPI 2 field
invalidation was 19.003 ms, so isolated p95 success does not guarantee every frame.

The sibling package's 5 tests and release native `--smoke` passed after this
change. Logs: `target/workbench-single-paint-tests.log` and
`target/workbench-single-paint-native.log`. Smoke verifies lifecycle/submission,
not visual/IME acceptance or GPU timing. Full verification initially found a
similar-names lint in the new test; bindings were renamed and verification rerun.
Full `./scripts/verify.ps1` then passed, including workspace tests, generated
project workflows and dependency boundaries. Log:
`target/milestone-4-5-single-paint-verify.log`.

## Native renderer increment

Added opt-in renderer diagnostics controlled by `GRIDTHORN_RENDER_PERFORMANCE`.
It retains at most 240 successful-frame samples and emits CSV on renderer drop;
normal runs collect no timers or samples. Pipeline upload code moved into focused
surface/uploads.rs to keep encoding within the existing complexity lint.
This is temporary measurement tooling for 4.5, not a public profiler subsystem.
No dependency or public API changed.

Native release workbench `--smoke` ran 119 rendered frames. Analyze frames 10–118
(109 samples): scripted locale changes, nested windows and transitions mixed in
one distribution. This is neither a steady idle run nor an interaction-specific
acceptance matrix. Animations use elapsed time, so geometry positions can vary
between runs. Adapter on the post-change run: NVIDIA GeForce RTX 3070, Vulkan,
NVIDIA driver 616.56, Fifo, physical surface 1000×800. Host monitor DPI/refresh and
power state remain unknown; this does not substitute for DPI 1/2 native validation.

Baseline geometry uses per-clipped-subtree vectors; optimized geometry appends
into a single frame vector and clamps only the appended range after each subtree.
This retains nested clipping and earlier/later sibling geometry. Added a sibling
isolation regression test; all 38 renderer tests passed.

| CPU operation | Before median/p95 ms | After median/p95 ms |
| --- | --- | --- |
| Colored/UI geometry | 5.865 / 8.973 | 2.252 / 4.468 |
| Resource preparation | 0.949 / 1.183 | 0.637 / 0.993 |
| Encode (includes geometry/resources) | 6.926 / 10.248 | 3.080 / 5.610 |

Generated vertex count median/p95 matches at 169896/219660; vertex bytes at
5436672/7029120. Geometry copies are reduced but vertex counts/upload size remain
large. Raw logs: `target/workbench-render-baseline.log` and
`target/workbench-render-append.log`. First baseline lacked configuration metadata;
post-change metadata comes from the same native workload/host. CPU submit/present
and acquire are recorded, but GPU execution and presented intervals are unmeasured.
Repeat results follow below.

The repeat (`target/workbench-render-append-repeat.log`) also completed 119
frames with matching median/p95 vertex counts. Warm frames 10–118: geometry
median/p95 2.261/4.204 ms, resources 0.645/0.980 ms, encode 3.114/5.384 ms.
Acquire median/p95 0.030/0.048 ms, submit 0.307/0.382 ms, present CPU call
0.174/2.994 ms. Present max was 3.266 ms; these calls cannot establish GPU work
or frame cadence. Two post-change native runs reproduce reduced geometry cost;
only one baseline run exists, and this mixed smoke cannot provide general
interaction budgets. GPU upload byte volume is unchanged, leaving span geometry
and GPU resource lifetime as the next measurement/optimization candidates.

Full `./scripts/verify.ps1` passed after removing clone-on-Copy calls in the new
regression test: formatting, check, Clippy, workspace tests, generated-project
workflows, dependency boundaries and diff whitespace. Log:
`target/milestone-4-5-render-verify.log`. No new example source or manifest changes
were needed for these native runs. GPU execution, frame intervals, allocation
counts and the full milestone workload/acceptance matrix remain outstanding.

## Retained renderer increment

Starting engine revision: `8f3cd5dc606129a1b6a676c88eba884409177c4d`, clean working
tree. Previous increments are committed; this increment changes renderer code,
its domain tests and documentation. Examples HEAD remains
`c89adb9a5317007b3469782c1c8da9d8b4b1b04a`; existing working-tree workbench is used.

Retain one colored/UI input snapshot, CPU geometry and immutable GPU vertex buffer
per pipeline. Check camera, colored sprites, UI order/clips, overlay and extent.
Raster reuse requires shared pixel storage and exact position/DPI; independently
rasterized equal text conservatively rebuilds. Public equality is unchanged.
Textured resources are still rebuilt per frame and excluded from the retained key.
Empty frames remove the colored buffer. Reconfiguration recreates the cache;
occlusion retains it. Dirty geometry clears/refills the CPU vector, retaining
high-water capacity until reconfiguration/shutdown.

The initial cache attempt allocated a new vector on dirty frames and had costly
misses. Removed deep raster comparisons from the private predicate and reused
CPU vertex capacity. Discarded logs: `target/workbench-render-cache.log` and
`target/workbench-render-cache-shared.log`; those are not final gains.
New tests compare cached/fresh geometry across camera/sprites, order/clips,
overlay, empty frames, resize, raster storage, placement, DPI and tint.
All 40 renderer tests and package Clippy passed during iteration.

Final release logs: `target/workbench-render-retained.log` and
`target/workbench-render-retained-repeat.log`, 119 frames each; analyze 10–118
(109 samples). Same RTX 3070/Vulkan/616.56/Fifo/1000×800 configuration.
Wall-clock animations mean faster runs spend more of the 120-frame script inside
transitions; cache-hit fractions are not a fixed idle/interaction matrix.

| CPU operation | First median/p95 ms | Repeat median/p95 ms |
| --- | --- | --- |
| Geometry including cache check | 0.786 / 1.966 | 0.631 / 0.985 |
| Resource preparation | 0.643 / 1.014 | 0.609 / 0.766 |
| Encode (includes geometry/resources) | 1.465 / 2.946 | 1.286 / 1.806 |

Hits: 14/109 and 12/109; hit geometry p95 2/1 microseconds, encode 24/15
microseconds. Colored upload on hits is zero. Generated vertex-data bytes total
594088704 per warm run; actual uploads total 540508800/548163072 (9.0%/7.7% lower
in these mixed scripts). Dirty-frame upload volume is unchanged. These counters
exclude textures, staging/padding and GPU bandwidth. Peak retained CPU vertex
capacity in the repeat is 12582912 bytes (12 MiB), excluding snapshot/raster data,
GPU buffers, driver/staging and other memory. One high-water vector is retained;
this is not a universal total engine memory limit.

Host present-call interval p95/p99: 21.252/26.714 and 18.277/24.652 ms; worst warm
interval 36.795/31.990 ms. First interval is zero and excluded. This is CPU cadence
including engine/event-loop work, not compositor/display timing. PresentMon is
not on PATH; actual presented intervals/GPU execution remain unmeasured. These
values do not close native acceptance. Next establish GPU/display measurements,
isolated idle/editing/IME/animation workloads and native DPI/refresh metadata.

Full `./scripts/verify.ps1` passed: formatting, workspace check/Clippy/tests,
generated-project workflows, dependency boundaries and whitespace. Log:
`target/milestone-4-5-retained-verify.log`. Both final release native smoke runs
passed. No public API, dependency edge or sibling example source changed.

## GPU-pass and isolated native workload increment

Starting engine HEAD: `0138eae221cb415fc4cca547b1fa5752d5a92bb2` with a clean tree.
Examples retain their existing untracked workbench. Added opt-in GPU timestamps
only on adapters supporting TIMESTAMP_QUERY. One query/readback slot, nonblocking
device polling and bounded storage avoid waiting in the render loop. Busy slots
are skipped; errors/pending samples are reported. CPU/GPU frame indices continue
across pipeline reconfiguration. Default renderer feature requests remain unchanged.
Tests validate timestamp byte decoding, backend period, incomplete/reversed data,
non-finite/zero periods; native Vulkan readback is exercised by actual runs.

The sibling workbench adds mutually exclusive `--idle-smoke` / `--editing-smoke`
and `--locale=en-US|ru|ar-EG|ja`. Editing focuses the field, warms ten host frames,
selects the complete value and injects bounded mixed-script replacement via the
public router each subsequent frame; normal localized preview refresh also runs.
It measures injected text routing/selection/layout rather than OS IME. A domain
test verifies focus and replacement without unbounded text growth; 6 package tests
and package Clippy passed. No new dependency or SDK public API was added.

Two native release runs per mode/locale, 119 frames each; warm analysis uses
frame indices 10–118 (109 CPU/GPU pairs per configuration). All logs report native
scale factor 1 and physical 1000×800, RTX 3070/Vulkan/616.56/Fifo. Active Windows
power scheme queried after the runs was Balanced; monitor refresh and CPU power/
frequency during sampling are not recorded. Log naming:
`target/workbench-gpu-<idle|editing>-<en-US|ru|ar-EG|ja>[-repeat].log`.

| Mode/locale | GPU pass p95 µs, first/repeat | Renderer encode p95 ms, first/repeat | Host cadence p99 ms, first/repeat |
| --- | --- | --- | --- |
| Idle en-US | 32 / 32 | 0.075 / 0.064 | 9.576 / 9.832 |
| Idle ru | 33 / 33 | 0.074 / 0.060 | 9.630 / 9.672 |
| Idle ar-EG | 26 / 26 | 0.097 / 0.099 | 11.114 / 9.921 |
| Idle ja | 30 / 30 | 0.060 / 0.080 | 9.581 / 9.632 |
| Editing en-US | 31 / 31 | 1.615 / 1.262 | 10.301 / 10.826 |
| Editing ru | 32 / 32 | 1.345 / 1.393 | 10.257 / 9.969 |
| Editing ar-EG | 25 / 25 | 1.041 / 1.043 | 9.438 / 9.658 |
| Editing ja | 29 / 29 | 1.219 / 1.295 | 12.521 / 14.884 |

Idle: all 109 warm frames hit the colored/UI cache, with no colored uploads.
Editing: no warm cache hits, as expected for replacement every frame. No GPU
readback errors or pending shutdown samples. Japanese editing repeat collected
118 total GPU samples with one skipped cold frame; all warm pair counts remain
109. Other runs collected all 119 samples. Initial mixed smoke also succeeded:
`target/workbench-gpu-mixed.log`, 119 GPU samples, no skips/errors/pending.

GPU pass timing excludes upload execution, queue waits, resolve/copy work and
display/compositor timing. Host cadence remains a CPU present-call proxy. These
measurements establish the pass component only, not whole-GPU/engine CPU budgets
or acceptance. Native DPI 2, isolated clipboard/IME/pointer/scroll/animation,
whole-engine CPU and actual displayed intervals remain outstanding. No new
optimization chosen from these measurements; pass execution is small on this
host and remaining preparation/transfer/presentation costs need separate review.

Full `./scripts/verify.ps1` passed, including 41 renderer tests, workspace
formatting/check/Clippy/tests, generated-project workflows, dependency boundaries
and whitespace. Log: `target/milestone-4-5-gpu-verify.log`. All 16 isolated native
release runs and the mixed smoke passed; the workbench's 6 tests and package
Clippy passed. Roadmap completion marks include the GPU-pass diagnostics and
isolated idle/editing evidence; broader acceptance items remain open.
