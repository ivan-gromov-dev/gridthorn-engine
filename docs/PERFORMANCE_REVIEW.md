# Milestone 4.5 performance review

Started on 2026-10-03 after the maintainer reported poor native performance in
the multilingual workbench, including after example-side layout reuse. This is
an engine-wide review of implemented capabilities, with measured, focused fixes.
Milestone 4 remains open until this review and its own native acceptance finish.

The first increment adds a public-API CPU workload to the sibling workbench and
records debug/release samples and confirmed duplicate paint in
[the active checkpoint](work-in-progress/milestone-4-5.md). Display/live-clock metadata and whole-engine CPU/GPU/present acceptance remain outstanding.

The renderer's opt-in `GRIDTHORN_RENDER_PERFORMANCE` environment variable records
the first 240 successful presented frames in memory and writes `render_cpu` CSV
rows to stderr when the renderer is dropped. Unset it for normal runs. It also
reports adapter/backend/driver, physical surface extent and configured present mode.
Rows contain acquire, colored/UI geometry, resource preparation, encode, submit
and present CPU microseconds plus generated vertex count and vertex-buffer bytes.
Encode includes geometry/resources; do not add these nested timings to it.
Resources include textured geometry/batching and creation/upload calls. Submit
includes encoder finishing. CPU upload-call time is not GPU transfer time, and
CPU present-call time is not presented frame interval or GPU execution time.
Counts exclude texture bytes, staging/internal driver allocations and GPU memory.
No per-frame logging occurs during collection; opt-in pipeline timers continue
after the sample limit. The first row includes startup/cold work; warm analysis
must identify its exclusions and interaction sequence explicitly.

Renderer rows now also include uploaded vertex-data bytes, colored/UI cache-hit
status, host present-call intervals and retained CPU vertex capacity bytes.
`vertex_bytes` describes geometry used for drawing; it is no longer an upload
counter. Uploaded bytes exclude textures and backend staging/padding. Host
intervals are measured between successful CPU present calls; the first is zero,
and gaps include engine work, event-loop pacing and skipped/suspended rendering.
These are a cadence proxy, not compositor/display presentation timestamps.
PresentMon was not found on PATH on the reference host; displayed-frame timing
and whole-frame GPU execution remain unmeasured.

When enabled, diagnostics request `TIMESTAMP_QUERY` only if the adapter supports
it. `timestamp_query` in configuration metadata reports availability. An isolated
readback slot asynchronously resolves start/end render-pass timestamps; the frame
loop polls without waiting and skips busy slots. `render_gpu` rows report paired
frame index and GPU pass microseconds; `render_gpu_summary` reports collected,
skipped, errored and pending samples. Frame indices continue across reconfiguration.
Collection is limited to the first 240 successful frames. No blocking readback or
per-frame printing occurs, and normal runs request no timestamp feature.
Pass time excludes upload/transfer execution, queue waits, timestamp resolve and
display/compositor work. Unsupported timestamps, pending shutdown samples and
sampling gaps are unavailable data, not zero GPU work.

## Evidence and limits

`GRIDTHORN_TEXT_PERFORMANCE` enables per-service text diagnostics. At most 8192
successful layout calls and 8192 successful rasterize calls are stored separately.
`text_cpu` rows print at service destruction; `text_cpu_summary` reports stored
and total successful counts, making truncation explicit. Layout units are input
UTF-8 bytes; rasterize units are output horizontal pixel spans, not glyph/pixel
counts or GPU bytes. Durations include validation, shaping/layout diagnostics or
raster snapshot construction respectively, excluding returned-result destruction
and collector publication. Failed calls are not sampled. Indices are independent
operation counters, not host/present frames; the probe covers all callers, not
only paint. Default services create no collector or timers; timers stop for an
operation once its storage limit is reached. Glyph raster-cache hits can still
require reconstruction of the immutable span snapshot.

The same flag also prints `text_layout_cache` at service destruction: hit/miss,
eviction/oversized-bypass counts, final retained entries/key bytes/glyphs/lines,
and the independent peak of each retention count. Copied key bytes are UTF-8
text/family lengths, not allocator capacity or total process/backend memory.
Cold construction/first-prepare observations in `--performance` are fresh service
observations, not a controlled cold OS/font-file-cache baseline.

`GRIDTHORN_UI_PERFORMANCE` enables router-owned layout diagnostics. Up to 240
successful layout calls are stored; `ui_layout` rows print when the final cloned
router releases the shared collector. Arrangement includes layer ordering;
text geometry and paint are separate phases. Focused-decoration time is nested
within paint and consumes prepared field geometry for selection/caret decoration;
active preedit work is included and still prepares its separate composition text.
Indices count successful layout calls, not host or presented frames. Busy
collector locks skip samples, reported by `ui_layout_summary`; collection never
waits on a held lock. Default routers allocate no collector and take no phase
timestamps. Optional timers continue after the storage limit. Whole-frame CPU,
GPU execution and displayed-frame intervals are outside this probe.

The sibling workbench also supports `GRIDTHORN_WORKBENCH_PERFORMANCE` for bounded
input-system phase diagnostics. `workbench_cpu` rows distinguish prepare before
input, real input routing, scripted workload routing, effect handling (including
localized preview refresh), prepare after effects and empty-event anchor refresh.
Snapshot capture and router layout are nested prepare timings, not additional
costs to add. Host-frame indices differ from renderer and native callback indices.
This example-only probe consumes public APIs, excludes startup/animation/render
extraction/platform requests/stdout, and does not introduce an SDK profiling API.

GRIDTHORN_WINDOW_PERFORMANCE records up to 240 preparation, 240 redraw and
240 extraction
callbacks, independently, and prints `window_cpu,phase,sample,elapsed_us` at
shutdown. Preparation includes lifecycle idle (runtime schedules), render-frame
extraction, platform control application and redraw request. Redraw measures the
renderer call. Extraction measures only the lifecycle render_frame callback (the
standard runtime reads/clones RenderFrame from the world), when a renderer exists.
It excludes renderer set_frame, replacement/drop of its previous frame, GPU work
and preparation in game systems. Extraction is nested inside preparation: never
add them. Its collector stops taking timestamps at its independent cap.
These are wall-clock CPU callback durations, including any blocking
inside those calls, not thread CPU usage. They exclude event-loop waiting, native
input callbacks, initialization and shutdown. Phase indices are independent and
must not be treated as paired frame IDs or added as a whole-frame percentile.
Disabled collection takes no timestamps and stores no samples. Native sample
counts do not establish actual displayed-frame timing.

`GRIDTHORN_RUNTIME_PERFORMANCE` records up to 240 calls independently for
PollEvents, Input plus state/scene transitions, aggregate fixed updates, Update,
PostUpdate and Render. It emits `runtime_cpu,phase,sample,elapsed_us` when the
runtime is dropped. These wall-clock schedule durations include game systems and
blocking work. Startup, clock/control sampling, FrameTiming publication, window
input snapshots, extraction and shutdown are excluded. Fixed-update duration
includes all ticks and their FixedTime publication; zero-tick frames are retained.
PollEvents/Input samples can exist even if later clock advancement fails. Indices
are per phase, not universal frame IDs. Disabled/full collectors take no timestamps.
No public profiling API or whole-frame/GPU/display budget is implied.

