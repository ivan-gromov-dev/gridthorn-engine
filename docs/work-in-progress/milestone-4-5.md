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

## Shaped-layout cache retention review increment

Starting engine HEAD: `5b7463cc98a526ec8bdc593f9ef82b08ac654f8d`, clean tree.
Examples retain their existing workbench changes. Extended existing opt-in text
diagnostics with per-service cache hit/miss/eviction/oversized-bypass totals and
final/peak retained entry/key/glyph/line counts. Default cache instances collect
no diagnostic counters. Data prints at cache destruction; no per-call logging
or exact backend byte accounting was introduced.

A Weak-based domain test confirms cached buffers remain alive without a caller,
then are released after LRU eviction when no external layout remains. Separate
tests from the preceding increment preserve snapshots still held by callers.
This verifies ownership release, not allocator/OS page reclamation.

Full `./scripts/verify.ps1` passed, including 46 renderer tests and workspace
formatting/check/Clippy/tests, generated-project workflows, dependency boundaries
and whitespace: `target/milestone-4-5-cache-retention-verify.log`. Workbench Clippy,
7 tests and release build passed: `target/cache-retention-workbench-clippy.log`,
`target/cache-retention-workbench-tests.log`, `target/cache-retention-build.log`.
Sampling started after all checks/builds finished, without concurrent
agent-launched compilation. Headless workflow passed with diagnostics unset:
`target/cache-retention-headless.log`.

Two release native runs per idle/editing mode and four locales enabled only
`GRIDTHORN_TEXT_PERFORMANCE=1`. stdout/stderr logs:
`target/cache-retention-<mode>-<locale>-<1|2>.<stdout|stderr>.log`; cache summary:
`target/cache-retention-summary.csv`. Every editing run reports 3484 hits and
245 misses (93.43% hit rate), 181 evictions, zero oversized bypasses, peak/final
64 entries and peak 70 lines. Peaks by locale are deterministic across repeats:

| Locale | Peak copied key bytes | Peak diagnostic glyphs | Idle retained entries |
| --- | --- | --- | --- |
| en-US | 3534 | 1869 | 23 |
| ru | 4365 | 2068 | 23 |
| ar-EG | 3814 | 1753 | 23 |
| ja | 3983 | 1637 | 23 |

Idle reports 10 hits, 23 misses, no evictions or bypasses, and 29 retained lines.
The bounded replacement stream reaches the entry limit and evicts old inputs;
the deterministic peaks provide a workload retention envelope rather than an
universal memory budget or an indefinitely long leak test.

Two existing public `--performance` CPU workloads (four locales, logical DPI 1/2)
also passed, exercising fresh services, repeated layouts and 110 unique edit
values per configuration. Logs: `target/cache-retention-cpu-<1|2>.<stdout|stderr>.log`;
cold observation summary: `target/cache-retention-cold-summary.csv`. Fresh service
construction observations ranged 7.133–10.337 ms and first prepare 6.734–9.788 ms
across 16 configurations. These are single observations per configuration/run,
not percentile distributions, and the OS/font-file cache was not cleared. Each
CPU configuration reported 7160 hits, 133 misses, 69 evictions, peak/final 64
entries and zero bypasses. DPI 2 here is CPU geometry/raster testing, not native
monitor DPI acceptance.

Process memory was read every 50 ms while each child remained alive, through
diagnostic reporting at teardown, using Process.PeakWorkingSet64 and
PrivateMemorySize64. Summary: `target/cache-retention-process-summary.csv`.
Native observed peak working-set values ranged 205.7–211.5 MiB; sampled maximum
private bytes 380.1–386.7 MiB. CPU-only process observations were 31.3 / 30.9 MiB
working set and 14.0 / 22.3 MiB sampled private bytes. Polling can miss short-lived
peaks, including the final shutdown interval; diagnostic buffers/output handling
are included. These whole-process observations include renderer/driver/font/
runtime allocations and cannot attribute a cache-only memory delta without an
equivalent uncached baseline. No memory-limit tuning is justified from this data.

Retention/release checks are complete for this bounded workload. Remaining review
includes precise allocation/backend memory accounting, whole-engine CPU/GPU/display
acceptance, native OS IME/DPI 2 and other implemented domain scaling workloads.

## Runtime schedule diagnostics — 2026-10-03

Starting engine revision: f48f6946eda7df55bd0cbd2ef1e6b49260f3f5ea.
Added private opt-in bounded runtime stage timing, independent phase limits and
no timestamps when disabled/full. Existing lifecycle/order tests cover unchanged
execution; focused tests cover disabled collection and independent caps.
Example sources were unchanged; existing sibling changes were preserved.

Release build and four native Japanese idle/editing smoke runs passed, without
concurrent build/test work. Logs: target/runtime-<idle|editing>-<1|2>.stderr.log;
summary: target/runtime-summary.csv. Runtime/preparation collected 121 calls,
redraw 119; warm summaries exclude each phase's first ten calls independently.
Editing Input/transitions p95 1078/1126 us; other stages <=13 us. Window preparation
1150/1202 us. Idle Input 85/58 us; preparation 155/128 us. Redraw includes blocking
and cannot establish pure CPU or displayed cadence. No empty-schedule/scaling
claim or extraction-specific measurement is made. Next: isolate extraction and
empty-runtime overhead, then increasing entity/event/control workloads.

Verification: ./scripts/verify.ps1 passed (workspace checks, Clippy, domain and CLI end-to-end tests, dependency boundaries and documentation checks). Initial Clippy range_plus_one finding was corrected before the successful run. Release example build and all four native smoke runs passed.

## Empty runtime and render-frame extraction — 2026-10-03

Engine start: b58a6cd04717ff115973bfa816b971ec60a04c11; sibling HEAD:
c89adb9a5317007b3469782c1c8da9d8b4b1b04a. Engine started clean; existing sibling
manifest/lockfile/README changes and untracked workbench were preserved. No example
sources were changed. No dependency edges or public types changed.

Added independent bounded extraction timing to WindowPerformance around only
lifecycle.render_frame when a renderer exists. It reads/clones the world snapshot
in the standard runtime, excludes set_frame/drop of the previous renderer snapshot,
and is nested inside preparation. Disabled/full extraction collectors take no
clock reading. Focused collector tests: two passed, including the independent cap.

Added a manually ignored release probe under runtime/test/overhead.rs, using public
runtime/schedule contracts. Fresh runtime per configuration, startup then 1000
warm frames, 100 batches of 1000 frames. Configurations: 0/1/32 no-op systems on
six stages, 0/1/8 fixed ticks per frame with exact synthetic elapsed values.
GRIDTHORN_RUNTIME_PERFORMANCE unset. Batch wall time includes black_box, loop and
fixed-step checks; setup/teardown/stdout excluded. Two probe runs passed, 900 batch
rows each. Empty zero-tick means 2.435/2.386 us; eight ticks 6.842/6.751 us.
32-system zero-tick means 2.592/2.591 us; eight ticks 7.479/7.239 us. Full table is
in PERFORMANCE_REVIEW.md. Batch-mean percentiles cannot represent frame tails.

Builds completed before all measurements; no concurrent agent-launched builds or
tests. Release app test build and workbench build passed. Exact probe command:
cargo test -p gridthorn_app --release --locked measure_empty_runtime_and_schedule_dispatch -- --ignored --nocapture --test-threads=1.
Raw logs: target/runtime-overhead-<1|2>.log; target/runtime-overhead-summary.csv.
Four native Japanese smoke runs passed (idle/editing, twice each, only window
flag). Logs: target/extraction-<idle|editing>-<1|2>.stderr.log and .stdout.log;
summary target/extraction-summary.csv. 121 extraction/preparation and 119 redraw
samples each. Warm phase index >=10: extraction p95 idle 2/1 us, editing 2/2 us;
editing preparation 1038/1199 us, idle 121/118 us. Microseconds are truncated.

Disposition: no change to dispatch or snapshot cloning warranted by these bounded
measurements. No populated ECS, event/control count, native IME/DPI 2, backend
memory or display acceptance claim. Next: populated-world/event scaling and their
domain contracts; keep broad runtime/world/input/localization review open.

Verification completed: ./scripts/verify.ps1 passed (format, workspace check,
Clippy, workspace tests including CLI end-to-end fixtures, dependency boundaries
and diff whitespace). App: 114 passed, 2 ignored (existing desktop clipboard test and
new manually executed release probe). The two manual release probe runs passed;
all four native workbench runs passed. Full log:
target/milestone-4-5-runtime-extraction-verify.log.

## Homogeneous world and input publication scaling — 2026-10-03

Engine HEAD remains b58a6cd04717ff115973bfa816b971ec60a04c11. Previous runtime
extraction/empty-overhead increment is still uncommitted and preserved. This
increment adds only two manually ignored domain probes and their module wiring,
plus performance evidence/roadmap updates; no production behavior changes.
Sibling HEAD remains c89adb9a5317007b3469782c1c8da9d8b4b1b04a and no sibling files
were edited. Previously dirty manifests/README and untracked workbench are retained.

Probe files: runtime/test/world_scaling.rs and window/runtime/test/input_scaling.rs
under gridthorn_app/src. World: 0/1000/10,000/100,000 homogeneous u64 components,
resident zero-tick idle vs one fixed-tick system incrementing every component.
32 warm frames, 50 batches of 32, exact final counter values and visited count
verified outside timing. Two runs passed (400 batch rows each). Idle population
has no demonstrated overhead trend; 100,000-component traversal means 102.91 and
124.93 us/frame. No mixed/churn/archetype or game-system generalization.

Input: fresh empty runtime per case, 0/32/1024/16,384 events, pointer / full KeyD
press-release / indexed mixed-script commits. Eight warm frames, 32 batches of
eight, exact ordered event content checked after every batch outside timing;
next snapshot contains no leftover events. Timer includes fixture event cloning,
InputBuffer ingestion/snapshot, world replacement/drop and runtime run, with no
native window or input consumer. Two runs passed (384 batch rows each). At 1024
mean us/frame: pointer 55.38/51.20, keyboard 174.66/172.67, commit 148.87/143.48;
16,384: pointer 550.79/567.68, keyboard 3301.93/3387.77, commit 3699.07/3767.74.
Commit bytes/frame: 1558/51,114/840,858 for 32/1024/16,384 events. Full tables,
methodology and reproducible Cargo commands are in PERFORMANCE_REVIEW.md.

Release test build passed. Each probe was run twice alone with --release --locked,
--ignored --nocapture --test-threads=1 and GRIDTHORN_RUNTIME_PERFORMANCE unset.
No concurrent agent-launched build/test work during samples. Logs:
target/world-scaling-<1|2>.log and target/input-scaling-<1|2>.log; summaries:
target/world-scaling-summary.csv and target/input-scaling-summary.csv.
Initial targeted Clippy found precision-loss casting and manual modulus; both
were corrected before the measured release build. Batch-mean percentiles are not
frame-tail measurements. No evidence-based need for runtime/world optimization;
large input burst clone/allocation attribution remains before any input change.
Next: UI control/routing/localization scaling and mixed/churn ECS workload review.

Host metadata read after probes: CPU registry name AMD Ryzen 5 5600X 6-Core
Processor; Environment.OSVersion 10.0.26200.0; rustc 1.99.0 (b940084d7 2026-09-28);
powercfg active scheme Balanced. Live frequency/background load were not measured.

Verification completed: ./scripts/verify.ps1 passed (format, workspace check,
Clippy, workspace/CLI end-to-end tests, dependency boundaries and whitespace).
App: 114 passed, 4 ignored (desktop clipboard and three manual probes; the two
new probes were each executed successfully twice in release mode). Full log:
target/milestone-4-5-world-input-scaling-verify.log. Production behavior and
sibling sources are unchanged by this increment; prior increment changes remain.

## Control/routing and localization scaling, plain-scope fix — 2026-10-03

Engine HEAD remains b58a6cd04717ff115973bfa816b971ec60a04c11. Earlier increments'
uncommitted changes are preserved. Sibling files/dirty manifests, README and
untracked workbench remain unchanged by this increment. New manual probe modules:
ui/composition/test/scaling.rs and localization runtime/test/scaling.rs. Private
production change is Cow-based input/event scopes in routing/layers.rs: borrow
when layer_roots is empty, otherwise retain owned filtering. No dependency edge
or public type changes. Atomic tree/router replacement is unchanged.

UI: 16/128/1024 buttons, bitmap font, DPI 1, all controls visible in a tall logical
viewport; layout, empty route and 32 pointer first/last/miss cases. Eight warm
calls, 32 batches of eight. Two baseline and two post-fix runs passed (480 rows
per run). 1024-control first-event-burst means 7141.44/7192.09 → 2144.71/2027.17 us;
last 7619.23/7554.99 → 2099.14/2104.05; miss 7387.98/7322.85 → 2043.93/1999.91.
Empty route remains 2017.67/2036.18 us after; layout 2478.85/2538.02 us after,
not changed by the fix. Remaining large-tree base costs require attribution.
Before logs target/ui-scaling-<1|2>.log; after ui-scaling-after-<1|2>.log; respective
-summary.csv files preserve every configuration and batch-mean p95. No layered
native-workbench speedup claimed: its registered roots retain the owned path.

Localization: four catalogs/locales en-US,ru,ar-EG,ja; exactly 16/256/4096 entries
per catalog, non-English selection with explicit en-US fallback. 69 configurations
per run: literal/interpolation/plural-NUMBER/decimal-NUMBER/standalone-decimal plus
non-English-only fallback lookup. Prebuilt IDs/parameters/catalogs, 100 warm calls,
50 batches of 100. Two runs passed (3450 rows each); all mean ranges by operation
are recorded in PERFORMANCE_REVIEW.md, 0.12–1.76 us across the matrix. Locale and
output consistency verified outside timing. Cold construction/validation/replace
and failures are outside measurement. No optimization justified for warm calls.
Logs target/localization-scaling-<1|2>.log; summaries localization-scaling-summary.csv
and localization-scaling-ranges.csv. Same host/toolchain as the preceding probes.

