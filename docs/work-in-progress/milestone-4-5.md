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

## Native window callback CPU increment

Starting engine HEAD: `5c9504cbc31f392225b0f610e135a6071c4e7440`, clean tree;
examples HEAD remains `c89adb9a5317007b3469782c1c8da9d8b4b1b04a` with existing
workbench changes. This increment changes only engine sources/documentation.
`GRIDTHORN_WINDOW_PERFORMANCE` enables bounded independent preparation/redraw
callback samples, printed at drop without per-frame logging. Preparation includes
runtime schedules, frame extraction and platform control; redraw includes the
renderer call and its acquisition/presentation blocking. Event-loop waiting,
native input callbacks, startup and shutdown are excluded. Independent phase
indices cannot be paired to construct whole-frame samples. This reduces a CPU
measurement gap without closing whole-engine CPU acceptance.

The first two release cohorts (`target/window-<mode>-<locale>[-repeat].log`)
overlapped repository verification and generated-project compilation. They are
exploratory observations only, not controlled baselines; Japanese editing repeat
preparation p95 reached 46.303 ms under that contention. Do not compare these
cohorts as an engine regression. Subsequent controlled cohorts run after
verification completes, without concurrent agent-launched build/test work.

Full `./scripts/verify.ps1` passed: workspace formatting/check/Clippy/tests,
generated-project workflows, dependency boundaries and whitespace. Log:
`target/milestone-4-5-window-verify.log`. The new domain test covers disabled
collection and independent phase storage limits. No SDK API or dependency changed.

After verification, two sequential release runs per mode/locale succeeded without
concurrent agent-launched compilation. Environment enabled both window and render
diagnostics; executable:
`../gridthorn-examples/target/release/gridthorn_example_multilingual_workbench.exe`,
arguments `--<idle|editing>-smoke --locale=<en-US|ru|ar-EG|ja>`.
Logs: `target/window-controlled-<mode>-<locale>-<1|2>.log`; summary:
`target/window-controlled-summary.csv`. Metadata remains RTX 3070/Vulkan/616.56,
Fifo, native DPI 1, physical 1000×800. Display refresh and power/frequency during
sampling remain unrecorded. Each run collected 121 preparation and 119 redraw
callbacks. Excluding each phase's first ten leaves 111 and 109 samples respectively;
independent callback scheduling need not match host-frame counts.
Percentiles use nearest rank; all values below are milliseconds.

| Mode/locale | Preparation p95, first/repeat | Redraw p95, first/repeat |
| --- | --- | --- |
| Idle en-US | 0.142 / 0.128 | 9.497 / 9.375 |
| Idle ru | 0.116 / 0.116 | 9.147 / 9.247 |
| Idle ar-EG | 0.121 / 0.105 | 9.241 / 9.215 |
| Idle ja | 0.130 / 0.115 | 9.187 / 9.508 |
| Editing en-US | 6.336 / 5.431 | 5.140 / 5.464 |
| Editing ru | 6.553 / 6.382 | 4.836 / 4.738 |
| Editing ar-EG | 6.472 / 6.616 | 1.537 / 1.647 |
| Editing ja | 13.338 / 11.877 | 2.306 / 2.027 |

Japanese editing preparation p99 was 15.982 / 12.976 ms. These include UI routing,
localized preview refresh and layout rather than text shaping alone. The next
focused step is to separate those preparation components before choosing a fix.
In the first idle Japanese run, acquisition p95 was 8.769 ms versus encode
0.069 ms; editing acquisition p95 was 0.067 ms versus encode 1.472 ms. This
supports treating idle redraw wall time as partly pacing/waiting, rather than
attributing it entirely to renderer CPU geometry work.
Do not add phase percentiles or claim whole-frame budget acceptance: input
callbacks and actual displayed intervals remain unmeasured, and redraw includes
surface/presentation waits. Native DPI 2 and OS IME are still outstanding.

## Workbench native input-phase increment

Starting engine HEAD: `1252ee44735e44a3e4ed29e095d1491988a13dd6` with a clean
engine tree; examples HEAD remains `c89adb9a5317007b3469782c1c8da9d8b4b1b04a`
with existing manifest/README/workbench changes. This increment adds an opt-in
example-owned `GRIDTHORN_WORKBENCH_PERFORMANCE` collector: up to 240 successful
input-system samples, printed at drop. Disabled/full collection takes no phase
timestamps. Input preparation is extracted into a focused method; route/effect
ordering and existing smoke workloads are preserved.