Windows release Japanese native idle/editing smoke, two runs each on 2026-10-03,
with runtime and window diagnostics enabled, produced 121 runtime/preparation and
119 redraw samples per run. After excluding each phase's first ten calls, editing
Input/transitions p95 was 1078 / 1126 us; other schedule phases were at most 13 us
p95. Window preparation p95 was 1150 / 1202 us. Idle Input/transitions p95 was
85 / 58 us and preparation 155 / 128 us. Redraw p95 was 8.71–8.81 ms editing and
9.48–9.53 ms idle, including renderer blocking. These separate phase percentiles
cannot be subtracted or summed to infer extraction cost or presented cadence.
Entity/event scaling and native DPI 2 remain unmeasured; the empty-runtime probe below extends this baseline.
A preliminary Windows debug probe measured one workbench router layout at about
105 ms at DPI 1 and 119 ms at DPI 2; bitmap layout took about 0.2 ms. Empty routing
took 0.03–0.06 ms. These are diagnostic samples, not release baselines or measured
native frame times. The example now reuses unchanged layout, but the maintainer
still observes poor performance. The remaining CPU/GPU split is unmeasured.

Code inspection identified repeated text preparation, painting before hidden-layer
filtering, and raster text represented as many colored spans/rectangles. Their
relative costs must be measured before choosing an implementation. An atlas,
layout cache or retained GPU buffer is a candidate, not a committed design.


## Empty runtime and render-frame extraction baseline

On 2026-10-03, two Windows release probes measured `run_timed_frame` with no game
resources/entities and 0, 1 or 32 no-op systems on each of PollEvents, Input,
FixedUpdate, Update, PostUpdate and Render. Each configuration starts a fresh
runtime, runs Startup and 1000 warm frames, then times 100 batches of 1000 frames.
Elapsed input is exactly 0, 1 or 8 default fixed steps (16,666,667 ns each).
Runtime diagnostics are disabled. Timing includes clock/control/resource work,
loop/black_box overhead and per-frame fixed-step validation, but excludes setup,
startup/shutdown, output, input snapshots, windowing and rendering.

| Systems per stage | Fixed ticks per frame | Mean us/frame, runs 1 / 2 |
| --- | --- | --- |
| 0 | 0 | 2.435 / 2.386 |
| 0 | 1 | 2.959 / 3.351 |
| 0 | 8 | 6.842 / 6.751 |
| 1 | 0 | 2.463 / 2.543 |
| 1 | 1 | 2.985 / 3.018 |
| 1 | 8 | 6.949 / 6.434 |
| 32 | 0 | 2.592 / 2.591 |
| 32 | 1 | 3.290 / 3.160 |
| 32 | 8 | 7.479 / 7.239 |

The manual domain probe is ignored in normal CI; reproduce it alone with
`cargo test -p gridthorn_app --release --locked measure_empty_runtime_and_schedule_dispatch -- --ignored --nocapture --test-threads=1`
and `GRIDTHORN_RUNTIME_PERFORMANCE` unset. CSV rows carry whole-batch nanoseconds
and frame counts. Percentiles of divided batch durations describe batch means,
not individual-frame latency. These fixed synthetic elapsed values do not measure
host cadence or actual simulation-system work. Overlapping/noisy results do not
establish per-system marginal cost or general entity/event scaling.

Two further native Japanese idle/editing release runs each, with only window
diagnostics enabled, collected 121 preparation/extraction and 119 redraw samples.
After independently excluding the first ten calls, extraction p95 was 2 / 1 us
idle and 2 / 2 us editing (integer microseconds truncated by the collector).
Editing preparation p95 was 1038 / 1199 us; idle 121 / 118 us. Extraction is nested
in preparation; renderer replacement/drop and display timing remain unmeasured.
No runtime dispatch or snapshot-clone optimization is justified by this workload.
Game systems, populated-world/input/control scaling and other domains remain open.

## Populated world and input publication scaling

Two additional Windows release probe runs on 2026-10-03 measured public world
access and the runtime window bridge without a native window, rendering, UI or
OS input. No agent-launched build/test work ran concurrently with sampling.
Runtime diagnostics were disabled. These are synthetic wall-clock baselines,
not whole-engine frame budgets or populated game acceptance.
Host metadata read after the probes: AMD Ryzen 5 5600X, Windows version
10.0.26200.0, rustc 1.99.0 (b940084d7, 2026-09-28), configured Balanced power
scheme. Live clocks and OS background load were not captured.

World configurations use 0, 1000, 10,000 or 100,000 persistent entities, each with
one u64 Counter component (one homogeneous archetype). Resident-idle uses zero
ticks and no game systems; fixed-scan uses one default fixed tick per frame and
one system visiting every Counter through for_each_component_mut and incrementing
it. Each fresh runtime runs Startup, 32 warm frames and 50 batches of 32 frames.
Construction, entity spawn, shutdown, output and final correctness checks are
outside timing. Every component is checked against the exact expected tick count;
visited count must equal population.

| Entities | Resident-idle mean us/frame, runs 1 / 2 | Fixed-scan mean us/frame, runs 1 / 2 |
| --- | --- | --- |
| 0 | 3.50 / 2.35 | 3.79 / 3.66 |
| 1000 | 2.42 / 2.35 | 6.59 / 4.82 |
| 10,000 | 3.18 / 2.28 | 14.12 / 13.94 |
| 100,000 | 2.43 / 2.26 | 102.91 / 124.93 |

This workload gives no evidence of population-dependent resident-idle cost.
Traversal is population-dependent, but includes both world query/mutation and
the sample system's writes. It is not a pure ECS backend benchmark. Mixed
archetypes, churn, scene ownership/removal, random access and game data remain
unmeasured; no general world-scaling limit is established.

Input configurations use 0, 32, 1024 or 16,384 events per synthetic host frame,
with an empty runtime and zero fixed ticks. Pointer events have distinct relative
x deltas; full KeyboardEvents alternate KeyD press/release, logical "d", no repeat
or synthetic flag. Unicode commits carry distinct indexed Latin/Cyrillic/Arabic/
Japanese/combining text. Templates are built before timing. Eight warm frames
precede 32 batches of eight frames. Timed work clones each fixture event, delivers
it through RuntimeWindowLifecycle::input, snapshots InputBuffer, replaces the
world's InputState (including destruction of the old snapshot), and runs the timed
runtime. There is no input-consuming game system or UI routing. Event order/content
is checked against the complete template after every batch outside timing, and
another snapshot must have no leftover events. Setup/teardown/output are excluded.

| Events/frame | Pointer mean us/frame, runs 1 / 2 | Keyboard mean us/frame, runs 1 / 2 | Unicode commit mean us/frame, runs 1 / 2 |
| --- | --- | --- | --- |
| 0 | 3.52 / 2.59 | 3.04 / 3.69 | 2.59 / 2.44 |
| 32 | 3.53 / 3.58 | 7.74 / 8.40 | 7.03 / 7.18 |
| 1024 | 55.38 / 51.20 | 174.66 / 172.67 | 148.87 / 143.48 |
| 16,384 | 550.79 / 567.68 | 3301.93 / 3387.77 | 3699.07 / 3767.74 |

Unicode commit templates contain 1558 / 51,114 / 840,858 UTF-8 payload bytes at
32 / 1024 / 16,384 events respectively; keyboard logical strings add one byte per
event. These are payload sizes, not allocation or retained-memory measurements.
The fixture clone is additional harness work; results cannot be attributed only
to InputBuffer or native delivery. No thresholds truncate/coalesce event streams:
ordered input and immutable snapshots retain their existing semantics.