Builds completed before measurements; no concurrent agent builds/tests during
sampling. Engine release test build and sibling workbench release build passed.
Exact Cargo probe commands are in PERFORMANCE_REVIEW.md; both packages are named
in each to preserve feature unification. 54 targeted UI tests passed, including
new registration after plain routing, closed filtering, reopening and unchanged
detached geometry/paint. Native --smoke passed after change; logs:
target/ui-scope-native.stdout.log and .stderr.log. Initial targeted Clippy
format_push_string finding was corrected before sampling using writeln!.
Next: attribute remaining tree/layout/visual-refresh scaling, then layered/text
routing, cold localization publication and mixed ECS workloads.

Verification completed: ./scripts/verify.ps1 passed (format, workspace check,
Clippy, workspace/CLI end-to-end tests, dependency boundaries and whitespace).
App: 115 passed, 5 ignored; new UI/localization manual probes were executed twice
(and UI twice again after the fix). The native mixed smoke and 54 focused UI
checks passed. Full log: target/milestone-4-5-ui-localization-scaling-verify.log.
Changelog correction also removes earlier runtime-diagnostic entries mistakenly
duplicated in historical release sections; both new runtime/UI entries belong
only to Unreleased. No staging or commits were performed.

## Indexed UI node reads/commands — 2026-10-03

Starting engine revision e75b1a43beca31925e7a7396106d21ff8dfd55a7, clean working tree.
Sibling unchanged; prior manifest/lockfile/README dirt and untracked workbench
preserved. Production files: new node_index.rs plus composition mod/tree/layout;
regressions under composition/test/node_lookup.rs. No dependency/public type
expansion. UiTree holds Arc<NodeIndex> with ID → boxed child path; private root
ensures topology replacement is owned by tree.rs. New/replace validate before
building index; commands/styles do not change shape. Clones share metadata and
retain independent mutable roots; Debug keeps root/theme output via a narrowly
reasoned missing_fields_in_debug allowance. Initial Clippy finding was corrected
with that explicit contract before final verification.

Inspection: previous node reads and commands walked all siblings recursively;
paint and visual refresh repeat these lookups. Same public UI release probe now
adds construction/drop, including source root cloning/validation/index creation
and release. Both implementations ran twice, 18 configs, eight warm calls,
32 batches of eight, 576 raw rows per run. Sampling ran after all builds, without
concurrent agent-launched build/test work. Feature unification held constant by
naming app and localization packages in the exact existing probe command.

At 1024 buttons: layout means 3172.63/2687.73 → 713.66/728.03 us; empty routing
1975.26/1944.58 → 154.72/146.32 us; 32 first pointer events 2221.78/2046.54 →
214.39/185.68 us. Construction/drop 153.71/172.90 → 211.20/230.55 us. Full table
and limits in PERFORMANCE_REVIEW.md. Low-count layout is effectively unchanged
within variation; construction is more expensive at every measured size. No
whole-engine/native gain or single-call percentile is asserted.

Raw logs target/node-index-before-<1|2>.log and node-index-after-<1|2>.log;
combined summary target/node-index-summary.csv. Probe source and flags identical
before/after. Flat index copies 1024 child indices (8192 bytes on this host), plus
unmeasured map/allocator metadata. Per-index existing shape bounds conservatively
limit payload to 4096*63 usize values (~1.969 MiB on 64-bit); this is a structural
bound rather than allocator/process memory measurement. Caller-retained trees
may hold multiple topologies; clone sharing does not impose a process-wide cap.

58 targeted UI tests passed, including four semantic regressions for replaced/
moved/reordered IDs, sparse u64::MAX IDs, old-clone isolation, failed replacement,
unknown IDs and valid 4096-node / depth-63 trees. Release test build and sibling
workbench release build passed. Native --smoke passed; logs node-index-native
.stdout.log / .stderr.log under target. Next: layered/text routing and retained
memory, allocation/layout attribution and remaining domain scaling.

Verification completed: ./scripts/verify.ps1 passed (format, workspace check,
Clippy, workspace and CLI end-to-end tests, dependency boundaries, diff whitespace).
App: 119 passed, 5 ignored; the UI manual probe ran twice before and twice after.
Native mixed smoke passed. Full log: target/milestone-4-5-node-index-verify.log.
No staging/commits. Sibling HEAD c89adb9a5317007b3469782c1c8da9d8b4b1b04a and its
existing Cargo.lock/Cargo.toml/README modifications and untracked workbench remain.

## Registered-layer text-field routing — 2026-10-03

Continued on engine e75b1a43beca31925e7a7396106d21ff8dfd55a7 with the preceding
indexed-node increment still uncommitted; preserved all those changes. Sibling
HEAD and existing dirt unchanged. Added domain-owned manual layered_scaling.rs
and a DPI-2 closed/open/modal detached-layout regression in test/routing/layers.rs.
Production change is confined to routing/layers.rs: input scopes copy geometry
and placements without render primitives, and placement filters use BTreeSet IDs
instead of repeated linear membership searches. Placement order, immutable raw
layout, modal policy and atomic routing semantics are preserved. No dependencies.

Same release probe ran twice before, twice with paint-only change, twice with
combined change: 16/128/1024 bitmap fields, closed/open/modal, empty/32 pointer
events; 18 configurations, eight warm calls, 32 batches of eight (576 rows/run).
Layout/registration/opening outside timing; atomic clones/scopes/results inside.
No concurrent agent-launched builds/tests during sampling. Asset-font editing,
overlapping layers and native publication are excluded. Detailed protocol/table
and limits are in PERFORMANCE_REVIEW.md. Intermediate paint-only samples were
not stable enough to claim open/modal gains; membership filtering was then fixed.

At 1024 fields/32 events means before → combined (run 1/2): closed
16981.21/17724.97 → 9593.86/9800.97 us; open 31208.49/31140.13 →
15020.45/15677.30; modal 38009.30/38067.18 → 18477.88/17484.05.
Closed empty-route means did not improve (679.92/686.87 → 774.98/738.60).
Modal burst still exceeds 16.67 ms. No whole-engine/frame-budget claim.
Filtered scopes retain no paint but still copy text geometry per event; no
allocator-byte or process-memory delta was measured. Follow-up: repeated scope
setup, geometry copies, focus checks and hit testing, then shaped editing/layers.

Logs target/layered-before-<1|2>.log, paint-only layered-after-<1|2>.log,
combined layered-final-<1|2>.log; summary layered-summary.csv. Focused domain
tests: 51 passed, 2 manual probes ignored. Release probe build and sibling release
build passed. Native --smoke and --editing-smoke --locale=ja passed; logs
layered-native-mixed / layered-native-editing .stdout.log/.stderr.log under target.
Initial regression attempted to inspect a private routing method; rewritten to
exercise public hit_test_layers without widening production visibility.
Full verification log: target/milestone-4-5-layered-routing-verify.log.
No staging or commits. README/UI/roadmap/changelog updated in the same increment.

Verification completed: ./scripts/verify.ps1 passed (format, workspace check,
Clippy, workspace/CLI tests, dependency boundaries and diff whitespace). App:
120 passed, 6 manual probes ignored. Final release probe ran twice separately;
native mixed and Japanese editing smokes passed. Both repositories' prior dirt
remains preserved.

## Shared immutable field geometry — 2026-10-03

Engine starting revision 26661b17cc8531c86c3b8d6863f55e7685d3359a, clean working
tree: preceding indexed-node and layered-routing increments were committed before
this continuation. Sibling HEAD c89adb9a5317007b3469782c1c8da9d8b4b1b04a,
existing manifest/lockfile/README dirt and untracked workbench preserved.
Production: layout.rs stores prepared field geometry in Arc<BTreeMap>; both tree
layout and routing/presentation.rs publish fresh immutable maps. Existing scoped
clone now shares the geometry. No public API/dependency/cache expansion. Placement
and ID-set setup remains per event; batch scope reuse has not been implemented.

New domain regression test/layout_snapshot.rs retains an old clone over text edit
and fresh DPI-2 router layout. It verifies detached values and Weak-observed
release after last layout, including into_primitives; routing/router does not
persistently retain geometry. Targeted composition suite: 52 passed, 2 ignored.

Same layered probe twice before/after, no diagnostics/concurrent builds/tests,
18 configs and 576 rows/run. At 1024 fields/32 events means (run 1/2), us:
closed 9331.12/10333.95 → 2592.14/2919.88; open 15163.35/15465.58 →
8281.70/8198.14; modal 17499.75/26019.77 → 11911.35/12096.98.
Second modal baseline noisy (batch-mean p95 51624.40 vs 18696.35 us in run 1).
Final modal batch-mean p95 13624.80/14642.62 us; these are not single-frame tails
or whole-engine/display acceptance. Full protocol/table/limits in performance
review. Geometry no longer deep-copied per scope; fresh layout adds Arc allocation
and sharing has atomic ownership cost. One exploratory flat run: 1024-button
layout mean 686.26→736.55 us; no layout improvement/causal regression claimed.

Logs target/shared-geometry-before-<1|2>.log and after-<1|2>.log,
shared-geometry-summary.csv; exploratory shared-geometry-flat-before/after.log
and shared-geometry-flat-summary.csv. Release engine/sibling builds passed.
Native --smoke and --editing-smoke --locale=ja passed; shared-geometry-native
mixed/editing stdout/stderr logs under target. Full verify log:
target/milestone-4-5-shared-geometry-verify.log. README/UI/roadmap/changelog and
performance review updated together. Next: repeated scope/ID-set setup, then
asset-font editing/overlapping layers and remaining domain scaling. No commits.

Verification completed: ./scripts/verify.ps1 passed (format, workspace check,
Clippy, workspace/CLI end-to-end tests, dependency boundaries, diff whitespace).
App: 121 passed, 6 manual probes ignored; layered release probe ran twice before
and twice after outside full verify. Native mixed and Japanese editing smokes
passed. Final documentation whitespace check also passed.

## Batch-local input/pointer scopes — 2026-10-03

Engine revision 26661b17cc8531c86c3b8d6863f55e7685d3359a; preceding shared
geometry increment still uncommitted and preserved. Sibling revision/dirt unchanged.
New private routing/scopes.rs owns a base Cow layout and one lazy pointer layout
per routed batch. Base rebuilt on stack-length change (events only pop layers),
pointer rebuilt on hovered-layer/capture key change; commands preserve topology
and registered roots. route_events and hit_test_layers use that path; duplicate
event_layout implementation removed from layers.rs. No dependencies/public API.
Cursor refresh and live control/focus checks still run per event. Cache is local
and discarded on success/error. Closed/nonpointer paths reuse the base; plain
trees stay borrowed. At most two scope placement vectors; shared field geometry,
no registered-layer paint retention, temporary IDs discarded after filtering.

Three regressions in test/routing/layers.rs: two modal dismissals and subsequent
underlying/base activations in one DPI-2 batch; capture crossing layer boundaries,
release and new capture; atomic text-error rejection after dismissal/scope rebuild
with successful later dismissal. Targeted composition tests: 55 passed, 2 ignored.
Release engine and sibling workbench builds passed. Native --smoke and
--editing-smoke --locale=ja passed; batch-scope-native-mixed/editing stdout/stderr
logs under target. Full verification log target/milestone-4-5-batch-scopes-verify.log.

Same manual layered probe twice before/after, 18 configs and 576 rows/run, eight
warm calls, 32 batches of eight; no concurrent agent-launched builds/tests during
sampling. Each timed call starts with fresh scopes/atomic clones. At 1024 fields,
32 events means (run 1/2), us: closed 2820.56/2545.59 → 306.61/344.43;
open 8343.04/7917.84 → 602.64/577.47; modal 11440.99/11595.29 →
572.54/574.48. Empty calls show no consistent gain. Modal final batch-mean p95
849.74/668.91 us; no single-frame/native cadence or whole-engine-budget claim.
Raw target/batch-scope-before-<1|2>.log and after-<1|2>.log, summary
batch-scope-summary.csv. Full table/protocol/retention limits in performance review.
README/UI/roadmap/changelog updated together. No commits/staging. Next: shaped
editing and overlapping layers, then remaining domain scaling/memory attribution.

Initial full verification rejected similar local names (scopes/scoped); renamed
the owning local to prepared without changing runtime behavior, then restarted
full verification. Native workloads and sampling preceded this name-only fix.

Verification completed: ./scripts/verify.ps1 passed (format, workspace check,
Clippy, workspace/CLI end-to-end tests, dependency boundaries and diff whitespace).
App: 124 passed, 6 manual probes ignored. Final documentation whitespace check
passed; no staging/commits or changes to the sibling's existing dirt.

## Warm asset-font editing / overlapping layers — 2026-10-04

Engine revision 26661b17cc8531c86c3b8d6863f55e7685d3359a, with previous shared
geometry/batch-scope changes still dirty and preserved. Sibling revision
c89adb9a5317007b3469782c1c8da9d8b4b1b04a with existing Cargo.lock/Cargo.toml/
README dirt and untracked workbench. No production engine behavior changed in
this increment. Added sibling workbench interface/test/layered_editing.rs, cfg(test)
registration in interface/performance.rs, and workbench README probe instructions.
Engine README/roadmap/changelog/performance review updated in the same increment.