Rows contain prepare-before, real-input route, workload route, effects,
prepare-after and anchor refresh; nested snapshot/layout timings accumulate
within the same host input frame. Animation/headless preparation is not collected.
Effects in isolated editing include localized-preview refresh. Do not add nested
snapshot/layout durations to their parent prepare durations. The probe excludes
stdout/platform publication/render extraction and is not whole-frame CPU time.

Package Clippy and all 7 workbench tests passed; logs:
`target/workbench-phases-clippy.log`, `target/workbench-phases-tests.log`.
The new collector test checks disabled collection, storage limits and resets.
Release build passed: `target/workbench-phases-build.log`.

Full engine `./scripts/verify.ps1` passed (workspace formatting/check/Clippy/tests,
generated-project workflows, dependency boundaries and whitespace):
`target/milestone-4-5-phases-verify.log`. Native measurements started only after
verification and package builds finished. Two release runs per mode/locale passed,
with all three performance environment variables enabled and no concurrent
agent-launched compilation. Commands used the built workbench executable with
`--<idle|editing>-smoke --locale=<en-US|ru|ar-EG|ja>`.
Logs: `target/workbench-phases-<mode>-<locale>-<1|2>.log`; summary:
`target/workbench-phases-summary.csv`. Native metadata remains RTX 3070, Vulkan,
NVIDIA 616.56, Fifo, DPI 1, physical 1000×800; refresh/power conditions remain
unrecorded. Warm input samples use host frames 11–120, 110 per run, including the
exit-request frame; these do not require a matching successful presentation.
Nearest-rank p95 values below are milliseconds.

| Editing locale | Router layout p95, first/repeat | Effects/preview p95, first/repeat | Injected workload route p95, first/repeat |
| --- | --- | --- | --- |
| en-US | 5.439 / 5.910 | 0.049 / 0.093 | 0.028 / 0.034 |
| ru | 6.524 / 6.398 | 0.069 / 0.079 | 0.034 / 0.032 |
| ar-EG | 7.414 / 8.695 | 0.064 / 0.105 | 0.030 / 0.047 |
| ja | 12.319 / 11.336 | 0.098 / 0.055 | 0.043 / 0.030 |

Japanese layout p99: 14.093 / 12.524 ms. Aggregated layout microseconds account for
97.49–98.82% of the sum of the six outer input phases across each warm editing
cohort, excluding nested snapshot/layout from that sum. This is a measured
component share, not a whole-runtime CPU percentage. All warm idle frames skip
router layout; cached prepare-before p95 is 0.013–0.027 ms and prepare-after
0.004–0.009 ms. Empty anchor route and snapshot capture remain small in this
workload. Headless end-to-end passed with diagnostics unset and no diagnostic rows:
`target/workbench-phases-headless.log`.

No optimization chosen from aggregate layout timing alone. Router layout includes
sizing/arrangement, painting and focused field geometry; the next focused review
must distinguish these costs, including closed-layer sizing and unchanged text
preparation, before selecting a cache/invalidation change. OS IME, native DPI 2,
whole-engine CPU/GPU budgets and displayed-frame acceptance remain open.

## Router arrangement measurement reuse increment

Starting engine HEAD remains `1252ee44735e44a3e4ed29e095d1491988a13dd6`;
the preceding input-phase documentation was still uncommitted and is preserved.
Examples retain their existing workbench/manifest changes. Added private opt-in
router layout diagnostics via `GRIDTHORN_UI_PERFORMANCE`, with at most 240 rows,
nonblocking collector access and explicit skipped reporting. Cloned routers
share their collector; the final owner prints samples. Arrangement/layer ordering,
text geometry and paint are separate timings; focused decoration is nested paint.
Sample indices count successful layout calls, not native host/present frames.