Both probes are manually ignored in normal CI; run alone with runtime diagnostics
unset. Reproduction commands (each was run twice successfully):

```powershell
cargo test -p gridthorn_app --release --locked measure_populated_world_scaling -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_app --release --locked measure_input_publication_scaling -- --ignored --nocapture --test-threads=1
```

Raw CSV rows contain batch elapsed nanoseconds and frame counts. Derived p95 of
batch means is not individual-frame p95. No dispatch/traversal change is justified
by this baseline. Small event bursts are inexpensive here; larger synthetic bursts
warrant separate clone/allocation attribution before an input optimization. Native
OS mapping/IME/capture, consumption/routing, control counts, localization and mixed
ECS workload dispositions remain open.
## Control routing and localization scaling

Two Windows release runs per probe on 2026-10-03 used the same host/toolchain
recorded above, without concurrent agent-launched build/test work. These are
warm synthetic wall-clock measurements. Batch-mean percentiles do not represent
individual-call/frame tails; native display/IME and cold asset preparation remain
outside the results.

UI probe: 16 / 128 / 1024 flat enabled bitmap-font buttons under one column panel,
DPI 1, each 100x30 logical pixels, viewport tall enough to expose every button.
No text service, field editing, scrolling or registered layers. Every configuration
starts a fresh tree/router and prepares one detached layout before timing.
Eight warm calls precede 32 batches of eight calls. Layout calls include fresh
arrangement/paint and result destruction. Routing uses a borrowed prebuilt event
slice and the unchanged layout; it includes atomic tree/router cloning, input
scope preparation, hit testing, visual refresh and result destruction. Cases:
empty batch, or 32 CursorMoved events within the first button, last button or
outside all controls. No click effects are expected. Complete consumed/world
event indices and target IDs are checked outside timing.

| Controls | Layout mean us/call before, runs 1 / 2 | Empty routing before → after, runs 1 / 2 | 32 first-button events before → after, runs 1 / 2 |
| --- | --- | --- | --- |
| 16 | 7.02 / 8.08 | 4.91 / 6.07 → 2.58 / 2.50 | 75.41 / 81.43 → 4.10 / 4.09 |
| 128 | 103.59 / 101.47 | 74.81 / 68.26 → 48.48 / 43.57 | 660.58 / 648.59 → 51.12 / 48.80 |
| 1024 | 2388.90 / 2413.78 | 2331.84 / 2216.25 → 2017.67 / 2036.18 | 7141.44 / 7192.09 → 2144.71 / 2027.17 |

Code inspection identified an unchanged full UiLayout clone for each routing
scope even when no layers are registered. The router now borrows the immutable
layout in that case; registered closed/open roots keep their filtered owned
scopes, and atomic tree/router replacement is preserved. Existing layer tests
and a new plain-to-registered/closed/reopened layer transition test verify input
filtering and unchanged detached geometry/paint. All 54 targeted UI tests passed.
The release native workbench mixed smoke passed after the change.

At 1024 buttons, last-button 32-event means changed from 7619.23 / 7554.99 to
2099.14 / 2104.05 us; outside means from 7387.98 / 7322.85 to 2043.93 / 1999.91 us.
Layout is not changed by this fix (after means 2478.85 / 2538.02 us at 1024).
Remaining roughly 2 ms empty-routing cost and layout growth warrant separate
attribution. No gain is claimed for the layered native workbench; it registers
layer roots and does not take this plain-tree shortcut.

Localization probe: four installed catalogs (en-US, ru, ar-EG, ja), each with
exactly 16 / 256 / 4096 messages. Selected non-English locales have one explicit
en-US fallback. Every configuration builds/validates/prepares catalogs before
timing; message IDs/parameters are prebuilt. One result and resolved locale are
checked, then 100 warm calls precede 50 batches of 100 calls. Outputs are owned,
black-boxed and dropped within timed batches; output consistency is checked after.
Cases: last indexed literal, one text interpolation, plural plus NUMBER, decimal
NUMBER message, standalone decimal and a message present only in fallback.
English has no fallback case. Decimal input is 12345.5, minimum width 2, grouping
true; plural input 2; interpolation Player-日本語. This is 69 configurations and
3450 batch rows per run. Locale selection, cold parser/bundle work, replacement,
missing/invalid-message errors and long fallback chains are not timed.

| Operation | Range of mean us/call across sizes/locales/two runs |
| --- | --- |
| Literal | 0.12–0.21 |
| Interpolation | 0.27–0.44 |
| Plural + NUMBER | 0.81–1.06 |
| Decimal NUMBER message | 0.76–1.25 |
| Standalone decimal | 0.74–1.76 |
| One-locale fallback | 0.12–0.24 |

No localization optimization is justified by these warm samples. Standalone
number formatting still prepares its formatter per call as documented. Cold
validation/publication costs and broader localization disposition remain open.
Both probes are manually ignored in normal CI; reproduce alone in release mode
with GRIDTHORN_UI_PERFORMANCE unset (the two packages keep feature unification
identical to these measurements):

```powershell
cargo test -p gridthorn_app -p gridthorn_localization --release --locked measure_control_and_pointer_routing_scaling -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_app -p gridthorn_localization --release --locked measure_localization_catalog_and_formatting_scaling -- --ignored --nocapture --test-threads=1
```
## Indexed UI node lookup

A follow-up on 2026-10-03 isolated repeated whole-tree ID search: paint and router
visual refresh perform many node reads/commands, each formerly walking the root
recursively. UiTree now owns a private immutable BTreeMap from ID to child-index
path. Validation precedes construction; successful replace builds a new index,
and style/value/visual/scroll commands leave topology intact. Cloned trees share
lookup metadata through Arc while keeping independently owned mutable nodes.
The root is private to the tree owner so topology changes go through replace.
Public node IDs, command/error behavior, painter order and Debug output are retained.

The same UI probe now additionally measures repeated construction/drop through
UiTree::new(tree.root().clone(), default_theme), including fixture-node cloning,
validation and index allocation/drop. Graph authoring and initial setup remain
outside timing. The baseline and changed implementation each ran twice, with the
same 18 configurations, eight warm calls and 32 batches of eight calls (576 rows
per run). Existing no-layer scope borrowing is present in both versions. These
are warm batch means, not cold OS observations or individual-frame percentiles.

| Buttons | Operation | Before mean us/call, runs 1 / 2 | After mean us/call, runs 1 / 2 |
| --- | --- | --- | --- |
| 16 | Construction/drop | 2.30 / 1.54 | 4.33 / 3.25 |
| 16 | Layout | 9.06 / 7.27 | 7.70 / 7.26 |
| 16 | Empty routing | 2.60 / 2.57 | 2.19 / 2.24 |
| 128 | Construction/drop | 11.99 / 13.32 | 19.63 / 19.68 |
| 128 | Layout | 116.73 / 126.67 | 78.67 / 83.97 |
| 128 | Empty routing | 42.54 / 42.44 | 16.09 / 16.75 |
| 1024 | Construction/drop | 153.71 / 172.90 | 211.20 / 230.55 |
| 1024 | Layout | 3172.63 / 2687.73 | 713.66 / 728.03 |
| 1024 | Empty routing | 1975.26 / 1944.58 | 154.72 / 146.32 |
| 1024 | 32 first-button pointer events | 2221.78 / 2046.54 | 214.39 / 185.68 |
| 1024 | 32 last-button pointer events | 2246.53 / 2064.12 | 184.72 / 140.09 |
| 1024 | 32 outside pointer events | 2111.59 / 2010.84 | 229.81 / 182.33 |