Public-API fixture: three overlapping 600x300 field panels, short English/Russian/
Arabic/Japanese strings, nonmodal/modal top, 1000x800 logical viewport, DPI 1/2.
Operations: 32 alternating pointer events across exposed fields; select-all,
preedit route/paint, alternating bounded commit route/paint. Existing Noto assets
loaded once per config outside timing. Font-service locale en-US remains fixed.
Initial field-focus changes await native session-stop feedback; final fixture
provides TextInputChanged(active=false) outside timing. Early failed setup runs
suppressed commits; final-value assertion caught this and those runs are excluded.
First excluded warm call verifies preedit and altered composing paint. Helpers
separate authoring, pointer fixtures and validation to meet Clippy line limits.

Final source probe ran twice in release, 32 configs, ten warm +100 individual
samples/config (3200 rows/run); no diagnostics or concurrent agent-launched checks
while sampling. Final p95 editing-cycle ranges across scripts/DPI/policies/runs
64.60–514.80 us, largest sample 614.60 us. Pointer32 p95 15.50–37.90 us,
largest sample 95.80 us. These are individual-call percentiles, unlike earlier
batch-mean probes. Warm two-value, three-field workload has no new measured
bottleneck requiring a fix; no before/after improvement/whole-engine-budget claim.
Long/unique text, cold/cache churn, expanded layers, memory accounting and real
IME/native DPI2 remain open. Logs target/font-editing-<1|2>.log;
font-editing-summary.csv includes each policy/run/p99/max and font-editing-ranges.csv
provides table ranges. Detailed protocol/table/disposition in performance review.

Example package fmt check, Clippy --all-targets -D warnings, and regular tests
passed (7 passed, 1 ignored); ignored release probe ran twice explicitly.
Sibling release build passed. Native --editing-smoke --locale=<en-US|ru|ar-EG|ja>
passed, configured physical1000x800/DPI1; logs font-editing-native-<locale>
stdout/stderr under target. These are injected correctness smokes, not OS IME
acceptance or native timings. Engine ./scripts/verify.ps1 passed: format/check/
Clippy/workspace and CLI tests/boundaries/whitespace (app124 passed,6 ignored).
Full log target/milestone-4-5-font-editing-verify.log; example logs
font-editing-clippy.log / font-editing-example-tests.log. Final whitespace checks
for both repositories passed. No staging/commits. Next: mixed ECS/churn and cold
localization, retaining the explicit larger-text/cache/layer follow-ups.

## 2026-10-04 mixed world/cold localization checkpoint

Baseline engine 59449e9d23b3e1634b673140ac754c01d3470c82; clean working tree
at start, examples untouched. Added ignored domain probes in world/test/scaling.rs
and runtime/test/publication.rs plus their module registrations. Two isolated
release runs of each passed. Raw logs and summary under target/mixed-world-*,
cold-localization-* and mixed-cold-summary.csv. Full protocols, percentile ranges
and limits are in PERFORMANCE_REVIEW.md. No production change justified by these
fixtures. Next: long/unique text, cache churn, expanded layers and allocation
attribution; retain broader mixed ECS/reference-graph follow-ups. Full verification
pending below. No staging or commits.

Verification outcome: formatting, workspace check and Clippy passed after replacing
an empty-string assertion flagged by Rust 1.99 Clippy. Full verify reached tests
but failed in existing CLI generated_project_can_be_checked_and_run_offline:
Cargo could not write the temporary project's invoked.timestamp (os error 3).
Cause unconfirmed; not attributed to the performance probes. Full diagnostic log
is target/mixed-cold-verify.log. No unchanged retry. Tests for the workspace excluding
gridthorn_cli passed separately (target/mixed-cold-domain-tests.log), dependency
boundaries and git diff --check HEAD passed. Full gate remains incomplete because
of the CLI failure. The assertion correction changes no acquisition timing.

## 2026-10-04 text/cache/expanded-layer checkpoint

Continuing the prior uncommitted increment at engine HEAD
59449e9d23b3e1634b673140ac754c01d3470c82. Examples inspected with a per-command
safe.directory override; their Cargo.toml/Cargo.lock/README and untracked workbench
changes predate this increment and were not edited. Added ignored renderer
text/test/pressure.rs and app ui/composition/test/layer_pressure.rs, with module
registrations. Two isolated release runs each of text pressure, expanded layers
and Japanese attribution passed. Clippy for render/app all targets passed.
Protocols, results and limits are in PERFORMANCE_REVIEW.md; raw/summary logs in
target/text-pressure-*, layer-pressure-*, japanese-attribution-* and
text-layer-pressure-summary.csv. No production changes: long Japanese fallback
misses are the principal measured remaining cost (p95 ~11 ms versus explicit JP
primary ~1.3 ms); author-selected font semantics must be preserved. Bitmap expanded
layers need no fix for this fixture. Next: backend fallback/raster-allocation
attribution and long-field editing/asset-font expanded-layer workloads.
Full verification follows with TEMP/TMP explicitly routed to target/verify-text-tmp
for this process, testing a workspace-local temp location after the previous
system-temp CLI invoked.timestamp path failure. This does not change repository
configuration or establish the earlier failure's cause. No staging or commits.

Verification outcome: full ./scripts/verify.ps1 passed with CARGO_TARGET_DIR set
to the absolute engine target directory and RUST_TEST_THREADS=1 for this invocation
only (target/text-layer-verify-shared.log). Includes formatting, workspace check,
Clippy, all workspace unit/doc tests, generated-project offline CLI check/native
smoke, dependency boundaries and whitespace. Original temp-location experiment
failed with MSVC LNK1181 on nested generated-project paths
(target/text-layer-verify.log); no production/CLI fix made and no confirmed claim
about the preceding invoked.timestamp failure. Env overrides were restored.
This successful full gate also covers the earlier world/localization probe files.
All acquisition was completed before checks. No native performance speedup inferred
from the correctness smoke; native interaction/DPI2/IME acceptance stays open.

## 2026-10-04 request-local raster reuse checkpoint

Engine baseline 129f9de8bbcbfcd43183dd0e65a0045e25d2de3b, initially clean after
maintainer committed preceding increments. Examples still have pre-existing
manifest/lock/README/untracked workbench changes, untouched here. Renderer raster
loop delegates to focused text/raster_spans.rs; cache lifetime is one request,
128-entry/65536-span retention, direct path below 256 glyphs. No dependency/feature
edges or public signatures changed. Added ordered backend-reference pixel test
and bounded cache-saturation test under text/test/. Protocol/evidence/tradeoffs in
PERFORMANCE_REVIEW.md; README/roadmap/TEXT/changelog updated together.

Two isolated before runs and three final runs of the existing text-pressure probe
passed; two unconditional-cache prototype runs are retained as intermediate data.
A small-request regression in that prototype led to the measured threshold.
Russian/Arabic/Japanese long DPI2 medians improve, English mixed (all runs kept).
Shaping fallback misses unchanged; optional backend shape-run cache not enabled
without a cap. Renderer text tests 20 passed/2 ignored; focused Clippy passed before
final constant extraction. Sibling release build and native editing smokes for all
four locales passed (physical 1000x800/DPI1), with logs under target/raster-spans-*.
No real OS IME/DPI2/full-frame performance acceptance inferred.

Next: remaining domain I/O/worker/scaling review; retain explicit long-field,
asset-font expanded-layer, unique-glyph/cold raster, allocator/peak-byte and fallback
shaping follow-ups. Full verification follows using the previously successful
process-local CARGO_TARGET_DIR=<engine target>, RUST_TEST_THREADS=1 settings.
No staging or commits.

Final verification: ./scripts/verify.ps1 passed with process-local shared engine
target and sequential tests, overrides restored. Includes format/check/Clippy,
all workspace unit/doc tests, offline generated-project CLI check/native smoke,
dependency boundaries and whitespace. Full log target/raster-spans-verify.log.
All benchmark acquisition preceded checks. No staging/commits; examples unedited.

## 2026-10-04 I/O and graph optimization checkpoint

Engine baseline 545c6ad61a3a3d8ba5b9148674ada3fecbd7ae18, initially clean after
maintainer committed raster changes. Added ignored probes to assets/reload/test,
scene/persistence/test and app/scenario/saving/test. Existing save-file Directory
fixture is shared only within its test subtree. Focused store/dependency_graph.rs
owns borrowed reverse adjacency, propagation and Kahn ordering with an ordered
ready set. Removed repeated propagation/pending scans from storage.rs. No
crate/dependency edges, public signatures or serialization semantics changed.

Two isolated before runs/domain passed, two after asset runs passed. Reverse-chain
1024-file changed scan median ~198–204 ms -> ~60–61 ms; unchanged scans still ~55–57
ms after, ready poll unchanged at sub-ms typical. Asset tests 25 passed/1 ignored,
including new chain/diamond/dynamic lexical readiness tests and existing rollback.
Separate fresh-process memory runs (50 ms PeakWorkingSet64 monitor, 500 ms hold
with GRIDTHORN_IO_MEMORY=1, env restored) all passed: asset before39,731,200 bytes,
after40,169,472; scenes95,457,280; saves29,720,576. Not heap/phase/allocator budgets;
monitor runs excluded from latency data. Full protocol/tables/limits in review.

Scene 10K TOML and 262K-value codec saves exceed frame-sized durations; attribution
open, no unrelated format/async subsystem changes. Docs/README/roadmap/changelog
updated together. Logs and CSVs under target/asset-io-*, scene-io-*, world-save-io-*,
io-review-summary.csv, asset-io-before-after-summary.csv, *-memory.* and
io-process-memory.csv. Public asset-reload --smoke and full verify follow below.
Next domain: audio command/worker and platform lifecycle, retaining explicit
cold/branching/phase-memory/codec and prior text/runtime follow-ups. No staging or
commits; sibling examples are exercised but not edited.

Final verification passed: full ./scripts/verify.ps1 with process-local shared
engine target and sequential tests, overrides restored (target/io-review-verify.log).
Includes formatting/workspace check/Clippy, all workspace unit/doc tests, generated
project CLI/native smoke, dependency boundaries and whitespace. Public sibling
asset-reload --smoke passed: publication, dependencies, intentional decode rollback,
recovery and shutdown (target/io-review-asset-smoke.log). No native scene/save/frame
budget or cross-platform claim inferred. Final documentation whitespace check passed.

## 2026-10-04 Audio control and lifecycle callback checkpoint

Baseline cbf2a390c26b498c3b7f347ffb243ecf7b10f046; engine initially clean.
Added ignored release probes under audio/output/test and app/window/runtime/test.
No production, dependency or public API changes. Two isolated runs each passed;
20 retained samples/config after two excluded samples. Audio covers fresh mock
services, shared stereo clip clones, queue/play/control/suspend/resume/teardown.
Platform covers synthetic callbacks and 0/32/1024 shutdown counter systems.
An initial platform build used nonexistent ScheduleBuilder.world; corrected by
inserting the fixture resource through ApplicationRuntime.world before startup.

64 voices x80K frames Play median9.1–10.7ms, teardown2.6–3.3ms; controls and
suspend/resume in microseconds. 1024 trivial shutdown systems median18–19us.
Logs target/audio-control-{1,2}.log, platform-callbacks-{1,2}.log and derived
summary.csv. Detailed workload/timer/mock limits in PERFORMANCE_REVIEW.md.
No hardware/mixer/worker performance or OS suspension claim; native lifecycle,
worker handoff, memory, completed-voice retention and PCM conversion attribution
remain open. Next focused increment: audio conversion/retention attribution and
bounded fix if evidence supports it. No staging/commits or sibling changes.
Full verification follows with shared engine target and sequential tests.

Full ./scripts/verify.ps1 passed with restored process-local target/thread overrides.
Includes format/check/Clippy, workspace unit/doc tests, generated-project CLI/native
smoke, dependency boundaries and whitespace. Log target/audio-platform-verify.log.
Documentation-only final checkpoint addition passed git diff --check.

## 2026-10-04 Audio conversion and retention fix checkpoint

Same HEAD cbf2a390c26b498c3b7f347ffb243ecf7b10f046; preceding audio/platform
probe changes were uncommitted and preserved. Added private output/prepared_batch
with 16-entry/1MiB converted-frame cap and retained source identities, lifetime
one process call. Output process prunes Stopped handles even on empty queues;
active_voice_count filters Stopped immediately. No API signatures/deps changed.

Audio tests12 passed/1 ignored before adding the separate PCM probe. Full verify
covers final source (12 passed/2 ignored). Completion tests explicitly advance
mock mixer, preserve loops, and check128 completion cycles. Frame reuse tests
cover settings, identity, call lifetime, oversized/entry/combined-byte limits.
Two unchanged release control after runs and two PCM attribution runs passed.
64x80K Play9.1–10.7ms ->0.323–0.379ms; direct PCM9.4–9.5ms vs reused0.294–0.421ms.
Single-voice results mixed; no universal improvement/hardware/memory-budget claim.
Logs target/audio-reuse-after-{1,2}.log, audio-pcm-{1,2}.log,
audio-reuse-summary.csv; protocol and limits in PERFORMANCE_REVIEW.md.

Full ./scripts/verify.ps1 passed (target/audio-reuse-verify.log), shared target and
sequential test overrides restored. Includes format/check/Clippy, workspace
unit/doc tests, generated-project CLI/native smoke, boundaries and whitespace.
Public classic_2d --smoke passed (target/audio-reuse-native-smoke.log): native
window/audio worker lifecycle, not audible-quality/latency verification. Sibling
pre-existing manifest/lock/README/workbench changes remain untouched. No commits.
Next: example worker handoff/backpressure and long-lived/native audio memory or
remaining simulation/grid/pathfinding/collision scaling; broad milestone stays open.

## 2026-10-04 Example audio worker measurement checkpoint