The initial two native release editing probes per en-US/ja locale found arrangement
dominant. Logs: `target/ui-layout-before-<en-US|ja>-<1|2>.log`. Excluding the first
ten layout calls leaves 103 calls per run. Arrangement p95 was 5.558 / 6.160 ms
in en-US and 11.719 / 10.454 ms in ja. Japanese paint p95 was 2.080 / 1.924 ms;
text geometry 0.143 / 0.123 ms, with focused-decoration p95 0.136 / 0.133 ms
nested in paint. Measurements ran without agent-launched compilation.

Review found repeated measurement of identical captions/wrapping widths during
recursive sizing and arrangement, including content extents and auto heights.
The focused fix shares measurement results within a single arrangement pass.
Text and exact effective wrapping width form the key; the theme and font service
are fixed for the pass. Bitmap measurements bypass the cache. Cache keys/results
are discarded before returning the layout; at most 1024 entries and 1 MiB copied
UTF-8 keys are stored, excluding map/allocator overhead. Misses after either limit
still measure normally, and errors are never cached. No subtree is skipped and
closed layers retain their existing sizing semantics. No dependency or SDK API
was added. Focused tests cover reuse, different text/widths, errors, entry/key-byte
limits, fresh-cache results, disabled phase timers and busy-collector skipping.

Full `./scripts/verify.ps1` passed: formatting, workspace check/Clippy/tests
(application: 110 passed, 1 existing ignored), generated-project workflows,
dependency boundaries and whitespace. Log:
`target/milestone-4-5-arrangement-verify.log`. Workbench package Clippy and all
7 tests passed; logs `target/ui-layout-cache-workbench-clippy.log` and
`target/ui-layout-cache-workbench-tests.log`. Final release build:
`target/ui-layout-cache-final-build.log`. These checks finished before final
native measurements. No sibling example source change was needed in this increment.

Final release editing matrix: two runs per four locales, with only
`GRIDTHORN_UI_PERFORMANCE=1`, the same diagnostic setting as the before probes.
Arguments: `--editing-smoke --locale=<en-US|ru|ar-EG|ja>`. Logs:
`target/ui-layout-final-<locale>-<1|2>.log`; summary of before/final cohorts:
`target/ui-layout-summary.csv`. All editing runs collected 113 layout calls with
zero collector skips. Excluding the first ten leaves 103 samples per run.
Native metadata logs physical 1000×800 and scale factor 1. Reference GPU/backend
is from the earlier renderer-enabled cohort; this probe does not log adapter,
refresh or power/frequency. Runs had no concurrent agent-launched build/test work.
Nearest-rank p95 values below are milliseconds.

| Locale | Arrangement before, first/repeat | Arrangement final, first/repeat | Layout phases total before, first/repeat | Layout phases total final, first/repeat |
| --- | --- | --- | --- | --- |
| en-US | 5.558 / 6.160 | 1.629 / 1.388 | 6.993 / 7.642 | 3.070 / 2.675 |
| ru | Not measured in this before cohort | 1.397 / 1.380 | Not measured in this before cohort | 2.881 / 2.679 |
| ar-EG | Not measured in this before cohort | 1.798 / 1.756 | Not measured in this before cohort | 3.257 / 2.958 |
| ja | 11.719 / 10.454 | 2.758 / 2.560 | 13.941 / 12.204 | 4.221 / 4.419 |

Layout total is computed per sample by summing arrangement, text geometry and
paint before calculating percentiles; nested focused decoration is excluded from
that sum. The total excludes diagnostic collector publication and minor wrapper
work, and is not whole-engine frame CPU. Japanese total p99: 5.480 / 4.553 ms,
versus 14.694 / 13.591 ms before. Early after probes are exploratory repetitions
in `target/ui-layout-after-<en-US|ja>-<1|2>.log`, not the final cohort.
Native mixed smoke and headless behavior also passed:
`target/ui-layout-cache-mixed.log`, `target/ui-layout-cache-headless.log`.
The latter ran with UI diagnostics unset.

The measured arrangement issue is resolved for this workload without a persistent
text cache or a visibility/sizing semantic change. Remaining review includes
paint/raster preparation, closed-layer sizing, allocations, OS IME/native DPI 2
and whole-engine CPU/GPU/display acceptance. This component improvement does not
close the milestone's native frame-budget gate.

## Focused field geometry reuse increment