Repeated lookup costs shift from sibling searches to map lookup and traversal of
one stored path; construction and replacement pay for building paths. Copied path
payload for the flat 1024-button probe is 1024 usize values (8192 bytes on this
64-bit host). The existing 4096-node and depth-63 limits bound any one index to
at most 4096*63 child indices (1.969 MiB on 64-bit), excluding map entries, slice
headers, Arc and allocator metadata. This is a conservative structural bound,
not measured process memory or allocator retention. Clones of one topology share
one index; different replacements/caller-retained trees may own multiple indices.

Four new domain regressions cover moved/reordered sparse IDs, clone isolation,
rejected replacement and unknown commands, and accepted depth/node-count limits.
The focused UI suite passed 58 tests. Release workbench mixed native smoke also
passed; no native performance gain is inferred from a correctness smoke. Remaining
work includes measured retained/backend memory and layered/text routing, mixed
control trees and further allocation/layout cost attribution. No dependency or
public API expansion accompanies the index.
## Registered-layer text-field routing

The release probe `measure_layered_text_routing_scaling` measures one registered
direct-child panel with 16/128/1024 bitmap fields, each containing `Review 123`.
All fields are visible in the detached raw layout; the layer is closed, open
nonmodal or open modal. DPI is 1; viewport is `[300, fields * 30 + 100]` logical
pixels. This includes retained geometry/paint for closed roots via the public raw
layout path, rather than router layout that omits closed-root text geometry.
It excludes asset-font shaping, typing/preedit, overlapping layers and native input.

```powershell
cargo test -p gridthorn_app -p gridthorn_localization --release --locked measure_layered_text_routing_scaling -- --ignored --nocapture --test-threads=1
```

Each of 18 configurations warms eight calls then records 32 batches of eight;
two runs per implementation, 576 CSV rows per run. Fixture/layout/registration
and opening are outside timing. Calls include atomic tree/router cloning, scope
construction/filtering, routing and result release. Input is empty or 32
CursorMoved events over the first field, without clicks or edits. Checks outside
timing verify consumption/world-event order, absent effects and unchanged source
placements/paint. Percentiles are batch means, not single-call/frame tails.
Host/toolchain are unchanged; live clock/background activity are uncontrolled.

| Fields | State | 32-event mean before, us (runs 1/2) | Mean after, us (runs 1/2) |
| --- | --- | --- | --- |
| 16 | closed | 192.20 / 187.61 | 137.57 / 133.02 |
| 16 | open | 251.14 / 240.25 | 208.48 / 204.72 |
| 16 | modal | 287.05 / 285.82 | 282.79 / 242.17 |
| 128 | closed | 1582.61 / 1551.16 | 1130.57 / 1023.72 |
| 128 | open | 2212.19 / 2194.55 | 1583.40 / 1497.05 |
| 128 | modal | 2571.17 / 2515.12 | 1813.38 / 1700.86 |
| 1024 | closed | 16981.21 / 17724.97 | 9593.86 / 9800.97 |
| 1024 | open | 31208.49 / 31140.13 | 15020.45 / 15677.30 |
| 1024 | modal | 38009.30 / 38067.18 | 18477.88 / 17484.05 |

Scopes previously cloned full paint and searched linear ID lists inside placement
filters. They now copy scale/placements/prepared text geometry without primitives;
BTreeSet membership preserves the existing placement sequence. An intermediate
paint-only implementation was sampled twice: at 1024 fields/32 events closed
means were 13152.20/14259.86 us, open 31553.28/28909.18, modal
42679.26/35758.18. It does not establish stable open/modal improvement alone.
Final empty-route means at 1024 fields are 774.98/738.60 us closed,
508.11/547.97 open, 588.89/623.28 modal. Closed empty routing did not improve
against 679.92/686.87 us before. Small-case set construction is not assumed cheaper.

Input scopes retain no paint. Copied geometry/placements and temporary ID sets
remain proportional to the supplied layout/tree and are released with the scope;
no persistent cache is added. This is an ownership observation, not measured
allocator bytes or process memory. Geometry/scope setup still repeats per event,
and atomic tree clones remain. The 1024-field modal burst still exceeds 16.67 ms
on average; these samples do not satisfy the native workbench acceptance gate.
Next: attribute copies, repeated scope setup, hit testing and focus checks, then
cover asset-font editing and overlapping layers.

Raw logs: `target/layered-before-<1|2>.log`, paint-only
`target/layered-after-<1|2>.log`, final `target/layered-final-<1|2>.log`;
`target/layered-summary.csv` includes empty routes and batch-mean p95.
Builds completed before sampling; no concurrent agent-launched builds/tests ran.
A DPI-2 regression verifies closed/open/modal picking and immutable source geometry
and paint. Native mixed and Japanese editing smokes passed as correctness checks.

## Shared immutable editing geometry

The next focused increment removes field-geometry deep copies from registered
input scopes and UiLayout clones. Each fully prepared layout owns an immutable
Arc<BTreeMap<UiNodeId, TextGeometry>>; cloning shares field values, caret stops
and selection segments. Fresh tree/router layouts prepare a separate map. Scope
placement filtering remains private to each scope and does not mutate that map.
No process-wide cache, dependency or public API is added. Arc Debug preserves
the previous map contents in UiLayout diagnostics.

Repeated the exact registered-layer probe above twice before/after on the same
host/toolchain, without UI diagnostics or concurrent agent-launched builds/tests.
Each run has 18 configurations, eight warm calls, 32 batches of eight and 576
rows. These are per-call batch means, not individual-event/frame percentiles.
Only geometry ownership changed; workload/source/flags and feature unification
remain identical. Means in microseconds, run 1 / run 2:

| Fields | State | 32-event mean before | Mean after sharing |
| --- | --- | --- | --- |
| 16 | closed | 137.13 / 138.83 | 49.14 / 53.52 |
| 16 | open | 205.58 / 209.96 | 131.92 / 131.28 |
| 16 | modal | 275.95 / 236.89 | 170.10 / 184.23 |
| 128 | closed | 1157.60 / 943.36 | 303.46 / 318.36 |
| 128 | open | 1588.54 / 1491.38 | 912.26 / 1148.34 |
| 128 | modal | 1737.90 / 1814.59 | 1124.70 / 1277.30 |
| 1024 | closed | 9331.12 / 10333.95 | 2592.14 / 2919.88 |
| 1024 | open | 15163.35 / 15465.58 | 8281.70 / 8198.14 |
| 1024 | modal | 17499.75 / 26019.77 | 11911.35 / 12096.98 |

The second modal baseline varied substantially: batch-mean p95 51624.40 us
versus 18696.35 in run 1. Background load/live clocks remain uncontrolled;
do not derive a precise universal speedup from its elevated mean. Both final
modal runs improve against the lower first baseline; final batch-mean p95 is
13624.80/14642.62 us. This still excludes every other frame stage, asset-font
editing, multiple overlapping layers and native publication; it does not prove
the whole-engine 16.67 ms CPU or displayed-frame acceptance gate.
Final 1024-field empty-route means: closed 322.44/436.68 us, open
463.53/406.86, modal 525.81/533.95. Placements, ID sets and scope construction
still repeat per event; atomic tree cloning and focus/hit checks remain.