Engine HEAD cbf2a390c26b498c3b7f347ffb243ecf7b10f046 and preceding uncommitted
audio/platform work preserved. Examples HEAD c89adb9a5317007b3469782c1c8da9d8b4b1b04a;
pre-existing Cargo.toml/Cargo.lock/README and untracked workbench unchanged.
Added classic_2d/src/audio/test/scaling.rs plus cfg-test declaration/test mod;
updated only classic_2d README in sibling. No production behavior/dependencies
changed in this increment. Two isolated release runs passed,20 retained samples
per config after2 excluded. Actual headless Sound pickup+shutdown separately
from gated32-slot transport fixture. Burst16384 gated accepts32/drops16352 and
Pause(true) deterministically fails Full. Actual pickup acceptance unknown by API.
Logs target/audio-worker-{1,2}.log and audio-worker-summary.csv. Actual16384
enqueue median66–72us; shutdown38–45us with scheduling/startup limits. Protocol
and disposition in PERFORMANCE_REVIEW.md; no device-latency/delivery guarantee.

Sibling package fmt --check and Clippy --all-targets -D warnings passed; tests6
passed/1 ignored; headless-smoke and native --smoke passed. Full engine verify
passed using restored process-local shared target/sequential-test overrides;
log target/audio-worker-verify.log. Native smoke is lifecycle only. Engine and
sibling changed; no commits/staging. Next focused fix: reliably coalesce latest
pause/resume state independently of bounded effect loss, then gated regression
and repeated affected samples. Native output memory/worker stalls remain open.

## 2026-10-04 Reliable example pause fix checkpoint

Same engine/examples revisions as preceding worker checkpoint; all existing
uncommitted work preserved. Classic_2d audio/pause_state.rs owns one AtomicU8
mailbox. Sound.pause publishes latest state then tries Wake; worker consumes
pending state before each request. Full queue contains an eventual consumption
trigger, empty queue is woken, stale wakes carry no state. Intermediate states
coalesce. Effect queue remains32 slots/lossy; no engine worker API/deps added.

New audio/test/pause_delivery.rs verifies full-queue pause, latest resume,
alternating state, idle wake and stale wakes through Sound.pause. Scaling fixture
also checks final resume under saturation. Two release after runs passed;
actual16384 pickup median66–71us, no throughput claim due scheduling variance.
Logs target/audio-pause-after-{1,2}.log. Tests8 passed/1 ignored, fmt check and
Clippy all-targets -D warnings passed. Initial Clippy collapsible_if was fixed
with equivalent let-chain; Clippy and tests rerun successfully. Headless/native
smokes passed before this equivalent syntax cleanup. Full engine verify passed
(target/audio-pause-verify.log); process-local target/thread settings restored.
Sibling README and engine roadmap/review/changelog updated, both diffs checked.
No staging/commits; pre-existing sibling manifest/lock/rootREADME/workbench untouched.
Native application latency and worker progress are not guaranteed; next remaining
performance domains: long-lived audio/output memory, native/device latency, or
simulation/grid/pathfinding/collision scaling. Milestone remains open.

## 2026-10-04 Pathfinding scaling checkpoint

Engine baseline834df86eaeb99a2ba951a8fe7a9093d6d369c37c, initially clean. Sibling
existing manifest/lock/rootREADME/workbench and classic_2d audio changes preserved;
only pathfinding example exercised, no sibling edits. Added navigation/test/scaling
ignored probe for32/128/512 open/weighted/wall/budget workloads. Two before and
two after isolated release runs passed; full result fingerprints match all12 cases.
Costs/parents now HashMap lookups, ordered BTreeSet frontier unchanged. No deps/API
signatures/dense bounds allocation. Added exact canonical route/expansion/frontier
regular regression at budgets0/1/3/8/16. Grid tests23 passed/1 ignored.

512-square open median191–193ms ->91–95ms; weighted217–234 ->111–113ms;
wall92–99 ->45–50ms; budget1024 ->0.20–0.26ms. Full searches still not frame-sized.
After fresh-process resident peak32,194,560bytes via50ms PeakWorkingSet64 sampling;
500ms final hold, env restored, monitored samples excluded from latency tables.
This includes harness/validation/fingerprint; no baseline/heap/phase-memory claim.
Logs target/navigation-1.log, navigation-before-2.log, navigation-after-{1,2}.log,
navigation-summary.csv, navigation-memory.log and navigation-memory-summary.txt.
Protocol and exact workload limits in PERFORMANCE_REVIEW.md.

Full verify passed (target/navigation-verify.log), shared target/sequential-test
process overrides restored. Initial Clippy assertion-style failure corrected,
then full verification rerun. Includes fmt/check/Clippy/unit/doc tests, generated
CLI/native smoke, dependency boundaries and whitespace. Public sibling pathfinding
command passed and emitted expected diagnostic SVGs (target/navigation-example.log).
Docs/README/roadmap/changelog updated; final diff whitespace check passed. No commits.
Next: fixed simulation/placement/collision scaling; retain maze/real-cost/budget-
sweep/heap/pathfinding follow-ups and native/device audio deferrals.

## 2026-10-04 Placement and collision scaling checkpoint

Engine baselineef240d8756e640f72091a8f600c639cdde81dd92 initially clean after
maintainer committed pathfinding. Added ignored probes in placement/test/scaling
and collision/contact/test/scaling. Placement matrix1024/16384/65536 objects,
1/16 horizontal footprint cells;1024 lookups/validations and2048 relocations per
sample. Two before/after isolated release runs passed. Cell index is now HashMap;
objects BTreeMap and footprint offset order unchanged. No dependency/public API
changes. Existing deterministic errors/iteration, atomic rollback and coordinate
limits tests pass. Grid23 passed/2 ignored; collision6 passed/2 ignored.

Large16-cell footprint lookups0.50–0.54ms ->0.14ms; relocations6.86–6.93ms
->5.07–5.49ms. Validation gains mixed; footprint/object attribution and memory
remain open, no memory-reduction claim. Collision production unchanged. Two
ready-pair runs and two caller all-pairs runs passed.1024 circles create523776
pairs, sparse/dense median1.7–2.0ms; no broad phase introduced. Limits/protocol in
PERFORMANCE_REVIEW.md. Logs target/placement-before-{1,2}.log, placement-after-
{1,2}.log, collision-before-{1,2}.log, collision-pairs-{1,2}.log and derived
placement-collision-summary.csv. Acquisition did not overlap builds/checks.

Full verify passed (target/placement-collision-verify.log), process-local target/
thread overrides restored: fmt/check/Clippy/workspace unit/doc tests, generated
CLI/native smoke, dependency boundaries and whitespace. Public grid-placement
example passed square/isometric picking, preview, rollback/move/removal checks
(target/placement-example.log). Sibling unedited. Docs/README/roadmap/changelog
updated; final GRIDS/checkpoint prose whitespace checked. No staging/commits.
Next: fixed simulation and snapshots/RNG scaling; retain rejection/churn/heap/
realistic collision candidate selection follow-ups. Broad milestone remains open.

## 2026-10-04 Fixed simulation, snapshots and RNG checkpoint

Engine baseline6561281b0537beeea9990e3933fd7a1f2f45920d initially clean. Added
ignored probes in simulation/time/test/scaling, determinism/test/scaling and
app/scenario/test/scaling. No production/dependency/API/algorithm encoding changes.
Two isolated runs each passed:10000 clock calls normal/zero/catch-up/paused;
16384 direct/named draws with1/64/1024 streams; typed root1024/16384/262144 values
with64 commands and1/128 streams, capture/clone/restore;1000 exact ticks with
0/16/256 scalar fixed systems. Checks verify output checksums/final RNG positions,
assigned tick totals, final scalar work, complete snapshot/root equality and
independent restored one-tick continuation. Existing fixed vectors unchanged.

1000 ticks256 systems median4.5–4.6ms;2MiB scalar root capture0.49–0.55ms;
1024-stream16384 named draws0.96–1.01ms, direct~0.03ms. No production fix justified
by these synthetic workloads. Clone independence and RNG contracts preserved.
Clock probe excludes schedule execution; explicit ticks bypass pause. Snapshot
fixtures warm and retain reference runners/captures; no heap/peak/user-Clone budget.
Protocol and limits in PERFORMANCE_REVIEW.md; logs target/simulation-{1,2}.log,
snapshot-{1,2}.log, fixed-schedule-{1,2}.log, simulation-snapshot-summary.csv.

Full verify passed(target/simulation-snapshot-verify.log), shared-target/sequential
process overrides restored. Initial Clippy failures in new fixtures (names,
doc markup, method closure and semicolon) corrected before successful gate.
Includes format/check/Clippy/unit/doc tests, generated-project CLI/native smoke,
dependency boundaries and whitespace. No sibling edits or commits. Docs/README/
roadmap/changelog updated and final checkpoint whitespace checked. Remaining:
real/entity-heavy catch-up work, speed/queue workloads, nested roots/retained
snapshot heap peaks, named lookup attribution and native/device follow-ups.
Next risk-ordered domain: cold/warm build footprint and binary size, while broad
Milestone4.5 remains open for realistic/native acceptance and unresolved limits.

## 2026-10-04 Build and footprint baseline checkpoint

Same engine HEAD6561281b0537beeea9990e3933fd7a1f2f45920d; preceding uncommitted
simulation probes/docs preserved. No source/config/dependency/profile changes in
this increment. New ignored measurement root target/build-footprint-20261004-140430,
separate empty CLI/SDK targets. Offline locked release builds:CLI23.21s,
SDK161.54s, two no-op warms CLI0.38/0.30s and SDK0.71/0.65s. Registry/OS caches
warm, default jobs/profile; one initial sample each, not repeated cold distribution.
Generated template from measured CLI adapts copied lockfile via offline metadata
before locked builds; external package/version set adds none relative to engine.
SDK thirdparty artifact primed build10.75s, repeats0.60/0.54s; source-owned SDK
crates recompile under standalone workspace context. Example classic_2d existing
release-test cache3.58s, repeats0.72/0.62s (not cold). No sibling source/lock edits.

CLI.exe2234880bytes; generated.exe8419840; classic.exe9138688; facade-only
rlib51682 (not engine size). CLI/generated PDB separate, assets/runtime DLLs and
packaging excluded. Target-normal closures CLI51/SDK220 packages including root,
build/dev edges excluded. Logs/times/sizes/dependency trees/metadata/Cargo HTML
reports retained in measurement root. All commands isolated/sequential; normal
build caches not deleted. Detailed limits and reproduction in PERFORMANCE_REVIEW.

Measured CLI --help, generated release --smoke and classic release --smoke passed.
Full engine verify passed(target/build-footprint-verify.log), shared target and
sequential-test process overrides restored; includes previous uncommitted probes.
Final docs/checkpoint diff whitespace checked. No staging/commits. Next:
repeat true-empty examples/generated builds, critical-path/edit rebuild attribution,
feature/profile tradeoffs and complete packaging bytes; realistic/native domain
acceptance remains open. No regression or profile improvement claim from baseline.

## 2026-10-04 Cargo units and edited rebuild checkpoint

Engine baseline67cd899649a0009dbdbd8a7b6ec12dec388da795 initially clean. Only
CHANGELOG/ROADMAP/PERFORMANCE_REVIEW/checkpoint Markdown reviewed changes.
Retained prior measurement root; copied SDK crates/config/manifests under ignored
sdk-source and retargeted temporary generated-game manifest only. Priming13.90s
excluded. Original source contents restored via finally; tracked source untouched.
Actual equivalent implementation edits: model subtraction ->wrapping_sub for
0/1 input, copied RNG state ->wrapping_add(0). Alternate/restored source builds
2.51/2.62s model and6.51/6.68s SDK. Compilation chains model:game; SDK:simulation,
app,facade,game. External GPU/text deps not recompiled. Final release smoke passed.

Parsed dated SDK cold HTML UNIT_DATA, not mutable cargo-timing.html alias.
Longest unit naga120s; ECS101s; GPU/image/font units overlap. wgpu-core end161.4s
near total161.5s; not additive CPU durations or exact causal critical path.
Logs target/build-footprint-20261004-140430/edit-*.log, edit-times.csv,
cold-units.csv and dated Cargo reports. No profile/dependency/feature changes.
DocsOnly verification passed; final checkpoint/correction rechecked. Sibling
untouched; no staging/commits. Next controlled profile/backend/jobs experiments
or broader edit/native acceptance matrix; no regression/improvement claim.

## 2026-10-04 Cargo parallelism checkpoint

Same HEAD67cd899649a0009dbdbd8a7b6ec12dec388da795, existing Markdown changes
preserved. No production/config/profile/dependency changes. SDK offline locked
release empty-target serial builds-j4 206.89s,-j2 347.28s,default137.68s. Reports
confirm ncpu12/defaultjobs12; environment jobs override unset. One sample each,
prior default161.54s retained, no exact/universal optimization claim. Reduced
jobs shorten some units but slow whole build; default policy preserved.

Each new jobs target was resolved/validated under measurement root then removed
after successful build, metrics and report preservation (start free~4.4GiB).
Only newly-created artifacts cleaned; original target/source/evidence preserved.
Logical artifact totals~1.0GiB each, not disk/RAM peaks. Logs under retained
build-footprint-20261004-140430/jobs-{4,2,default}.log/html/units.csv and
jobs-times.csv. No memory/responsiveness claims. CIM hardware details unavailable;
Cargo runtime CPU count sufficient for settings used, no escalation requested.

Only changelog/roadmap/review/checkpoint Markdown changed in this increment.
DocsOnly verification passed; no staging/commits or sibling edits. Next: feature/
profile tradeoff experiments, repeated cold examples/generated builds, alternate
jobs edit matrix, realistic/native acceptance; broad milestone remains open.

## 2026-10-04 World/render/facade rebuild checkpoint