Starting engine HEAD: `0d46bcc63d198a783a44c0c4245700bcd97d9cdc`, clean tree.
Examples retain their prior manifest/README/workbench changes. Review found that
router layout prepares each visible field's geometry for input/native anchors,
then focused-decoration painting repeats preparation/shaping of the same field.
The fix passes the already prepared geometry into decoration paint. Selection,
caret, clipping and stale-editor-value fallback remain unchanged; active preedit
still prepares its distinct composition string. This is a focused duplication
removal, not a persistent text/raster cache or a promise of a whole-frame gain.

The domain regression test paints caret/selection from a prepared geometry without
a font service, verifies identical primitives, and confirms fresh layout still
requires a valid asset-font service. Existing routing/selection/preedit/DPI and
sibling workbench tests cover normal workflows. No SDK API/dependency edge or
example source change was added.

Before probes: two release editing runs per en-US/ja locale, with only
`GRIDTHORN_UI_PERFORMANCE=1`, no concurrent agent-launched build/test work.
Logs: `target/ui-decoration-before-<locale>-<1|2>.log`. Final measurements started
after repository verification and the package release build finished.

Full `./scripts/verify.ps1` passed, including 111 application tests (1 existing
ignored), workspace formatting/check/Clippy/tests, generated-project workflows,
dependency boundaries and whitespace. Log:
`target/milestone-4-5-decoration-verify.log`. Workbench package Clippy, 7 tests and
release build passed: `target/ui-decoration-workbench-clippy.log`,
`target/ui-decoration-workbench-tests.log`, `target/ui-decoration-build.log`.

Final release editing probes: two runs per four locales with only UI diagnostics
enabled and no concurrent agent-launched compilation. Logs:
`target/ui-decoration-after-<locale>-<1|2>.log`; summary:
`target/ui-decoration-summary.csv`. All editing runs collected 113 successful
layout calls with zero collector skips. Analysis excludes the first ten layout
calls, leaving 103 samples per run; nearest-rank p95 values below are microseconds.
Native metadata remains physical 1000×800, DPI 1. Adapter/refresh/power conditions
are not recorded by this probe, as in the preceding arrangement cohort.

| Locale | Focused decoration before, first/repeat | Focused decoration after, first/repeat | Paint before, first/repeat | Paint after, first/repeat |
| --- | --- | --- | --- | --- |
| en-US | 138 / 111 | 1 / 1 | 1313 / 1023 | 1223 / 1059 |
| ru | Not measured | 1 / 1 | Not measured | 1237 / 1061 |
| ar-EG | Not measured | 1 / 1 | Not measured | 1162 / 1020 |
| ja | 130 / 128 | 1 / 1 | 1962 / 1810 | 1914 / 1587 |

Focused time is nested paint, with integer microsecond reporting. Paint now calls
decoration only for prepared text fields, avoiding no-op decoration calls on other
nodes as well as duplicate field shaping. Layout-phase total is computed per row
from arrangement + text geometry + paint, excluding nested focused time. Japanese
total p95 before was 4.486 / 4.049 ms; after 5.413 / 3.711 ms. This spread does
not establish a whole-layout or whole-frame percentile improvement; the supported
claim is elimination of duplicate geometry preparation and reduced focused cost.

Native mixed smoke and headless public workflows passed:
`target/ui-decoration-mixed.log`, `target/ui-decoration-headless.log`.
Remaining paint cost includes caption shaping/rasterization; review it separately
before introducing retained raster caches. Native OS IME/DPI 2, whole-engine CPU,
GPU/display acceptance and remaining domain scaling reviews remain outstanding.

## Text-service shaping/raster phase increment

Starting engine HEAD remains `0d46bcc63d198a783a44c0c4245700bcd97d9cdc`; the
focused-field reuse increment was still uncommitted and is preserved. Examples
retain their prior changes. Added private opt-in `GRIDTHORN_TEXT_PERFORMANCE`
collection per font service, with independent limits of 8192 successful layout
and rasterize operations. Default services create no collector or timers.
Full collectors stop taking timestamps for that operation but retain successful
call totals. Failed operations are excluded. Rows print only at service drop.