Ownership regression retains an old layout clone across a text edit and fresh
DPI-2 router layout, then routes against the old snapshot. Old/fresh values remain
independent. Weak ownership confirms that neither routing nor the surviving router
keeps geometry alive after the last layout/scope; transferring primitives also
releases the layout's geometry. Sharing retains one geometry map per originating
snapshot, regardless of clone count. Callers retaining multiple fresh layouts
still retain multiple maps: no aggregate byte cap or process-memory reduction is
asserted. Fields/placements/paint are bounded by existing tree/layout contracts.

Tradeoff: fresh arrangement allocates an empty Arc map, then successful geometry
preparation replaces it with a separately owned Arc map; scoped clones increment
and decrement atomic ownership. A single exploratory flat bitmap-button run
before/after recorded layout means 7.02→7.52 us (16), 77.66→72.52 (128),
686.26→736.55 (1024). This is not repeated evidence of layout improvement or a
causally attributed regression; the measured routing benefit does not make fresh
layout creation free. Native mixed and Japanese editing smokes passed as
correctness checks. Targeted composition tests: 52 passed, 2 manual probes ignored.

Logs `target/shared-geometry-before-<1|2>.log` and
`target/shared-geometry-after-<1|2>.log`; summary
`target/shared-geometry-summary.csv` includes empty calls and batch-mean p95.
Exploratory flat logs `target/shared-geometry-flat-<before|after>.log`, summary
`target/shared-geometry-flat-summary.csv`. Next: repeated layer scope/ID setup,
then asset-font editing, overlapping layers and remaining domain scaling.

## Batch-local routing scopes

The next increment prepares the base input scope once per route_events call and
retains at most one lazily prepared pointer scope. Repeated events in the same
layer/capture scope reuse placements and ID filtering. A popped layer changes the
stack length and rebuilds the base before the next event, clearing the pointer
scope. A changed layer under the cursor or capture replaces the pointer scope.
This is valid because routing only pops layers and tree commands preserve topology
and registered roots during a batch. Hit_test_layers uses the same preparation.
All scope state lives in private routing/scopes.rs and is discarded on success or
error; it is not a persistent router cache. Each call still includes fresh atomic
tree/router cloning and first scope preparation. Cursor/hover/focus and live
control-value/enabled checks continue for each event.

Repeated the unchanged registered-layer release probe twice before/after, alone
without diagnostics/concurrent agent-launched builds or tests. Same host/toolchain,
18 configs, eight warm calls, 32 batches of eight, 576 rows/run; workload and
feature unification are unchanged. Scopes reset on every timed call, so the gain
comes from reuse among the 32 events, not across calls. Means in microseconds:

| Fields | State | 32-event mean before (runs 1/2) | Mean after (runs 1/2) |
| --- | --- | --- | --- |
| 16 | closed | 51.80 / 49.23 | 5.86 / 5.50 |
| 16 | open | 147.63 / 133.60 | 8.50 / 9.24 |
| 16 | modal | 181.59 / 172.63 | 10.36 / 9.86 |
| 128 | closed | 324.96 / 301.43 | 26.30 / 29.03 |
| 128 | open | 871.50 / 818.15 | 48.86 / 49.45 |
| 128 | modal | 1180.39 / 1180.40 | 56.88 / 57.77 |
| 1024 | closed | 2820.56 / 2545.59 | 306.61 / 344.43 |
| 1024 | open | 8343.04 / 7917.84 | 602.64 / 577.47 |
| 1024 | modal | 11440.99 / 11595.29 | 572.54 / 574.48 |

At 1024 fields final empty-route means are closed 311.16/331.34 us, open
412.49/459.07, modal 492.01/465.09; baseline empty-route means are respectively
343.45/314.41, 438.36/405.76 and 497.99/504.94. Empty calls have no repeated
scope work to remove, so no consistent empty-call gain is claimed. Final modal
32-event batch-mean p95 is 849.74/668.91 us. Percentiles are still means of eight
calls, not individual-event/frame tails. The static bitmap/one-layer workload
does not establish asset-font editing, overlapping-layer burst performance,
whole-engine CPU/GPU budgets or displayed-frame cadence.

Memory remains bounded to the base plus one pointer placement snapshot per batch;
both share the caller's immutable field geometry and omit paint for registered
layers. Temporary membership sets are released after filtering. Changing pointer
scope replaces the cached snapshot; alternating layers/capture can still rebuild
on every event. No allocator-byte/process-memory or peak-memory reduction is
claimed. Plain-tree scopes remain borrowed; no render primitives are cloned on
that path. Existing immutable layout and atomic rejection contracts are preserved.

Three focused regressions cover two modal dismissals followed by underlying/base
clicks in one DPI-2 batch, capture movement between layers followed by release and
new capture, and rejection after a dismissal/scope rebuild and oversized text
commit. The rejected call restores the original stack, focus and text, and a
subsequent dismissal works. Targeted composition tests: 55 passed, 2 manual
probes ignored. Native mixed and Japanese editing smokes passed as correctness
checks. Raw logs `target/batch-scope-before-<1|2>.log` and
`target/batch-scope-after-<1|2>.log`; summary `target/batch-scope-summary.csv`
includes empty routes and batch-mean p95. Next: asset-font editing and overlapping
layers, then remaining domain scaling and retained-memory attribution.

## Warm asset-font editing with overlapping layers — 2026-10-04

The sibling workbench now owns the ignored public-API probe
`interface/test/layered_editing.rs::measure_overlapping_font_editing`. It loads
the existing Noto Sans/Arabic/JP assets through the normal font service, then
authors three overlapping 600×300 panels at offsets [60,30], [120,60], [180,90].
Each has one 600×120 text field. All three layers are open; the top is nonmodal
or modal. Viewport is 1000×800 logical pixels at DPI 1/2. Four short strings use
English, Russian, Arabic and Japanese scripts. Font-service locale remains en-US:
these are script fixtures, not a localization-publication benchmark.

```powershell
cargo test --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_multilingual_workbench --release --locked measure_overlapping_font_editing -- --ignored --nocapture --test-threads=1
```

Run alone with UI/text diagnostics disabled. Two runs, 32 configurations each,
ten warm calls plus 100 individually timed calls (3200 raw rows/run). Font loading,
tree creation, opening, initial layout, input fixtures and text-session stop
acknowledgment are outside timing. `pointer_32` routes 32 alternating positions in
exposed areas of the three fields, changing the pointer-scope key. Modal input
blocks lower fields. `preedit_commit_paint` selects all top-field text, routes
composition, prepares its paint, routes one of two alternating commits, then
prepares/replaces the final layout. Timing includes selection, atomic routes,
shaping/raster preparation, result release and prior-layout replacement. It
excludes GPU/frame submission, platform requests, native IME and displayed cadence.
Warm caches see only the fixed preedit and two bounded committed values.

Percentiles below are nearest-rank statistics of individual calls; ranges span
both top-layer policies and both runs, not combined-percentile calculations.
All values are microseconds on the previously recorded host/toolchain; background
activity and live CPU clocks remain uncontrolled.