Engine HEAD remains67cd899649a0009dbdbd8a7b6ec12dec388da795; preceding Markdown
changes preserved. Extended ignored copied SDK fixture with equivalent generic
world spawn and renderer RGBA body edits, plus an unused additive facade version
function. Alternate/restored release builds: world8.41/7.93s, render7.64/7.93s,
facade2.85/2.66s. Logged chains world->scene/app/facade/game,
render->app/facade/game, facade->game; no third-party compilation reported.
Original copy bytes restored; final generated release --smoke passed.

Initial results discarded after identifying redundant finally restoration changing
mtime and contaminating the next case. Script fixed, fixture re-primed outside
timing, all six final measurements repeated sequentially. Evidence under retained
measurement root: edit-domain-probe.ps1, edit-domain-times.csv,
edit-{world_generic,render_color,facade_api}-{1,2}.log and edit-domain-smoke.log.
Only Markdown reviewed changes; no production API/profile/dependency changes.
Next: realistic API/generic edits, true-empty examples/generated builds and native
acceptance matrix. Broad milestone remains open.

Verification: ./scripts/verify.ps1 -DocsOnly passed; SHA-256 comparison confirms
all three copied sources match tracked originals. Final smoke exit0. No sibling
changes, staging or commits in this increment.

## 2026-10-04 Generated-project empty-target checkpoint

Same engine HEAD67cd899649a0009dbdbd8a7b6ec12dec388da795 and examples
HEADc89adb9a5317007b3469782c1c8da9d8b4b1b04a. Existing Markdown changes preserved;
no tracked code/config or sibling edits. Retained generated-game/copied SDK and
adapted lockfile used unchanged. New previously nonexistent
build-footprint-20261004-140430/generated-empty-20261004 target;
release locked offline build142.41s, no-op repeats0.70/0.72s. Cargo progress
counts227/0/0, not distinct packages. Registry/OS caches warm, one empty sample.

Executable8421376bytes, PDB4911104bytes, logical target1104095557bytes; not disk
or memory peaks. Normal/new targets retained. Final generated --smoke exit0,
headless two fixed ticks plus shutdown, no native display coverage.
Logs/script/CSV: generated-empty-* under retained measurement root; dated timing
reports in new target. No performance gain or dependency/profile change claimed.
Next: repeated empty-target examples/generated acquisitions, packaging, realistic
edit/profile workloads and outstanding native interaction/display acceptance.

Verification: ./scripts/verify.ps1 -DocsOnly passed; generated-empty-{0,1,2}
builds and final headless smoke passed. Log: generated-empty-docs-verify.log.
No staging or commits.

## 2026-10-04 Example empty-target checkpoint

Same engine HEAD67cd899649a0009dbdbd8a7b6ec12dec388da795 and examples
HEADc89adb9a5317007b3469782c1c8da9d8b4b1b04a, existing working-tree changes
preserved. No sibling edits. classic_2d release locked offline build with native
output uses new absent classic-empty-20261004 target under retained measurement
root;147.39s empty,0.75/0.67s no-op repeats,228/0/0 Compiling progress lines.
One empty sample, source/registry/OS caches warm, not native audio attribution.

EXE9138688bytes, PDB5345280, target logical1182135505, assets107609bytes;
not physical disk, peak memory or relocatable package. New release headless smoke
and native30-frame smoke both exit0, no sound-quality/frame-budget claim.
Logs/script/CSV retained as classic-empty-*; original and new targets retained.
No code/profile/dependency changes. Next: repeat/reorder cold trials, feature/dev
matrix, packaging and outstanding native performance/interaction acceptance.

Verification: ./scripts/verify.ps1 -DocsOnly passed; three release builds and
both smokes passed. Log: classic-empty-docs-verify.log. No staging or commits.

## 2026-10-04 Direct import/package audit checkpoint

Same engine HEAD, existing engine/sibling changes preserved. No source/build/
profile/dependency changes. dumpbin14.44.35228.0 reads retained release images;
case-normalized direct DLL names CLI11, generated23, classic26. All import
VCRUNTIME140 plus six CRT API-set names. No recursive/dynamic/clean-host audit.
Classic extra names combase/mmdevapi/winrt-error; no audio cost attribution.

New package-stage-20261004 copies EXEs and classic assets; logical file bytes
CLI2234880, generated8421376, classic9246297. CLI --help and generated/classic
headless smokes pass from staged working directories. Source inspection and
binary bytes verify classic compile-time absolute source asset path, so staging
success cannot prove relocation while source assets exist. Original assets untouched.
No runtime/PDB/installer bytes included or packaging capability implemented.
Evidence package-* scripts/logs/CSV/stage under retained measurement root.
Next: explicit asset-resolution policy, clean-host/dynamic runtime audit and
outstanding native performance/interaction matrix; broad gates remain open.

Verification: ./scripts/verify.ps1 -DocsOnly passed; all dumpbin calls and three
staged local workflows passed. Log: package-docs-verify.log. Sibling Git status
unchanged from start; no staging/commits.

## 2026-10-04 Example asset relocation checkpoint

Starting engine67cd899649a0009dbdbd8a7b6ec12dec388da795, examples
c89adb9a5317007b3469782c1c8da9d8b4b1b04a. Existing changes preserved. Sibling
classic main/presentation/audio now use focused assets resolver; adjacent assets
folder overrides source fallback as a whole, incomplete folder fails. New domain
resolution tests; package tests10passed/1ignored and all-target Clippy passed.
README documents precedence, relocation and deployment limits. No SDK/dependency
or profile changes. Existing pause-mailbox work remains.

Ignored standalone copied example compiled release after target-filtered offline
metadata adapts only temporary lockfile. Initial unfiltered metadata failed on
restricted non-Windows registry unpack; filtered Windows metadata succeeded.
Copied source path temporarily moved under validated ignored measurement root;
headless/native smokes pass with source absent. Missing packaged atlas/WAV each
exit1. All fixture assets/source restored finally; original sibling files never
moved. Evidence classic-relocation-* under retained root. Prior classic build
cache executable rebuilt; original measured artifact retained in package-stage.
Next: clean-host/dynamic runtime coverage and remaining native performance gate.

Full verify first failed CLI-generated project compilation due to disk-full
errors; format/check/Clippy had passed. Preserved reports/EXE/PDB from own two
empty-target caches into *-evidence then removed only those validated ignored
build caches, freeing space. Logs/CSV/original staged artifacts and normal targets
remain. Full ./scripts/verify.ps1 rerun with process-local shared engine target
and sequential tests passed; env restored. Logs target/classic-assets-full-verify.log
and target/classic-assets-full-verify-shared.log. No SDK dependencies/API changed;
no staging/commits. Final documentation and sibling whitespace checked.

## 2026-10-04 Native slider checkpoint

Existing engine/sibling changes preserved. Workbench adds --slider-smoke: ten
warm frames, press11, alternate20/80 drag, release120. Public router/physical
coordinates at window DPI, normal Changed localization/layout refresh. Domain
test covers DPI1/2, values/effects and capture release. Package8passed/1ignored,
all-target Clippy/release build passed; incompatible flags rejected before window.

Two native release runs per four locales:119 GPU/renderer samples, all no skips/
errors/pending, scale1 physical1000x800 RTX3070/Vulkan616.56/Fifo. Warm109 separate
input/render/window samples. Input sum p950.485-0.667ms, encode0.918-1.385ms,
GPU pass26-33us, host cadence p999.769-11.455ms. Not whole-engine CPU/GPU or
actual display acceptance. No new production bottleneck fix justified.
Evidence target/workbench-slider-* logs/summary.py/CSV; env diagnostics restored.
Next: scrolling/locale/window isolation, native DPI2, OS clipboard/IME and actual
presented intervals. No SDK/dependency change. Broader native gate stays open.

Full ./scripts/verify.ps1 passed with process-local shared engine target and
RUST_TEST_THREADS=1; previous values restored. Log target/workbench-slider-full-verify.log.
Sibling formatting/diff whitespace passed. All eight native runs and CLI flag
rejection passed. No staging or commits.

## 2026-10-04 Locale-switch native checkpoint

Same starting revisions/current working trees preserved. Workbench --locale-smoke
warms10 host frames then cycles public Select + Changed effects through all four
catalogs every frame. Test verifies captions/roundtrip/editor preservation at
injectedDPI2. Package9passed/1ignored, all-target Clippy/release build passed;
conflicting flags rejected. No SDK/dependency change.

Two native release en-US-start cycles, scale1 physical1000x800 RTX3070/Vulkan
616.56/Fifo.119 rendered each, GPU119/118 with one skipped pre-warm sample in
repeat, no errors/pending. Warm105 per distribution excludes first cycle and exit.
Input p951.417/1.385ms, encode1.244/1.269ms, GPU pass33/32us; host cadence
p9911.206/10.860ms. First-cycle layout observations separate in review. No
whole-frame/display acceptance or production optimization claim. Evidence
workbench-locale-* under engine target, diagnostics env restored.
Next: scroll/windows/animation isolation, nativeDPI2, OS clipboard/IME and actual
displayed intervals. Broad milestone stays open.

Full ./scripts/verify.ps1 passed with process-local shared engine target and
sequential tests; env restored. Log target/workbench-locale-full-verify.log.
Sibling formatting/whitespace passed, final documentation whitespace checked.
No staging/commits.

## 2026-10-04 Scroll native checkpoint

Existing engine/sibling changes preserved. Workbench --scroll-smoke opens1000x400
only for this mode;10warm frames then96logical-pixel down/up wheel events via
router at actual DPI. Rejects absent overflow. Focused scrolling module/tests
validate DPI1/2 offsets/return, consumed wheel, layout dirtiness and unchanged
editor/language. Package11passed/1ignored, all-target Clippy/release build passed;
conflicting flags rejected. No SDK/dependency changes.

Eight native release runs, two/locale, scale1 RTX3070/Vulkan616.56/Fifo.119renderer
samples/run, selected109CPU/GPU samples/run. ru-first/ar-first/ja-repeat each
skip one pre-warm GPU sample, no errors/pending. Input p950.643-1.471ms,
encode0.419-0.820ms, GPU pass12-13us; no full-frame or1000x800 acceptance claim.
Evidence target/workbench-scroll-* logs/summary, env restored. Next isolated
windows/animation, selection/clipboard/IME, nativeDPI2 and actual display timing.
Broad milestone stays open.

Full ./scripts/verify.ps1 passed with process-local shared engine target and
sequential tests; env restored. Log target/workbench-scroll-full-verify.log.
Sibling formatting/diff whitespace and final docs whitespace passed. No commits.

## 2026-10-04 Windows/animation native checkpoint

Existing engine/sibling changes preserved. Workbench adds --windows-smoke and
--animation-smoke: identical8-action review/confirmation/popup cycle after10warm
frames, only toggle differs. Activated effects run normal handler; locales/editor
unchanged. Test validates layer order, offset start and control preservation at
injectedDPI2. Package12passed/1ignored, all-target Clippy/release build passed;
conflicting flags rejected. No SDK/dependency/profile changes.

16native release runs, scale1 physical1000x800 RTX3070/Vulkan616.56/Fifo.
119renderer samples/run, selected109GPU/CPU each. Six runs skip one pre-warm GPU
sample; no errors/pending. Preparation p95static0.600-0.833ms,
animated0.835-1.091ms; encode maxp951.768ms. No whole-frame/display acceptance
or new bottleneck fix claim. Host-time transitions + frame-based actions mean
mixed active/idle samples and possible supersession; do not subtract percentiles.
Evidence target/workbench-windows-* and workbench-animation-*; env restored.
Next native selection/clipboard/IME, actualDPI2, whole-frame/display measurements
and final acceptance. Broader milestone remains open.

Full ./scripts/verify.ps1 passed with process-local shared engine target and
sequential tests; env restored. Log target/workbench-windows-full-verify.log.
Sibling formatting/whitespace and final docs whitespace passed. No commits.

## 2026-10-04 Selection/preedit native checkpoint

Existing working trees preserved. Workbench --selection-smoke/--preedit-smoke
share extracted editor-focus preparation with editing mode.10warm frames then
caret/full selection or alternating bounded Japanese/Arabic compositions;cancel120.
DPI1/2 test verifies focus/selection/preedit/layout dirtiness/cancel and unchanged
committed text. Package13passed/1ignored, all-target Clippy/release build passed;
conflicting flags rejected. No SDK/dependency/profile change.

16native release runs, scale1 physical1000x800 RTX3070/Vulkan616.56/Fifo,
119renderer each and109selectedCPU/GPU samples each. Five runs skip one pre-warm
GPU sample, no errors/pending. Input p95selection0.467-0.752ms,
preedit0.399-0.628ms; encode maxp951.808ms. Env restored, raw workbench-selection-*
and workbench-preedit-* logs/summary, package workbench-editor-* evidence.
No genuine OS IME/clipboard, nativeDPI2 or full-frame/display acceptance claim.
Next actual clipboard/IME, reference display metadata/DPI2 and whole-frame timing.

Full ./scripts/verify.ps1 passed with process-local shared engine target and
sequential tests; env restored. Log target/workbench-editor-full-verify.log.
Sibling formatting/diff whitespace and final docs whitespace passed. No commits.

## 2026-10-04 Display metadata checkpoint

Existing engine/sibling changes preserved. Opt-in native window diagnostics add
initial window_configuration via private performance/display.rs: physical size,
window scale, optional monitor name/size/origin/system refresh. Missing/zero refresh
stays unavailable. No dependency/public API/display settings change or per-frame
monitor poll. Snapshot at creation does not track later monitor changes.