Layout duration includes validation, shaping and detached layout construction;
raster duration includes validation, glyph-cache lookup/iteration, pixel-span
merging and immutable snapshot construction. Layout units are UTF-8 input bytes;
raster units are output spans, not sampled pixels or uploaded GPU bytes. The
probe includes every caller, including measurement and editing geometry; it is
not a paint-only timer. Independent indices must not be paired as native frames.
The domain test covers separate operation limits and counts beyond truncation.

Full `./scripts/verify.ps1` passed (including 42 renderer tests), along with
workbench package Clippy, 7 tests and release build. Logs:
`target/milestone-4-5-text-phases-verify.log`,
`target/text-phases-workbench-clippy.log`, `target/text-phases-workbench-tests.log`,
`target/text-phases-build.log`. Native sampling began after all checks/builds
finished. No sibling example source change was needed. Disabled-diagnostics
headless workflow passed with no `text_cpu` rows: `target/text-phases-headless.log`.

Two sequential native release runs per idle/editing mode and four locales passed
with only `GRIDTHORN_TEXT_PERFORMANCE=1` enabled and no concurrent agent-launched
build/test work. Logs: `target/text-phases-<mode>-<locale>-<1|2>.log`; summary:
`target/text-phases-summary.csv`. Native logs report physical 1000×800 and DPI 1;
adapter/refresh/power conditions are not logged by this probe. Each idle run
performed only 33 layout and 10 rasterize calls for initial preparation. Each
editing run performed 3729 layout and 1130 rasterize calls, exactly 33 and 10
per each of the 113 router layouts in the established workload. Successful totals
equal stored totals in every run; no truncation occurred.

| Editing locale | All layout-call CPU sum ms, first/repeat | All rasterize-call CPU sum ms, first/repeat | Layout call p95 µs, first/repeat | Rasterize call p95 µs, first/repeat |
| --- | --- | --- | --- | --- |
| en-US | 147.670 / 152.270 | 38.380 / 38.950 | 100 / 105 | 106 / 106 |
| ru | 169.720 / 144.390 | 48.230 / 43.090 | 120 / 95 | 146 / 123 |
| ar-EG | 186.790 / 177.540 | 36.590 / 33.840 | 158 / 154 | 83 / 78 |
| ja | 298.010 / 299.000 | 40.750 / 38.500 | 425 / 426 | 118 / 115 |

Sums include cold/initial operations over the full run and use integer
microsecond reporting. Per-call nearest-rank percentiles exclude the first 100
calls of each operation independently (3629 layout and 1030 raster samples).
These exclusions are not matched host frames and must not be used to form
whole-frame samples. Calls have differing text sizes; these are workload call
distributions, not normalized per-character costs. Idle has no samples after
that exclusion, not a zero-cost shaping/raster benchmark. Raster output totals
are deterministic across repeats: approximately 1.84–2.56 million spans for the
editing cohorts, but no allocation/memory or GPU-upload budget is inferred.

The measured review priority is shaping, including arrangement/geometry as well
as paint, rather than glyph rasterization alone. Code review shows every dirty
UI layout still reshapes unchanged captions and reconstructs their snapshots;
these timings do not individually identify which captions repeat. The next
focused cache decision must validate text/style/font-owner matching, eviction,
bounded retained memory and cold/warm behavior before implementing persistent
reuse. Native OS IME/DPI 2, whole-engine CPU/GPU/display acceptance and other
domain scaling reviews remain open.

## Service-local shaped layout reuse increment

Starting engine HEAD: `06c7678a5bbdd5526f0fd4cddd163cd06d0bbd1a`, clean tree.
Examples retain their existing workbench/manifest changes. Implemented a private
per-service LRU for identical text and complete TextStyle. Validation still runs
before cache lookup; family, font size, line height, width, wrapping and alignment
all participate. Each service fixes its font set/shaping locale and owns its cache;
recreating it after font reload cannot reuse a prior service's entries.

TextLayout now shares its immutable backend buffer and line diagnostics through
Arc; cache hits clone ownership handles rather than copying buffer/glyph arrays.
Returned layouts survive eviction/service destruction for diagnostics; rasterize
still rejects a foreign service owner. DPI and color remain raster inputs rather
than logical-shaping keys. Glyph-cache clearing retains shaped layouts, consistent
with the existing raster-only clearing contract. No public API or dependency edge
was added, and no raster snapshot cache was introduced.