| Script | DPI | Editing-cycle median range | Editing-cycle p95 range |
| --- | --- | --- | --- |
| English | 1 | 108.10–120.90 | 124.60–161.20 |
| English | 2 | 216.30–237.00 | 276.10–316.40 |
| Russian | 1 | 89.90–121.00 | 165.70–192.20 |
| Russian | 2 | 258.90–346.30 | 390.10–502.70 |
| Arabic | 1 | 58.40–58.70 | 64.60–84.40 |
| Arabic | 2 | 125.10–148.80 | 213.10–321.30 |
| Japanese | 1 | 77.90–85.50 | 140.40–151.90 |
| Japanese | 2 | 199.80–291.80 | 342.30–514.80 |

Across all configurations, 32-pointer median is 15.10–34.70 us and p95
15.50–37.90 us; largest sampled pointer call 95.80 us. Largest editing cycle
614.60 us. These small warm fixtures show no new bottleneck requiring a production
fix. This disposition is limited to the recorded workload; no before/after speedup,
native whole-engine budget, glyph coverage or general text-size guarantee is
asserted. Long/unique text, many fields/layers, cold caches, cache churn, clipboard,
bidi visual-navigation acceptance, actual IME and allocator/retained-byte accounting
remain open. Modal and nonmodal fixture costs must not be compared as an isolated
modal-policy effect: pointer paths and background load differ.

Correctness checks verify layer stack/focus, event consumption/order, lower-layer
modal picking, actual preedit and changed preedit paint in the first excluded warm
call, committed final text, cleared composition and nonempty final paint. Initial
probe development lacked the adapter's TextInputChanged(active=false) feedback
after field-focus changes: commits were suppressed while awaiting stop. Final-value
validation caught that invalid fixture. Corrected feedback is supplied outside
timing; early failed runs are excluded. Setup/sampling/validation are separate
helpers after Clippy's function-length check. No production behavior changed.

Final logs `target/font-editing-<1|2>.log`, summary
`target/font-editing-summary.csv` (individual configs/p99/max) and
`target/font-editing-ranges.csv`. Sample acquisition finished before checks/builds.
Example package Clippy and seven regular tests passed; one manual probe is ignored
by default and ran twice explicitly. Native --editing-smoke passed for en-US, ru,
ar-EG and ja, configured 1000×800 physical pixels at DPI 1; stdout/stderr logs
`target/font-editing-native-<locale>.*.log`. These are injected-event correctness
smokes, not DPI-2 native or real OS IME acceptance. Next scaling priority is mixed
ECS/churn and cold localization; retain explicit long-text/cache/layer follow-ups.

## Execution order

Work through the Milestone 4.5 checklist in ROADMAP. Each increment records its
baseline, hypothesis, change, regression coverage and before/after measurements.
Keep fixes in the owning domain and keep every increment buildable. Public
semantics, dependency direction and authoritative determinism remain requirements.
Record difficult-to-reverse backend/resource decisions through an ADR.

1. Establish reproducible workloads and a domain inventory before optimizing.
2. Resolve the observed UI/text/presentation bottleneck first, measuring CPU work
   separately from GPU execution, present waits and frame pacing.
3. Review the remaining implemented domains and fix measured bottlenecks in
   risk-reduction order. A reviewed domain may need no change; record that evidence.
4. Run the complete workload matrix again, document supported limits, and obtain
   maintainer acceptance of the native workbench before returning to Milestone 4.

## Reproducible measurements

Record both repository revisions, Rust/compiler versions, profile/features,
OS, CPU/GPU/driver, display refresh, window size, DPI, locale, input sequence,
workload size and power conditions. Release is the acceptance profile; retain
debug measurements separately as development-usability evidence. Distinguish
cold startup/font/cache work from warmed steady state. Repeat comparable runs
and report sample counts, duration, median, p95, p99 and worst frame/work time.

Measure schedule stages, text shaping/rasterization, layout/paint, routing,
geometry extraction, GPU uploads/submission/execution and present waits separately.
Record allocations, retained/peak memory, primitive/vertex counts, uploaded bytes
and draw calls where relevant. Report unavailable counters explicitly. Use bounded
logs or opt-in instrumentation; diagnostic collection must not dominate the run.
Do not infer GPU costs from CPU submission timing or claim gains from FPS alone.

## Workload and ownership matrix

| Domain                     | Representative workload                                                                                                         | Review focus                                                                                      |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- |
| Runtime/world              | Empty runtime, schedule systems, increasing entity counts                                                                       | Idle overhead, resource access, iteration, cloning, schedule work                                 |
| Input/UI/localization      | Workbench idle, hover, slider drag, scrolling, typing/selection, clipboard, IME, language changes, nested windows and animation | Invalidation, repeated formatting/layout/paint, hidden nodes, text anchors, routing allocations   |
| Fonts/text                 | English, Russian, Arabic and Japanese at DPI 1/2; short and bounded long fields                                                 | Shaping/fallback, cache lifetime, raster samples, memory, editing latency                         |
| Rendering/presentation     | Workbench, Crystal Trail and Timber Harbor; increasing sprite/text counts                                                       | Geometry, clipping, batching, uploads, resource churn, GPU work, present waits                    |
| Assets/scenes/saves        | Asset-reload plus scene/world-save examples; increasing documented data sizes                                                   | Decode/I/O, worker publication, serialization, memory peaks, frame stalls                         |
| Simulation/grids/collision | Headless simulation and Timber Harbor; increasing grid/entity/query sizes                                                       | Fixed-tick cost, pathfinding/placement, collision queries, snapshots/RNG, scaling and determinism |
| Audio/platform             | Existing native audio and window lifecycle workflows                                                                            | Command/worker cost, suspension/shutdown, pacing, unavailable-device behavior                     |
| CLI/build footprint        | Generated-project check/run/build, engine/examples builds                                                                       | Cold/warm build time, dependency growth, binary size, reproducibility                             |

Use existing sibling examples where possible. Benchmark scaffolding belongs to
its domain or a focused sibling example. This milestone does not introduce the
Milestone 6 inspector, profiler UI or development protocol. Native runs start on
the available Windows host; record Linux/macOS coverage or explicit platform
limits without presenting untested results as cross-platform guarantees.

## Acceptance gate

For the native workbench, the initial release target on the recorded reference
hardware is 60 FPS at 1000×800 logical pixels, DPI 1 and 2, across the four locales.
Measure idle and continuous interaction/animation separately: p95 engine CPU work
and p95 GPU execution must each fit the 16.67 ms frame budget; warmed p99 presented
frame intervals should stay within 33.33 ms. Record present-mode/display constraints,
excluded external stalls and cold-operation latency explicitly. These are targets,
not current capabilities. Any change to the target needs a documented reason and
maintainer agreement; persistent workbench slowness cannot be waived as a generic
performance deferral.

For the other domains, establish workload sizes and budgets with the baseline,
then assess scaling, measured regressions and bounded resource use. Avoid promising
arbitrary world sizes or untested device guarantees. Every implemented domain needs
a review disposition: measured and acceptable, fixed and remeasured, or a concrete
limitation with evidence and an explicit follow-up. Record remaining limits in the
owning contracts and completion review.

Completion requires reproducible before/after results, regression checks for fixed
behavior, full repository verification, successful native workloads and maintainer
acceptance. After Milestone 4.5, complete Milestone 4's language/platform and native
IME acceptance; neither milestone is closed by compilation or smoke alone.

## Mixed world and cold localization increment (2026-10-04)