Two Japanese native release idle runs report DISPLAY2,1920x1080,origin0/0,
144000mHz,scale1/window1000x800; renderer RTX3070/Vulkan616.56/Fifo. Powercfg
before/after Balanced, not clock/thermal/background measurements. Do not fill old
metadata retrospectively or infer display cadence. Domain2passed, package Clippy,
release build/enabled native checks passed. Disabled native check after removing
env entries emits no window diagnostics (empty present flags still enable them).
Evidence target/window-display-* logs. NativeDPI2/actualdisplay/fullGPU/OSIME and
clipboard remain open. Next actual frame timing and native reference matrix.

Full ./scripts/verify.ps1 passed with process-local shared engine target and
sequential tests; env restored. Log target/window-display-full-verify.log.
Final documentation/code whitespace passed. No sibling source changes or commits.

## 2026-10-04 Paired callback timing checkpoint

Existing engine/sibling changes preserved. Private window performance frames
collector pairs completed preparation callbacks with next completed redraw,
retains at most240 rows and reports preparation count. Unprepared redraws and
unfinished preparation are omitted. Four focused performance tests passed.
This excludes native event handling/wait/display and includes redraw blocking;
not whole-engine activeCPU/display acceptance. Next: use paired native evidence,
then actual display/fullGPU/nativeDPI2/OSIME and clipboard acceptance.

Full ./scripts/verify.ps1 passed with process-local shared CARGO_TARGET_DIR and
RUST_TEST_THREADS=1 (target/window-paired-full-verify-shared.log); initial temporary
build target failure retained in window-paired-full-verify.log. Two native release
idle runs exited0,119pairs each,109warm rows,p95=9612/9424us; count1 throughout.
No sibling source changes, staging or commits. Evidence/limits in performance review.

## 2026-10-04 Paired native interaction matrix checkpoint

Existing working trees/revisions preserved. No source/dependency/sibling edits.
Ran9isolation modes x4locales x2repeats sequentially with window/render diagnostics:
72exit0,119pairs/render rows each,preparation count1 throughout. Warm109pairs except
locale105. p95ranges9.052-10.743ms,p99 up to12.281ms. Largest22.402ms selection/ru
sample attributed locally to preparation1.020ms + redraw21.382ms; renderer acquire/
encode/present elevated, cause unproven. No speculative production optimization.
GPU59runs119collected,13runs118/1skipped,all errors0/pendingfalse. Native scale1,
DISPLAY2/144000mHz/RTX3070/Vulkan616.56/Fifo; scroll1000x400,others1000x800.
Power scheme Balanced observed during/after. Raw target/window-paired-matrix-*
and CSV summary; detailed evidence/limits in PERFORMANCE_REVIEW.md.
Next: actual displayed intervals/fullGPU/nativeDPI2/OSIME/clipboard and reference
controls; broad milestone/acceptance remains open. No staging or commits.

Full ./scripts/verify.ps1 passed with shared process-local engine target and
RUST_TEST_THREADS=1; log target/window-paired-matrix-verify.log. Final whitespace
check passed. Environment overrides were scoped to tool processes.

## 2026-10-04 Long-field editing checkpoint

Existing engine/sibling revisions and unrelated changes preserved. WPR available
with GPU/DesktopComposition profiles,no recording; PresentMon/WPA/GPUView absent
onPATH. Did not change display settings or infer display timing from ETW availability.
Sibling layered_editing.rs adds ignored public long-field preedit/commit probe;
README documents command/limits.4content scripts x8/64/256repeats xsyntheticDPI1/2,
3overlapping panels/topmodal,10warmups+100cycles.2sequential release passes:
20configs each,4largestDPI2 initial layouts reject Text(TooLarge) with no samples.
First exploratory failure retained separately. Warm2value/repeatedglyph costs,
fontservice localeen-US; notnative/alloc/cold/isolated raster. p95x256DPI1
45.291-96.676ms; x64DPI2 up to33.502ms. Evidence target/long-field-editing-complete-*
and summaryCSV; detailed table/rejections in PERFORMANCE_REVIEW.md.
Next attribute long-field shaping/raster/clipping/decoration, preserve safety bound,
then focusedfix/rollback regression if evidence warrants. Broad milestone staysopen.
No staging/commits.

Verification passed: engine fullverify target/long-field-full-verify.log; sibling
package13passed/2ignored, all-targetClippy, formatting and bothdiff whitespace.
Two manual longfield release runs passed. Env overrides scoped to toolprocesses.

## 2026-10-04 Indexed long-field cluster geometry checkpoint

Existing working trees preserved. Sibling attribution entrypoint measure_long_field_phases
requiresUI/textdiagnostics and shares unchanged long cases.1phase run before/after;
2normal samples before (priorstep)/after. UIgeometry p95x256DPI1 before->after:
en15886->1418us,ru22019->1963us,ar11878->1313us,ja7871->1106us. Normalrux256
cyclep95 71.561-96.676->44.313-45.885ms; Japanese53.660-61.457ms after stillpaint/
layout dominated. Smaller/DPI2 costs variable, no uniformspeedup. Same4TooLarge
rejections retained. Fix TextGeometry::shaped per-glyph fullboundaryscan/vector
replaced by partition_point borrowedrange; noAPI/dependency/rasterlimit change.
3focused regressions: inclusive combining/ligature endpoints, distantparagraphs,
RTLcaret/hit/selectionsegments.5filtered tests passed. Native8 editing/selection
ru/ja repeats passed. Sibling13passed/3ignored,Clippy/formatting/releasebuild passed.
Evidence long-field-phases-{before,after},long-field-after-{1,2},comparisonCSVs,
long-field-fix-native-* undertarget. docs/UI.md/roadmap/changelog/README updated.
Next remaininglongfieldpaint/raster/Japaneselayout attribution and broadergate.
No staging/commits.

Fullverify passed target/long-field-geometry-full-verify.log (process-local shared
target/sequentialtests). Finalbothrepo whitespacepassed; env scopedtoolprocesses.

## 2026-10-04 Clipped raster span checkpoint

Engine starting87d220719e09a7a63499e0f402a9b788f23cbd2a clean; sibling pre-existing
changes at c89adb9a5317007b3469782c1c8da9d8b4b1b04a preserved. Renderer public
rasterize_clipped uses actual ink boxes/conservative2px edge margin and skips draw
spans outside layout-local physical clip. All glyph images still count before
culling, preserving TooLarge/raster failures. UI label/control/preedit connects
effective clips; extracted list paint routine, no new dependency/public backend type.
2renderer equivalence/error tests pass; sibling ancestorclip regression passes.
2normal long-field repeats pass20timed/4rejected cases;1phase run passes. x256DPI1
cyclep95 ru44.313-45.885->7.622-8.167ms,ja53.660-61.457->20.351-21.630ms.
Japanese decoration/layout cost remains. Full detailed limits/results in review.
Native24smokes editing/preedit/scroll x4locales x2 pass;119render/pairs each,
GPU23runs119collected/1run118+1skipped,errors0/pendingfalse. Sibling14passed/3ignored,
Clippy/format/releasebuild pass. Evidence target/raster-clip-* logs/comparisonCSVs.
Next Japanese composition/layout attribution, wider nativeDPI2/OSIME/clipboard/
actualdisplay/fullGPU/heap acceptance. No staging/commits.

Fullverify passed target/raster-clip-full-verify.log with process-local sharedtarget/
sequentialtests. Finalformat/engine+sibling whitespace pass; no commits.

## 2026-10-04 Narrow composition cache checkpoint

Existing clipped-raster working tree preserved at engine87d220719e09a7a63499e0f402a9b788f23cbd2a;
sibling baseline unchanged c89adb9a5317007b3469782c1c8da9d8b4b1b04a. New ignored
renderer narrowJapanese probe widths1/12/600, primarySans/JP,10warm+100calls,
2before/2after passes. Width1/12 layouts2048/1792lines bypassold1024linecap;
fallback missp95 12.5-15.6ms,JP2.1-2.5ms. Cachelinebudget calibrated4096;
entry/key/glyphlimits unchanged, warm100hits each after,p95<=0.4us (timerlimit).
2newcache regressions reuse/geometry + forcedlineeviction/unownedrelease pass;
6cache tests total pass. MainUIJapanese cache192->4032lines,3->5entries,
16927->28209keybytes,5959->9799glyphs,110bypass->0. Explicit memoryretentiontradeoff,
opaqueheap/processpeaks unknown.2normalpubliclong probes20configs+4sameTooLarge
rejections pass;Japanesecyclep95 20.351-21.630->6.314-7.746ms.1phase afterpass:
geometry942us/paint3386us/decoration258us p95,1878hits/5misses. Evidence
narrow-layout-* and narrow-cache-* logs/comparisonCSVs undertarget; docs/review
record fullconditions/limits. No source edits in sibling thisincrement.
Next broaderworking-set/coldfallback/rasterallocation/memorypeaks and acceptance.
No staging/commits.

Fullverify passed target/narrow-cache-full-verify.log with process-local shared
engine target/sequentialtests. Sibling14passed/3ignored,Clippy/build pass; native8
editing/preedit ru/ja repeats pass,119render each,GPU4x119/4x118+1skip,errors0/pendingfalse.
Finalformat/bothwhitespace pass; env scopedtoolprocesses,no staging/commits.

## 2026-10-04 Text cache memory checkpoint

Engine starts clean at e907c020f05db9341107edde027b939b98c0df47; sibling existing
changes preserved. New ignored renderer memory probe holds six release phases,
asserts retention budgets and verifies narrow backend buffers release under
pressure. Two separate Windows process acquisitions pass. Diagnostic capacity
3ordinary layouts430150bytes ->5including narrow886648bytes (+456498bytes);
line pressure15entries/3850lines/192586bytes. No production change. Private/
resident process ranges, sampling transitions and excluded backend/allocator
storage documented in PERFORMANCE_REVIEW; no exact heap or allocation peak claim.
Two existing long-text pressure probes pass; all four scripts hit100/100 for
single layouts,8repeat/32working-set hits100/100;96working-set hits0/100, and
256repeat/32working-set hits0/100. Larger line budget does not eliminate bounded
working-set churn. Evidence target/cache-memory-{1,2}.{log}, sampleCSVs and
cache-pressure-current-{1,2}.log. Next cold fallback/raster allocation attribution
and remaining native/platform/domain acceptance. No staging/commits.

Full ./scripts/verify.ps1 passed (target/cache-memory-full-verify.log), including
Clippy, workspace tests and dependency boundaries; shared target/sequential tests
scoped to process. Clippy's slice-size diagnostic corrected with size_of_val,
without changing the recorded capacity. Final whitespace check passed.

## 2026-10-04 Fresh-service Japanese attribution checkpoint

Engine e907c020f05db9341107edde027b939b98c0df47; previous memory-probe changes
preserved. New ignored text/test/cold_japanese.rs separates service creation,
first layout, warm-font unique request and identity-verified hit. Two release
runs pass160freshservices each, primarySans/JP,256repeats widths1/12/600 plus
64repeats width600. Fallback unique p9511.879-13.802ms for256 persists after
warming; JP1.739-4.421ms. Service not OS/process cold; no backend attribution
or production optimization claim. Raster64/DPI1 first/repeat equality passes;
full spans1766016/1760696bytes vs clipped220752/225876bytes. These are retained
per-snapshot slice bytes, excluding backend/scratch/allocator/GPU and peakheap.
Evidence target/cold-japanese-{1,2}.log and summaryCSVs; full conditions in review.
Next allocation instrumentation and broader cold/unique/native/domain acceptance.
No sibling source changes, staging or commits.

Full ./scripts/verify.ps1 passed (target/cold-japanese-full-verify.log), including
Clippy, workspace tests and dependency boundaries; shared target/sequential tests
scoped to process. Final formatting and whitespace checks passed.

## 2026-10-04 Raster temporary storage checkpoint

Existing uncommitted memory/cold probes preserved at enginee907c020f05db9341107edde027b939b98c0df47.
Workspace unsafe forbid prevents custom allocator hook; no rule/dependency change.
Test-only raster storage flag sums outputVec/glyph cache vectors/scratch capacities,
not allocator events or totalheap.2release diagnostic runs identical: outputVec
1835008bytes full/229376clipped; glyphvectors121856Sans/82432JPbytes for both.
Production explicitly drops GlyphSpans before snapshot construction; shorter
temporary lifetime, no speedup/OSpeak claim.2diagnostic-free after probes pass,
including full output equality/clipped smaller. Evidence target/raster-storage-*
logs; full conditions/exclusions in review. Next external allocator/backend peak
attribution and broader workload/native acceptance. No staging/commits/sibling edits.

Full ./scripts/verify.ps1 passed (target/raster-storage-full-verify.log), including
Clippy, all workspace tests, dependency boundaries and raster equivalence/error
coverage. Process-local shared target/sequential tests; final formatting and
whitespace checks passed.

## 2026-10-04 Complex localization publication checkpoint

Enginee907c020f05db9341107edde027b939b98c0df47 and previous working tree preserved.
New runtime/test/complex_publication.rs ignoredrelease probe:16/256/4096messages,
4locales,20freshservices percase,16-message chains and nested string/plural/NUMBER.
2runs pass240services each, candidate replacement, cyclic candidate rejection,
oldasset/string lifetime and publishedstate preservation. At4096messages
validationp9523.393-30.713ms,candidatevalidation23.766-35.468ms,replace1.591-2.108ms,
firstformat51.5-99.8us. Freshservice/inmemory,warmprocess; noheap/worker/nativeframe
or backendphase claim. Initial trial appendedextra cyclicmessage; final fixture
replacesanchor to ensure4096case rejects cycle rather than4097messagebudget,
and asserts contextualcyclic Validation error. Evidence target/complex-localization-*
logs/CSVs. No production/sibling/dependency changes, staging or commits.
Fullverify passed target/complex-localization-full-verify.log; sharedtarget/sequential
tests process-scoped. Next expanded asset-font layers/mixed ECS/input attribution
and remaining heap/native/domain acceptance. Finalformat/whitespace checks passed.