The LRU retains at most 64 entries, 256 KiB copied text/family keys, 16384
diagnostic glyphs and 1024 diagnostic lines; limits cause eviction, while an
individually oversized result is returned without retention. Limits do not count
opaque Buffer allocations, font/glyph-cache memory, map/allocator overhead or
layouts retained by callers. This is a bounded retention policy, not an exact
whole-service byte budget. Tests exercise complete-style/text matching, font-owner
isolation, invalid metrics after warm reuse, LRU recency, aggregate key/glyph/line
limits, oversized rejection and rasterization of still-live layouts after eviction.

Full `./scripts/verify.ps1` passed, including 45 renderer tests, workspace
formatting/check/Clippy/tests, generated-project workflows, dependency boundaries
and whitespace: `target/milestone-4-5-layout-cache-verify.log`. Workbench package
Clippy, 7 tests and release build passed: `target/layout-cache-workbench-clippy.log`,
`target/layout-cache-workbench-tests.log`, `target/layout-cache-build.log`.
Final native measurements started after all checks/builds completed, with no
concurrent agent-launched build/test work. No example source changes were needed.

Two native release runs per idle/editing mode and all four locales used only
`GRIDTHORN_TEXT_PERFORMANCE=1`, matching the preceding text-service baseline.
Logs: `target/layout-cache-text-<mode>-<locale>-<1|2>.log`; summary:
`target/layout-cache-text-summary.csv`. Each editing run still records 3729 layout
and 1130 rasterize calls without truncation. Input-byte and output-span aggregates
match the before cohort exactly in all eight editing runs. Counts measure public
layout requests, including cache hits, not actual shaping executions. Idle still
records only 33 initial layout and 10 initial raster calls.

| Editing locale | All layout-call CPU sum before ms, first/repeat | All layout-call CPU sum after ms, first/repeat | All rasterize-call CPU sum after ms, first/repeat |
| --- | --- | --- | --- |
| en-US | 147.670 / 152.270 | 29.700 / 26.890 | 38.900 / 40.300 |
| ru | 169.720 / 144.390 | 37.710 / 26.940 | 59.130 / 40.340 |
| ar-EG | 186.790 / 177.540 | 28.430 / 28.230 | 34.830 / 34.440 |
| ja | 298.010 / 299.000 | 33.630 / 33.180 | 42.090 / 40.200 |

Sums include cold/initial work. Per-call p95 excludes the first 100 calls of each
operation, independently, as before; Japanese layout p95 is now 113 / 117 µs
versus 425 / 426 µs. Raster p95 remains 116 / 116 µs, versus 118 / 115 µs before.
The cache reduces repeated shaping; it does not reduce request counts or eliminate
raster snapshot construction. Retained backend/allocator memory and cold service
construction time have not been measured in this increment.

A separate eight-run editing cohort enabled only `GRIDTHORN_UI_PERFORMANCE=1`,
matching the prior decoration-reuse UI baseline. Logs:
`target/layout-cache-ui-<locale>-<1|2>.log`; summary:
`target/layout-cache-ui-summary.csv`. All runs collected 113 layout calls with
zero skips; excluding the first ten gives 103 warm samples. Total is computed
per row as arrangement + text geometry + paint, excluding nested focused paint.
Nearest-rank p95/p99 values below are milliseconds.

| Locale | UI-layout total p95 after, first/repeat | UI-layout total p99 after, first/repeat |
| --- | --- | --- |
| en-US | 0.624 / 0.849 | 0.674 / 1.400 |
| ru | 0.867 / 0.681 | 1.059 / 0.718 |
| ar-EG | 0.620 / 0.605 | 0.674 / 0.664 |
| ja | 0.733 / 0.721 | 0.778 / 0.782 |

Japanese UI-layout p95 before was 5.413 / 3.711 ms, en-US 2.585 / 2.429 ms in
the decoration-reuse cohort. This is a layout-component improvement, not
whole-engine frame CPU/GPU/display acceptance. Native logs report 1000×800 and
DPI 1; refresh/power conditions remain unrecorded. Native mixed smoke and
headless workflows passed: `target/layout-cache-mixed.log`,
`target/layout-cache-headless.log`. Remaining review includes raster-snapshot
preparation, retained memory, OS IME/native DPI 2 and other implemented domains.