Engine baseline 59449e9d23b3e1634b673140ac754c01d3470c82, Rust 1.99.0
(b940084d7 2026-09-28), Windows reference host recorded above. Two sequential
release runs, no concurrent agent-launched builds/checks during acquisition.
CPU clocks, background activity and power conditions remain uncontrolled.
These are individual-call nearest-rank percentiles, 50 samples/config/run.

```console
cargo test -p gridthorn_world --release --locked measure_mixed_world_and_scene_churn -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_localization --release --locked measure_cold_localization_publication -- --ignored --nocapture --test-threads=1
```

World fixture has 1000/10000/100000 persistent entities split evenly between
Position, Velocity and 64-byte Inventory components, plus a replaceable scene
with 10% as many entities. Each entity has one domain component; scene entities
also have ownership metadata. This does not measure arbitrary multi-component
queries or structural component insertion/removal. Three traversals mutate one
integer each. One untimed traversal precedes 50 cycles of traversal, scene removal
and repopulation. Removal includes owner matching, temporary entity collection
and backend despawn; spawning includes per-entity scene-ID cloning. Output and
count assertions are outside timing. Final counts verify persistent survivors.

| Persistent entities | Traversal p95 us | Scene removal p95 us | Scene spawn p95 us |
| --- | --- | --- | --- |
| 1000 | 5.60–9.80 | 25.20–57.50 | 6.80–9.60 |
| 10000 | 17.90–29.00 | 134.30–143.20 | 74.90–102.70 |
| 100000 | 162.40–167.20 | 1299.50–1343.60 | 703.10–729.90 |

Largest sampled removal was 2042.20 us; largest traversal 183.20 us.
No production change is justified by this bounded workload. Large scene teardown,
more owner partitions, memory peaks and arbitrary component combinations remain
unmeasured; these samples do not establish a fixed-tick or whole-frame budget.

Localization fixture uses 16/256/4096 messages, one NUMBER message and otherwise
literal ASCII labels, for en-US/ru/ar-EG/ja. Each sample constructs a fresh asset
and service, formats the first numeric message, then replaces the catalog with a
separately validated changed candidate. The initial cycle is excluded; 50 fresh
cycles follow. Source generation, candidate validation, correctness checks and
service destruction are outside publication timing. Replacement includes releasing
the previous bundle. Tests verify locale chain, exact changed labels and retained
old output. Fresh-service caches are cold; process/OS/compiler caches are warm.
This is not cold process startup, filesystem I/O or a complex reference-graph test.

| Messages | Validation p95 us | Construction p95 us | First NUMBER p95 us | Replacement p95 us |
| --- | --- | --- | --- | --- |
| 16 | 12.70–25.80 | 2.70–5.60 | 1.70–5.40 | 3.80–9.80 |
| 256 | 336.50–409.60 | 37.50–48.40 | 4.30–6.50 | 67.20–130.60 |
| 4096 | 5350.20–7212.80 | 475.90–729.60 | 8.90–15.40 | 899.80–1643.20 |

Ranges span locales and runs, not pooled percentiles. Largest validation was
9602.20 us; largest replacement 2040.50 us. Keep source validation away from
latency-sensitive polling as required by LOCALIZATION.md. No runtime optimization
is justified by these samples; preparation/publication is bounded but consumes
part of a shared frame budget. Multi-locale simultaneous publication, complex
references, retained bytes and allocation peaks remain unmeasured.
Raw logs: target/mixed-world-<1|2>.log and target/cold-localization-<1|2>.log;
per-run median/p95/max summary: target/mixed-cold-summary.csv. With 50 samples, nearest-rank p99 equals the recorded maximum. Neither probe runs
in the default test suite; these are reproducible manual workloads, not timing
thresholds in CI. Remaining long/unique-text, expanded-layer and input-allocation
work stays open in the roadmap.

## Long text, cache pressure and expanded layers (2026-10-04)

Baseline engine 59449e9d23b3e1634b673140ac754c01d3470c82 plus the preceding
uncommitted world/localization probes, Rust 1.99.0 and the Windows host recorded
above. Examples were inspected only and retain their pre-existing changes.
No production behavior, font selection, cache limits or dependencies changed.
Two sequential isolated release runs of each probe, diagnostics unset and no
concurrent agent-launched builds/checks during acquisition. CPU clocks, background
activity and power conditions remain uncontrolled.

```console
cargo test -p gridthorn_render --release --locked measure_long_text_and_cache_pressure -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_render --release --locked measure_japanese_layout_attribution -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_app --release --locked measure_expanded_layer_pressure -- --ignored --nocapture --test-threads=1
```

Text uses the renderer's licensed Noto Sans/Arabic/JP fixtures, explicit service
locales en-US/ru/ar-EG/ja, primary Noto Sans at 24 logical pixels, width 600 and
WordOrGlyph wrapping. Script phrases repeat 8/64/256 times, followed by a unique
numeric suffix. Working sets of 1/32/96 strings test immediate reuse, aggregate
geometry pressure and the 64-entry limit. All strings are prepared once before
100 individual timed calls cycling through that working set. Returned backend
buffer identity is checked against weak references outside timing to identify
actual hits, rather than assuming that a warm backend means a layout-cache hit.
A live original layout is retained and its measurement verified after churn;
this caller-held buffer is outside cache retention limits. Missing glyphs are
checked during setup. Source generation/output/validation and final layout release
are excluded; internal eviction/allocation is included. Inputs share repeated
phrases, so backend caches are warm; this is not unique-glyph/font discovery.

For 256 repetitions, one-string workloads hit all 100 times, p95 0.20–0.30 us.
Both 32/96-string workloads miss all 100 times: long entries exhaust the aggregate
16K-glyph/1024-line budgets before the 64-entry limit. This is expected bounded
retention behavior, not evidence to raise limits without memory measurements.

| Script | 256 repeats, 32/96 strings: layout p95 us | 64 repeats, DPI 2: warm raster p95 us |
| --- | --- | --- |
| English | 1098.90–1947.00 | 2937.80–3945.80 |
| Russian | 1288.70–1413.80 | 4176.90–5399.50 |
| Arabic | 2767.80–2887.50 | 2465.80–2641.10 |
| Japanese | 11192.10–11824.70 | 4730.20–6319.70 |

Raster uses 8/64 repeats, one layout, DPI 1/2 and 100 individual calls after one
untimed raster call. Full snapshot equality is validated outside timing. Snapshot
creation and output geometry allocation are included, final release is excluded.
The CSV hit column describes buffer reuse for layout; for raster it is a fixed
placeholder and does not measure raster output caching. Long 256-repeat rasters
are excluded from this bounded probe. No GPU upload/execution or native frame
budget is measured. Percentiles are nearest-rank per config/run; ranges span runs
and configurations, never pooled percentiles. Largest sampled Japanese layout
was 14175.40 us, and DPI-2 64-repeat Japanese raster 8508.80 us.

A separate Japanese attribution fixture repeats 256 times across 96 strings,
using primary Noto Sans versus Noto Sans JP, with WordOrGlyph versus None wrapping.
Same fonts, service locale, font size, width and validation; two repeated runs:

| Primary/wrapping | Layout p95 us |
| --- | --- |
| Noto Sans / WordOrGlyph | 11078.20–11201.60 |
| Noto Sans / None | 10978.40–11106.40 |
| Noto Sans JP / WordOrGlyph | 1319.60–1354.10 |
| Noto Sans JP / None | 1281.90–1306.00 |