## 2026-10-04 Expanded asset-font layer checkpoint

Enginee907c020f05db9341107edde027b939b98c0df47/siblingc89adb9a5317007b3469782c1c8da9d8b4b1b04a
existing changes preserved. Sibling domain expanded_fonts probe measures48configs:
3/16/64 layers x1/16 unique shortfields x4scripts xDPI1/2,10warm+20timedlayoutcalls.
Initial childoverlay trial failedfirstfocus; correctedcolumnflow before2retained
runs. Bothpassfocus/topfieldhits; finalrunadds exactprimitivecount verification.
At1024fields p95en55.75-59.98/75.75-81.31msDPI1/2,ja188.00-233.69/294.62-307.17ms.
CPU preparation only, noediting/GPU/display/heap/actualcachemiss attribution.
Evidence target/expanded-fonts-* logs/summaryCSVs. No production/dependencychange;
next phase attribution for expandedfontprep and mixedECS/input/nativeacceptance.
No staging/commits.

Expanded-font fullverify passed target/expanded-fonts-full-verify.log with process
sharedtarget/sequentialtests. Sibling14passed/4ignored, all-targetClippy passed;
final manualprobe passes all48configs. Finalbothrepo formatting/whitespace passed.

## 2026-10-04 Remaining Text/UI CPU review

Starting engine0339437c5d6dc16ce0ecc5e7ed849dc7ddc909e9, siblingc89adb9a5317007b3469782c1c8da9d8b4b1b04a;
engine initially clean, sibling's existing edits/untracked workbench preserved.
Scope: Japanese fallback, raster allocations/ownership, long-field editing and
expanded/hidden asset-font UI; no mixedECS/input/native frame gate expansion.
Phase acquisition identifies95232 misses/0hits across three stages; bounded
PreparedText shares layouts within one UI pass, leaving31744 misses. Cache drops
before UiLayout publication; permanent LRU/font/hidden sizing contracts unchanged.
RasterText shares its outputVec without copying; capacity survives with snapshots.
GlyphSpans reuses128keys/65536spans across calls, clears on tint/raster-cache reset,
uses direct sampling at entry saturation and releases scratch before publication.

Two final48-config expanded-font runs pass. Japanese1024field p95DPI1
92.18–145.49ms,DPI2 119.16–135.89ms vs previous188.00–233.69/294.62–307.17.
Two long-field and two overlapping editing/routing matrices pass;Japanese256/DPI1
preeditcommit p955.264–6.571ms. All256/DPI2 largest fields still reject at safety
limit, not silently timed. Separate closed-layer runs attribute retained sizing
cost; cold Japanese diagnostics attribute long unique misses to backend shaping.
Detailed means, workloads/host variation and concrete follow-ups in PERFORMANCE_REVIEW.
Two raster-storage process runs pass clone/cache/output-owner lifetime assertions;
output/spans/backendimage capacities and externally sampled private/resident ranges
recorded with transition exclusions. No exact allocator/backendheappeak claim.

Focused renderer29passed/6ignored and facade10passed, including128-field independent
raster equivalence across edits/width/DPI. Budget tests validate each retention
limit/atomic bypass; existing clipped/RTL/cluster/error/lifetime coverage retained.
Headless, --performance, mixed --smoke, --selection-smoke and --preedit-smoke pass
(actual nativeDPI1, Japanese injection; no real OSIME/user visual acceptance).
Evidence target/text-ui-* logs/summaryCSVs; no staging or commits.
Next: final full engine and sibling verification; remaining milestone scope is
mixedECS/input plus native whole-frame/GPU/display/platform acceptance and other
open domains. Opaque backendheap/events and large-tree incremental layout have
explicit measured follow-ups; CPU review closure does not close native gates.

Final ./scripts/verify.ps1 passed with process-scoped CARGO_TARGET_DIR=engine/target,
CARGO_BUILD_JOBS=1 and RUST_TEST_THREADS=1 (target/text-ui-final-verify.log): formatting,
workspace check, Clippy, all workspace tests and dependency boundaries. An earlier
unconstrained rerun failed while compiling a fresh temporary CLI project with
rustc STATUS_STACK_BUFFER_OVERRUN; diagnostics remain in text-ui-full-verify.log.
Shared targets/sequential work resolve that run without changing repository build
policy. Added regression ensures failed oversized layouts do not publish partial
shape timings; all six diagnostic caps remain independent.
Sibling package14passed/6ignored and all-target Clippy passed (text-ui-example-*
logs). Final release rebuild/headless/mixed native smoke/injected Japanese preedit
also passed (text-ui-final-*-smoke.log, native actualDPI1); selection smoke passed
in the preceding acquisition. Both repositories' final formatting/whitespace pass.
No dependency edge, source staging or commit introduced. CPU Text/UI review is
recorded complete with explicit workload limits, not native frame/OSIME acceptance.

## 2026-10-04 Renderer resources and display acquisition

Starting engine HEAD310edf85a8466493d5d281006a47232245a98318 (clean), sibling
HEADc89adb9a5317007b3469782c1c8da9d8b4b1b04a with existing classic_2d/workbench changes
preserved. Scope: batching, dirty geometry uploads, decoded texture lifetimes and
frame pacing across workbench, Crystal Trail and Timber Harbor. No sibling edits.
Read root/context, crate/render/app/docs instructions and diagnose skill.

Confirmed surface/uploads recreated every texture for every adjacent sprite batch
on every frame. New device-local TextureCache retains only current batch identities;
cloned assets share resources, separately decoded/reloaded assets remain distinct,
empty frames evict. TexturedFrame compares camera/extent/position/size/tint/UV and
decoded identity; unchanged frames retain batch vertex buffers. Painter order and
adjacent batching are unchanged. Diagnostics append textured batches, uploaded RGBA
bytes, retained unique textures and decoded bytes (not allocated VRAM).

Native before/after two repeats each: classic_2d --smoke, tycoon_slice --smoke,
workbench --idle-smoke/--editing-smoke --locale=ja. Baseline release builds finished;
final modified builds include an experimental desired_maximum_frame_latency=1.
DO NOT retain that setting without positive evidence; default-depth matrix is in
target/render-matrix-* and experimental runs in target/render-latency-one-*.
Two baseline classic display captures were empty due short-lived process-name
filter; later all-process collection filtered by exact child PID captures classic.
Official signed standalone PresentMon2.6.0 downloaded to ignored target, no install,
no service/group/driver/power/display changes. ETW uses GridthornRenderReview session,
bounded timed shutdown. GPU adapter RTX3070/Vulkan616.56, Fifo, nativeDPI1,
DISPLAY2 1920x1080 144Hz, Balanced; rustc1.99.0. Live clocks/HWS unknown.

Native default-depth matrix: four locales x idle/editing/windows/animation x two
runs,120 host frames each. Renderer quantiles exclude frames0..9; PresentMon excludes
first10 captured rows and omits NA/zero display-change values while retaining raw
rows. Short samples and dropped presentations limit percentiles. Some display p99
exceeds33.33ms (Arabic windows97.224, Japanese animation125.0149) despite renderer
CPU host-present gaps around11ms. Cause not established; frame-pacing gate remains
open, no compositor/environment waiver. Resource p95 tycoon4359/4215us before vs
121/117us after, warm texture uploads zero and three retained textures18,878,368bytes.

Focused surface11tests pass. Ignored native GPU resource probe passes static/dirty
1/32/1024 sprites, nonadjacent A/B/A batches with two unique resources, reload and
empty eviction. target/render-resource-probe-1.log. At1024 sprites p95 unchanged3.6us,
dirty94.4us;100 timed samples after10 warmups. Current source needs final formatting,
Clippy and fullverify, repeat GPU probe and example tests/headless/native checks.
Next: finish latency-one comparison, keep/revert based on evidence, resolve or record
remaining display diagnosis accurately; update PERFORMANCE_REVIEW/ROADMAP/CHANGELOG,
then complete ./scripts/verify.ps1 with sharedtarget/jobs1/testthreads1. No commits.

Queue-depth experiment update: desired latency1 passed32 short native captures;
reverse depth2 comparison also passed eight previously problematic short captures.
Causality is not demonstrated, so context.rs returned to its original default
configuration; no presentation policy change will be retained. Saved experiment
logs remain in target/render-latency-{one,final,reverse,game}-*. Source context.rs
should have no final diff. Sibling workbench now supports --long-smoke for1200
frames in idle/editing/windows/animation; window scripts repeat120-frame actions.
A two-cycle layer/editor preservation regression extends its existing domain test.
Current measurement: target/render-long-*; five scenarios x two runs, external14s
ETW captures, first120 captured rows to be excluded for warm display statistics.
Renderer/window diagnostics remain240 capped; no tests/builds during acquisition.
After acquisition, format both repos, run GPU probe again/fullverify/example tests,
rebuild games against original presentation policy and verify their native smokes.

Final renderer verification: ./scripts/verify.ps1 passed with process-scoped
CARGO_TARGET_DIR=engine/target, CARGO_BUILD_JOBS=1, RUST_TEST_THREADS=1,
CARGO_INCREMENTAL=0 (target/render-review-final-verify-4.log): formatting, workspace
check, Clippy, all tests and dependency boundaries. Earlier checks caught exact
float comparison lint (now a declaration-level expectation explaining cache
invalidation), a redundant bind-group return binding and the empty assertion lint;
all corrected. An intermediate test build exhausted the disk (OS112). Verified and
removed only target/debug/incremental,17,023,799,832bytes of disposable compiler cache;
measurement logs/source/saved comparison binaries preserved. No repository build
policy changed. Final free space exceeds11GiB.

Affected example packages: classic_2d10passed/1ignored; workbench14passed/6ignored;
tycoon_slice22passed. All-target Clippy and final release build pass in sharedtarget
with scoped jobs1/tests1/incremental0; logs render-review-example-{tests,clippy}.log
and render-review-final-build.log. GPU resource probes both pass; second1024-sprite
dirty p95176.3us with one9.0152ms maximum, retained in evidence. Engine renderer
ordinary suite56passed/8ignored. Final three-package headless/native smokes pass,
unsupported --slider-smoke --long-smoke rejects before window creation, Japanese
--animation-smoke --long-smoke passes. Final both-repo formatting/whitespace pass.

The ten1200-frame long captures under original presentation policy all contain1197
ETW rows; after first120 rows,904–1077 displayed intervals each. P99range9.6896–
16.6607ms, worst27.7584ms, no warm interval above33.33ms. Short-run outlier cause is
unassigned and no presentation speedup/policy change is claimed. Renderer review
closed in ROADMAP for documented Windows/DPI1 workloads; nativeDPI2/full interaction/
whole-engine CPU/GPU/OSIME/maintainer acceptance and other milestone domains remain
open. PERFORMANCE_REVIEW contains resource/render/display dispositions, reproducible
commands and raw-log references. Added sibling edits are only workbench main/runtime,
window workload/two-cycle domain test and README; earlier sibling edits preserved.
No dependency changes, source staging or commits. Next initiative remains mixed
ECS/input attribution and the other outstanding domain/native gates.

2026-10-04 native acceptance update: engine HEAD99ba188, initially clean working
tree. Windows Computer Use reached the workbench; real locale click and dialog
open/dismiss were observed, but no new performance capture, clipboard or OS IME
acceptance was established. Both displays reported1920×1080 and100% scaling;
display2 standard scale choices were100–175%. No scale setting was changed.
The maintainer subsequently reported manual testing and accepted the1000×800
logical-pixel criterion at DPI1/2. Recorded this narrowly in ROADMAP and
PERFORMANCE_REVIEW without asserting automated DPI2 evidence or measured budget
compliance. Full CPU/GPU/display measurements and complete native interaction,
clipboard/OSIME evidence remain open. Next: extend whole-frame CPU attribution
and collect the remaining measurable native workloads, preserving this manual
acceptance separately from measured results.

The maintainer subsequently confirmed native clipboard and OSIME had already
been tested manually and instructed marking their behavior accepted. ROADMAP
and PERFORMANCE_REVIEW now record this manual acceptance. Do not request or
repeat Windows UI automation for functional clipboard/IME acceptance. No
measurement trace or per-locale/DPI performance results accompanied this
confirmation; quantitative performance remains distinct.

Whole-process CPU diagnostic increment: added cpu-time workspace dependency
and private app window/performance/process.rs with domain tests. Safe backend
reads Windows user+kernel time for all process threads; deltas span completed
redraws, exclude startup before first redraw, retain at most4096 samples, and
break pairing after failed reads. Wall deltas are separate, not displayed
intervals. OS accounting resolution must be assessed before budget claims.
Focused window performance tests7passed; initial fullverify caught empty-assert
Clippy style, corrected. Second fullverify currently running in
target/native-cpu-verify-2.log with jobs1/testthreads1/incremental0. No native
capture with the new counter yet. Windows UI input was stopped by tool Escape
signal; no further Computer Use calls this turn. Next: finish fullverify, build
release workbench and evaluate OS CPU accounting precision in warmed runs.

CPU increment verification complete: fullverify2 exit0, including CLI end-to-end
fixtures, all workspace tests, Clippy, boundary checker and whitespace. Release
workbench build passes, sibling Cargo.lock adds cpu-time/winapi entries while
preserving pre-existing changes. Native Japanese idle --long-smoke passes with
zero CPU clock errors.1079 warm deltas after redraw120: total CPU796.875ms,
wall15014.15ms, meanCPU0.739ms/frame;46 positive rows, minimumpositive15.625ms,
maximum31.25ms, p95zero. All zero rows retained. Windows accounting is too coarse
for per-frame percentile acceptance; do not mark the CPU budget passed. Durable
protocol/evidence added to PERFORMANCE_REVIEW. This run did not collect new
external GPU/display ETW. Quantitative native performance gate remains open.
Next: validate high-resolution CPU attribution via scheduler ETW and complete
remaining GPU/display/interaction performance matrix. Native clipboard/OSIME
functional behavior and window/DPI criteria have maintainer manual acceptance;
do not repeat their functional UI automation. No source commits or staging.

Latest disposition,2026-10-04: maintainer explicitly requested closing the native
performance gate in documentation after discussing precise ETW measurement effort.
ROADMAP and PERFORMANCE_REVIEW now close this gate by maintainer acceptance with
known measurement limits. Do not describe complete quantitative coverage or
CPU/GPU/display budget compliance as measured. Precise CPU tracing, controlled
cold/warm whole-engine baselines and missing matrix cells are documented follow-ups,
not blockers for this accepted gate. Other domain reviews, final matrix/verification
for those domains and the milestone completion review remain open. Next initiative:
broader mixed ECS/component churn and attribution of larger input bursts, followed
by the remaining domain dispositions in roadmap risk-reduction order.

Documentation closure validation: scoped git diff --check passes for all three
updated documents. verify.ps1 -DocsOnly rejects the pre-existing CPU diagnostic
code/manifest changes in this working tree; their fullverify2 already passed
before this prose-only disposition update. No code or executable snippets were
changed in the closure increment.

## 2026-10-04 Runtime/world/input/localization closure increment

Scope: complete only the runtime/world/input/localization domain gate. Starting
engine revision e2d81302d44e8cfb18faa8dc169704cffac8e4ec; engine worktree clean.
No sibling changes planned. Existing native acceptance remains accepted with its
recorded limitations. Added structural mixed-component/scene-partition and input
phase workloads; first baseline compile rejected two incorrect runtime method
calls, corrected to run_timed_frame and shutdown. Baseline acquisition in progress.
Next: repeat release baselines, assess snapshot/ingestion copies, apply only
measured fixes, remeasure, record four domain dispositions and full verification.

Domain closure result: structural mixed-component workload and regular stale-ID/
continuation regression added under world/test; input phase workload extends the
runtime input-domain probe through65536 events, including preedit/mixed streams.
InputBuffer now borrows event state before queueing ownership and transfers the
queue into snapshots, reserving only the preceding event count for the replacement.
Required preedit/held-state clones and focus-cancellation event order remain.
A4096-event snapshot lifetime/focus regression passes. No new dependency/API.

Two before and two final release input-phase/full-publication runs pass; a
queue-transfer-only trial increased pointer ingestion, so the final reserved
replacement was selected and remeasured. Final16384-event whole-path mean ranges:
pointer392.31–439.46us, keyboard1578.39–1751.90us, commits1373.64–1503.17us;
before555.90–567.85/3168.39–3378.86/3352.08–3687.07us respectively.
Two final structural-world, empty-runtime, warm-localization and complex-catalog
runs also pass. Four domain dispositions and measured limits are in
PERFORMANCE_REVIEW's Runtime world input localization domain review subsection.
ROADMAP closes only this domain gate; all other milestone gates remain unchanged.

Engine focused world/input tests and scoped Clippy pass. Fullverify attempt1
caught duplicate payload match arms; fixed. Attempt2 passed formatting, check,
Clippy, every workspace test, CLI end-to-end and dependency boundaries, but its
final whitespace check rejected a trailing ROADMAP blank line introduced during
prose editing. Removed that blank line; final fullverify pending. Complete failure
diagnostics remain in target/runtime-domain-full-verify{,-2}.log.

Sibling build against final input implementation passes. Executable --headless,
--editing-smoke --locale=ja and --preedit-smoke --locale=ja all exit0. Native window
reports1000x800 physical pixels, scale1,120frames; injected smoke does not replace
accepted manual OSIME/clipboard testing or measure native event latency. No
sibling source/manifest/lockfile changes were made; its initial Cargo.toml/lock,
README/classic_2d changes and untracked multilingual-workbench remain preserved.
Build/smoke logs: target/runtime-domain-workbench-build.log,
target/runtime-domain-workbench-headless.log and runtime-domain-native-{editing,preedit}.log.
Next: obtain final fullverify pass, then hand off this completed domain gate.

Final verification complete: ./scripts/verify.ps1 exit0 with jobs2/testthreads1,
including formatting, workspace check, all-target Clippy, all workspace tests,
CLI generated-project end-to-end, dependency boundaries and whitespace
(target/runtime-domain-full-verify-3.log). Affected sibling workbench tests:
14passed,6manual probes ignored,0failed (target/runtime-domain-workbench-tests.log).
The16required release workload logs were checked for successful test completion.
Final scoped formatting/whitespace checks pass. Later changes are prose only:
README/Immediate target now point at the remaining domain gates rather than the
accepted native gate. No source staging/commits or sibling edits. This requested
runtime/world/input/localization review is complete within its documented envelopes.
The next roadmap domain is assets/reload, scenes/saves, audio/platform lifecycle.

## 2026-10-04 Assets/scenes/saves closure increment

Scope: cold-service/process, branching/error-path and I/O/codec/serializer/heap
attribution for the assets/scalar-scene/typed-save domain gate only. Starting
engine43991ad7647f95468c6bff7c0496c8f805cd35aa, clean worktree. No sibling edits.
DHAT0.3.3 added as test-only workspace/dev dependency to assets/scene/app; no
production unsafe code or game dependency. Heap phases track newly allocated
bytes with pre-existing inputs excluded; whole-workflow peaks include fixture
setup, retained inputs, outputs and cleanup. Heap acquisition is separate from
unprofiled timing. OS file cache is uncontrolled; fresh process/service is not
called physical cold disk. No registry/WPR/cache/power-plan changes.

Asset workloads cover diamond/fanout/disconnected graphs, raw/PPM/PNG, missing-file
rollback, decode rejection and recovery. Publication factored into a private
method so the same real publication can be measured after a test receives the
worker reply, without including worker waiting. Scalar-scene probes add mixed
integer/text fields, serializer-only/caller-I/O, preparation/commit and early/late
failure costs. Save probes split snapshot, codec, envelope and actual durable
replacement; include large queued-command/RNG metadata and read/rename/codec/
UTF8/file-limit errors. First asset compilation caught test visibility/slice
coercion errors, corrected; retained diagnostics in target/io-asset-compile-failed.log.
CPU baseline acquisition in progress. Next: repeat baseline and heap windows,
attribute retained copies, fix measured costs if justified, remeasure, document
three domain dispositions and run full verification/public workflows.

This increment is now complete. Two before/after CPU runs for assets/saves,
two unchanged scene CPU runs, one baseline heap/workflow acquisition for the
affected raw/save configurations and two final phase/workflow acquisitions for
raw512fanout,PNG16diamond,scene10000 and save262144 pass. Eight fresh-process
monitored runs exit0. Raw scan peak new allocations67259136→209564 bytes;
whole-workflow135296385→67537738–67537742 bytes. Save load total allocations
21404784→17158475 bytes without a lower peak. TOML dominates scenes; no format
or registry rewrite. Results, protocols and domain dispositions are durable in
PERFORMANCE_REVIEW.md; contracts,README,CHANGELOG and ROADMAP reflect them.
Physical cold disk,network storage and cross-platform device budgets are explicit
deployment limits, not measured by fresh service/process. CPU runs retain the
inactive test allocator wrapper equally before/after; normal facade dependency
closure excludes DHAT and its new transitive dependencies. New source/root
ownership regressions pass; existing UI assertions gain explicit usize empty-array
types solely because the dev-only serde_json dependency affects inference.

Full ./scripts/verify.ps1 exit0 with process-local jobs2,testthreads1,shared target
(target/io-closure-full-verify.log): formatting,workspace check,all-target Clippy,
all workspace tests,CLI end-to-end,dependency boundaries and whitespace. Intentional
CLI bad-code/exit/missing-bin fixtures print diagnostics but all suites pass.
Public asset-reload --smoke,scene-serialization and world-saving examples exit0
(target/io-closure-{asset-smoke,scene-example,save-example}.log). No sibling edits;
initial dirty files and untracked workbench remain preserved. Exact acquisition
logs and summary CSV paths are in the durable review. The milestone checkpoint
remains because Milestone4.5 is still active. Next domain: remaining audio/native
device and platform lifecycle costs/disposition, then other open roadmap gates.

## 2026-10-05 Audio/native lifecycle closure increment

Scope: remaining native output/mixer control latency, repeated-output process
memory and Windows lifecycle disposition. Engine starts at
80e131cab31791c827136b24c2bd42818c194010 with a clean worktree. Sibling changes
are preserved; no example edits are planned. Added ignored domain probes using
the existing native-output feature and backend APIs, without dependencies or
production API changes. Native acquisition uses silent PCM and zero gain.

Pilot caught asynchronous publication of paused state versus playback position;
the probe now allows two 25ms settled observations before requiring position
stability. Native stream is HyperX Cloud Stinger Core Wireless + 7.1, stereo F32
48kHz, supported buffer480 frames. Manual sleep was performed by the maintainer
while the same process held a paused stream. Maintainer reported waking; releasing
the file gate preserved position, resumed Playing in4.536ms, confirmed subsequent
position progress and completed shutdown in15us. This is explicit audio lifecycle
and maintainer-confirmed sleep, not automatically delivered runtime power hooks.
System-event query does not corroborate the exact sleep interval and is not used
as proof. Logs target/audio-manual-sleep.{stdout,stderr}.log; pilot failures and
compilation diagnostics remain in target/audio-native-{build,pilot}.log.

Repeated fresh-process workloads: voices1/16 use22 cycles twice; voices64 use360
cycles twice. Each cycle shares one10-second PCM clip across looping voices,
observes actual mixer progress, all-voice pause/resume, stable paused position,
stop and one natural completion, empty handle/command cleanup, stream errors and
backend callback CPU fractions. OS process private/resident memory is sampled
every250ms by target/audio-native-acquire.ps1. No other benchmarks/builds run
alongside acquisition. Six output logs and target/audio-native-memory.csv retain
raw observations. Final measurements, native window shutdown and full verification
are pending; domain gate is not yet marked complete.

Acquisition complete: all six output processes exit0. The64voice runs each cover
360cycles,23040 looping submissions and about108.5s. After two excluded cycles,
submit p950.735–0.807ms; mixer progress p9518.533–18.570ms; pause/resume
p9510.859–10.994/10.725–10.731ms. Callback CPU p95 fractions about0.0577,
maximum0.10146 across mixed active/idle callback samples. Each64voice process has
406post-startup memory observations; private peaks6340608/6287360bytes and
resident15147008/15130624bytes, with near-plateau private sample means. No claimed
heap/device allocation bound or hours-long stability. Two22iteration native release
probes identify asynchronous frame release: median486.465–487.528ms, versus2us
drop signalling, matching the backend's detached500ms stream-manager poll.

Three real Windows/GPU window probes pass: initialization476.488–571.693ms,
1024-system shutdown50.3–56.3us, finish/resource drop38.583–42.347ms. Synthetic
callback tests remain separate from actual OS sleep. Native/headless classic_2d
smokes exit0 with no audio initialization/playback diagnostic; logs
target/audio-platform-classic-{build,native-smoke,headless-smoke}.log. Sibling
working-tree changes remain identical to the initial list; no sibling edits.

WASAPI loopback was additionally possible through existing CPAL output-device
input mode. No microphone or saved recording. Maintainer silenced other output.
Absolute-only detector/pre-play pilots failed; coherent1000Hz/RMS gating rejects
interference and has normal silence/DC/other-tone/non-finite regressions. Two
retained runs pass with median28.4126/28.5029ms,p9528.7674/29.6629ms. A subsequent
confirmation times out before first detection and is retained as a capture
reproducibility limit. After adding state/position/peak diagnostics, two final
runs pass: median28.6840/28.8856ms,p9529.3851/29.3356ms. Each successful capture
run explicitly reports one excluded startup underrun/overrun; no measured-cycle
capture error. This is callback delivery latency, not acoustic/wireless latency.
Durable review records all outcomes and concrete follow-ups, not just successful
logs. No unsupported capture API or device-recovery subsystem is implemented.

Feature Clippy initially rejects long probe functions; extracted manual wait and
diagnostic helpers. Later Clippy requires array chunks, safe midpoint and a
sample-sized float progress threshold. An automated manual-gate confirmation
also caught periodic-position aliasing: comparing a100ms loop at exactly100ms
can see the same position after genuine playback. The helper now polls for more
than one sample of progress. Final guard run2cycles/64voices with an already
released gate passes (no second actual OS sleep claimed), log
target/audio-native-final-guards-3.log. Final native-feature Clippy passes and
audio tests13passed,5manual probes ignored,0failed. Logs
target/audio-platform-feature-{clippy,tests}-final.log. First fullverify passes;
final fullverify after these test-only changes also exits0 with process-local
jobs2/testthreads1 (target/audio-platform-full-verify-final.log). Formatting,
workspace check/all-target Clippy/tests, generated-project CLI end-to-end,
dependency boundaries and whitespace all pass. classic_2d audio regressions:
2passed,1manual probe ignored,0failed (target/audio-platform-classic-audio-tests.log).
README/ROADMAP/RUNTIME_APIS/CHANGELOG and the durable review now close the
audio/platform gate within recorded envelopes and explicit limits. Existing
sibling dirty paths are unchanged. No staging/commits. Milestone4.5 remains open;
next are simulation/grid/collision/snapshot domain closure and the other gates.