This attributes the dominant difference to primary/fallback selection rather than
line wrapping for this fixture. It does not isolate backend font matching from
shaping or guarantee equivalent font metrics/appearance. Explicit Japanese primary
selection is an application styling choice; the engine must preserve authored
font semantics. Long Japanese fallback cache misses remain a measured limitation;
backend attribution and an equivalent-semantics fix remain open. No engine-wide
speedup is claimed. Raster span generation/allocations also deserve separate
attribution before changing cache/resource policies.

Expanded UI fixture has 3/16/64 overlapping open panels, 1/16 bitmap fields each,
1000x800 logical viewport/DPI 1, with optional top modal scope. Fields contain
Review 123; each panel clips and has a distinct offset. After 10 excluded cycles,
50 cycles time one batch of 32 cursor moves inside the top field and one router
layout. Correctness checks outside timing verify complete input consumption, top
picking and open-layer count. Setup/registration/source construction and final
layout release are excluded; layout assignment releases its previous snapshot
inside timing. This workload always targets the top layer; it does not measure
alternating lower scopes, layer transitions, actual IME, asset-font editing or
native rendering. Modal/nonmodal ranges are observations, not isolated policy gains.

| Layers x fields | Layout p95 us | Pointer32 p95 us |
| --- | --- | --- |
| 3 x 1 | 5.70–6.30 | 4.60–5.10 |
| 3 x 16 | 78.30–91.90 | 25.40–30.10 |
| 16 x 1 | 29.70–36.70 | 13.50–15.70 |
| 16 x 16 | 472.60–547.00 | 127.40–138.70 |
| 64 x 1 | 118.30–147.60 | 54.10–96.30 |
| 64 x 16 | 1818.60–1844.80 | 657.80–688.70 |

No production fix justified by the expanded bitmap-layer fixture. Largest layout
was 2070.30 us and pointer batch 1114.50 us. With 50 samples nearest-rank p99 is
max; text's 100-call p99 is separate from max. Allocator counts, opaque backend
memory, long-field editing/preedit, asset-font expanded layers and DPI-2 native
acceptance remain open.
Logs: target/text-pressure-<1|2>.log, layer-pressure-<1|2>.log and
japanese-attribution-<1|2>.log. Summaries with median/p95/p99/max and layout hit
counts: target/text-layer-pressure-summary.csv and japanese-attribution-summary.csv.
Ignored manual probes do not impose CI timing thresholds. The milestone's full
native budget and remaining domain review remain outstanding.

Verification: full ./scripts/verify.ps1 passed with process-local
CARGO_TARGET_DIR=<absolute engine target> and RUST_TEST_THREADS=1, including all
workspace tests and the generated-project offline check/native smoke. Log:
target/text-layer-verify-shared.log. The shared shorter build output avoids the
nested MSVC path failure observed in the temp-location experiment; it does not
prove the cause of the earlier invoked.timestamp failure. Environment overrides
were restored; no persistent configuration or CLI behavior changed.

## Request-local raster span reuse (2026-10-04)

Baseline 129f9de8bbcbfcd43183dd0e65a0045e25d2de3b, clean engine working tree at
start, Rust 1.99.0 and previously recorded Windows host. No dependency/features
changed. Examples retain their existing manifest/lock/README/untracked workbench
changes; this increment edits engine files only.

Source inspection identified repeated per-pixel tint conversion and span building
for every glyph occurrence, even when Swash already cached the glyph image. The
renderer now reuses tinted, glyph-relative integer spans within one rasterize call
for layouts with at least 256 glyphs. Cache keys include font/glyph, size and
subpixel placement; tint is fixed for the request. Absolute integer translation,
ordered output and adjacent-span merging preserve the existing pixel stream.
Short requests use direct sampling: the initial unconditional cache prototype
increased short Arabic/Japanese DPI-1 medians and was corrected before handoff.

The request cache retains at most 128 entries and 65536 spans; saturation uses a
reusable scratch vector. Cache/scratch state is dropped before rasterize returns.
Vector capacity, temporary scratch, final output, Swash images and font caches are
not covered by a new byte budget. The existing one-million image-sample work limit
still counts every glyph occurrence, including cache hits, and errors preserve
caller-held snapshots. Allocation counts and peak bytes remain unmeasured; this
change avoids repeated span preparation but does not claim allocation-free drawing.

Used the existing measure_long_text_and_cache_pressure command above. Two before
runs, two intermediate unconditional-cache runs, and three final thresholded runs;
100 individual warm raster calls/config after one excluded raster. Acquisition
was isolated from agent-launched builds/checks. Third final run investigates the
inconsistent English sample; all final runs are retained rather than discarding
it. Background activity/clocks/power remain uncontrolled. Tables use per-run
nearest-rank percentiles; ranges are not pooled statistics.

| 64 repetitions / DPI 2 | Before median ms | Final median ms | Before p95 ms | Final p95 ms |
| --- | --- | --- | --- | --- |
| English | 2.608–2.696 | 2.112–2.932 | 2.853–3.138 | 2.216–5.127 |
| Russian | 3.856–3.882 | 2.929–3.128 | 4.330–4.729 | 3.199–3.654 |
| Arabic | 2.246–2.252 | 1.705–1.735 | 2.471–2.533 | 1.780–2.035 |
| Japanese | 4.467–4.519 | 3.074–3.147 | 4.856–5.683 | 3.268–4.012 |

These repeated-phrase workloads show consistent Russian/Arabic/Japanese gains;
English is mixed, with final run 1 reaching 9.448 ms max and 5.127 ms p95 while
runs 2/3 have medians 2.144/2.112 ms. No universal speedup or native frame-budget
claim is made. 8-repeat fixtures remain on the direct path; their full results
are in the summary, with the same uncontrolled host variability. Unique-glyph
workloads, cold rasterization and general cache-saturation latency remain open.
Long Japanese fallback shaping misses are unchanged by this raster optimization.
The optional backend shape-run cache was inspected but is not enabled: its
retention is an age-trimmed map without an entry/byte cap, so enabling it alone
would not establish the engine's bounded-retention requirements.

Regression coverage compares the expanded ordered raster pixel stream against
independent backend sampling for mixed Latin/Cyrillic/Arabic/Japanese, combining
marks, repeated/unique glyphs, DPI 1/1.25/2, colored partial alpha and transparent
tint. Cold rebuilds match snapshots; request-cache saturation respects retained
limits. Existing error/work-limit/font-owner/DPI/cache-clear tests continue passing.

Raw logs target/raster-spans-before-<1|2>.log,
raster-spans-after-<1|2>.log (intermediate only), and
raster-spans-final-<1|2|3>.log. Final summary with median/p95/p99/max:
target/raster-spans-final-summary.csv. Renderer text tests passed (20 passed,
2 ignored); final full verification is recorded in the checkpoint. Sibling release
build and native --editing-smoke --locale=<en-US|ru|ar-EG|ja> passed at configured
1000x800 physical pixels/native DPI 1. Logs target/raster-spans-workbench-build.log
and raster-spans-native-<locale>.log. Correctness smokes do not prove real OS IME,
DPI-2 native performance or displayed-frame timing.

Full ./scripts/verify.ps1 passed with the previously documented process-local
shared target/sequential-test settings, including workspace tests, generated-project
CLI smoke, dependency boundaries and whitespace. Log target/raster-spans-verify.log;
overrides restored. Native Workbench acceptance and the remaining domain matrix
stay open.
