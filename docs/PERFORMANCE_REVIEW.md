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
Vertex counters exclude texture bytes. Dedicated texture counters now report
uploaded and retained decoded RGBA payload; staging/internal driver allocations
and physical GPU memory remain unmeasured.
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
PresentMon was initially absent from PATH. The renderer review below now records
standalone PresentMon ETW display intervals and process GPU busy/wait evidence;
this does not close the full native interaction/DPI acceptance matrix.

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
successful calls for each of layout, rasterize, backend shape, diagnostic extract,
raster loop and snapshot construction are stored separately.
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

`shape` covers buffer construction/settings/text and backend shaping/layout;
`extract` covers engine line/glyph diagnostics and layout-cache insertion, on misses
only. `raster_loop` covers image lookup, work/clip checks and span construction;
`snapshot` covers the shared output owner after scratch release. These are nested
subphases of layout/rasterize, with independent counters and truncation. They do
not isolate backend fallback internals or allocator events; do not add their
percentiles or assume their sample indices describe the same request.

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

GRIDTHORN_WINDOW_PERFORMANCE also emits an initial window_configuration snapshot
with native physical size/DPI and optional current-monitor name/extent/origin/system
refresh in millihertz. None means unavailable; this is not displayed-frame timing.
It records up to 240 preparation, 240 redraw and
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

### Whole-process CPU accounting precision — 2026-10-04

`GRIDTHORN_WINDOW_PERFORMANCE` now records `process_frame_cpu` rows with redraw
sequence, whole-process CPU nanoseconds, wall nanoseconds and elapsed session
nanoseconds. A private safe OS clock adapter includes user/kernel time from all
process threads, including input and worker activity between completed redraws.
Startup before the first redraw is excluded. Collection retains at most 4096
rows; a failed clock read clears pairing so multiple frames are not merged.
Existing callback and renderer phase clocks remain independent.

The first release Japanese idle run used 1200 host frames at native DPI 1,
1000×800 physical pixels, DISPLAY2 1920×1080/144 Hz. After excluding redraws
1–120, 1079 CPU deltas span 15014.15 ms wall time and 796.875 ms accounted CPU
time (approximately 0.739 ms CPU per redraw on average). Clock errors: zero.
Only 46 deltas are positive; the smallest positive value is 15.625 ms, maximum
31.25 ms, and nearest-rank p95 is zero. Zero deltas remain in the statistics.
This demonstrates coarse Windows accounting on this host, not zero CPU work.
The units are nanoseconds, but the observed accounting granularity prevents
using these frame percentiles to establish the 16.67 ms CPU budget. Aggregate
CPU consumption is useful; precise per-frame CPU attribution remains open and
needs scheduler tracing or another validated high-resolution method.

The safe backend uses Windows
[GetProcessTimes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocesstimes)
through [ProcessTime](https://docs.rs/cpu-time/1.0.0/cpu_time/struct.ProcessTime.html).
This increment introduces only an internal diagnostic dependency, with no new
public profiling API or unsafe engine code. Native GPU/display capture and the
remaining quantitative matrix are not closed by this run.

Evidence: `target/native-cpu-idle-ja.log`, `target/native-cpu-release-build.log`,
`target/native-cpu-tests.log` and `target/native-cpu-verify-2.log`. Seven focused
diagnostic tests pass, including independent CPU/wall deltas, parallel work,
clock failure recovery and bounded retention. Full Windows verification passed,
including workspace tests, Clippy and dependency boundaries; the release native
smoke also passed. The sibling lockfile adds the diagnostic dependency while
preserving its prior workbench/game changes.

### Maintainer functional acceptance and measured targets

On 2026-10-04, after reviewing the coarse Windows CPU accounting and the effort
needed for precise tracing, the maintainer explicitly instructed closing the
native performance gate in documentation. The gate is accepted with the existing
recorded CPU/GPU/display evidence and manual functional acceptance. This disposition
replaces the requirement to complete the quantitative native matrix before closing
this particular gate; it does not establish full-matrix budget compliance.
Earlier sections retain the status at the time of their measurement; this
disposition supersedes their statements that native acceptance remains open.
Precise per-frame CPU attribution, controlled cold/warm whole-engine baselines
and missing quantitative interaction/locale/DPI cells remain documented follow-ups,
not blockers for the accepted native gate. No additional measurements were made
for this decision. Other Milestone 4.5 domain gates and the final completion review
remain open.

On 2026-10-04, the maintainer reported manual testing and accepted the
1000×800 logical-pixel window criterion at DPI 1/2. This is maintainer-reported
acceptance; no automated DPI 2 measurement or new CPU/GPU/display capture is
attached to that decision. The performance targets below remain unchanged.
The maintainer also confirmed prior manual testing of native clipboard and OS
IME behavior; their functional acceptance is complete. No timing traces or
per-locale/DPI performance matrix were supplied with that confirmation. The
complete measured interaction matrix is incomplete and retained as a follow-up
under the maintainer disposition above. These decisions do not close Milestone 4.5.

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

## Assets, scalar scenes and typed world-save I/O (2026-10-04)

Baseline 545c6ad61a3a3d8ba5b9148674ada3fecbd7ae18, clean engine tree at start,
Rust 1.99.0 and previously recorded Windows host. Ignored domain probes exercise
public subsystem APIs; only test fixture visibility changes in world-save tests.
No dependencies or public signatures changed. Release runs are sequential, two
per domain, no concurrent agent-launched builds/checks during timing acquisition.
Every config has two excluded cycles plus 20 timed individual calls per operation.
Percentiles are nearest-rank per run; with 20 samples p99 equals max. Disk cache,
CPU clocks, antivirus/background activity and power conditions are uncontrolled;
these are warm repeated-file measurements, not physical-disk cold throughput.

```console
cargo test -p gridthorn_assets --release --locked measure_asset_reload_io_and_publication -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_scene --release --locked measure_scene_persistence_scaling -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_app --release --locked measure_world_save_io_scaling -- --ignored --nocapture --test-threads=1
```

### Reload propagation and publication

Raw fixtures: 16/128/1024 files of 4096 bytes and 128 files of 65536 bytes.
Texture fixture: sixteen 256x256 P6 PPM images (196623 encoded bytes/image,
262144 decoded RGBA bytes/image). All configurations have a reverse-lexical chain:
asset-0000 depends on asset-0001, continuing to the last leaf. Each initially
unchanged scan is followed by a phase alternating edits to that leaf. Both a
synchronous store and independent background worker scan the same files. File
creation/editing, initial registration/dependency validation, correctness checks
and printing are outside timed spans. Synchronous scan includes reads, byte
comparison, propagation, ordering, decoding, preparation and commit. Frame request
is timed separately; request-to-poll includes worker scheduling, scan, a 1 ms sleep
between empty polls and publication. It is not an isolated worker CPU duration.
Ready poll times only the call that returns Some; empty_poll_max is the largest
empty call during that request. Scan results must exactly match reversed IDs;
source snapshots are checked after publication and old bytes remain immutable.

The old algorithm repeatedly scanned the complete graph for propagation and
selected each ready node by scanning the remaining set. Changed reverse chains
incurred quadratic traversal work. Reload now builds borrowed reverse adjacency,
visits dependents once and uses dependency counts plus an ordered ready set.
The lexicographically smallest currently ready ID is selected after every removal,
including newly ready IDs. Unchanged scans return after all file reads/comparisons.
Reads, decode/prepare and atomic commit/rollback semantics are preserved.

| 1024 x 4096 raw, changed leaf | Before median ms | After median ms | Before p95 ms | After p95 ms |
| --- | --- | --- | --- | --- |
| Synchronous scan | 198.322–204.406 | 59.889–61.271 | 215.341–221.681 | 62.965–63.839 |
| Request-to-poll | 202.850–207.470 | 61.470–62.526 | 214.257–219.019 | 63.103–67.600 |
| Ready poll | 0.481–0.502 | 0.432–0.521 | 0.649–0.778 | 0.684–0.895 |

After unchanged 1024-file scans: median 55.432–57.107 ms, p95 59.453–60.489 ms;
before median 57.371–58.446 ms. Changed-chain preparation improves roughly 3.3x;
publication is not claimed faster. Content polling still reads every registered
file, so large synchronous scans remain unsuitable for frame polling. Worker
round-trip latency is not a frame stall; completed snapshot destruction can be.

| Changed fixture after fix | Scan p95 ms | Ready poll p95 ms |
| --- | --- | --- |
| 16 x 4096 raw | 1.108–1.193 | 0.004–0.005 |
| 128 x 4096 raw | 8.364–9.017 | 0.046–0.060 |
| 128 x 65536 raw | 26.373–26.448 | 0.916–1.231 |
| 16 x 256x256 PPM | 25.936–26.962 | 0.767–1.141 |

Shutdown after idle completion is recorded once per config, not as a percentile
or active-I/O lifecycle benchmark. Graph regressions cover dynamically ready
lexical priority, unchanged external prerequisites, diamond overlap, and a reversed
1024-node chain; existing tests cover failed batch rollback/retry, retained texture
identity, worker failure and frame-boundary publication. PNG, branching/large fanout,
concurrent editing, active shutdown and cold filesystem cases remain open.

### Scalar scene persistence

100/1000/10000 scene-owned entities each have registered Health with one positive
u64 scalar; one persistent entity must survive every replacement. Each cycle
captures, encodes to TOML, parses, prepares independent domain values, then commits
replacement. Parsed documents match capture; every loaded scalar is validated and
persistent ownership preserved. Commit includes old-scene removal/new insertion.
Document/source generation, equality checks, sorting/assertions and final local
value destruction are outside timed spans. This service performs no filesystem I/O.

| 10000 entities, 1348993-byte document | Median ms | p95 ms |
| --- | --- | --- |
| Capture | 5.498–5.944 | 7.010–7.120 |
| TOML encode | 31.254–34.452 | 36.508–40.739 |
| TOML parse | 49.238–50.337 | 54.353–56.965 |
| Prepare | 3.656–4.117 | 5.018–6.576 |
| Commit | 2.106–2.422 | 2.890–3.163 |

At 1000 entities encode p95 2.645–3.253 ms and parse 5.067–5.728 ms.
Large-scene TOML work exceeds a 16.67 ms frame-sized interval in this fixture;
callers must coordinate explicit preparation/load boundaries. This does not imply
an automatic asynchronous scene loader or a new format. No registry/ECS mutation
optimization is justified by these samples; serializer/parser and allocation
attribution remain open. Many registered types, nested game adapters, unregistered
world populations and native loading stalls remain unmeasured.

### Typed world saving

Root Vec<u64> has 1024/16384/262144 increasing integers, one queued command, one
named RNG stream and default fixed configuration. Existing test codec creates a
comma-separated payload plus command suffix; encode includes per-number String
allocation/join, decode parses numbers. Snapshot cloning and codec cost remain
inside the engine save/load spans. Each cycle saves/loads in-memory, then saves
through create-new sibling/write/sync/rename and loads the actual file. Canonical
saves, exact file bytes and no leaked temporary files are checked outside timing.
This measures the specified game codec, not a pure envelope serializer or arbitrary
nested authoritative-world throughput. File loading is synchronous and warm.

| 262144 values, 1724250-byte document | Median ms | p95 ms |
| --- | --- | --- |
| Save document | 18.260–18.935 | 21.310–23.482 |
| Load document | 6.511–6.621 | 7.154–7.705 |
| Save file | 22.127–22.841 | 23.648–24.399 |
| Load file | 12.751–12.888 | 14.145–14.354 |

No claim that save/load fits a live frame. Keep the existing explicit between-tick
contract; durability is not traded for a faster save. Game-codec and snapshot-clone
attribution, complex roots/queues/RNG registries, error-path latency, files near the
16 MiB limit and network/cold storage remain open. No production save/scene change
is justified before attributing costs to envelope versus game codec/library/I/O.

### Separate process-resident memory observations

Windows PowerShell launches each compiled release test executable directly in a
fresh process with its one ignored probe filter, --ignored --nocapture
--test-threads=1 and GRIDTHORN_IO_MEMORY=1. Start-Process uses hidden windows and
separate stdout/stderr files. Every 50 ms the monitor refreshes Process and records
the maximum PeakWorkingSet64 until exit. The flag adds a 500 ms hold after all
configs/cleanup to permit the final cumulative peak read; it does not alter timed
operations. Exit codes are checked and the flag restored. These extra runs are
excluded from latency statistics because monitoring can perturb scheduling.

| Entire probe matrix in one fresh process | Peak working set bytes | MiB |
| --- | --- | --- |
| Assets before graph fix | 39731200 | 37.89 |
| Assets after graph fix | 40169472 | 38.31 |
| Scalar scenes | 95457280 | 91.04 |
| World saves | 29720576 | 28.34 |

This is cumulative process-resident memory across setup/all configs/validation,
including binary/runtime pages and allocator retention. It is not Rust heap,
allocation count, phase-by-phase peaks, a cold idle-subtracted budget, system file
cache or a cross-domain comparison on equivalent workloads. Only one memory run
per row; no memory reduction is claimed. Actual peak-byte ownership/retention and
allocator instrumentation remain open.

Logs: target/asset-io-<1|2>.log (before), asset-io-after-<1|2>.log,
scene-io-<1|2>.log, world-save-io-<1|2>.log; summaries
io-review-summary.csv and asset-io-before-after-summary.csv. Memory logs:
asset-memory-before.stdout/stderr.log and gridthorn_<assets|scene|app>-memory.stdout/stderr.log;
records asset-memory-before.csv and io-process-memory.csv. Manual probes remain
ignored by default; no CI timing thresholds or platform-wide budgets are promised.
Verification and public asset-reload smoke outcome are recorded in the checkpoint.

Full ./scripts/verify.ps1 passed with the previously documented process-local
shared target/sequential tests; settings restored (target/io-review-verify.log).
Public sibling asset-reload --smoke passed dependency publication, rollback after
intentional malformed image, recovery and shutdown (target/io-review-asset-smoke.log).
No displayed-frame or cross-platform performance acceptance is implied.

## Audio control and platform callbacks — 2026-10-04

Baseline: cbf2a390c26b498c3b7f347ffb243ecf7b10f046. This increment adds
measurement fixtures without changing production behavior, dependencies or APIs.
Run these separately, with no builds or other benchmark processes in parallel:

```powershell
cargo test -p gridthorn_audio --release --locked measure_audio_control_scaling -- --ignored --nocapture
cargo test -p gridthorn_app --release --locked measure_platform_lifecycle_callbacks -- --ignored --nocapture
```

Two isolated runs per probe passed. Each configuration has two excluded warm-up
iterations and 20 retained samples; each iteration constructs a fresh service
outside the timed phases. The recorded median uses the upper middle sample and
p95 uses nearest rank (19th of 20). Logs: `target/audio-control-{1,2}.log`,
`target/platform-callbacks-{1,2}.log`; derived `target/audio-platform-summary.csv`.
Sub-microsecond values approach timer resolution and are not stable budgets.

Audio uses Kira's mock backend, stereo PCM16 WAV decoded before sampling, 8 kHz,
256/8,000/80,000 frames and 1/16/64 voices playing clones of the same clip.
Queue timing includes cloning, ID allocation and collecting IDs. Play processing
includes queue drain, PCM-to-stereo-frame conversion, allocation and manager
submission. Suspend/resume time command submission, not completion. Controls
process one volume change and stop per voice. Teardown drops the manager and
its pending sounds; mock processing does not advance the mixer. Assertions outside
timing check queue emptiness, controlled voice counts and suspension flags.

| Workload/phase | Run 1 median / p95 | Run 2 median / p95 |
| --- | --- | --- |
| 1 voice, 80K frames, play | 0.259 / 0.305 ms | 0.253 / 0.305 ms |
| 64 voices, 256 frames, play | 0.299 / 0.528 ms | 0.294 / 0.444 ms |
| 64 voices, 80K frames, enqueue | 12.3 / 25.9 us | 7.1 / 14.4 us |
| 64 voices, 80K frames, play | 10.745 / 12.359 ms | 9.147 / 10.927 ms |
| 64 voices, 80K frames, suspend | 4.5 / 6.8 us | 3.5 / 5.2 us |
| 64 voices, 80K frames, resume | 3.8 / 6.7 us | 3.4 / 4.8 us |
| 64 voices, 80K frames, volume+stop | 31.5 / 41.5 us | 23.5 / 29.6 us |
| 64 voices, 80K frames, teardown | 3.263 / 4.552 ms | 2.621 / 3.638 ms |
| 1024 shutdown systems, callback | 19.3 / 27.9 us | 17.9 / 21.4 us |

Platform fixtures use 0/32/1024 shutdown systems, each incrementing a shared
counter. Startup and schedule construction are excluded. Timed suspend/resume
callbacks reset the frame timer and enqueue focus loss; shutdown runs the
schedule. Assertions check timer state, the exact shutdown counter and idempotent
second shutdown. Suspend/resume medians are 0–0.3 us at 1024 systems. This does
not include native event-loop dispatch, GPU surface recreation, OS suspension,
user resource destructors or audio integration. Automatic platform audio
suspend/resume remains deferred by the existing audio contract.

Disposition: control submission is small in these fixtures; larger repeated clips
make Play and teardown materially more expensive. Source inspection shows a fresh
frame allocation/conversion for every Play, but the combined phase does not isolate
its share. Next, attribute conversion and completed-voice retention before choosing
a bounded reuse policy. No unbounded decoded-sound cache is introduced. Worker
handoff/queue backpressure, real mixer processing, output-device latency, long-lived
playback cleanup, native shutdown and memory peaks remain open. Mock voice counts
represent handles controlled by the service, not audibly active voices. No native
frame budget, audible correctness or cross-platform acceptance is inferred.

## Audio batch reuse and completed voices — 2026-10-04

Continues the preceding probes on baseline cbf2a390c26b498c3b7f347ffb243ecf7b10f046;
the preceding uncommitted probe/docs changes are preserved. Output now removes
handles reporting Stopped at each process call, even for an empty queue. The count
excludes completed voices immediately; paused/looping voices remain controlled.
Cleanup requires continued process calls; no background cleaner is introduced.

A private prepared_batch module shares converted frames for clones with identical
sample storage, channel count and sample rate within one process call. Source
clips are retained in the batch to prevent address reuse. Retention is capped at
16 entries and 1 MiB of converted frames; saturated/oversized conversions bypass
reuse. The cache drops on return, including errors. Per-voice volume and looping
settings are reconstructed independently. Separate decoded clips and separate
process calls do not share conversions. This cap bounds extra retained frame
references, not all playback/mixer allocations or source sample bytes.

Two isolated after runs of the unchanged control probe passed. Compared with both
preceding before runs, 64 voices x80K frames Play median9.147–10.745ms becomes
0.323–0.379ms, p950.447–0.455ms. Teardown median2.621–3.263ms becomes
0.111–0.135ms. One voice x80K frames remains mixed: before0.253–0.259ms,
after0.245–0.283ms; no universal single-play speedup is claimed. 64x256-frame Play
median0.294–0.299ms becomes0.222–0.269ms. Logs target/audio-reuse-after-{1,2}.log;
comparison target/audio-reuse-summary.csv. Workload and quantile conventions are
unchanged from the preceding section; no builds ran alongside acquisition.

The separate conversion probe retains 64 constructed sounds from one stereo
80K-frame clip; it alternates direct conversion and batch reuse, two excluded
iterations and20 retained samples per mode. Timing excludes dropping sounds and
frame-equivalence assertions; it includes sound-data allocation/construction and
collecting outputs. This isolates conversion from manager submission, although
fixed mode order and allocator history remain limitations. Two runs give direct
median9.398/9.495ms (p9511.820/10.274ms) and reused median0.294/0.421ms
(p950.479/0.563ms), supporting conversion as the measured repeated-Play cost.

```powershell
cargo test -p gridthorn_audio --release --locked measure_pcm_conversion_reuse -- --ignored --nocapture
```

Logs target/audio-pcm-{1,2}.log. Tests cover sample/frame equivalence, independent
settings, distinct decode identity, per-call lifetime, oversized bypass, entry and
combined-byte saturation, natural completion with surviving looping voices, and
128 mixer-advanced playback cycles without handle accumulation. The mock backend
is explicitly advanced for completion tests; control performance runs retain the
preceding submission-only protocol. No output-device latency, audible correctness,
allocator peak reduction or worker handoff claim is inferred. No dependency or
public signature changes. Worker/native/memory follow-ups remain open.

## Example audio worker handoff — 2026-10-04

The sibling classic_2d example owns its audio worker; no engine worker subsystem
is added. The worker uses sync_channel(32), nonblocking try_send for Pickup and
Pause, blocking Shutdown delivery followed by join. Two isolated release runs of
its ignored audio/test/scaling probe passed. Each 1/32/1024/16384-request workload
has two excluded iterations and20 retained samples; quantiles use the preceding
upper-middle median/nearest-rank p95 convention. Logs target/audio-worker-{1,2}.log
and derived target/audio-worker-summary.csv in the engine workspace.

```powershell
cargo test --manifest-path ../gridthorn-examples/Cargo.toml -p classic_2d --release --locked measure_audio_worker_handoff -- --ignored --nocapture
```

The actual Sound::new(false) path loads/decodes the assets and spawns a fresh
worker before timing. Initialization may still execute asynchronously during
submission/shutdown. It exercises queue draining and voice-command construction,
without opening an audio device or invoking the mixer. Timed burst submission
calls public-in-example pickup; its API hides success, so accepted/dropped counts
are explicitly unknown. A joined worker is asserted outside timing.

The second fixture uses the same Request type and32-slot channel, but gates the
consumer until the burst completes. It deterministically accepts min(requests,32)
and drops the rest; it checks that Pause(true) is rejected when full. After release,
Shutdown is sent and joined; accepted pickup counts match exactly. This fixture
measures transport only, not the game's worker/output service. Thread creation,
ready handshake, gate release and assertions are excluded. Shutdown timing includes
blocking send, OS scheduling, processing accepted messages and join; it is not an
isolated worker service duration or a delivery-latency percentile.

| Workload/phase | Run1 median / p95 | Run2 median / p95 |
| --- | --- | --- |
| 32 requests, actual headless enqueue | 0.4 / 0.4 us | 0.4 / 0.4 us |
| 32 requests, actual headless shutdown | 118.6 / 127.5 us | 115.0 / 157.1 us |
| 16384 requests, actual headless enqueue | 72.1 / 81.5 us | 65.8 / 75.8 us |
| 16384 requests, actual headless shutdown | 37.7 / 151.8 us | 45.2 / 64.1 us |
| 16384 requests, gated transport enqueue | 50.5 / 120.1 us | 50.2 / 52.7 us |
| 16384 requests, transport shutdown | 46.2 / 58.9 us | 51.8 / 58.7 us |

Disposition: queue capacity prevents unbounded pending-request growth and try_send
keeps pickup submission nonblocking, but full-queue Pause drops are a concrete
reliability gap. The gated16384 burst accepts32 and drops16352; a shorter measured
enqueue duration does not imply successful audio delivery. No production behavior
is changed in this measurement increment. Next fix should preserve bounded effect
submission while reliably communicating the latest pause/resume state, with a
saturated-queue regression test. Native-device backlog, worker stalls, long-lived
output memory, device latency and hardware/platform shutdown remain open. Native
smoke proves a runnable lifecycle only, not overload behavior or audible quality.

## Reliable example pause state — 2026-10-04

Fixes the classic_2d pause-loss disposition above without adding an engine worker
API. Pause/resume now stores the latest desired state in a single atomic byte
before trying to enqueue a Wake. The consumer takes pending state before handling
each request. If the channel is full, an existing queued request triggers that
check; if it is empty, Wake unblocks recv. Wake carries no historical state, so
stale notifications cannot overwrite a newer pause/resume. Intermediate states
coalesce. Effects remain nonblocking/lossy with32 pending slots; shutdown still
uses blocking delivery and join. Worker progress is required: this is state
preservation, not bounded audible application latency or device recovery.

Regression tests call Sound.pause against a gated full queue and verify lone
pause, pause+resume, and alternating final pause. Idle/stale-wake tests verify
latest state and a later new transition. The performance transport fixture now
also publishes pause+resume through Sound.pause under saturation, then asserts
that the resumed state is consumed while all32 accepted effects are counted.
No source/device API signatures or engine dependencies change.

Two after runs passed with the preceding20-sample protocol. At16384 requests,
actual headless enqueue medians70.8/66.1us (p95201.7/91.6us), shutdown39.7/58.4us
(p9568.8/96.9us). Gated enqueue medians58.1/55.1us; transport shutdown59.8/56.8us.
Added two-call saturated pause+resume publication median0–0.1us, near timer
resolution; no precise sub-microsecond cost/budget is inferred. Gated shutdown
now consumes the additional mailbox state, so it is not an identical before
fixture. Actual pickup accepted counts remain unknown; retained scheduling
variance prevents claiming a throughput speedup. Logs target/audio-pause-after-
{1,2}.log. The reproducible command is unchanged from the preceding worker section.
Native device latency, long-lived memory and OS lifecycle remain open.

## Pathfinding scaling and hash lookups — 2026-10-04

Baseline834df86eaeb99a2ba951a8fe7a9093d6d369c37c, engine initially clean after
preceding audio work was committed. Ignored navigation/test/scaling probe covers
32/128/512-square rectangles, start(0,0), goal(side-1,side-1). Open terrain costs1;
weighted costs1+((column*17+row*31)%9); wall blocks column side/2, leaving valid
endpoints on disconnected sides. Budgeted open terrain settles at most
min(1024,side*side/4); other workloads allow side*side expansions. Fixtures are
analytical cost callbacks, not tilemap/occupancy lookups or game-agent workloads.

```powershell
cargo test -p gridthorn_grid --release --locked measure_navigation_scaling -- --ignored --nocapture
```

Two isolated before and two after runs passed, two excluded warmup iterations
and20 retained samples/config. Upper-middle median and nearest-rank p95 as above.
Timing includes endpoint validation, full synchronous search, all internal
allocation and result construction; assertions, fingerprinting, output printing
and result destruction are excluded. Before logs target/navigation-1.log and
navigation-before-2.log; after navigation-after-{1,2}.log; derived
navigation-summary.csv. First baseline predates the outside-timing fingerprint
addition. No benchmark acquisition overlapped builds or checks.

| 512x512 workload | Before median (two runs) | After median (two runs) | After p95 (two runs) |
| --- | --- | --- | --- |
| Open,262144 settled | 190.99 /193.41 ms | 91.41 /94.78 ms | 127.60 /105.38 ms |
| Weighted | 216.78 /234.34 ms | 111.26 /113.25 ms | 123.47 /129.53 ms |
| Unreachable wall,131072 settled | 91.85 /99.21 ms | 45.21 /49.91 ms | 58.31 /58.41 ms |
| Open,budget1024 | 0.373 /0.512 ms | 0.262 /0.203 ms | 0.406 /0.268 ms |

The focused fix replaces BTreeMap cost/predecessor stores with standard HashMap.
Neither table is iterated for decisions or results: the BTreeSet frontier still
chooses (cost,column,row), removes superseded entries and yields sorted diagnostic
frontier. Parent lookup reconstructs the same route. No dense whole-rectangle
allocation, A* heuristic, dependency or public signature is introduced. Hashing
has randomized seeds, but only lookup results affect the ordered algorithm.

All12 before/after configurations match same-toolchain DefaultHasher fingerprints
of complete Debug results (status/path/cost/visited/frontier); fingerprints are
an experimental comparison, not stable serialization or collision-free proof.
Regular tests add exact open-grid Manhattan expansion order, canonical tied route
and frontier checks at budgets0/1/3/8/16. Existing weighted/tie/repeatability,
unreachable/budget, endpoint/zero-cost and coordinate-limit tests pass. Probe
assertions check statuses, wall reachability size, path endpoints/adjacency/cost,
open optimal cost and frontier sorting outside timing.

Each sample records capacity bytes for owned path/visited/frontier vectors only;
this excludes search maps/queue, allocator overhead, terrain and process state.
A separate fresh after-process run sampled Windows PeakWorkingSet64 every50ms,
with a500ms post-workload hold enabled by GRIDTHORN_NAVIGATION_MEMORY. Observed
peak32,194,560 bytes (about30.70MiB) across the entire12-configuration matrix,
including test harness and fingerprint/validation allocations. No baseline
subtraction, per-query/phase attribution, heap peak or memory reduction claim.
Monitor log target/navigation-memory.log and navigation-memory-summary.txt;
monitored samples are excluded from the latency tables. Env override restored.

Disposition: lookup change roughly halves large-query time in these fixtures, but
512-square full searches remain well above frame durations. Use explicit settled
budgets where appropriate; BudgetExceeded does not prove unreachable and calls
cannot resume. No universal real-time limit is inferred from analytical terrain.
Maze/corridor topology, repeated agent queries, real occupancy costs, budget sweeps,
heap peaks, error paths and cross-platform measurements remain follow-ups. Fixed
simulation, placement, collision and snapshots are still unreviewed in this domain
increment. Public sibling pathfinding example passes and emits its SVG diagnostics.

## Placement occupancy and collision scaling — 2026-10-04

Baseline ef240d8756e640f72091a8f600c639cdde81dd92, engine initially clean. Placement
probe covers1024/16384/65536 objects, horizontal footprints1/16 cells. Anchors
are(index*32,0), so large case occupies1,048,576 cells. Population construction
is excluded. Each batch times1024 occupied-cell lookups using index*61 modulo
population;1024 free-row validations; and1024 object moves to row1 and back,
2048 relocations total. Validate includes resolved-cell allocation/destruction;
relocate includes footprint clone, validation, remove/commit and object-table work.
Assertions outside timing verify object count and moved/released anchor cells;
existing domain tests cover all footprint cells, conflicts, overflow precedence,
atomic rollback, overlapping moves, sorted objects and deterministic errors.

Two isolated before and after runs per placement fixture passed,2 excluded
iterations plus20 retained samples/config; quantiles as preceding sections.
Lookup-only cell storage changed from BTreeMap to HashMap; objects remain BTreeMap
and footprints retain ordered offsets. Cells are never iterated for public
ordering/errors; first-conflict/overflow checks and authoritative placement
semantics are preserved. Debug map order is not a stable format. No dependencies,
public signatures, terrain policy or whole-grid allocation change.

| 65536 objects,16-cell footprints | Before medians (two runs) | After medians (two runs) | After p95 (two runs) |
| --- | --- | --- | --- |
| 1024 occupied lookups | 0.537 /0.501 ms | 0.14 /0.14 ms | 0.24 /0.18 ms |
| 1024 free validations | 1.004 /1.013 ms | 0.83 /0.95 ms | 1.40 /1.14 ms |
| 2048 relocations | 6.931 /6.859 ms | 5.07 /5.49 ms | 7.39 /6.88 ms |

Single-cell65536-object relocation median0.672/0.682ms ->0.55/0.55ms. This is a
lookup improvement, not complete attribution of footprint cloning or object
storage. Logs target/placement-before-{1,2}.log, placement-after-{1,2}.log and
placement-collision-summary.csv. Allocator/heap/resident peaks and retained
capacity tradeoffs were not measured in this increment. Hash capacity and
footprint/object storage are explicit memory follow-ups; no memory saving claim.

Collision production behavior is unchanged. Ready-pair batches cover AABB/AABB,
circle/circle and circle/AABB,1024/16384/262144 pairs. Construction is excluded;
separations repeat1/2/4 units with unit radii/extents, giving overlapping/touching/
separated cases. Timed overlaps counts boolean hits; timed contact black-boxes
complete contact values then counts hits. Exact expected hit counts are checked
outside timing. No pair index, spatial query, world lookup, contact allocation or
response simulation is included. Two release runs/config passed. At262144 pairs,
overlaps median2.30–2.37ms for AABB,0.90–0.91ms circle,0.88–1.01ms mixed;
contact medians2.44–2.54/1.02–1.03/1.13–1.25ms respectively. These simple aligned
fixtures do not establish all geometry branch costs or a game frame budget.

Separate caller-owned all-pairs fixture covers64/256/1024 circles, all centered
at origin for dense or spaced4 units for sparse. It includes pair enumeration,
uses each unordered pair exactly once and asserts all/zero hits.1024 objects
produce523776 pairs: two-run median1.70–1.94ms dense,1.73–1.97ms sparse, with
p95 roughly2.00–2.58ms. Candidate count grows quadratically independent of sparse
geometry; narrow-phase API supplies no broad phase. A new spatial subsystem is
not justified solely by these synthetic fixtures. Games should measure their
candidate-generation policy. Collision memory here is caller-owned input vectors;
no heap/process memory budget is inferred. Logs target/collision-before-{1,2}.log
and collision-pairs-{1,2}.log. Fixed phase order/cache warmth and scheduling
variance limit comparisons; no collision throughput improvement is claimed.

```powershell
cargo test -p gridthorn_grid --release --locked measure_placement_scaling -- --ignored --nocapture
cargo test -p gridthorn_collision --release --locked measure_collision_scaling -- --ignored --nocapture
cargo test -p gridthorn_collision --release --locked measure_collision_all_pairs -- --ignored --nocapture
```

Run commands separately without overlapping builds/checks. Follow-ups: rejection-
heavy placement, negative/sparse shapes, large relocation churn, clone/allocation
attribution, memory peaks, realistic mixed/corner/containment collision workloads,
candidate pruning and cross-platform runs. Fixed simulation and snapshots/RNG
remain next domain increments. Broad milestone checks remain open.

## Fixed simulation, snapshots and RNG — 2026-10-04

Baseline6561281b0537beeea9990e3933fd7a1f2f45920d, engine initially clean after
placement/collision work was committed. Added ignored release probes under
simulation/time/test, simulation/determinism/test and app/scenario/test. No
production algorithm, RNG encoding, snapshot ownership, dependency or API change.
Each probe/config has two excluded iterations and20 retained samples; two
isolated runs passed, upper-middle median and nearest-rank p95. No acquisition
overlap with other builds/checks. Logs target/simulation-{1,2}.log,
snapshot-{1,2}.log, fixed-schedule-{1,2}.log and simulation-snapshot-summary.csv.

```powershell
cargo test -p gridthorn_simulation --release --locked measure_ -- --ignored --nocapture
cargo test -p gridthorn_app --release --locked measure_snapshot_scaling -- --ignored --nocapture
cargo test -p gridthorn_app --release --locked measure_fixed_schedule_scaling -- --ignored --nocapture
```

Clock:10000 calls per sample,10ms fixed step, catch-up limit8. Zero/normal/catch-up
input is0/10/200ms per frame; paused input10ms. Fresh clocks/controls are created
outside timing. Total assignments are asserted0/10000/80000/0 outside timing;
catch-up repeatedly accumulates backlog rather than dropping it. Timed arithmetic
includes checked duration/tick calculations, integer scaling and black-box timing
results, not execution of assigned systems. Existing clock tests cover backlog,
control/speed fractions, overflow atomicity and explicit tick indices.

Exact-tick runner:1000 ticks per sample,0/16/256 fixed systems each incrementing
one root scalar. Fresh schedules/runner and Startup precede timing. Timing includes
per-tick Input/state-transition/fixed orchestration and resource access; no game
work beyond scalar increments. Assert completed_ticks=1000 and root increment
count outside timing. This is headless run_ticks, not native catch-up-frame/GPU or
paused wall-clock acceptance; explicit ticks intentionally bypass pause controls.

| Workload | Median, run1 /run2 | p95, run1 /run2 |
| --- | --- | --- |
| 10000 normal clock calls | 0.28 /0.35 ms | 0.34 /0.42 ms |
| 10000 catch-up clock calls | 0.36 /0.36 ms | 0.39 /0.48 ms |
| 10000 paused clock calls | 0.11 /0.11 ms | 0.11 /0.12 ms |
| 1000 ticks,0 fixed systems | 1.09 /1.22 ms | 1.73 /1.78 ms |
| 1000 ticks,16 fixed systems | 1.32 /1.36 ms | 1.55 /2.20 ms |
| 1000 ticks,256 fixed systems | 4.53 /4.60 ms | 4.95 /6.88 ms |

RNG:16384 draws per sample, round-robin names stream-0000 onward,1/64/1024
registered streams, master seed42. Registration and fixture clones are excluded.
Named path includes name lookup/validation-result handling and SplitMix64; direct
path uses a vector of the identical derived stream states. XOR of values and
all final stream states must match exactly outside timing. Direct median about
0.03ms at every size; named median0.14–0.17/0.54–0.55/0.96–1.01ms respectively
(1024-stream p951.30–1.45ms). Fixed named-then-direct phase order, warm name/cache
history and modulo/index overhead limit attribution. This is lookup overhead,
not evidence to alter algorithms or bypass a game's named-stream ownership.
Existing fixed vectors pin seeds/outputs; no randomness compatibility change.

Snapshots: root Vec<u64> sizes1024/16384/262144, each initialized7,64 queued
u64 commands,1/128 registered streams including economy. Capture, snapshot.clone
and restore are timed separately, including allocations/old-root destruction
where performed by the API. Fixture initialization, comparisons and one-tick
continuation are excluded. An independent compatible runner restores the initial
snapshot and runs one tick to establish expected root/commands/RNG; every measured
iteration reproduces that state, restores the initial root and checks equality.
The game fixture appends an RNG/tick/command-derived value, so restoration replaces
a mutated vector (potentially grown capacity), not an unchanged root. Captured
snapshot remains at tick0; existing tests cover exact clock/control/exit restoration
and incompatibility rollback. No arbitrary ECS/world snapshot or serialization.

262144 values are2MiB of scalar payload excluding commands/streams/metadata.
Capture medians0.49–0.55ms, clone0.30–0.34ms, restore0.37–0.47ms across both
stream counts and runs; worst recorded p95 among these phases about0.66ms.
These are repeated warm fixture phases with multiple reference snapshots/runners
retained; no cold-cache, heap/resident peak, allocation-count or general user Clone
budget is inferred. Owning root clone is required for independent snapshots.

Disposition: no justified production fix in this increment. Clock arithmetic is
small in these fixtures; exact-tick overhead and name lookup are measurable, but
real fixed-system/root workloads are needed before changing contracts or adding
stream handles, pools or incremental snapshots. Follow-ups: native runtime catch-up
schedules, speed changes, entity-heavy simulation, queue/backlog workloads, larger
and nested roots, retained snapshot count and heap peaks, concurrent/background
snapshot policy, root destruction and name-lookup attribution, cross-platform runs.
Supported milestone limits remain provisional; broad domain gate stays open.

## Release build and executable footprint baseline — 2026-10-04

Engine baseline6561281b0537beeea9990e3933fd7a1f2f45920d with preceding uncommitted
simulation probes/docs preserved. Windows x86_64-pc-windows-msvc, rustc1.99.0
(b940084d7), Cargo1.99.0, default release profiles and job selection. No custom
profile, feature/dependency change, LTO/strip/panic policy or packaging change.
Measurement root: target/build-footprint-20261004-140430. Each timed Cargo build
uses --release --locked --offline, process wall time includes Cargo invocation,
stdout/stderr redirected to logs. Commands run sequentially without overlapping
builds/checks. CLI and SDK get separate new target directories; existing engine
and sibling targets are preserved. Source/registry caches and OS filesystem caches
are warm, so empty-target is not an uncached-download or cold-storage benchmark.

| Build | Initial wall time | Two unchanged warm repeats |
| --- | --- | --- |
| gridthorn_cli,empty target | 23.21 s | 0.38 /0.30 s |
| gridthorn default SDK,empty target | 161.54 s | 0.71 /0.65 s |
| CLI-generated minimal game,SDK dependency artifacts primed | 10.75 s | 0.60 /0.54 s |
| classic_2d,existing release-test artifact cache | 3.58 s | 0.72 /0.62 s |

This is one initial sample/workload, not a repeated cold-build distribution or
an optimization before/after comparison. SDK is a library build without optional
grid/native-output features; it still compiles app/renderer/text/world dependencies.
Generated game shares the measured SDK target but compiles project-owned SDK
crates again under its standalone workspace context; third-party artifacts are
primed. Example uses its pre-existing sibling target and enables native audio;
its first number must not be presented as cold. Warm no-op times include Cargo
fingerprint/manifest work; they do not measure an incremental source edit.

The measured CLI executable created generated-game using --engine-path pointing
at crates/gridthorn. A copied engine lockfile initially failed --locked because
it required adaptation to the standalone project. Offline cargo metadata adapted
only the temporary generated lockfile before acquisition; subsequent builds use
--locked. Comparing metadata found no generated external package/version absent
from the engine resolution. Root/sibling lockfiles were not changed.
Generated-resolve.log/generated-metadata.json retain resolved context.
This preparation is excluded from generated build time, as is project generation.

| Release artifact | Bytes | Approx MiB |
| --- | --- | --- |
| CLI gridthorn.exe | 2234880 | 2.13 |
| generated-game.exe | 8419840 | 8.03 |
| classic_2d.exe | 9138688 | 8.72 |
| facade libgridthorn.rlib | 51682 | 0.049 |

Facade rlib contains only that library's surface; it excludes subsystem/dependency
rlibs and is not total engine binary size or a runnable deliverable. Windows PDBs
are separate: measured CLI2543616 bytes, generated game4894720 bytes. Executable
sizes exclude PDBs, external game assets/fonts/audio, platform DLL/runtime needs,
installer/compression and debug/test binaries. No asset/DLL distribution audit or
runtime memory claim is inferred. Example assets are external to its executable.

Target-filtered normal-dependency trees (--target x86_64-pc-windows-msvc --edges
normal --prefix none --format '{p}') have51 distinct package/version lines for CLI
and220 for default SDK, each including the root. Counts exclude build/dev edges,
other platform-only dependencies and standard library; they are dependency breadth,
not bytes, critical-path attribution or compile-unit counts. Graph/log evidence
includes GPU/text/ECS/native-platform dependencies, but no single crate is declared
the dominant bottleneck without timing attribution. Cargo HTML timing reports
for empty-target builds are under cli/cargo-timings and sdk/cargo-timings.

Reproduce empty-target and warm acquisition in a new ignored target directory;
run the second command twice without source changes. Use another directory for
SDK so its initial build does not inherit CLI artifacts:

```powershell
cargo build -p gridthorn_cli --release --locked --offline --target-dir target/build-footprint-new-cli --timings
cargo build -p gridthorn_cli --release --locked --offline --target-dir target/build-footprint-new-cli
cargo build -p gridthorn --release --locked --offline --target-dir target/build-footprint-new-sdk --timings
cargo build -p gridthorn --release --locked --offline --target-dir target/build-footprint-new-sdk
```

Existing directories invalidate empty-target labeling; create new names rather
than deleting normal targets. Capture Stopwatch wall time externally as above.
Complete raw evidence in measurement root: times.csv, sizes.csv, cli-cold/cli-warm-
{1,2}.log, sdk-cold/sdk-warm-{1,2}.log, generated-{0,1,2}.log, example-{0,1,2}.log,
*-dependencies.txt, engine/generated-metadata.json. Classic executable size is
reported above; sizes.csv records CLI/generated/facade only. CLI --help,
generated release --smoke and classic_2d release --smoke passed; these exercise
runnable lifecycle, not runtime performance/IME/audio-quality acceptance.

Disposition: establish baseline without profile/dependency changes. Cold-from-
empty SDK compilation is materially longer than CLI, but there is no prior
controlled release baseline demonstrating regression. Follow-ups: repeated
empty-target builds, genuinely empty-target examples/generated projects, controlled
single-domain edit/rebuild costs, Cargo critical-path/unit attribution, optional
feature matrix, dependency/build-artifact bytes, full asset/runtime packaging,
profile tradeoff experiments with correctness/runtime checks and cross-platform
runs. Broad build/footprint milestone items remain open.

## Cargo units and source-edit rebuilds — 2026-10-04

Engine baseline67cd899649a0009dbdbd8a7b6ec12dec388da795, initially clean after
prior build/simulation changes were committed. Continues the preceding retained
measurement root target/build-footprint-20261004-140430. No tracked source,
manifest, dependency or build-profile change. SDK crates/manifests/config were
copied to ignored sdk-source, and generated-game's temporary dependency path was
redirected there. Offline metadata adapted only its temporary lockfile. Priming
that path fixture took13.90s; it is excluded from edit comparisons.

Parsed UNIT_DATA from the dated original SDK cold Cargo HTML report into
cold-units.csv. Use the dated report, not cargo-timing.html, which is a mutable
alias replaced by later builds. Unit elapsed durations overlap heavily; they
are not CPU time and must not be added into build wall time.

| Cold compile unit | Unit start /elapsed seconds |
| --- | --- |
| naga | 33.68 /120.29 |
| bevy_ecs | 52.24 /100.99 |
| moxcms | 34.67 /77.61 |
| wgpu-hal | 80.99 /75.97 |
| image | 52.72 /75.65 |
| read-fonts (one of two versioned units) | 19.11 /72.77 |
| wgpu-core | 95.20 /66.20 |

wgpu-core finishes near161.40s at the end of the161.54s wall build, followed in
finish order by wgpu-hal, app, naga and ECS. Metadata pipelining permits dependent
units to begin before upstream codegen ends. This identifies long units and the
observed tail, not an exact causal critical path or proof that removing one crate
saves its full duration. Scheduler contention, default parallel jobs, optimization
and linker work are not separately attributed. Feature/backend or job-limit
experiments remain follow-ups before changing dependencies or profiles.

Source edits are actual implementation-text changes, not timestamp-only touches:
generated model changes i16 subtraction to wrapping_sub for values constrained
to0/1; copied RNG changes the local state read to wrapping_add(0), preserving
SplitMix64 outputs. For each workload, one build uses the alternate source and
one restores the original. Both are timed source edits; neither is a no-op warm
repeat. Files are restored in a finally block. No-op algebra does not represent
all realistic codegen changes, public API edits or generic-heavy edits.

| Source edit | Two release build wall times | Compiling packages in logs |
| --- | --- | --- |
| Generated game model | 2.51 /2.62 s | generated-game |
| Copied SDK RNG implementation | 6.51 /6.68 s | simulation, app, facade, generated-game |

All use cargo build --manifest-path <temporary generated Cargo.toml> --release
--locked --offline --target-dir <measurement root>/sdk --timings, sequentially,
with no overlapping build/check. Source read/write is outside Stopwatch timing.
Same target/cache/profile/toolchain as preceding baseline; path priming establishes
a stable copied source context. No external/GPU/text dependencies recompile in
these edits. Release profile is unchanged; no custom incremental setting or fast-
compile profile introduced. Log evidence is edit-game_model-{1,2}.log,
edit-sdk_rng-{1,2}.log, edit-times.csv and Cargo HTML reports. Final generated
release --smoke passed after the copied source expressions were restored.

Disposition: separate no-op Cargo checks (~0.5–0.7s) from source-edit rebuilds
(~2.5s local model,~6.5s simulation edit) and genuinely empty-target compilation
(~161s SDK). The evidence supports keeping normal caches and clean domain
boundaries; it does not establish a regression or justify changing runtime,
RNG compatibility, feature defaults or release tuning. Next controlled experiments
can vary one costly backend/feature or build parallelism at a time and compare
runtime/correctness tradeoffs. Broader edit matrix (renderer/world/public API,
procedural macros, generic code), repeated cold examples/generated projects,
exact critical-path/CPU attribution, artifact/package bytes and cross-platform
measurements remain open. Reviewed changes in this increment are Markdown only;
measurement sources/logs stay under ignored target.

## Cargo job limits — 2026-10-04

Continues baseline67cd899649a0009dbdbd8a7b6ec12dec388da795 with preceding
uncommitted Markdown review changes preserved. Default/4/2-job SDK release builds
use identical unchanged sources, lockfile, profile, target/toolchain and offline
registry cache. Cargo reports ncpu=12; CARGO_BUILD_JOBS is unset and project config
sets no jobs override. Default report confirms jobs=12. Physical CPU model/cores
were not available through the permitted CIM query; no physical-core assumption.

Serial order is -j4, -j2, then fresh default, each in its own new empty target
under the retained measurement root. Source/registry/OS caches remain warm; no
other agent build/check overlaps acquisition. Stopwatch covers Cargo invocation
only; report copying, artifact enumeration and cleanup occur after stopping it.
One sample per setting: these are not confidence intervals, randomized trials or
proof of a globally optimal job count.

| Cargo setting | Empty-target release wall time | Logical artifact bytes |
| --- | --- | --- |
| -j4 | 206.89s (3m27s) | 1077449289 |
| -j2 | 347.28s (5m47s) | 1077463685 |
| default (12 jobs) | 137.68s (2m18s) | 1077456489 |

Prior independent default sample was161.54s; this variation prevents exact savings
or universal thresholds. The tested limits are slower than both default samples.
Against the fresh default, -j4 takes about1.50x and -j2 about2.52x as long.
Artifact byte totals are sums of file lengths, not physical disk use, peak disk
requirements or process memory; ~1GiB is essentially unchanged across settings.
No RAM/responsiveness/thermal/power advantage is measured or claimed.

Some individual units complete faster with lower concurrency: naga elapsed
96.08s default,64.66s at4 jobs,55.63s at2; ECS79.72/50.41/44.84s respectively.
Whole-build time still rises. This is consistent with lower unit contention but
less overlapping work; wall unit durations and one serial-order sample do not
isolate CPU contention or establish a causal critical-path model. Cargo reports
max-concurrency13/5/3 while jobs are12/4/2; use its explicit jobs value to identify
the requested policy rather than assuming every recorded overlapping unit holds
an independent job slot. No compile-job algorithm or crate backend changed.

Exact acquisition commands (new target required each time):

```powershell
cargo build -p gridthorn --release --locked --offline -j 4 --target-dir target/jobs-probe-new-4 --timings
cargo build -p gridthorn --release --locked --offline -j 2 --target-dir target/jobs-probe-new-2 --timings
cargo build -p gridthorn --release --locked --offline --target-dir target/jobs-probe-new-default --timings
```

Raw evidence: target/build-footprint-20261004-140430/jobs-times.csv, jobs-{4,2,
default}.log, self-contained jobs-{4,2,default}.html, and parsed jobs-*-units.csv.
Free space at start was about4.4GiB. After each successful build the report was
copied out and only that run's newly-created jobs target was removed, after
resolving/checking its absolute path under the measurement root. Original SDK,
CLI, examples, generated sources, normal targets and earlier evidence are retained.
Reports/logs remain reviewable after cleanup; artifact sizes were sampled before
removal. No user/source files or lockfiles were modified.

Disposition: preserve default Cargo jobs policy on this test machine; no project
or global jobs override is added. Further experiments could test intermediate
limits, repeat/reorder settings and measure memory/responsiveness on constrained
hardware before choosing per-machine limits. CI job counts, dev/check/test builds,
source-edit rebuilds under alternate jobs, full example/generated cold builds,
profile/backend feature tradeoffs and cross-platform runs remain open. This
increment changes only reviewed Markdown; builds above passed and DocsOnly
verification applies to the resulting diff.

## World, renderer and facade source-edit rebuilds — 2026-10-04

Continues engine baseline `67cd899649a0009dbdbd8a7b6ec12dec388da795` and the
isolated copied SDK/generated-game fixture under
`target/build-footprint-20261004-140430`. Prior uncommitted Markdown changes are
preserved. Same Windows target, Rust/Cargo 1.99.0, default release profile/jobs,
locked offline resolution and primed dependency cache as preceding experiments.
No tracked Rust, manifest, dependency or configuration changes.

Each case has two sequential builds: apply the alternate source, then restore
original bytes and build again. Stopwatch covers Cargo invocation with redirected
logs; source writes and final smoke are excluded. These are two source-edit
observations, not no-op repeats, confidence intervals or a performance speedup.

| Copied source edit | Alternate / restored release wall time | Packages reported compiling in both builds |
| --- | --- | --- |
| Generic `WorldAccess::spawn` body | 8.41 / 7.93 s | world, scene, app, facade, generated-game |
| `Color::rgba` body | 7.64 / 7.93 s | render, app, facade, generated-game |
| Add unused facade version function | 2.85 / 2.66 s | facade, generated-game |

World edit extracts `StoredComponent(component)` into a local before spawning.
Renderer edit extracts the identical RGBA array into a local. Both preserve
observable behavior but need not represent realistic codegen or generic changes.
The facade case adds `build_probe_version() -> &'static str`, delegating to
`version()`, then removes it. This is an additive unused API in the temporary
copy only; no production API was added. No third-party package reports compilation
in these six final builds. Simulation does not report compilation in the final
world cases; the log list describes this fixture/cache, not a promise about every
consumer or complete metadata invalidation. Logs are Cargo progress evidence,
not a count of rustc processes or rebuilt machine-code functions.

The initial acquisition rewrote restored files again in `finally`, changing
mtime and contaminating the next case with previous-domain compilation. Those
numbers were discarded. The retained script restores in `finally` only when
contents differ, while the second timed iteration explicitly restores original
bytes. A separate untimed build re-primed the fixture before final acquisition.
Final logs/CSV replace initial observations. Raw evidence: `edit-domain-probe.ps1`,
`edit-domain-reprime.log`, `edit-domain-times.csv`,
`edit-{world_generic,render_color,facade_api}-{1,2}.log` and
`edit-domain-smoke.log` in the measurement root. Final generated release
`--smoke` passed; all three copied files match tracked originals by SHA-256.

Disposition: subsystem edits cost about 7.6–8.4 seconds in this warm release
fixture, compared with about 2.7–2.9 seconds for the unused facade API edit.
This extends the earlier model/RNG matrix without justifying dependency or
profile changes. It does not establish development-profile latency, changed
public signatures, procedural macro costs, cold builds, native runtime correctness
or a cross-platform limit. True-empty examples/generated builds, repeated trials,
realistic generic/API edits and packaging remain open.

## Generated project with empty build target — 2026-10-04

Continues engine `67cd899649a0009dbdbd8a7b6ec12dec388da795`, preserving earlier
uncommitted Markdown measurements. Examples revision is
`c89adb9a5317007b3469782c1c8da9d8b4b1b04a`; no sibling files changed.
Use the existing CLI-generated `generated-game`, restored copied SDK sources and
already adapted temporary lockfile from the preceding experiments. No generation,
registry resolution, source-copy preparation or lockfile adaptation is timed.
Rust/Cargo 1.99.0, Windows x86_64-pc-windows-msvc, default release profile/features
and default jobs remain unchanged. CPU power/frequency and background load were
not controlled. Source/registry/OS caches remain warm.

A previously nonexistent target directory,
`target/build-footprint-20261004-140430/generated-empty-20261004`, isolates all
compiler artifacts from the prior SDK/CLI/example targets. The script rejects an
existing directory instead of deleting or reusing it. Acquisition runs sequentially
with no overlapping Cargo checks/builds. Stopwatch includes Cargo invocation and
redirected output; enumeration and smoke occur after timing.

| Build state | Wall seconds | Cargo `Compiling` progress lines |
| --- | --- | --- |
| Empty target | 142.41 | 227 |
| First unchanged warm repeat | 0.70 | 0 |
| Second unchanged warm repeat | 0.72 | 0 |

The initial observation includes dependencies, SDK and game compilation/linking.
The 227 lines are Cargo progress entries, not distinct package/version counts,
rustc process counts or an exact compile-unit metric. Warm repeats are no-op
fingerprint checks, not source-edit rebuilds. This is one empty-target sample,
not a cold-build distribution or uncached registry/download/storage benchmark.
The earlier generated-project 10.75-second build used primed SDK dependency
artifacts and must remain separately labeled. The 137.68/161.54-second SDK-only
samples differ in workspace context and workload; their proximity does not prove
that adding the game is free or that compilation improved.

Final executable is 8421376 bytes (about 8.03 MiB); PDB is 4911104 bytes
(about 4.68 MiB). Target logical file lengths sum to 1104095557 bytes
(about 1.03 GiB), including dependencies, reports and build intermediates.
This is neither physical disk usage nor peak RAM/disk consumption. Earlier
executable size was 8419840 bytes; this small difference is not attributed to a
code regression or profile change. Build paths/context and linking differ;
no controlled binary reproducibility experiment was performed. Assets, runtime
DLLs, installer size and deployment requirements remain excluded.

The final executable's `--smoke` passed with exit 0. Its source runs a headless
runtime frame with two fixed ticks and shutdown; it does not open a window,
exercise GPU presentation, test IME or establish native runtime acceptance.
Normal build targets and the newly measured target are retained.

Exact acquisition is retained as `generated-empty-probe.ps1` in the measurement
root; rerunning requires a different unused target directory name. Build command:

```powershell
cargo build --manifest-path target/build-footprint-20261004-140430/generated-game/Cargo.toml --release --locked --offline --target-dir target/build-footprint-new-generated --timings
```

Run the same command twice more without modifying sources for warm comparisons.
Raw evidence in the measurement root: `generated-empty-times.csv`,
`generated-empty-sizes.csv`, `generated-empty-{0,1,2}.log` and
`generated-empty-smoke.log`. Dated Cargo timing reports remain under the new
target's `cargo-timings`; the original dated report must be used for cold unit
analysis because `cargo-timing.html` is replaced on subsequent builds.

Disposition: generated-project empty-target and unchanged warm states are now
measured separately. No measured regression justifies changing dependency defaults
or release profiles. Repeated empty-target samples, genuinely empty-target
examples, dev/check profiles, realistic source/API edits, packaging and
cross-platform coverage remain open. The broader build gate and milestone native
acceptance are not closed by this increment.

## Example with empty build target — 2026-10-04

Engine revision `67cd899649a0009dbdbd8a7b6ec12dec388da795`, examples revision
`c89adb9a5317007b3469782c1c8da9d8b4b1b04a`. Measurements describe current working
trees: preceding engine Markdown changes and existing sibling manifest/lockfile,
classic audio and workbench changes are preserved. No sibling files are edited.
`classic_2d` consumes tracked engine paths and enables native audio output; the
minimal generated game uses a copied SDK without that optional output feature.
Their workspace/feature contexts differ, so elapsed differences do not isolate
the cost of native audio or game logic.

Rust/Cargo 1.99.0, Windows x86_64-pc-windows-msvc, default release profile/jobs,
locked offline resolution. Source, registry and OS caches remain warm; power,
CPU frequency and background load are uncontrolled. New target
`target/build-footprint-20261004-140430/classic-empty-20261004` was absent before
acquisition. Script rejects existing targets; normal engine/sibling/generated
build caches are retained. Cargo commands run sequentially without overlapping
checks/builds. Stopwatch includes Cargo invocation/output redirection only.

| Build state | Wall seconds | Cargo `Compiling` progress lines |
| --- | --- | --- |
| Empty target | 147.39 | 228 |
| First unchanged warm repeat | 0.75 | 0 |
| Second unchanged warm repeat | 0.67 | 0 |

This is one empty-target sample and two no-op repeats, not a repeated cold-build
distribution, source-edit workload or uncached registry/storage benchmark.
Progress lines are not unique package counts or rustc process counts. The prior
3.58-second example build used existing release-test artifacts; it remains
separately labeled and is not a comparable empty-target baseline. No speedup or
regression is inferred from these two contexts.

Release EXE is 9138688 bytes (about 8.72 MiB), matching the previously measured
example executable size. PDB is 5345280 bytes (about 5.10 MiB). Logical target
file lengths total 1182135505 bytes (about 1.10 GiB), including dependency and
build intermediates/reports. External example assets total 107609 bytes:
`atlas.ppm` 10501, `music.wav` 88244 and `pickup.wav` 8864. These are file lengths,
not physical storage, peak memory/disk or compressed distribution sizes. Assets
resolve through the source package path; this is not a relocatable packaging test
or an audit of runtime DLLs/device requirements.

The new executable passed both `--headless-smoke` and `--smoke`, exit 0.
Headless exercises asset decoding/game extraction without a device/window.
Native mode opens a window, starts a round and exits after 30 frames with the
example audio worker enabled. An empty stderr/stdout log and successful exit
prove lifecycle completion, not audible output quality, native device/mixer
latency, frame budgets, IME, manual interaction or suspend/resume acceptance.
No native renderer timing was enabled for this build measurement.

Reproduction requires an unused target name:

```powershell
cargo build --manifest-path ../gridthorn-examples/Cargo.toml -p classic_2d --release --locked --offline --target-dir target/build-footprint-new-classic --timings
```

Repeat the unchanged command twice for no-op warm measurements. Full acquisition
script/log evidence remains under `target/build-footprint-20261004-140430`:
`classic-empty-probe.ps1`, `classic-empty-times.csv`, `classic-empty-sizes.csv`,
`classic-empty-{0,1,2}.log`, `classic-empty-headless.log` and
`classic-empty-native.log`. Dated Cargo reports remain in the new target's
`cargo-timings`; the mutable alias does not preserve initial unit timings.

Disposition: an existing runnable example now has independent empty-target and
unchanged warm observations, complementing CLI/SDK/generated-project baselines.
No production dependency/profile change is justified. Broader examples/features,
repeated empty targets, debug/check builds, realistic edits, full packaging and
cross-platform trials remain open. The broad build gate and whole milestone
native acceptance remain unfinished.

## Direct DLL imports and staged package footprint — 2026-10-04

Engine revision `67cd899649a0009dbdbd8a7b6ec12dec388da795`; existing engine
Markdown and sibling example changes are preserved. Use the previously measured
CLI, generated empty-target and classic_2d empty-target release executables.
No build, dependency, profile, source or sibling file changes in this increment.
Microsoft COFF/PE Dumper 14.44.35228.0 (`dumpbin /DEPENDENTS` and `/IMPORTS`)
reads each image; complete outputs are retained. DLL names are normalized to
lowercase and deduplicated, since images include differently cased names.

| Artifact | Unique direct DLL names | EXE bytes | Staged EXE plus assets bytes |
| --- | --- | --- | --- |
| CLI | 11 | 2234880 | 2234880 |
| Generated game | 23 | 8421376 | 8421376 |
| classic_2d | 26 | 9138688 | 9246297 |

All three images import `vcruntime140.dll` and six `api-ms-win-crt-*` names
(math, runtime, string, stdio, locale, heap). Counts are import names in these
images, not total recursively loaded libraries, distributable files or measured
DLL memory. No DLL installation/copied runtime was performed. Local smoke
success only establishes that the exercised paths work on this development host.
A clean Windows installation/runtime prerequisite check remains unperformed.

Generated/classic additionally import window/graphics-related names such as
`user32.dll`, `gdi32.dll`, `opengl32.dll` and `dxgi.dll`. Relative to generated,
classic has three additional names: `combase.dll`, `mmdevapi.dll` and
`api-ms-win-core-winrt-error-l1-1-0.dll`. The native audio-enabled fixture differs
in source, features and workspace context; this comparison is an import-table
inventory, not isolated audio binary-size or latency attribution. Driver/runtime
modules loaded dynamically, transitive dependencies, API-set resolution and
unused linked paths are outside this audit. Static imports do not establish
which backend executes or which components belong in an installer.

A new `package-stage-20261004` under the measurement root contains each EXE in
its own directory, plus the example's three assets in `classic/assets`. No PDB,
runtime DLL, installer or compression output is included. File-length totals
exclude folder metadata and are not physical disk size, resident memory or a
complete deployment bill of materials. The script rejects an existing stage.
CLI `--help`, generated `--smoke` and classic `--headless-smoke` all passed from
the respective staged working directories, exit 0. No native staged run was
needed to repeat the previously completed native lifecycle smoke.

A concrete portability limit remains: classic loads its atlas and WAV files
through compile-time `CARGO_MANIFEST_DIR`, not the staged assets directory.
Source inspection shows both load sites, and byte inspection confirms the
absolute sibling package path embedded in the EXE. Since that source directory
remains available, successful staged smoke cannot prove that staged assets were
used. No original files were renamed/removed to force failure. The example's
source-release workflow remains runnable; a relocatable distribution requires
an explicit asset-location policy and validation on a machine without the source
checkout. This audit does not silently implement a packaging capability.

Raw evidence under `target/build-footprint-20261004-140430`:
`package-probe.ps1`, `package-{cli,generated,classic}-{dependents,imports,smoke}.log`,
`package-direct-imports.csv`, `package-classic-extra-imports.csv`,
`package-embedded-path.csv`, `package-stage-sizes.csv` and staged copies.
Reproduction uses the same `/DEPENDENTS` and `/IMPORTS` commands on the desired
release artifact; a new unused stage directory is required for the script.

Disposition: direct import names and executable/asset staging bytes are measured;
full runtime/deployment footprint remains open. No performance regression or
profile/dependency fix is inferred. Follow-ups are recursive/dynamic module and
clean-host coverage, explicit runtime prerequisites, relocatable asset resolution,
installer/compression tradeoffs and platform-specific packaging. The broad build
footprint and native milestone gates remain open.

## Example asset relocation fix — 2026-10-04

The preceding staged audit found classic_2d loading assets only through its
compile-time source path. The sibling example now resolves one asset directory
for both presentation and audio: an existing `assets` directory beside the EXE
has priority, otherwise source-package assets remain the development fallback.
An incomplete adjacent directory fails at the normal texture/audio load boundary;
there is no per-file fallback that mixes package and source assets. No SDK API,
dependency edge, profile or automatic packaging subsystem was added.

The focused resolver lives in `classic_2d/src/assets/resolution.rs`, with a small
module surface and domain tests. Two tests verify development fallback and that
an empty adjacent asset directory overrides a valid source atlas and produces a
load error. Package tests passed: 10 passed, 1 ignored; package all-target Clippy
with warnings denied passed. Existing audio pause-mailbox changes are preserved.

A separate standalone copy of the current example under ignored engine target
was compiled in release against tracked engine paths with native audio enabled.
Only its temporary manifest/lockfile were adapted; sibling manifests/lockfile
were unchanged. Offline metadata initially attempted to unpack a non-Windows
package into the restricted Cargo registry. Filtering metadata to
x86_64-pc-windows-msvc avoided that unused-platform acquisition and succeeded.
Release build used --locked --offline and the retained classic target; no
empty-target build timing is claimed for this reused cache.

EXE and assets were copied into `classic-relocation-package`. The copied source
directory, whose path is embedded in this EXE, was temporarily moved to a checked
sibling path under the ignored measurement root. While the compile-time source
path was absent, both headless and native 30-frame smokes passed, exit 0. Removing
the packaged atlas and pickup WAV in separate checks caused expected exit 1;
assets and source fixture were restored in finally blocks. Only these newly
created ignored fixtures were moved, never user source assets. This demonstrates
asset relocation on the current development host; native sound quality, clean
Windows runtime prerequisites and displayed-frame budgets remain unverified.

Evidence under `target/build-footprint-20261004-140430`:
`classic-relocation-probe.ps1`, `classic-relocation-metadata.json`,
`classic-relocation-build.log`, `classic-relocation-{headless-smoke,smoke}.log`,
`classic-relocation-missing-{atlas.ppm,pickup.wav}.log`, copied source/package.
Engine target logs `classic-assets-tests.log` and `classic-assets-clippy.log`
record package checks. The reused classic release target's executable was rebuilt;
original measured executable bytes remain in `package-stage-20261004/classic`.
Historical build/size observations still describe their earlier artifacts.

Disposition: the measured source-path asset portability limit is fixed and
validated for this example. This is a reliability/package behavior change, not a
measured rendering or compilation speedup. Direct/dynamic DLL deployment,
clean-host installation, full packaging footprint and native milestone acceptance
remain open. No total deployment capability is marked complete.

Full `./scripts/verify.ps1` passed with process-local `CARGO_TARGET_DIR` set to
engine target and `RUST_TEST_THREADS=1`; previous environment values were restored.
The first default run passed format/check/Clippy but failed the generated CLI
project build with disk-full errors in its separate temporary target. Full logs
are `target/classic-assets-full-verify.log` and
`target/classic-assets-full-verify-shared.log`; the successful rerun includes all
workspace tests, generated-project workflows and dependency boundaries.

To provide build space, the two newly created empty-target measurement caches
were removed only after validating their absolute paths under the measurement
root and copying timing reports/EXE/PDB into
`generated-empty-20261004-evidence` and `classic-empty-20261004-evidence`.
Earlier CSV/logs and original staged artifacts remain; normal targets, user source
and sibling assets were preserved. Historical statements that the temporary
build targets were retained describe the state at acquisition; archived evidence
is now the location for their reports/binaries. Reproduction still needs unused
fresh targets. Final documentation and sibling diff whitespace checks passed.

## Isolated native slider drag — 2026-10-04

Engine revision remains `67cd899649a0009dbdbd8a7b6ec12dec388da795`; sibling revision
`c89adb9a5317007b3469782c1c8da9d8b4b1b04a` with existing workbench/manifest/audio
and preceding asset-resolution working-tree changes preserved. Adds mutually
exclusive `--slider-smoke` to the public-API workbench. Ten host frames warm the
unchanged UI. Frame 11 moves to slider value 80 and presses left; subsequent
frames alternate 20/80 while holding capture, and frame 120 moves/releases.
Physical coordinates derive from current layout content and actual window DPI.
Normal Changed effects refresh localization and prepare updated layout. No SDK
API, dependency or normal interactive behavior changed.

A domain test covers DPI 1/2 logical-equivalent viewports, warmup, Changed effects,
values, localized refresh and capture release (later pointer motion does not
change the value). Package tests: 8 passed, 1 ignored. Package all-target Clippy
passed after changing an empty assertion to the required equality form. Release
build passed. Conflicting slider/idle flags return nonzero before opening a window.
DPI 2 here is injected headless validation, not a native monitor measurement.

Two sequential native release runs per locale, 120 host input frames and 119
rendered/GPU samples each. All eight report physical 1000x800, scale factor 1,
RTX 3070/Vulkan/NVIDIA 616.56/Fifo with timestamp queries. No GPU readback errors,
skips or pending samples. Refresh, power/frequency and display intervals remain
uncontrolled/unmeasured. Flags GRIDTHORN_WORKBENCH_PERFORMANCE,
GRIDTHORN_RENDER_PERFORMANCE and GRIDTHORN_WINDOW_PERFORMANCE were enabled only
for runs and restored afterward; avoid manual input during acquisition.

| Locale | Input phase sum p95 ms, first/repeat | Encode p95 ms, first/repeat | GPU pass p95 us, first/repeat | Host cadence p99 ms, first/repeat |
| --- | --- | --- | --- | --- |
| en-US | 0.631 / 0.646 | 1.385 / 1.346 | 32 / 32 | 11.029 / 11.455 |
| ru | 0.614 / 0.614 | 1.372 / 1.286 | 33 / 33 | 10.842 / 11.446 |
| ar-EG | 0.485 / 0.667 | 0.918 / 1.036 | 26 / 26 | 9.769 / 11.077 |
| ja | 0.640 / 0.634 | 1.197 / 1.126 | 30 / 30 | 11.262 / 10.620 |

Nearest-rank distributions each contain 109 samples. Workbench input frames
11-119 select active drag and exclude final shutdown/release; renderer/GPU
indices 10-118 and window callback phase indices 10-118 are separate warm ranges.
These indices do not establish one-to-one callback pairing. Input phase sum is
prepare-before + real-input-route + scripted-workload + effects + prepare-after +
anchor per sample; snapshot/layout timings are nested and excluded from the sum.
It is input-system timing, not whole-engine CPU work. Repeated values reuse warm
shaping/cache histories; this does not model arbitrary new captions or slider
ranges. Existing UI effect stdout occurs outside input phase timers and can affect
callback/cadence observations. Native redraw callback p95 ranges 8.919-9.177 ms,
including acquire/present calls; it is not pure engine CPU execution. Do not add
percentiles of separate phases/callbacks into an invented frame distribution.
GPU pass excludes upload/queue/display work; host cadence is a CPU present-call
proxy, not actual displayed intervals.

Raw logs `target/workbench-slider-<en-US|ru|ar-EG|ja>-<1|2>.log`,
`target/workbench-slider-summary.py`/CSV/log retain median/p95/p99/max and counts
for each phase. Tests/Clippy/build and conflicting-flags logs use
`target/workbench-slider-*`. Reproduce each locale twice with the three diagnostic
variables and the release example `--slider-smoke --locale=<locale>`; the example
README records its public command. No allocations/heap peaks, OS pointer latency
or native DPI 2 are measured.

Disposition: bounded injected slider dragging on the recorded native DPI 1 host
shows no new measured bottleneck requiring a production fix. Remaining pointer,
scrolling, locale-switch, nested-window/animation isolation, clipboard/IME, native
DPI 2 and displayed-frame/whole-GPU coverage remain open. This workload and its
headless DPI test do not close the broader native interaction or acceptance gate.

Full ./scripts/verify.ps1 passed (target/workbench-slider-full-verify.log), using
process-local shared engine target and sequential tests; prior env restored.
Sibling formatting and diff whitespace passed. No staging/commits.

## Isolated native locale switches — 2026-10-04

Continues engine revision67cd899649a0009dbdbd8a7b6ec12dec388da795 and sibling
c89adb9a5317007b3469782c1c8da9d8b4b1b04a working trees, preserving prior changes.
Workbench adds mutually exclusive --locale-smoke. Initial locale is chosen by
--locale, ten host frames leave it unchanged, then each frame selects the next
of en-US/ru/ar-EG/ja through the public tree command. A Changed effect enters the
normal localized-caption/preview refresh and prepared-layout invalidation path.
Editor text, worker count, windows and animations are not scripted to change.
This measures catalog/UI switching separately from pointer/list navigation.

Focused domain test validates warmup, four successive catalog selections,
caption changes/return to English and unchanged editor contents at injected DPI2.
Package tests9passed/1ignored, package all-target Clippy and release build passed.
Conflicting locale/idle smoke flags reject before window creation. No SDK API,
dependency, release profile or normal interactive behavior changed.

Two sequential native release runs starting en-US, cycling all four captions.
Both record physical1000x800/native scale1, RTX3070/Vulkan/NVIDIA616.56/Fifo,
timestamps enabled. Rendered119 frames per run. GPU samples119/118; second run
skips one frame before the selected warm range, with no errors/pending samples.
All selected warm ranges contain105 samples including105 GPU samples each.
Diagnostic env values were enabled only for acquisition and restored afterward.
Display refresh/power/frequency/background activity remain uncontrolled or unknown.

| Phase | First median/p95 ms | Repeat median/p95 ms |
| --- | --- | --- |
| Input phase sum | 0.877 / 1.417 | 0.866 / 1.385 |
| Localized refresh effects | 0.042 / 0.054 | 0.044 / 0.056 |
| Prepare after effects | 0.796 / 1.337 | 0.782 / 1.308 |
| Renderer encode | 0.980 / 1.244 | 0.987 / 1.269 |
| GPU render pass | 0.031 / 0.033 | 0.031 / 0.032 |

Nearest-rank input distributions use host frames15-119, excluding ten idle frames,
first full locale cycle11-14 and final exit frame120. Renderer/GPU indices14-118
and independent window callback indices14-118 exclude startup/first-cycle work
conservatively; no exact host/render/callback pairing is claimed. Input sum uses
prepare-before/input-route/workload/effects/prepare-after/anchor, without nested
snapshot/layout duplication. First-cycle prepare-after observations, first/repeat:
Russian3.425/3.356ms, Arabic4.065/3.951ms, Japanese2.497/3.151ms,
return-English0.786/0.876ms. These are individual first-switch observations in a
font/cache-warmed process, not cold-process latency distributions.

Warm host present-call cadence p99 is11.206/10.860ms, worst11.379/10.914ms.
Input sum excludes animation, render extraction, platform callbacks and existing
UI-effect stdout. Renderer encode excludes acquire/present waits and GPU work;
GPU pass excludes uploads/queue/display. Cadence remains a CPU proxy. Do not add
separate phase percentiles to claim whole-engine CPU/GPU or displayed-frame
acceptance. The mixed locale cycle has27/26/26/26 input observations per selected
catalog and is reported as one deterministic cycle distribution, not four
independent locale benchmarks. Native DPI2/OS list input remains unmeasured.

Raw evidence target/workbench-locale-en-US-{1,2}.log,
workbench-locale-summary.py/CSV/log and workbench-locale-{tests,clippy,build,
invalid-flags}.log. CSV retains median/p95/p99/max/counts per phase. Reproduce
with the README's --locale-smoke command twice, enabling workbench/render/window
diagnostics; keep startup/first-cycle exclusion explicit. No allocation or peak
memory claim. No new production bottleneck fix justified by these measurements.
Remaining isolated scrolling/windows/animation, native DPI2, clipboard/IME,
whole-GPU/display intervals and final acceptance remain open.

Full ./scripts/verify.ps1 passed (target/workbench-locale-full-verify.log) with
process-local shared target and sequential tests, previous env restored. Sibling
formatting/whitespace and final documentation whitespace passed.

## Isolated native wheel scrolling — 2026-10-04

Continues the recorded engine/examples working trees with existing changes
preserved. Workbench adds mutually exclusive --scroll-smoke. This mode alone
opens a1000x400 window, leaving the normal/other smoke1000x800 configuration
unchanged. The short viewport guarantees overflow in the existing base panel.
After ten unchanged host frames, scripted physical cursor coordinates and wheel
pixel deltas route through public APIs. Deltas are96 logical pixels down on odd
frames and up on even frames, scaled to the actual window DPI. Existing layout
invalidations prepare scrolled/clipped paint. A nonoverflowing viewport returns a
contextual workload error rather than silently benchmarking unchanged UI.
No SDK API, dependency, asset or release-profile change.

Domain tests verify actual bounded scroll offsets and return to origin at DPI1/2,
consumed wheel events, layout invalidation, unchanged editor/language controls and
nonoverflow rejection. Cursor motion over a noninteractive label can still be
forwarded; the test asserts wheel consumption rather than blanket pointer-event
blocking. Package11passed/1ignored, all-target Clippy and release build passed.
Conflicting scroll/idle flags reject before opening a window.

Two sequential native release runs per locale, each120 host input frames and119
renderer samples. Physical1000x400, native scale1, RTX3070/Vulkan/NVIDIA616.56/Fifo,
timestamp queries. GPU samples118 in ru-first/ar-EG-first/ja-repeat, with one
skipped frame before the selected warm range; other runs119. No errors or pending
samples. Every selected warm CPU/GPU distribution contains109 samples. Workbench,
renderer and window diagnostic env values were restored after acquisition.
Display refresh/power/frequency/background conditions remain uncontrolled/unknown.

| Locale | Input sum p95 ms, first/repeat | Encode p95 ms, first/repeat | GPU pass p95 us, first/repeat | Host cadence p99 ms, first/repeat |
| --- | --- | --- | --- | --- |
| en-US | 0.859 / 1.052 | 0.457 / 0.695 | 13 / 13 | 11.204 / 9.584 |
| ru | 1.471 / 1.048 | 0.820 / 0.651 | 12 / 12 | 11.107 / 9.601 |
| ar-EG | 0.734 / 0.643 | 0.450 / 0.419 | 12 / 12 | 9.560 / 9.778 |
| ja | 1.005 / 1.015 | 0.460 / 0.529 | 12 / 13 | 9.725 / 9.568 |

Nearest-rank input distributions select host frames11-119; renderer/GPU and window
callback indices10-118 are independently selected warm ranges, not proven exact
callback pairing. Input sum includes prepare-before/input-route/workload/effects/
prepare-after/anchor and excludes nested snapshot/layout columns. It excludes
animation, render extraction and event-loop/platform work. Encoder/GPU-pass and
CPU present-call cadence retain the exclusions documented in earlier workloads.
Do not combine phase percentiles or infer whole-frame CPU/GPU/display budgets.
The smaller viewport and changing visible text/vertex volume prevent attributing
lower encoder/pass values relative to other workloads to an optimization.

Raw target/workbench-scroll-<en-US|ru|ar-EG|ja>-<1|2>.log and
workbench-scroll-summary.py/CSV/log retain phase median/p95/p99/max/counts. Tests,
Clippy, release build and flag rejection logs use workbench-scroll-* names.
Reproduce each locale twice with the README's --scroll-smoke command and
workbench/render/window diagnostics. These are injected wheel events on native
DPI1, not OS wheel-input latency, native DPI2 or measured allocation/heap peaks.

Disposition: this bounded short-viewport scroll fixture shows no new bottleneck
requiring a production fix. The repeatable scenario adds native scroll coverage;
1000x800 acceptance, isolated windows/animation, selection/clipboard/IME, native
DPI2, actual displayed intervals and whole-GPU work remain open.

Full ./scripts/verify.ps1 passed (target/workbench-scroll-full-verify.log) with
process-local shared engine target/sequential tests; env restored. Sibling
formatting/diff whitespace and final documentation whitespace passed.

## Isolated window actions and host-time transitions — 2026-10-04

Continues recorded engine/examples working trees, preserving earlier changes.
Workbench adds --windows-smoke and --animation-smoke, mutually exclusive with
other native smoke modes. They share the same fixed120-host-frame action script:
review open11, confirmation open26/close41, popup open56/close71, review close86,
review reopen101/close116. Initial locale/editor/count remain unchanged. On frame1
only the example animation toggle is configured: false/true respectively.
Activated effects use the normal handler, preserving existing layer focus and
platform-request handling; they are not injected OS clicks. A focused domain test
checks identical layer sequences, editor/locale preservation and initial transition
offset versus static offset at injected DPI2. Package12passed/1ignored, all-target
Clippy/release build passed; conflicting windows/animation flags reject before UI.
No SDK/dependency/profile or normal interactive behavior change.

Two sequential native release runs per mode/locale (16 runs), physical1000x800,
scale1, RTX3070/Vulkan/NVIDIA616.56/Fifo, timestamp queries. All complete119
renderer samples. One pre-warm GPU sample is skipped in windows en-US-repeat,
ru-first, ar-EG-first, ja-repeat and animation en-US-repeat/ja-first; other runs
collect119. No GPU errors/pending. All selected warm CPU/GPU ranges contain109
samples. Workbench/render/window diagnostic env restored after acquisition.
Display refresh, power/frequency/background conditions remain uncontrolled/unknown.

| Mode / locale | Native preparation p95 ms, first/repeat | Encode p95 ms, first/repeat | GPU pass p95 us, first/repeat | Host cadence p99 ms, first/repeat |
| --- | --- | --- | --- | --- |
| Windows / en-US | 0.702 / 0.674 | 1.503 / 1.488 | 44 / 44 | 11.266 / 11.211 |
| Windows / ru | 0.833 / 0.731 | 1.572 / 1.588 | 47 / 47 | 11.297 / 11.095 |
| Windows / ar-EG | 0.610 / 0.600 | 1.292 / 1.150 | 36 / 36 | 11.229 / 11.144 |
| Windows / ja | 0.735 / 0.630 | 1.489 / 1.353 | 41 / 41 | 11.226 / 11.177 |
| Animation / en-US | 0.932 / 0.997 | 1.697 / 1.595 | 44 / 44 | 10.478 / 10.893 |
| Animation / ru | 1.088 / 1.062 | 1.768 / 1.768 | 47 / 47 | 10.291 / 9.707 |
| Animation / ar-EG | 0.835 / 0.883 | 1.309 / 1.257 | 36 / 36 | 9.752 / 9.841 |
| Animation / ja | 1.073 / 1.091 | 1.445 / 1.482 | 41 / 43 | 11.248 / 9.683 |

Nearest-rank preparation/renderer/GPU distributions independently select phase
indices10-118; input diagnostics use host frames11-119. Preparation callback
includes runtime frame scheduling/animation work but excludes native event-loop
waiting and redraw/presentation. It remains a partial CPU measure, not complete
engine/event-loop execution. Input-phase sum p95 ranges0.540-0.749ms static and
0.399-0.530ms animated; animation runs in the subsequent Update stage and is
excluded from those input timers. Lower input percentile with animation is not
an optimization: different dirty-layout histories and mixed frame distributions
prevent attributing it to a speedup. Do not subtract mode percentiles to claim
isolated transition cost or add separate phase percentiles into a frame budget.

The normal transitions use elapsed host time with350ms duration while actions
use fixed frame numbers. Transitions can be superseded by the next open, and
closing a layer follows the current implementation's animation lifecycle. This
is mixed active-transition/unchanged-frame coverage, not109 guaranteed active
animation frames, completed350ms transitions for every layer or a controlled
animation-time before/after experiment. The script includes8 action frames in
the selected109-input range. Static mode contains unchanged open/closed frames.
GPU-pass exclusions and CPU cadence proxy limitations remain unchanged; actual
presented intervals, full GPU execution and native DPI2 remain unmeasured.

Raw target/workbench-<windows|animation>-<en-US|ru|ar-EG|ja>-<1|2>.log and
workbench-<windows|animation>-summary.py/CSV/log retain phase median/p95/p99/max
and counts. Package checks use workbench-windows-{tests,clippy,build}.log; flag
rejection uses workbench-windows-invalid-flags.log. Reproduce both README modes
twice per locale with workbench/render/window diagnostics, avoiding manual input.
No allocation/heap peak, native mouse latency or visual acceptance is inferred.

Disposition: these bounded window/transition cycles show no newly measured
production bottleneck requiring a fix. Native DPI1 isolation now covers idle,
injected editing, slider, catalog switches, short-viewport scrolling and fixed
window/animation scripts. Remaining native selection/clipboard/IME, DPI2,
whole-frame CPU/GPU/display intervals and maintainer acceptance remain open.

Full ./scripts/verify.ps1 passed (target/workbench-windows-full-verify.log), with
process-local shared engine target/sequential tests and previous env restored.
Sibling formatting/diff whitespace and final documentation whitespace passed.

## Native selection and injected preedit isolation — 2026-10-04

Continues the recorded working trees, preserving existing engine/sibling work.
Workbench adds mutually exclusive --selection-smoke and --preedit-smoke with
shared editor-focus preparation extracted from existing editing mode. During ten
warm frames the public navigation API focuses the editor. Selection then
alternates caret-at-zero/full-value selection; preedit injects bounded Japanese
`にほん` and Arabic combining-text compositions with byte-end cursor positions.
Frame120 cancels composition. Existing input/layout/paint/native-anchor handling
runs; committed text and localization parameters remain unchanged. No SDK API,
dependency, release profile or normal interactive behavior change.

Domain test covers DPI1/2 focus, expected selection/preedit, cancellation, dirty
layout preparation and unchanged committed controls. Existing editing replacement
test still passes after focus extraction. Package13passed/1ignored, package
all-target Clippy/release build passed. Conflicting selection/preedit flags reject
before window creation. These tests do not validate OS keyboard/IME/clipboard.

Two native release runs per mode/locale (16), physical1000x800/native scale1,
RTX3070/Vulkan/NVIDIA616.56/Fifo, timestamps. All119renderer samples. One pre-warm
GPU sample skipped in both selection en-US runs, selection ja-repeat and both
preedit en-US runs; all other runs119GPU. No errors/pending samples. Selected
warm distributions each109samples, including109GPU. Workbench/render/window
diagnostic environment values were restored after runs. Display refresh,
power/frequency/background conditions remain uncontrolled or unrecorded.

| Mode / locale | Input sum p95 ms, first/repeat | Encode p95 ms, first/repeat | GPU pass p95 us, first/repeat | Host cadence p99 ms, first/repeat |
| --- | --- | --- | --- | --- |
| Selection / en-US | 0.499 / 0.752 | 1.131 / 1.808 | 32 / 32 | 11.050 / 11.710 |
| Selection / ru | 0.637 / 0.473 | 1.413 / 1.125 | 33 / 33 | 11.187 / 9.593 |
| Selection / ar-EG | 0.516 / 0.467 | 1.119 / 0.943 | 26 / 26 | 9.560 / 10.942 |
| Selection / ja | 0.555 / 0.522 | 1.056 / 1.009 | 30 / 30 | 9.885 / 9.573 |
| Preedit / en-US | 0.478 / 0.572 | 1.164 / 1.153 | 32 / 33 | 9.759 / 10.984 |
| Preedit / ru | 0.548 / 0.491 | 1.141 / 1.111 | 33 / 33 | 9.916 / 9.557 |
| Preedit / ar-EG | 0.399 / 0.551 | 0.897 / 1.016 | 27 / 27 | 9.708 / 11.054 |
| Preedit / ja | 0.628 / 0.507 | 1.212 / 1.109 | 31 / 31 | 9.793 / 9.702 |

Nearest-rank input samples use host11-119, excluding warmup and final cancel/exit.
Renderer/GPU and window callbacks independently use indices10-118; no exact
host/callback pairing is claimed. Input phase sum is prepare-before/input-route/
workload/effects/prepare-after/anchor, without nested snapshot/layout duplication.
The existing bounded editor text and two repeated preedit strings exercise warm
cache histories, not arbitrary long compositions or grapheme-walk/key throughput.
No normal Changed effect is generated because committed text does not change;
localized-preview refresh is therefore not part of these modes.

Native submission and text-anchor requests do not establish actual Windows IME
candidate placement, keyboard layouts, composition commit correctness or system
clipboard performance. GPU-pass/encode/cadence exclusions are unchanged: uploads,
queue/display work and complete event-loop CPU are not covered by these separate
numbers. No whole-frame budget, allocation/heap peak or optimization speedup is
inferred; en-US selection repeat variance remains in raw distributions.

Raw target/workbench-<selection|preedit>-<en-US|ru|ar-EG|ja>-<1|2>.log and
workbench-<selection|preedit>-summary.py/CSV/log retain phase median/p95/p99/max
and counts. Package checks use workbench-editor-{tests,clippy,build}.log; conflicting
flags use workbench-editor-invalid-flags.log. Reproduce twice per locale with
README commands and workbench/render/window diagnostics, avoiding manual input.

Disposition: injected native selection/composition coverage has no newly measured
production bottleneck requiring a fix. Actual OS clipboard/IME, nativeDPI2,
whole-engine CPU/full GPU/display timing and maintainer acceptance remain open.

Full ./scripts/verify.ps1 passed (target/workbench-editor-full-verify.log), with
process-local shared engine target/sequential tests and previous env restored.
Sibling formatting/diff whitespace and final documentation whitespace passed.

## Reference display metadata increment — 2026-10-04

Engine window diagnostics now emit one `window_configuration` snapshot when a
native window is created and GRIDTHORN_WINDOW_PERFORMANCE is present. Private
window/performance/display.rs reads the existing winit window's current monitor:
window physical extent/scale, optional monitor name/extent/origin and optional
system refresh rate in millihertz. No new dependency, public monitor API, monitor
selection or display setting change. Unavailable values remain None and zero
refresh is normalized to None; Some values are backend reports, not invented
fallbacks. Logging happens during initialization outside callback sample timing.
No monitor queries occur in disabled runs. Each snapshot describes that creation,
not continuous monitor/refresh tracking after a move or configuration change.

Two native release Japanese idle smokes, physical1000x800, scale1, reported
monitor `\\.\DISPLAY2`, extent1920x1080, origin[0,0], refresh144000millihertz
(144Hz system report). RTX3070/Vulkan/NVIDIA616.56/Fifo renderer configuration
remains recorded separately. Both powercfg observations before/after acquisition
report Balanced GUID381b4222-f694-41f0-9685-ff5bb260df2e. This establishes metadata
for these new runs only: earlier absent monitor/refresh/power observations cannot
be retroactively filled. Power-plan selection does not measure CPU/GPU frequency,
thermal state, throttling or resource contention during sampling. Backend nominal
refresh does not measure VRR/compositor/displayed frame intervals or effective FPS.
Native DPI2 remains unmeasured; no Windows scaling/resolution was changed.

Window performance domain tests2passed; package all-target Clippy and release
workbench build passed. Enabled smokes verify exactly one configuration snapshot
per run. A separate native idle run with diagnostic environment entries removed
passes and emits neither window_configuration nor window_cpu. The first disabled
check still inherited present empty diagnostic entries; explicit removal verified
the existing presence-based flag semantics. No production gating fix was needed.
No new behavior test substitutes for the actual native creation path.

Evidence target/window-display-{tests,clippy,build}.log,
window-display-native-{1,2}.log, window-display-disabled-native.log and
window-display-power-{before,after}.log. Reproduce by setting window/render/
workbench diagnostic flags, running release --idle-smoke --locale=ja twice and
recording powercfg /getactivescheme before/after; remove environment entries for
the disabled check. Monitor fields use Rust debug Option formatting (Some/None),
not a stable public CSV/serialization API. Quotes/backslashes in names are escaped.

Disposition: native reference display information can now accompany future
performance acquisitions instead of remaining implicitly unknown. Complete
reference hardware/power conditions, nativeDPI2, actual displayed intervals,
whole-engine CPU/full GPU coverage, OS clipboard/IME and final acceptance are
still outstanding. This metadata collection is measurement tooling, not a runtime
performance optimization or the Milestone5 device configuration subsystem.

Full ./scripts/verify.ps1 passed (target/window-display-full-verify.log), with
process-local shared engine target/sequential tests and previous env restored.
Final documentation/code whitespace passed; no sibling source changes in this increment.

## Paired native callback CPU instrumentation — 2026-10-04

Opt-in `GRIDTHORN_WINDOW_PERFORMANCE` now emits bounded
`window_frame_cpu,sample,preparations,elapsed_us` rows. Each row sums completed
preparation callbacks since the preceding redraw and the next completed redraw.
Multiple preparations are accumulated explicitly; redraws without preparation
and unfinished preparation at shutdown produce no row. Retention stops at 240
rows. Existing independent phase diagnostics remain available.

This is paired callback CPU elapsed time, including renderer blocking in redraw,
not whole-engine active CPU or actual displayed-frame intervals. Native event
translation, device callbacks, event-loop waiting, compositor and display work
remain outside this sum. Extraction is already inside preparation and must not
be added again. Pairing does not prove a successful surface presentation.

Focused tests cover exact accumulation, skipped redraws, unfinished preparation
and bounded retention. No frame budget acceptance or optimization claim follows
from adding this diagnostic.

Two Japanese release idle-smoke runs exited successfully, each with119 paired
rows, all preparation counts1. Excluding first10 rows leaves109: nearest-rank
p50=6023/6174us, p95=9612/9424us, p99=9825/11028us, max=10983/11038us.
Evidence: target/window-paired-native-{1,2}.log and window-paired-build.log.
These repeat samples do not establish the full interaction/DPI/display gate.
The initial full verify failed in generated-project CLI test with temporary-target
filesystem errors; repeat uses process-local shared engine target as in earlier
checkpoints. Formatting, workspace check and Clippy passed in the initial run.

Full ./scripts/verify.ps1 passed using process-local shared CARGO_TARGET_DIR and
sequential tests (target/window-paired-full-verify-shared.log), including the
previously failing generated-project CLI workflow and dependency boundaries.

## Paired callback native interaction matrix — 2026-10-04

Measured the existing release workbench sequentially in nine isolation modes,
four starting locales (en-US/ru/ar-EG/ja), two repeats each:72 successful runs.
Engine HEAD67cd899649a0009dbdbd8a7b6ec12dec388da795 and examples
HEADc89adb9a5317007b3469782c1c8da9d8b4b1b04a with the existing uncommitted
changes, including paired callback instrumentation. No new source/dependency change.
Window diagnostics and renderer CPU/GPU-pass diagnostics enabled; other diagnostic
flags were not set by this command. NVIDIA RTX3070/Vulkan, driver616.56, Fifo,
DISPLAY2 at1920x1080/system144000mHz, native scale1. Windows power scheme observed
Balanced during the matrix and after completion. Live clocks/thermal/background
load were not controlled. Viewport1000x800 except scrolling1000x400.

Every run contains119 paired callback rows with exactly one preparation per row,
and119 renderer rows. Exclude pair indices0-9 (109 warm rows/run), except locale
switching excludes0-13 (105 rows) to omit the initial switch cycle. Percentiles
use nearest rank on individual callback sums, not sums of separate percentiles.
The table ranges over eight runs per mode and reports milliseconds.

| Mode | Paired callback p95 range | Paired callback p99 range |
| --- | --- | --- |
| Idle | 9.292–9.514 | 9.400–11.484 |
| Editing | 9.338–9.760 | 9.495–11.309 |
| Slider | 9.261–9.516 | 9.521–11.020 |
| Locale switching | 9.052–9.621 | 9.413–11.267 |
| Scrolling | 9.270–9.496 | 9.438–11.276 |
| Static windows | 9.292–10.743 | 10.440–11.261 |
| Animated windows | 9.196–9.786 | 9.664–11.236 |
| Selection | 9.322–10.332 | 9.578–12.281 |
| Injected preedit | 9.332–10.190 | 9.580–11.302 |

GPU diagnostics report119 collected in59 runs and118 collected/one skipped in13;
all report zero errors and no pending sample. No inference of zero GPU cost for
skipped samples. Full per-locale/repeat distributions and maxima are retained in
`target/window-paired-matrix-summary.csv`; raw evidence is
`target/window-paired-matrix-<mode>-<locale>-<repeat>.log`.
Reproduce with the existing release binary, both diagnostic flags set to1,
`--<mode>-smoke --locale=<locale>`, sequentially, twice for each combination.

Largest selected paired sample: selection/ru/repeat1/index54=22402us;
independent phase rows at index54 show preparation1020us and redraw21382us.
Renderer index54 records acquire6048us, encode5182us, submit337us, present9808us
and host present-call interval34388us. Index alignment here is a local observation
of this run's one-preparation/one-redraw sequence, not a new cross-probe contract.
The row shows blocking and encoding contributions; it does not identify their
OS/driver cause or prove a UI regression. No speculative production fix follows.

Disposition: paired native DPI1 callback evidence now covers all existing isolation
modes and locales. Callback elapsed sums include redraw blocking and exclude native
event handling and event-loop waiting. Renderer intervals are host-call proxies;
actual displayed intervals, full GPU execution and whole-engine active CPU remain
unmeasured. Scroll uses a different viewport; animated actions may supersede active
transitions. Injected selection/preedit/editing do not validate OSIME/clipboard.
NativeDPI2, controlled reference conditions and maintainer acceptance remain open;
this matrix does not close the performance gate or justify optimization claims.

Full ./scripts/verify.ps1 passed with process-local shared engine target and
sequential tests (target/window-paired-matrix-verify.log). No new source changes.

## Long-field asset-font preedit/commit probe — 2026-10-04

PresentMon/WPA/GPUView are absent from PATH on this host; built-in WPR reports
GPU/DesktopComposition profiles and no active recording. No recording was started:
an ETW file without validated event analysis would not close display acceptance.
Continued with the next open CPU workload rather than changing system display DPI.

The sibling workbench adds ignored `measure_long_field_editing` beside its existing
layered font-editing probe. It uses public APIs, Noto Sans/Arabic/JP assets, a
1000x800 logical viewport, synthetic DPI1/2, three overlapping clipped panels
with120-pixel-tall fields and a top modal layer. Four content phrases repeat
8/64/256 times. Font service locale stays en-US, so labels identify content scripts,
not localization changes. Each cycle selects the entire top-field value, routes
injected preedit, lays out/paints, commits alternating suffix0/1 and lays out/paints
again. Final assertions check layer order, focus, committed value and cleared preedit.
Ten warmups precede100 individually timed cycles/configuration. Two sequential
release runs, UI/text diagnostics unset, no concurrent build/test during acquisition.
Source strings/event construction, initial font/layout setup, preflight size check,
CSV printing and final validation are excluded; two layouts and result destruction
are included. The preflight font service remains resident during the timed probe;
no memory/allocation measurement or cold-service baseline is implied.

Reproduce from sibling examples root:

```console
cargo test -p gridthorn_example_multilingual_workbench --release --locked measure_long_field_editing -- --ignored --nocapture --test-threads=1
```

Both runs pass20 timing configurations with100 samples each. All four256-repeat
DPI2 layouts reject with `UiCompositionError::Text(TextError::TooLarge)` before
measurement; rejected configurations have no timing samples. The first exploratory
run stopped at this error; retained as target/long-field-editing-1.log. Completed
runs are target/long-field-editing-complete-{1,2}.log; individual percentiles/maxima
are in target/long-field-editing-summary.csv. Nearest-rank p95 ranges across repeats,
in milliseconds, follow. Strings have different scalar/byte counts; do not compare
script rows as equal-size workloads.

| Content | 8 repeats DPI1 / DPI2 | 64 repeats DPI1 / DPI2 | 256 repeats DPI1 |
| --- | --- | --- | --- |
| English | 0.774–0.870 / 1.882–1.905 | 7.480–8.365 / 14.464–14.744 | 47.493–59.540 |
| Russian | 0.830–1.064 / 2.394–2.742 | 12.066–12.140 / 26.673–32.465 | 71.561–96.676 |
| Arabic | 0.550–0.637 / 1.172–1.520 | 8.148–8.944 / 14.777–15.127 | 45.291–54.508 |
| Japanese | 0.770–0.806 / 2.649–2.670 | 8.264–11.336 / 27.559–33.502 | 58.601–61.098 |

At256 repeats, UTF8bytes/scalars are English3072/3072, Russian7680/4096,
Arabic5632/3072 and Japanese5632/2048. Japanese64-repeat content is1408bytes/
512scalars. These are warm repeated-glyph, two-value working sets, not unique-glyph
or long unique-string churn. Composition and two paints share one timed cycle;
these timings are not one native frame or isolated editing/raster attribution.

Disposition: long-field edit/preedit cost and initial DPI2 rejection are now
measured through the public UI path. Costs grow substantially despite panel clips.
The rasterizer's existing one-million-covered-pixel guard can report TooLarge;
this increment does not remove that safety bound or prove which phase dominates.
Next: attribute shaping, raster spans, decoration and clipping before selecting a
focused fix; add transactional failure regression if editing exposes partial state.
Asset-font expanded layers, heap peaks, native DPI2/OSIME/clipboard, actual display
intervals and the overall performance acceptance gate remain open. No production
behavior/dependency change or optimization claim in this increment.

Engine full ./scripts/verify.ps1 passed with shared process-local target/sequential
tests (target/long-field-full-verify.log). Sibling workbench package:13passed,
2ignored in normal tests; both manual long-field runs passed. All-target Clippy,
workspace formatting and sibling/engine diff whitespace passed. Logs:
target/long-field-package-{tests,clippy}.log. Engine changes in this increment are
documentation only; sibling changes are the focused probe and README. No commits.

## Long-field geometry attribution and indexed cluster ranges — 2026-10-04

The sibling probe now has a separate ignored `measure_long_field_phases` entrypoint
requiring UI/text diagnostic flags. It runs the same long-field cases. The normal
measurement still requires those flags unset. One diagnostic acquisition before
and one after the fix complement two normal, diagnostic-free acquisitions on each
side. No concurrent builds/tests during acquisition. Reference revisions, fixtures,
viewport, synthetic DPI, warm repeated content and environmental limits match the
preceding increment. Clocks, thermals and background load remain uncontrolled.

Router diagnostics retain221 samples/case: initial layout plus110preedit/commit
cycles with two layouts each. Exclude indices0-20 to retain200warm individual
layout samples. Text-service diagnostics for the main service contain1883layout
and776raster calls/case; exclude first183layout and76raster calls (setup plus
10warm cycles), leaving1700/700. The separate preflight service is excluded from
attribution. All main router summaries report skipped0. Phase percentiles below
are from single diagnostic runs, include timer overhead and are not repeated-run
confidence intervals. Decoration is nested in paint; text layout/raster calls
are nested within UI work and must not be added to UI phase percentiles.

| 256-repeat DPI1 content | Geometry p95 before → after (us) | Paint p95 before → after (us) | Warm text-layout p95 before → after (us) | Raster-call p95 before → after (us) |
| --- | --- | --- | --- | --- |
| English | 15886 → 1418 | 18784 → 15134 | 3 → 2 | 4942 → 4226 |
| Russian | 22019 → 1963 | 20370 → 20288 | 4 → 5 | 5469 → 5410 |
| Arabic | 11878 → 1313 | 14367 → 13679 | 3 → 3 | 3873 → 3641 |
| Japanese | 7871 → 1106 | 39569 → 36224 | 12460 → 12111 | 6080 → 5503 |

This identifies geometry as a substantial independent cost for long fields.
`TextGeometry::shaped` previously scanned every grapheme boundary and allocated a
new vector for every glyph. It now uses two partition searches in the already
sorted boundary list and borrows the inclusive cluster-endpoint slice. Glyph
visual order, RTL subdivision, paragraph offsets, geometry ownership and output
remain unchanged. Private shaped input now accepts the existing public TextLine
slice, permitting focused synthetic RTL/paragraph regression coverage. Tests cover
ligature endpoints, combining graphemes, distant paragraphs, empty ranges, RTL
caret/hit positions and segment byte offsets. No new public API/dependency/cache
or raster safety-limit change.

Two normal release runs after the fix pass all20timed configurations and report
the same four largestDPI2 initial-layout rejections. Before/after cycle p95 ranges
from the two runs per side (milliseconds):

| 256-repeat DPI1 content | Before cycle p95 range | After cycle p95 range |
| --- | --- | --- |
| English | 47.493–59.540 | 33.408–37.389 |
| Russian | 71.561–96.676 | 44.313–45.885 |
| Arabic | 45.291–54.508 | 27.406–33.001 |
| Japanese | 58.601–61.098 | 53.660–61.457 |

Smaller/DPI2 cases vary and do not all improve; for example English8/DPI2 p95 is
1.882–1.905ms before and2.265–2.437ms after, Russian64/DPI2 is26.673–32.465ms
before and24.862–35.907ms after. Do not claim a uniform end-to-end speedup. The
geometry phase and removed full-boundary scan support the focused fix; remaining
raster/paint and Japanese layout costs require further attribution. These warm
layout-call timings do not isolate cold shaping or fallback allocation costs.
No total allocation/heap-peak measurement is implied by removal of the temporary
per-glyph vector. Panel clips still act after complete text rasterization.

Evidence: target/long-field-phases-{before,after}.log,
long-field-phase-comparison.csv, long-field-geometry-comparison.csv,
long-field-after-{1,2}.log; before repeats remain in
long-field-editing-complete-{1,2}.log. Reproduce the normal command from the
preceding section. For diagnostics, set GRIDTHORN_UI_PERFORMANCE and
GRIDTHORN_TEXT_PERFORMANCE to1 and use filter measure_long_field_phases instead.

Eight native release smokes passed, editing/selection for ru/ja twice each, with
window/render diagnostics (target/long-field-fix-native-<mode>-<locale>-<repeat>.log).
These are the existing bounded short-field interaction scripts; they do not prove
native long-field, DPI2, actual OSIME/clipboard or display acceptance. The next
focus is remaining long-field paint/raster work and Japanese layout misses while
preserving clipping output, last-good behavior and the raster safety bound.

Full ./scripts/verify.ps1 passed with process-local shared target/sequential tests
(target/long-field-geometry-full-verify.log). Sibling package tests, all-target
Clippy, formatting, two normal/manual phase acquisitions and eight native smokes
passed. Final engine/sibling diff whitespace passed. Env overrides scoped to tool
processes. No commits.

## Ink-aware clipped raster draw spans — 2026-10-04

Engine starting HEAD87d220719e09a7a63499e0f402a9b788f23cbd2a had a clean working
tree; sibling HEADc89adb9a5317007b3469782c1c8da9d8b4b1b04a retained the existing
uncommitted changes. Baseline is the preceding indexed-geometry implementation.
No dependency edge changed. The renderer adds public TextSystem::rasterize_clipped
with a physical UiRect relative to the layout origin (rectangle color ignored).
It inspects actual glyph-image placement/extent, including ink overhang, and omits
span reconstruction only when the whole ink box lies outside the clip. Two pixels
of conservative edge padding retain ink across separately rounded bounded logical
origins/clips. Partially visible glyphs keep all spans for the existing exact
ordered primitive clipping. Measurement and full shaping/field geometry remain.

All glyph images are still looked up and counted before culling; missing-glyph
raster errors and the one-million-glyph-image-sample guard retain their behavior,
even for invisible text. Normal rasterize remains the unculled path. Clipped
snapshots may still contain pixels outside the requested rectangle and must be
submitted with that same clip. This is draw-data culling, not a crop API or removal
of the work budget. Glyph backend cache population/cold glyph rasterization and
full-layout costs remain. UI label/control/preedit paint passes its effective
ancestor/content clip in layout-local physical coordinates; bitmap paint is
unchanged. List painting is extracted into its focused routine to keep control
composition bounded. No automatic field truncation or caret scrolling is added.

Two renderer regressions compare final nondegenerate ordered vertex positions
and tint against an unculled raster under the same clip, with mixed-script text,
combining marks/ligatures, positive/negative fractional origins and DPI1/1.25/2.
Downstream clipping retains collapsed zero-area quads; these are omitted from the
visible-output comparison, not treated as a pixel difference. Error coverage checks
fully invisible requests, unchanged TooLarge/foreign-layout/invalid-DPI results
and retained last-good snapshots. A sibling public-API integration test verifies
ancestor-clipped labels at fractional/negative placement and DPI1/1.25/2 retain
complete value/measurement while reducing raster draw data. No test-only SDK
dependency or privileged sibling access is introduced.

Repeated normal long-field probes use the same public workload,20supported cases
with100warm samples each, plus four largestDPI2 initial-layout rejections. Before
is target/long-field-after-{1,2}.log; after is target/raster-clip-long-{1,2}.log.
UI/text flags are unset for normal acquisition; one separate diagnostic acquisition
uses both flags. No concurrent builds/tests during acquisition. Viewports/font
fixtures/synthetic DPI and environmental limits match the preceding probe; clocks,
thermal/background load remain uncontrolled. Nearest-rank cycle p95 ranges across
the two runs on each side (milliseconds), for256phrase repetitions/DPI1:

| Content | Before clipped spans | After clipped spans |
| --- | --- | --- |
| English | 33.408–37.389 | 7.568–12.092 |
| Russian | 44.313–45.885 | 7.622–8.167 |
| Arabic | 27.406–33.001 | 6.710–7.869 |
| Japanese | 53.660–61.457 | 20.351–21.630 |

64-repeat/DPI2 after p95 ranges are English11.688–12.281ms,
Russian12.468–12.477ms, Arabic11.293–11.362ms and Japanese12.776–15.684ms.
Small cases are variable: Arabic8/DPI2 is1.079–2.295ms after versus1.399–1.790ms
before, so no uniform speedup or native frame-budget acceptance is claimed.
Complete percentiles/maxima: target/raster-clip-cycle-comparison.csv.

One UI diagnostic run per side, excluding initial layout/ten warm cycles, retains
200individual layouts per supported case. Paint p95 for256/DPI1 falls from
15134/20288/13679/36224us (English/Russian/Arabic/Japanese) to
2494/3162/2483/15394us. Geometry p95 remains1176/2012/1374/872us after. Japanese
focused-decoration p95 remains13102us, nested within paint: composition text still
requires separate layout and this warm workload does not isolate cold fallback
misses/allocations. Do not add separate phase percentiles into a cycle distribution.
Evidence target/raster-clip-phases.log and raster-clip-phase-summary.csv; baseline
phase evidence remains target/long-field-phases-after.log. Existing probe commands
in the previous section reproduce both acquisitions with the appropriate flags.

24native release smokes passed: editing/preedit/scroll, four locales, two repeats.
Each has119renderer and119paired callback rows; GPU23runs collected119 and one
collected118/skipped1, all errors0/pendingfalse. Native scale1 on the same
RTX3070/Vulkan616.56/Fifo/reference display; scrolling uses1000x400, other modes
1000x800. Raw logs target/raster-clip-native-<mode>-<locale>-<repeat>.log. These
bounded short-field/injected native workloads do not prove native long-field,
DPI2, OSIME/clipboard or actual displayed intervals. Snapshot memory peaks and
full-GPU execution remain unmeasured. The next text focus is remaining Japanese
composition/layout misses, followed by the wider milestone acceptance matrix.

Full ./scripts/verify.ps1 passed with process-local shared engine target and
sequential tests (target/raster-clip-full-verify.log), including dependency
boundaries and new renderer tests. Sibling package14passed/3ignored, all-target
Clippy, release build and24native smokes passed; final formatting/engine+sibling
whitespace passed. Env overrides scoped to tool processes. No commits.

## Narrow Japanese composition cache-budget attribution — 2026-10-04

Continued from the clipped-raster increment at engine
HEAD87d220719e09a7a63499e0f402a9b788f23cbd2a with its existing uncommitted changes;
sibling HEADc89adb9a5317007b3469782c1c8da9d8b4b1b04a and unrelated changes preserved.
The previous main Japanese256/DPI1 service reports1770hits/113misses/110bypasses,
retaining3entries/16927key bytes/5959glyphs/192lines. Other content scripts largely
hit the cache. The narrow remaining preedit width yields many visual lines, beyond
the old1024line retention cap; the layouts are valid but repeatedly not retained.

A focused ignored renderer probe `measure_narrow_japanese_layout` measures one
256-repeat Japanese phrase at width1/12/600, primary Noto Sans or Noto Sans JP,
font size20, localeen-US, supplied Noto fixtures, WordOrGlyph wrapping. Each fresh
service runs10warmups+100individual calls/configuration. Backend-buffer weak
references prove actual retained identity rather than assuming cache reuse.
Font loading/source construction/missing-glyph checks/output are outside timing;
service validation, cache lookup, misses/shaping and returned layout construction
are inside. Two sequential release acquisitions before and after, diagnostics unset,
no concurrent agent builds/checks. Reference Windows/compiler and uncontrolled
clocks/thermal/background conditions match previous measurements.

```console
cargo test -p gridthorn_render --release --locked measure_narrow_japanese_layout -- --ignored --nocapture --test-threads=1
```

| Primary family / width | Layout lines | Warm hits before → after (each run) | Before p95 range (us) | After p95 range (us) |
| --- | --- | --- | --- | --- |
| Noto Sans / 1 | 2048 | 0 → 100 | 13747.5–15563.9 | 0.4–0.4 |
| Noto Sans / 12 | 1792 | 0 → 100 | 12509.6–12574.9 | 0.3–0.4 |
| Noto Sans / 600 | 64 | 100 → 100 | 0.2–0.3 | 0.2–0.4 |
| Noto Sans JP / 1 | 2048 | 0 → 100 | 2509.1–2531.0 | 0.2–0.4 |
| Noto Sans JP / 12 | 1792 | 0 → 100 | 2101.9–2274.0 | 0.3–0.3 |
| Noto Sans JP / 600 | 62 | 100 → 100 | 0.2–0.3 | 0.2–0.4 |

The comparison attributes repeated work to line-budget bypass and shows that
fallback adds miss cost on this workload. It does not remove cold shaping/fallback
cost, change chosen fonts or prove general unique-string behavior. Cache-hit
submicrosecond timing is near host timer granularity; reported nearest-rank values
are individual-call samples, not a precise hardware lower bound.

The bounded service-local LRU line limit is now4096, admitting the measured pair
of narrow layouts plus ordinary field layouts. Entry64/key256KiB/glyph16384limits
and oversized bypass/eviction policy remain. This calibration explicitly quadruples
maximum retained diagnostic line count; the main Japanese UI working set grows
from192to4032lines,3to5entries,16927to28209key bytes and5959to9799glyphs.
Opaque backend allocation size and process/heap peaks are not measured by these
counts. Additional bounded storage is the tradeoff; do not describe this as a
memory saving or claim unchanged memory. Existing live caller layouts are outside
these cache retention limits. No dependency edge/public API/font or raster-budget
change. The cache budget remains provisional.

Regression coverage adds real narrow primary/fallback layouts at three widths,
checks full diagnostic lines/measurement/shared-buffer identity, and forces
line-budget eviction with multiline layouts to verify release of unowned buffers.
Existing aggregate budget/oversized/key/glyph/LRU/lifetime tests also pass; the
aggregate-line fixture scales to the new cap rather than fixing old eviction counts.

Two complete normal public UI long-field probes still pass20supported configs,
100warm cycles each, plus the same four largestDPI2 initial-layout rejections.
Japanese256/DPI1 cycle p95 falls20.351–21.630ms →6.314–7.746ms; medians
18.991–19.313ms →5.950–6.233ms. One separate phase acquisition after the fix
uses the same200warm individual UI layouts: geometry p95942us, paint3386us,
focused decoration258us (nested in paint), compared with preceding872/15394/
13102us. Main cache now reports1878hits/5misses/0evictions/0bypasses with4032
peak/final lines. Normal repeats remain diagnostic-free; do not combine phase
percentiles into a native frame distribution or infer cold/heap/display acceptance.

Evidence target/narrow-layout-{before,after}-{1,2}.log and comparisonCSV;
target/narrow-cache-long-{1,2}.log, narrow-cache-cycle-comparison.csv,
narrow-cache-phases.log. Previous normal baseline is raster-clip-long-{1,2}.log;
phase baseline is raster-clip-phases.log. Normal/manual commands from the preceding
sections reproduce the UI probe and its diagnostic acquisition. Wider working-set
pressure may still evict or bypass; remaining fallback/raster allocations, cold
publication/memory peaks and nativeDPI2/OSIME/clipboard/fullGPU/actualdisplay and
maintainer acceptance remain open.

Eight native release editing/preedit smokes (ru/ja twice each) passed,119renderer
samples each; GPU four runs119collected and four118collected/one skipped, all
errors0/pendingfalse. Evidence target/narrow-cache-native-<mode>-<locale>-<repeat>.log.
Full ./scripts/verify.ps1 passed with process-local shared target/sequential tests
(target/narrow-cache-full-verify.log), including all retention/lifetime regressions.
Sibling package14passed/3ignored, all-target Clippy/release build passed. Final
formatting/engine+sibling whitespace passed. No sibling source change this increment,
staging or commits; environment overrides scoped to tool processes.

## 2026-10-04 Text cache memory after line-budget calibration

At engine e907c020f05db9341107edde027b939b98c0df47, two separate release
processes ran the ignored renderer `measure_layout_cache_memory` probe alone.
It warms three Japanese strings (256 phrase repetitions) at widths 600/1/12,
clears retention, then holds six phases for one second each. Three width-600
layouts model ordinary fields; widths 1/12 of the first string add composition
layouts. Twenty unique 256-line strings force line-budget eviction. Weak backend
buffer handles verify both narrow buffers are released after pressure. Every
phase asserts all four retention budgets. No production behavior changed.

| Phase | Entries | Lines | Engine diagnostic capacity bytes |
| --- | ---: | ---: | ---: |
| Warm empty | 0 | 0 | 0 |
| Three ordinary fields | 3 | 192 | 430150 |
| Ordinary plus two narrow fields | 5 | 4032 | 886648 |
| Line pressure | 15 | 3850 | 192586 |
| Cache cleared / service dropped | 0 | 0 | 0 |

The extra retained engine diagnostic capacity is 456498 bytes (445.8 KiB).
This sums entry-vector capacity, key/family strings, diagnostic line slices,
glyph-vector capacity and glyph-family strings. It excludes backend buffers,
Arc headers, allocator overhead, font/shaping caches and caller-held layouts;
it is neither total heap allocation nor a cache memory upper bound.

The external Windows sampler launches the release unit-test executable directly
with `--ignored --exact text::layout_cache::memory_test::measure_layout_cache_memory
--nocapture --test-threads=1`, redirects stdout, and polls its PID every 50 ms.
It reads phase markers and `Process.PrivateMemorySize64`/`WorkingSet64`.
Across the two acquisitions, ordinary-field private bytes are 14495744–15261696;
narrow-phase samples are 13873152–17887232. The corresponding sampled resident
ranges are 22159360–22585344 and 22335488–25579520 bytes. These process-wide
ranges include phase transitions, allocator reuse and unrelated backend storage;
they do not isolate an incremental backend-buffer cost. This is sampled memory,
not a captured allocation peak. Even the last six samples vary during narrow
phases, so a settled heap delta is not claimed.

After clearing retention, private bytes are 10366976–10989568 and resident bytes
19107840–19161088. Dropping the service reduces these to 2600960–2879488 and
11882496–12300288 respectively. Remaining process memory is not proof of a leak;
buffer lifetime is checked directly. The larger cache remains a provisional
bounded retention tradeoff; exact backend heap peaks and broader domain memory
acceptance remain open. Evidence: `target/cache-memory-{1,2}.log` and
`target/cache-memory-{1,2}-samples.csv`. Build the executable with
`cargo test -p gridthorn_render --release measure_layout_cache_memory --no-run`;
use the executable path Cargo reports, rather than sampling Cargo's parent PID.

Two release runs of `measure_long_text_and_cache_pressure` also pass after the
calibration (36 layout configurations and 16 full-raster configurations per run,
100 calls each). All four scripts retain 100/100 hits for working-set 1 at
8/64/256 repetitions, and for working-set 32 at 8 repetitions. At 64 repetitions
and working-set 32, Japanese retains 100/100 hits; English/Russian/Arabic each
have zero. All working-set 96 cases and all 256-repetition working-set 32 cases
have zero hits. Caller-held original geometry remains valid after churn, and
missing-glyph checks pass. The larger line budget does not remove pressure from
the entry/glyph budgets. These full-raster measurements do not exercise clipped
UI rasterization. Evidence: `target/cache-pressure-current-{1,2}.log`; command:
`cargo test -p gridthorn_render --release measure_long_text_and_cache_pressure
-- --ignored --nocapture --test-threads=1`, with text diagnostics unset.

## 2026-10-04 Fresh-service Japanese phases and raster output storage

New ignored renderer `measure_cold_japanese_phases` separates service creation,
first layout, changed-string layout and identical changed-string cache hit.
Two release runs use the three existing asset fonts, en-US locale, primary
Noto Sans versus Noto Sans JP, font size20, 256 repetitions at widths1/12/600
and64 repetitions at width600. Each configuration has20 fresh-service samples
without warmup, for160 service creations per run. Asset validation happens before
measurement. Services are fresh within an already-running process: this is not
cold OS file I/O, first process startup, or isolated backend-function attribution.
The changed string appends ` 1`, retaining nearly identical content/coverage.

| Primary / repetitions / width | First-layout p95 ms | Warm-font unique p95 ms |
| --- | ---: | ---: |
| Sans /256/1 | 13.199–14.434 | 13.008–13.711 |
| Sans /256/12 | 13.318–14.988 | 13.211–13.802 |
| Sans /256/600 | 11.764–12.181 | 11.879–12.635 |
| JP /256/1 | 2.770–4.530 | 2.553–4.421 |
| JP /256/12 | 2.629–2.655 | 2.106–2.212 |
| JP /256/600 | 1.970–2.191 | 1.739–2.239 |
| Sans /64/600 | 3.122–3.319 | 3.026–3.030 |
| JP /64/600 | 0.566–0.570 | 0.432–0.447 |

Nearest-rank p95 is sample19 of20 sorted individual calls; a small-sample host
tail, not an acceptance threshold. Service-creation p95 ranges0.837–1.939ms
across configurations/runs. Every hit shares the preceding layout's backend
buffer; hit p95 is0.4–1.4us, near timer granularity. Missing glyphs are zero.
The primary/fallback comparison includes changed glyph metrics/layout and does
not isolate fallback search from shaping or diagnostic geometry. It does show
that this fallback-heavy unique-string cost persists after warming font state;
the retention calibration primarily helps identical requests. No automatic
primary-family selection or backend optimization is implemented in this increment.

For64repetitions/width600/DPI1, each service additionally measures its first full
raster call, repeated full call and glyph-cache-warm clipped call with a600x40
physical clip. First/repeated full snapshots compare equal. The first snapshot
remains caller-held for this comparison; output bytes are per snapshot, not total
live storage. The clipped output is strictly smaller on every sample.

| Primary | First full p95 ms | Repeated full p95 ms | Warm clipped p95 ms | Full / clipped output bytes |
| --- | ---: | ---: | ---: | ---: |
| Sans | 1.728–1.729 | 1.348–1.642 | 0.264–0.284 | 1766016 /220752 |
| JP | 1.353–1.379 | 0.993–1.080 | 0.215–0.256 | 1760696 /225876 |

Output bytes use the engine-owned `TextPixel` slice size (28bytes per span on
this host). They exclude Arc headers, construction-vector capacity, request-local
glyph-span storage, backend glyph images, allocator overhead and GPU buffers.
The probe neither counts allocation events nor measures temporary/total heap
peaks. Layout/service CSV output_bytes=0 means unmeasured, not zero allocation.
The clipped acquisition follows full raster and is not a cold clipped comparison.
Only DPI1 and this fixed top clip are covered; no frame/display distribution is
inferred. Allocation instrumentation and broader cold/unique workloads remain open.

Reproduce with `cargo test -p gridthorn_render --release
measure_cold_japanese_phases -- --ignored --nocapture --test-threads=1`, with
GRIDTHORN_TEXT_PERFORMANCE unset. Evidence: target/cold-japanese-{1,2}.log and
target/cold-japanese-{1,2}-summary.csv. The probe verifies cache identity, coverage,
snapshot equivalence and smaller clipped storage. Existing memory-probe working
tree changes are preserved; no sibling source change or production change.

## 2026-10-04 Raster temporary-container capacity and early release

The workspace forbids unsafe code, so no custom GlobalAlloc instrumentation is
introduced. A test-only GRIDTHORN_RASTER_STORAGE_PROBE flag reports output-vector
capacity, target snapshot slice bytes and request-local glyph-span vector capacity
immediately before snapshot construction. It sums the cached per-glyph vectors
and scratch vector, excluding HashMap buckets/entries, backend image/shaping
caches, allocator overhead and Arc headers. This is a container-capacity
measurement, not allocation event counting or a total/temporary heap peak.

Two separate release acquisitions of the existing cold Japanese probe report
identical capacities for all20 samples per primary font and raster operation:

| Primary / output | Output Vec capacity bytes | Snapshot slice bytes | Glyph vectors plus scratch bytes |
| --- | ---: | ---: | ---: |
| Sans / full | 1835008 | 1766016 | 121856 |
| JP / full | 1835008 | 1760696 | 82432 |
| Sans / clipped | 229376 | 220752 | 121856 |
| JP / clipped | 229376 | 225876 | 82432 |

First/repeated full calls have identical reported capacity on this workload.
Clipping reduces output capacity but retains the same glyph-vector capacity:
the first visible lines still cover the repeated glyph set. Summing these columns
is not an observed heap peak; they have different lifetimes. Instrumented timings
include environment lookup/stdout diagnostics and are excluded from latency claims.
The probe is compiled only into renderer unit tests; normal builds have no new
environment flag, diagnostic output or instrumentation overhead.

Rasterization now explicitly drops request-local GlyphSpans after output assembly
and before converting the output vector to its immutable snapshot. This frees
the measured82432/121856 bytes of vector capacity (plus unmeasured map storage)
earlier, avoiding its lifetime overlap with snapshot construction. It does not
change glyph-cache retention or prove a corresponding OS/private-byte reduction.
No allocator, dependency, raster safety limit or public API changes. Existing
coverage/order/tint/fractional-DPI/clipping/error regressions remain the verification
contracts. Two diagnostic-free release cold Japanese probes pass after this change,
including repeated full output equality and smaller clipped output. No speedup
or total heap-peak reduction is claimed from this lifetime change.

Reproduce capacity acquisition by setting GRIDTHORN_RASTER_STORAGE_PROBE=1 for
the renderer release unit-test `measure_cold_japanese_phases`, run alone with
`--ignored --nocapture --test-threads=1`. Keep GRIDTHORN_TEXT_PERFORMANCE unset.
Evidence: target/raster-storage-{1,2}.log; diagnostic-free after runs:
target/raster-storage-after-{1,2}.log. Full allocator/backend peak attribution
remains open; the safe container probe does not complete that acceptance item.

## 2026-10-04 Complex fresh-service localization publication

The new ignored localization-domain probe `measure_complex_localization_publication`
extends the earlier simple catalog fixture. It uses16/256/4096 messages, four
locales (en-US/ru/ar-EG/ja), and20 fresh-service samples per configuration with
no warmup exclusions. Each16-message group has an anchor and a15-reference
chain. Non-anchor messages combine a string selector with nested cardinal plural
selection and NUMBER minimumFractionDigits2. The deepest formatted message
resolves the chain with role=worker, n=2 and amount=12345.5. Russian selects Few;
other fixture locales select Other (the fixture omits Arabic two). All catalogs
have the same English test labels; this tests locale rules, not translation quality.

Source generation and parameter construction are outside the timed intervals.
Separate measurements cover source validation, service construction, first
format, candidate validation, explicit replacement and invalid-candidate validation.
A revision changes every anchor. Publication is verified by formatting the deepest
message; old asset handles and owned formatted strings remain valid. The rejected
candidate replaces node-0's value with a self-reference without adding messages;
the probe requires a cyclic-reference Validation error, rather than a message-count
or syntax rejection, and verifies the published string and locale chain remain.
This models a caller discarding failed decoding before publication; it does not
claim that replace_catalog itself processes invalid source.

Two release runs each pass240fresh services, valid candidate replacements and
cyclic-candidate rejections. Nearest-rank p95 is sample19 of20 sorted individual
calls. Ranges below span all four locales and both runs, in microseconds:

| Messages | Initial validate p95 | Construct p95 | First format p95 | Replace p95 | Reject cyclic candidate p95 |
| --- | ---: | ---: | ---: | ---: | ---: |
| 16 | 62.0–134.3 | 4.4–15.3 | 26.2–73.1 | 5.8–13.4 | 68.6–113.2 |
| 256 | 1409.9–2192.9 | 47.7–69.7 | 33.2–71.9 | 62.4–101.2 | 1201.9–1895.9 |
| 4096 | 23392.9–30713.1 | 581.1–949.6 | 51.5–99.8 | 1590.7–2108.2 | 22238.6–30024.7 |

At4096messages source size is736337–736593UTF8bytes; candidate-validation p95
is23766.4–35467.7us across runs/locales. Validation includes parsing, supported
expression/type checks and reference-graph validation; these backend phases are
not individually attributed. Replacement includes preparing the new bundle and
releasing the runtime's old bundle; an old asset handle remains caller-held.
First format includes resolver/number/plural initialization for the referenced
message, not formatting every catalog message. Fresh services occur in a warm
process with in-memory source: this is not cold OS I/O or worker handoff.

Disposition: preserve explicit off-polling candidate validation and bounded
publication. The measured large complex validation can exceed one60Hz frame;
that comparison is a workload observation, not a real frame-stall measurement
or guaranteed limit. No runtime optimization/public API/dependency change is
introduced. Allocation/heap peaks, wider reference fanout/depth, alternate selector
branches and background handoff remain unmeasured. Milestone acceptance stays open.

Reproduce with `cargo test -p gridthorn_localization --release
measure_complex_localization_publication -- --ignored --nocapture --test-threads=1`.
Evidence: target/complex-localization-{1,2}.log and summaryCSVs. Full verification
passed via ./scripts/verify.ps1 (target/complex-localization-full-verify.log).

## 2026-10-04 Expanded public asset-font layers

The sibling workbench ignored `measure_expanded_font_layers` probe extends the
bitmap-layer fixture to real asset fonts through public SDK APIs. It uses3/16/64
overlapping clipped panels with1/16 fields each, four content scripts, synthetic
DPI1/2, a1000x800 logical viewport and Noto Sans20 with the existing Arabic/JP
fallback assets. Panel children use column flow: all16 fields fit a720-high panel.
Each field appends a unique layer/field suffix. All layers remain painted; only
the last is modal. Font/tree construction, initial raw layout and opening layers
are outside timing. Each of48configurations makes10 warm and20 measured full
router layout calls, including text shaping/raster and focused decoration.

Two complete release acquisitions pass. Focus stays on the first top-layer
field and hit testing resolves that field. Each layout contains one text primitive
per authored field, up to1024. A subsequent complete run also asserts the exact
primitive count for every call. The initial trial had overlaid children; column
flow was fixed before both retained measurements. No production change is made.

| Content script /1024 fields | DPI1 layout/paint p95 ms | DPI2 layout/paint p95 ms |
| --- | ---: | ---: |
| English | 55.75–59.98 | 75.75–81.31 |
| Russian | 63.88–65.47 | 95.82–120.10 |
| Arabic | 81.68–93.42 | 102.01–117.23 |
| Japanese | 188.00–233.69 | 294.62–307.17 |

P95 is nearest-rank sample19 of20 sorted individual calls, and ranges span the
two acquisitions. This is intentionally heavy overlapping paint with unique
strings. A1024-key working set exceeds the64-entry text-layout cache, but actual
hit/miss phase totals are not acquired here. Do not assign all cost to misses,
fallback or routing without attribution. Layers are not occlusion culled or
virtualized, no texts are edited during timing, and script labels do not imply
locale/catalog changes. Font loading and cold first-frame costs are excluded.
No GPU submission, actual display intervals, OSIME/clipboard or heap peaks are
measured. These timings do not establish realtime acceptance for1024 fields.

Disposition: expanded asset-font preparation is a remaining measured scaling
cost, especially Japanese and DPI2; attribute shaping/raster/layout/router phases
before choosing caching, virtualization or occlusion behavior. Such behaviors
would need explicit lifecycle/input contracts rather than being inferred from
this overlapping fixture. Wider mixed ECS/input and native acceptance remain open.

Reproduce from the sibling workspace with `cargo test
-p gridthorn_example_multilingual_workbench --release --locked
measure_expanded_font_layers -- --ignored --nocapture --test-threads=1`, diagnostics
unset. Evidence target/expanded-fonts-{1,2}.log and summaryCSVs in engine target;
final count-asserting acquisition target/expanded-fonts-final.log. This adds only
sibling domain test/documentation and preserves its existing working tree.

## Text/UI CPU review disposition — 2026-10-04

The requested remaining CPU text/UI review is complete for the recorded Windows
workloads. Engine starting revision is 0339437c5d6dc16ce0ecc5e7ed849dc7ddc909e9;
sibling revision is c89adb9a5317007b3469782c1c8da9d8b4b1b04a with its existing local
changes preserved. This closes the CPU review item, not the native whole-frame,
actual display, DPI-2 or real OS-IME acceptance gates. No default primary family,
paragraph segmentation, hidden-layer participation or raster safety limit changed.

### Attribution and focused changes

A fresh pre-change expanded phase acquisition ran English/Japanese, DPI1/2,
64 overlapping layers x16 unique fields, ten warm layouts and20 timed layouts.
All four services reported95232 layout misses and zero hits: initial tree layout
and30 router layouts each request the1024 values in three separate stages.
The64-entry persistent LRU churns between arrangement, field geometry and paint.
Backend shaping dominates Japanese layout misses; engine diagnostic extraction
is much smaller. The new per-UI-pass prepared-text cache shares shaped storage
across all three stages and preedit, then drops before returning UiLayout. Limits:
1024 entries,256KiB copied UTF-8 text/family keys,16384 diagnostic glyphs,4096 lines.
Exhaustion bypasses retention without changing successful output or errors.
The permanent text-service LRU is unchanged. Final services report31744 misses;
English also has1178 service hits at pass-budget saturation, Japanese zero hits.
The expensive repeated stage requests are now served by the temporary pass cache.

Representative warm phase means, from20 diagnostic layouts per case, in ms:

| Japanese /1024 fields | Arrange before → after | Geometry before → after | Paint before → after |
| --- | ---: | ---: | ---: |
| DPI1 |55.14 →58.77 |56.67 →2.45 |82.98 →17.06 |
| DPI2 |59.64 →59.25 |56.54 →2.33 |118.09 →35.04 |

Arrangement includes layer ordering; paint includes focused decoration. These
means locate the remaining cost rather than adding independent percentiles.
The pass cache adds map/key/style bookkeeping during arrangement. Diagnostic
text samples truncate independently at8192 calls per phase; counts remain complete.
No whole-frame or exact backend fallback-call attribution is inferred.
Evidence: target/text-ui-expanded-before-phases.log and
text-ui-measure_expanded_font_phases-final.log, with summaryCSVs.

Text services now reuse glyph-relative tinted spans between short and long raster
calls, bounded to128 glyph keys/65536 retained spans. Tint changes release the
previous set; clear_raster_cache clears spans and backend images. Entry saturation
uses direct sampling; span saturation does not retain the new glyph. Scratch is
released before publication. Retained vector/map capacity is outside the span-count
limit. Backend image lookup, actual ink clipping and the one-million image-sample
limit still apply to every request, including wholly offscreen glyphs.

The output Vec formerly copied into a newly allocated Arc slice. Snapshots now
share that Vec through an immutable internal owner. This removes the second
output allocation/copy but retains spare capacity until the last clone releases it.
In the fresh-service64-repeat Japanese fixture at20px/DPI1, snapshot construction
median was about0.3ms before; after sharing the vector it is below the diagnostic
one-microsecond resolution. Raster-loop work remains separately measurable.
This is an allocation/copy removal, not a total heap-peak or universal speedup claim.
Evidence: target/text-ui-japanese-{before,after}-phases.log and summaryCSVs.

### Diagnostic-free repeated outcomes

Two complete final release acquisitions cover48 expanded configurations:
3/16/64 layers x1/16 fields xfour scripts xDPI1/2,10 warm+20 timed layouts.
Each checks modal focus, top-layer hit testing and exact primitive counts.

| Script /1024 fields | Previous p95 DPI1 /DPI2 ms | Final p95 DPI1 /DPI2 ms |
| --- | ---: | ---: |
| English |55.75–59.98 /75.75–81.31 |38.13–46.69 /52.47–67.18 |
| Russian |63.88–65.47 /95.82–120.10 |48.35–58.15 /78.49–82.07 |
| Arabic |81.68–93.42 /102.01–117.23 |49.71–66.11 /63.48–81.61 |
| Japanese |188.00–233.69 /294.62–307.17 |92.18–145.49 /119.16–135.89 |

Previous ranges are the two retained expanded-fonts acquisitions above; final
ranges span the two new acquisitions. Host scheduling variation is visible,
especially Japanese/DPI1; do not claim a uniform multiplier. All cases still
perform the authored overlapping paint, without virtualization or occlusion culling.
These intentionally heavy1024-field layouts remain above16.67ms. Evidence:
target/text-ui-measure_expanded_font_layers-final-{1,2}.log and summaryCSVs.

Two complete long-field acquisitions cover four scripts,8/64/256 phrase repeats
and synthetic DPI1/2,10 warm+100 individually timed preedit/commit cycles. Japanese
256/DPI1 p95 is5.264–6.571ms;64/DPI1 is3.264–4.803ms and64/DPI2 is8.882–9.706ms.
An intermediate acquisition after pass reuse/output sharing but before persistent
span reuse had256/DPI1 p95 9.568ms in run1. The largest256-repeat DPI2 fixture
rejects for every script at the existing raster work limit; no timing samples are
assigned to rejected configurations. Tests preserve old snapshots on raster errors.
Two overlapping-font acquisitions also repeat the32-pointer-event and short
preedit/commit workloads with modal and nonmodal scopes, four scripts and DPI1/2.
Evidence: target/text-ui-measure_{long_field_editing,overlapping_font_editing}-final-{1,2}.log;
long phase evidence: text-ui-measure_long_field_phases-final.log.

### Japanese unique misses and hidden sizing limits

The fresh-service phase probe separates backend shape from extraction. For the
256-repeat,600px-wide Japanese string, Sans fallback shaping p95 is13.370ms and
JP-primary shaping2.333ms, across40 cold/unique misses per family; extraction
p95 is0.150/0.163ms respectively. This family comparison changes font semantics,
so it diagnoses a workload rather than prescribing a primary-font substitution.
The locked backend's advanced fallback path shapes the primary and fallback runs,
then scans/removes missing cluster positions and searches replacement glyphs;
these repeated scans provide a concrete backend follow-up. Timings attribute the
cost to the backend stage, not to individual scans or allocator events. Unique
requests still pay it; exact matching layouts hit the existing service LRU.
Follow-up: profile/optimize backend fallback-run replacement with exact coverage,
clusters, bidi and glyph/raster equivalence, without silently splitting authored
paragraphs or forcing another primary family.

Two visibility acquisitions register64x16 fields, close all layers, then open
one16-field layer. Each configuration has10 warm+20 timed calls; primitives must
be0/16. Japanese all-closed p95 is62.33–76.45ms atDPI1 and64.02–87.61ms atDPI2.
Closed managed layers still participate in sizing by contract; geometry/paint
removal does not eliminate their unique shaping. Opening one layer is similar.
This confirms the remaining large-tree cost independently of paint. Follow-up:
design explicit incremental/subtree measurement invalidation, preserving hidden
intrinsic size, content extents, focus, scroll and native text-anchor lifetimes.
Do not rebuild this whole fixture every frame; unchanged caller-owned layouts
can already be reused. Evidence: target/text-ui-visibility-{1,2}.log and summaryCSVs.

### Raster storage ownership and process observations

A new separate-process lifecycle probe uses64 Japanese repeats,24px font,600px
width andDPI1. It holds seven phases for one second, checks clone sharing, cache
release and final output-owner release. Both acquisitions report identical capacities:

| Phase | Unique output Vec capacity bytes | Retained span Vec capacity bytes | Backend image data capacity bytes |
| --- | ---: | ---: | ---: |
| Before raster |0 |0 |0 |
| Full raster |3670016 |186368 |14016 |
|32 additional clones |3670016 |186368 |14016 |
| Plus clipped raster |4128768 |186368 |14016 |
| Raster cache cleared |4128768 |0 |0 |
| Snapshots dropped |0 |0 |0 |

Counts exclude Arc/Vec/map metadata, allocator rounding, layouts/fonts, backend
font/shape scratch and GPU data. Clones share output; there is no second full
output buffer at snapshot construction. The full+clipped vectors retain about
3.94MiB capacity; this is actual retained capacity, not logical span bytes.

Each executable is launched hidden and sampled externally through GetProcess at
nominal25ms intervals. Per-phase summaries exclude the first/last three samples
because marker/read/process polling can straddle a phase transition. Settled
private bytes are7.082–7.086MB before raster,10.707–12.493MB full/cloned,
11.485–11.489MB full+clipped,6.803–6.898MB after snapshots drop and1.466–1.511MB
after service drop. Full/cloned resident bytes are19.497–21.328MB. These are
process accounting observations, not allocator events, exact allocation/heap
peaks or an idle-subtracted universal budget. The cache-clear private-byte plateau
illustrates allocator retention; the ownership assertions establish actual release.
Evidence: target/text-ui-raster-memory-{1,2}.{log,samples.csv,settled.csv}.

Reproduce with renderer filters attribute_japanese_phases (text diagnostics on)
and measure_raster_storage_lifecycle (diagnostics off), in release/locked mode
with --ignored --nocapture --test-threads=1. Workbench filters
measure_expanded_font_phases and measure_long_field_phases require UI/text diagnostics;
measure_font_layer_visibility and the diagnostic-free matrices require them unset.

Disposition: duplicate UI shaping and output copying are fixed and remeasured;
short/repeated raster spans have bounded service reuse with release/tint tests.
Long/unique Japanese shaping, large hidden/overlapping trees and largest DPI2
raster requests have concrete measured limits and the follow-ups above. Opaque
backend heap peaks/allocator events and native long-field/real OSIME/display
acceptance remain unmeasured; they are not silently declared complete.

The normal workbench --performance acquisition reports Japanese router-layout
p95 0.388/0.914ms and edit-prepare0.497/0.946ms at syntheticDPI1/2. This is a small
warm CPU workload, not whole-engine frame or GPU/display acceptance. Headless,
mixed native smoke, native selection and injected preedit at actualDPI1 pass.
The final verification results are recorded in the accompanying checkpoint.

## Renderer batching, uploads and decoded-resource lifetime — 2026-10-04

Engine HEAD `310edf85a8466493d5d281006a47232245a98318`, examples HEAD
`c89adb9a5317007b3469782c1c8da9d8b4b1b04a`; the existing sibling working tree
was preserved. Rust 1.99.0, Windows reference host, Ryzen 5 5600X, RTX 3070,
Vulkan driver 616.56, Fifo, Balanced power plan. Native monitor metadata reports
1920x1080,144Hz,DPI1. Game surfaces are960x540; workbench1000x800. NativeDPI2,
other platforms, live clocks/thermal/background conditions remain unmeasured.

The textured path previously recreated/uploaded one texture and vertex buffer
per adjacent batch every frame. It now retains bind groups by decoded allocation
identity, including reuse across nonadjacent batches. An unchanged textured frame
also retains its ordered vertex buffers. Position/size/tint/region/order/camera/
extent or decoded identity changes rebuild geometry; a separately loaded equal
image remains a distinct identity. Absent batch identities are evicted on the next
prepared frame, and pipeline replacement/drop releases cache ownership. Backend
in-flight work can retain its own resource references; no immediate physical-VRAM
release guarantee is implied. Suspended/occluded rendering retains the last frame.

Adjacent shared-atlas batching and painter order remain unchanged. Crystal Trail
uses one textured batch/one64x16RGBA atlas (4096bytes). Timber Harbor's warm smoke
uses ten ordered batches but only three unique decoded textures (18,878,368bytes).
Do not merge nonadjacent batches across intervening painter content. Workbench
bitmap/shaped text goes through colored/UI geometry, not these textured batches:
its idle frame reuses the existing colored buffer; editing/layer animation still
regenerates and uploads colored geometry. Shaped text is currently raster pixel
quads rather than a GPU glyph atlas. This increment does not introduce such an atlas.

Two sequential release baseline and two modified native smoke runs per example;
Japanese workbench idle/editing scripts, Crystal Trail29 successful presentations,
Timber Harbor59 and workbench119. Renderer warm samples exclude indices0..9
(19/49/109 samples per run). Resources timing includes textured geometry and
GPU resource/upload calls; encode includes geometry/resources, so do not add them.
Modified acquisitions also enable bounded window timings for metadata; baseline
ones do not. Separate measurements and host variation prevent a universal speedup
claim, although eliminated repeated uploads and retained identities are exact.

| Warm workload | Resource CPU p95 before / after, microseconds | GPU pass p95 before / after, microseconds |
| --- | --- | --- |
| Crystal Trail | 89–118 /58–61 | 7–8 /8 |
| Timber Harbor | 4215–4359 /117–121 | 477–489 /35–36 |
| Workbench Japanese idle | 0–1 /0 | After30 |
| Workbench Japanese editing | 441–460 /430–438 | After29 |

Native warm modified texture uploads are zero in all three examples. Timber
Harbor upload spikes remain only when geometry changes; warmed median vertex
upload bytes fall from75840 to0. Timber Harbor collected49 warm GPU samples
per modified run versus27/29 before; these different sample populations are explicit.
Decoded byte counters describe RGBA payload, not allocated VRAM, staging or driver
memory. The renderer appends `textured_batches`, `uploaded_texture_bytes`,
`retained_textures` and `retained_texture_bytes` to its bounded CSV diagnostics.
The pre-existing `uploaded_vertex_bytes` now also excludes unchanged textured buffers.

The ignored renderer-domain native probe exercises1/32/1024 sprites sharing a
256x256RGBA texture,100 samples after10 warmups for unchanged and alternating-camera
frames. It asserts one batch with6 vertices per sprite, zero warm texture uploads,
three ordered A/B/A batches sharing two resources, fresh decoded-image replacement,
and empty-frame eviction. Frame construction/cloning and assertions are excluded
from timings; changed geometry generation and buffer creation are included.
Across two runs, p95 unchanged/dirty:0.1/9.7–10.6us at1sprite,
0.2/9.5–10.8us at32,3.6–4.5/94.4–176.3us at1024. The second1024-sprite dirty
run has a9.0152ms worst sample; it is retained, not treated as a stable cost.
This is CPU preparation, not GPU execution, unique-texture scaling or a game frame.
Unit tests separately assert resource-drop ownership and cloned/equal/reloaded identity
semantics, geometry/order/UV/tint/camera/extent invalidation, and existing batch tests
continue to own painter-order and sprite-region output equivalence.

Reproduce native resource assertions from the engine root:

```console
cargo test -p gridthorn_render --release --locked measure_native_texture_reuse_and_dirty_geometry -- --ignored --nocapture --test-threads=1
```

Evidence: `target/render-before-etw-*.log`, `target/render-after-*.log`,
`target/render-review-summary.csv`, `target/render-resource-probe-{1,2}.log`.
Disposition: the recorded sprite/text batching, dirty uploads and decoded-texture
lifetime review has a focused verified fix and concrete workload limits. Unique-asset
cache lookup is linear; populations substantially larger than the three game textures,
exact heap/VRAM peaks and persistent buffer reuse on changing geometry need their own
measurements if real games demonstrate a bottleneck. Native frame/display acceptance
is a separate gate described below.

## Native renderer/display disposition — 2026-10-04

A signed standalone [PresentMon2.6.0](https://github.com/GameTechDev/PresentMon/releases/tag/v2.6.0)
was downloaded to ignored target without installation. Signature: Intel Corporation,
valid; SHA256B2A706BC6AD475749E3B7E3409263AA1E6906D45BDCF993F6DBC0F660188F1AF.
No service, privilege-group, driver, monitor/DPI or power-plan changes. The named
ETW session GridthornRenderReview terminates at each timed capture. An initial
trial overlapped a compile and is excluded. Retained acquisitions run sequentially
without concurrent builds/tests, preserve missing/dropped rows, and use nearest-rank
percentiles. Process-name filtering missed both short Crystal Trail baseline runs;
subsequent game captures collect all processes and filter by the exact child PID.
Empty captures are not accepted as performance results.

[PresentMon's metric contract](https://github.com/GameTechDev/PresentMon/blob/v2.6.0/README-ConsoleApplication.md#csv-columns)
defines MsBetweenDisplayChange as the previous frame's displayed duration before
the next displayed present. NA/zero rows remain in raw evidence and are omitted
only from displayed-interval quantiles. MsGPUTime includes GPU waiting; MsGPUBusy
is process GPU activity rather than this renderer's pass alone. MsCPUBusy includes
work between presents and is not isolated engine CPU execution. Vulkan/DWM/HWS,
VRR and driver instrumentation constraints prevent treating these as exact hardware
scanout/heap or whole-engine timing guarantees. HWS/VRR state was not established.
Native recordings report Composed: Flip, sync interval1, no tearing.

The default presentation configuration requests maximum frame latency2 in the
current backend. Its short final matrix contains four locales x idle/editing/
windows/animation x two repeats,120 host frames each. It records119 renderer frames
per run; after excluding0..9 there are109 renderer samples. Some runs have display
p99 above33.33ms: English editing34.6872, Russian animation34.7051, Arabic windows
97.224 and Japanese animation125.0149ms. Renderer encode/host-present times alone
do not explain them; these observations are retained rather than silently waived.

An experiment requesting latency1 ran five scenarios twice and the complete32-run
matrix. All32 complete matrix captures have displayed p99 between8.4247 and16.1883ms.
The initial ten-scenario trial has one missing capture; it is excluded from complete
matrix counts. Games also pass native smoke at that setting: displayed p99
Crystal Trail8.6847–9.7076ms, Timber Harbor9.5912–11.0953ms, after ten captured rows.
However, reverse latency2 repeats of all four formerly problematic scenarios also
pass (displayed p99 range8.5241–16.4227ms). The comparison therefore establishes
neither causality nor a reliable presentation-policy benefit. The experimental
configuration was reverted completely; no queue-depth/present-mode change remains.

The workbench now offers --long-smoke with idle/editing/windows/animation workloads:
1200 host frames instead of120; window scripts repeat their120-frame action sequence.
The existing domain test runs two complete cycles and verifies layer order, editor
and locale preservation. Other native smoke modes reject the flag. Diagnostics
still cap at240 samples, so full tail analysis uses external ETW, not invented
renderer samples. The repeated long runs use the unchanged original presentation
configuration and all available locales across the previously problematic scenarios
plus Japanese idle as control. Ten captures each contain1197 ETW rows. Warm display
analysis excludes the first120 captured rows (roughly one action cycle) and filters
only NA/zero displayed values, leaving904–1077 displayed durations per run.

| Warm long workload, two runs | Display p95 range, ms | Display p99 range, ms | Worst duration, ms |
| --- | --- | --- | --- |
| English editing | 9.5494–13.8785 | 9.6896–16.6607 | 27.7584 |
| Russian animation | 12.4512–12.4920 | 13.6215–16.6282 | 25.3339 |
| Arabic windows | 9.5820–11.1177 | 9.7094–12.9056 | 16.2697 |
| Japanese animation | 9.4882–12.3712 | 9.6917–13.7091 | 22.2651 |
| Japanese idle | 11.1245–11.5904 | 12.6331–12.6377 | 16.6522 |

No warmed long displayed duration exceeds33.33ms. The earlier short-run display
outliers did not reproduce in these longer warmed confirmations; their cause
remains unassigned. Do not relabel them as a proven compositor defect or claim a
frame-pacing speedup. The renderer has no newly demonstrated persistent pacing
bottleneck on these recorded Windows/DPI1 workloads. Cold/native startup latency,
longer-duration environmental variation, nativeDPI2 and the complete nine-mode
interaction/clipboard/OSIME matrix remain in the milestone-wide acceptance gate.
This is renderer-review closure within explicit measured limits, not Milestone4.5
or Milestone4 closure and not maintainer visual/IME acceptance.

Reproduce after building the sibling release workbench against this engine:

```powershell
$env:CARGO_TARGET_DIR = (Resolve-Path './target').Path
$env:CARGO_BUILD_JOBS = '1'
cargo build --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_multilingual_workbench --release --locked
$env:GRIDTHORN_RENDER_PERFORMANCE = '1'
$env:GRIDTHORN_WINDOW_PERFORMANCE = '1'
$capture = Start-Process -FilePath './target/PresentMon-2.6.0-x64.exe' -ArgumentList @('--process_name','gridthorn_example_multilingual_workbench.exe','--output_file','target/render-long-display.csv','--timed','14','--terminate_after_timed','--no_console_stats','--session_name','GridthornRenderReview') -WindowStyle Hidden -PassThru
Start-Sleep -Milliseconds 800
& ./target/release/gridthorn_example_multilingual_workbench.exe --animation-smoke --locale=ja --long-smoke
$capture.WaitForExit()
```

Evidence: target/render-matrix-*, target/render-latency-{one,final,reverse,game}-*,
target/render-long-* and target/render-long-summary.csv. Quantile evidence also
contains sample counts, medians and maxima; no run was discarded for a slow result.

Final validation: full ./scripts/verify.ps1 passed (target/render-review-final-verify-4.log), including all tests and dependency boundaries. Affected example tests and all-target Clippy passed; three headless/native smokes and the final long animation smoke passed. Both GPU resource probes passed. See the checkpoint for scoped execution settings, retained failed diagnostics and compiler-cache disk recovery. Final formatting/whitespace checks passed; no dependency changes or commits.

## Runtime world input localization domain review (2026-10-04)

This disposition closes the Milestone 4.5 runtime/world/input/localization gate
for the workloads below and supersedes earlier statements that these four domain
reviews remain open. It does not close the milestone or its other domain gates.
The existing UI/text review and native workbench acceptance remain separate.

Engine starting revision: e2d81302d44e8cfb18faa8dc169704cffac8e4ec. Sibling
revision: c89adb9a5317007b3469782c1c8da9d8b4b1b04a, with pre-existing example
changes preserved. Rust 1.99.0 (b940084d7, 2026-09-28), x86_64-pc-windows-msvc,
Windows reference host recorded above, release profile, runtime diagnostics unset.
Balanced power scheme was read again; CPU metadata access was denied in the sandbox,
so the earlier Ryzen 5 5600X inventory is retained, not presented as a new reading.
Live clock/background load remain uncontrolled. Acquisitions were sequential with
no concurrent agent-launched builds/tests. All slow samples remain in the evidence.
These are warm-process CPU workloads, not native device latency or frame timings.

### Broader mixed components and scene churn

The new `measure_structural_world_churn` workload uses 1000/10000/100000 persistent
entities and 1/16/64 scene-owner partitions. Each partition initially holds
floor(count/10/partitions) entities. Domain components are Position(u64),
Velocity(u64) and Inventory([u64;8]); initial entities have Position plus either
Velocity or Inventory. Every cycle inserts/replaces Velocity on every persistent
entity and Inventory on every even-indexed entity, then traverses all three types.
Random access updates Position and reads Velocity in the permutation
(index*7919)%count. Each cycle removes and repopulates one scene partition.
Repopulation includes entity creation, scene-ID cloning, component insertion and
archetype transitions through public WorldAccess methods.

Two release runs record 51 individual cycles/configuration. Cycle 0 records the
first structural additions and initial cache/backend work separately; the table
uses the following 50 cycles. There is no timing threshold in CI. Population,
scene enumeration and correctness checks/output are outside each interval;
insertion/access success checks and spawn's returned-ID vector are included.
The regular small-fixture regression checks exact Position/Velocity continuation,
persistent survival, removed-ID rejection after slot reuse and final removal
counts. There is no public component-removal or general despawn API: this review
uses implemented scene deletion rather than backend-only access or a new API.

| Persistent entities | Insert/replace p95 us | Three traversals p95 us | Random access p95 us | Partition removal p95 us | Partition spawn p95 us |
| --- | ---: | ---: | ---: | ---: | ---: |
| 1000 | 46.9–74.4 | 8.6–14.5 | 18.1–28.4 | 4.1–30.6 | 1.1–14.3 |
| 10000 | 456.0–734.0 | 41.8–72.9 | 286.3–397.4 | 10.1–201.6 | 3.0–182.7 |
| 100000 | 4432.2–5498.1 | 527.3–840.5 | 4122.0–6849.0 | 114.1–2033.9 | 27.6–1483.7 |

Ranges span partitions and runs, not pooled percentiles. At 100000 entities,
removing the sole 10000-entity scene has p95 1981.8–2033.9 us; removing one of
64 partitions (156 entities each) has p95 114.1–225.4 us. Removal queries all
scene owners before collecting/despawning matching entities; it is not constant
cost per removed entity. Largest warmed random-access call is 9858.9 us.
Structural addition cycle 0 remains in the raw logs rather than being called a
warm replacement. No backend/query caching or scene index is justified by these
bounded results. Game code should use traversal for bulk work and budget structural
changes and scene replacement explicitly; independent phase p95 values cannot be
summed into a measured full-frame p95.

### Large input bursts and removal of queue copies

`measure_input_burst_phases` separates fixture cloning, InputBuffer ingestion,
snapshot, runtime resource publication/destruction of the preceding snapshot,
and empty timed-runtime dispatch. It uses 1024/16384/65536 events for pointer,
keyboard, Unicode commit, changing preedit and a round-robin mixture. Templates
are constructed before timing. Two runs/configuration contain one warmup plus
50 individual samples; complete order/content equality is checked outside timing.
Keyboard state alternates presses/releases of KeyD. Preedit cursor endpoints are
valid UTF-8 boundaries. A separate regular regression retains old snapshots across
4096 mixed events, an empty held-state frame and focus/preedit cancellation.

At 65536 events, InputEvent occupies 64 bytes on this target (4194304 bytes for
initialized queue elements). String payloads are 0/65536/3396762/1758362/1305166
bytes for pointer/keyboard/commit/preedit/mixed respectively. These are measured
fixture lengths and type sizes, not allocator totals, capacities or heap peaks.
The prior push cloned every event, and snapshot cloned the complete event vector
and payloads before clearing/dropping the original queue. Both copies are removed:
push inspects the borrowed event and queues the owned value; snapshot transfers
the queue, independently clones held/preedit state and clears edges. FocusLost is
queued before its generated CompositionCancelled, preserving observable ordering.

An intermediate queue-transfer version lost reusable capacity and increased
pointer-ingestion work. The final version reserves one replacement queue sized to
the preceding event count before transferring ownership. It retains no historical
high-water mark after an empty frame. This still allocates a replacement vector;
retained old snapshots still own their events and can consume caller-controlled
memory. Preedit and held-state ownership still require independent cloning. No
coalescing, truncation, event limit or public API change is introduced.

| 65536 events, phase | Before p95 us | Final p95 us |
| --- | ---: | ---: |
| Pointer snapshot | 1229.0–1413.2 | 21.2–23.9 |
| Keyboard snapshot | 5041.2–5564.9 | 28.4–32.4 |
| Unicode commit snapshot | 9410.5–11940.8 | 20.3–23.7 |
| Preedit snapshot | 7172.6–7208.9 | 27.5–34.0 |
| Mixed snapshot | 5050.4–9180.3 | 18.6–23.1 |
| Unicode commit ingestion | 5223.3–5228.1 | 1431.3–1435.1 |
| Keyboard ingestion | 7296.3–7956.8 | 4017.9–4372.9 |
| Preedit ingestion | 4734.0–4997.5 | 5093.1–5283.8 |

Preedit ingestion alone did not improve; its required current-preedit cloning and
replacement remain. Final publication/drop p95 for 65536 commit events is
3096.9–3308.0 us, and fixture cloning remains substantial. The fix eliminates
specific engine queue copies, not all input allocation or application work.

The existing complete runtime-bridge workload was also run twice before and twice
after. It includes per-event fixture cloning, lifecycle ingestion, snapshot,
InputState replacement/destruction and zero-tick dispatch; 32 batches of eight
frames follow eight warmup frames. Values below are mean us/frame across the
256 measured frames per run, not individual-frame percentiles.

| 16384 events/frame | Before mean us/frame | Final mean us/frame |
| --- | ---: | ---: |
| Pointer | 555.90–567.85 | 392.31–439.46 |
| Keyboard | 3168.39–3378.86 | 1578.39–1751.90 |
| Unicode commit | 3352.08–3687.07 | 1373.64–1503.17 |

The 32-event final means are 3.19–3.79 us pointer, 5.84–7.92 us keyboard and
3.96–7.14 us commit. Noise prevents a uniform small-burst speedup claim.
Large synthetic bursts are attributed and remeasured; no native OS delivery,
clipboard, per-device key-cardinality or UI-consumption latency is inferred.

### Runtime and localization confirmation

Repeated empty-runtime workloads retain 0/1/32 no-op systems per stage and
0/1/8 fixed ticks. Each run uses 100 batches of 1000 frames after a warm batch.
Zero-system zero-tick means are 2.38–2.60 us/frame; 32 systems/stage with eight
ticks has means 6.92–7.74 us/frame and p95 batch means 7.43–10.06 us.
Fixed-step counts are checked. No schedule/runtime optimization is justified;
no-op dispatch does not predict real system execution time.

Localization's existing warm four-locale workloads and fresh-service complex
publication workloads were each repeated twice. At 4096 messages, p95 batch-mean
formatting spans 0.13–0.25 us fallback, 0.49–0.63 us interpolation,
1.06–1.67 us NUMBER messages and 0.88–1.81 us plural/number messages.
Complex publication uses 20 individual fresh-service calls/configuration, with
15-reference chains, nested selectors/numbers and cyclic-candidate rejection.
Across four locales and both runs at 4096 messages, p95 source validation is
23.27–37.47 ms, candidate validation 24.32–34.63 ms, rejection 23.64–36.78 ms,
construction 0.56–1.16 ms, first format 57.1–102.7 us and replacement 1.61–2.35 ms.
The existing atomic-publication/old-output/error checks pass. Large complex source
validation exceeds a 16.67 ms frame and must remain off latency-sensitive polling;
publication itself also consumes shared presentation time. Existing bounded source,
message/expression/reference limits and explicit publication policy remain suitable.
No new localization optimization or automatic worker is justified by these samples.

### Four domain dispositions and limits

| Domain | Disposition | Supported review envelope and follow-up |
| --- | --- | --- |
| Runtime | Measured; preserve implementation | Empty dispatch through 32 systems/stage and eight ticks, plus prior populated/native extraction evidence. Real game systems, parallel scheduling and cross-platform timings need their own workload. |
| World/ECS | Measured; preserve implementation | Public mixed insertion/replacement, traversal, random access and scene create/delete through 100000 persistent entities plus about 10% scene entities and 64 partitions. Larger worlds, wide/heap-owning components, arbitrary component removal, allocator peaks and additional ownership/query patterns are unmeasured. |
| Input | Fixed and remeasured | Ordered bursts through 65536 events and full runtime publication through 16384 events. Queue copies removed; preedit/held state and caller-retained snapshots still cost memory. Native OS/clipboard latency, many distinct held keys, allocator peaks and larger sustained queues remain follow-ups. |
| Localization | Measured; preserve implementation and off-polling validation | Four locales, warm formatting and simple/complex 4096-message publication/rejection. Multi-locale simultaneous replacement, wider reference fanout, heap peaks and worker handoff need separate evidence. |

The comparison to the existing 16.67 ms frame reference is contextual, not a new
promise that every maximum workload fits together in one frame. These domain
limits are explicit evidence-based follow-ups, not unfinished structural-churn or
input-copy attribution work. No dependency edge, release profile, authoritative
ordering or public compatibility contract changes. Timing probes stay opt-in;
regular CI checks semantics rather than unstable performance thresholds.

Reproduction (run sequentially, twice, in release with runtime diagnostics unset):

```powershell
cargo test -p gridthorn_world --release --locked measure_structural_world_churn -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_app --release --locked measure_input_burst_phases -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_app --release --locked measure_input_publication_scaling -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_app --release --locked measure_empty_runtime_and_schedule_dispatch -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_localization --release --locked measure_localization_catalog_and_formatting_scaling -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_localization --release --locked measure_complex_localization_publication -- --ignored --nocapture --test-threads=1
```

Raw evidence: target/input-phase-{before,final}-{1,2}.log,
target/input-total-{before,final}-{1,2}.log, target/structural-world-final-{1,2}.log,
target/runtime-closure-{1,2}.log, target/localization-{warm,complex}-closure-{1,2}.log.
Intermediate queue-transfer/reservation experiments remain in input-*-after-* and
input-*-reserved-* logs. Summary CSVs retain per-run median/p95/max or batch-mean
statistics; nearest-rank p99 of 50 or 20 individual samples equals the maximum.
Correctness/verification and native smoke results are recorded in the checkpoint.

Final validation for this domain closure: full ./scripts/verify.ps1 passed
(target/runtime-domain-full-verify-3.log), including all workspace tests, CLI
end-to-end, all-target Clippy and dependency boundaries. The affected sibling
workbench passes 14 regular tests, release build, headless validation and native
Japanese editing/preedit smokes. The latter use injected events at native DPI 1;
no new manual OS IME/clipboard or native-latency claim is made. Earlier failed
verification attempts and the corrected style/whitespace diagnostics remain in
the checkpoint. Final formatting and diff whitespace checks pass.

## Assets/scenes/saves attribution and disposition — 2026-10-04

This closes the asset/scene/save performance gate within the envelopes below,
superseding the attribution follow-ups in the earlier warm-I/O section. Baseline
43991ad7647f95468c6bff7c0496c8f805cd35aa, clean engine tree at start; same Windows
Ryzen 5 5600X host and Rust 1.99.0. Audio/platform and other milestone gates remain
separate. No public signatures, formats, durability promises or dependencies in
the normal facade closure change. DHAT 0.3.3 is a dev dependency of the three
domain test executables. CPU runs retain the inactive test allocator wrapper;
absolute production costs may differ, while before/after comparisons use the
same wrapper. Its transitive serde_json dependency requires explicit
usize types in five existing empty UI assertions; their behavior is unchanged.

### Acquisition and reproducibility

Two sequential release CPU runs before/after each affected domain; scenes have
two runs with unchanged production behavior. Each configuration runs 21 cycles:
sample 0 is reported separately, samples 1–20 give median and nearest-rank p95;
p99 equals max at this sample count. Each asset cycle registers a new service;
each save cycle initializes a target root; scenes prepare a new load boundary.
No concurrent builds/checks during timing. OS cache, storage activity, antivirus,
CPU clocks and power conditions are uncontrolled. Fixtures are written before
loading: **cold means first service/call or fresh process, not verified physical
cold disk**. No cache flush, WPR/registry modification or power override was used.
Device-cold/network-storage measurements require a deployment workload; this
review does not claim them or hard frame/memory budgets.

```console
cargo test -p gridthorn_assets --release --locked measure_asset_cold_branching_errors -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_scene --release --locked measure_scene_cold_errors_and_phases -- --ignored --nocapture --test-threads=1
cargo test -p gridthorn_app --release --locked measure_save_cold_errors_and_phases -- --ignored --nocapture --test-threads=1
```

Heap runs are separate from CPU runs. Set GRIDTHORN_IO_HEAP=phase or workflow;
the probe then performs one cycle. GRIDTHORN_IO_SIZE selects one size; assets
also accept GRIDTHORN_IO_KIND=raw|ppm|png and GRIDTHORN_IO_GRAPH=diamond|fanout|disconnected.
Two final heap runs per selected configuration reproduce allocation counts/peaks;
the before-fix raw/save heap runs have one acquisition each. DHAT wraps the system
allocator only in cfg(test); profiling elapsed time is excluded from CPU tables.
[HeapStats](https://docs.rs/dhat/0.3.3/dhat/struct.HeapStats.html) reports requested
allocation bytes, total blocks/bytes, maximum live bytes and live bytes at return.
Phase windows exclude pre-existing inputs, while retained output is included.
Workflow windows include setup, existing snapshots held by the harness, isolated
codec/serialization outputs, error fixtures and cleanup. They are larger than a
minimal application workflow; allocator overhead, stacks, GPU/file cache and
allocator-internal profiler storage are not Rust requested bytes. Phase peaks
cannot be added or subtracted from independently timed end-to-end calls.

Eight additional fresh-process runs launch the compiled test executable directly,
one selected configuration and GRIDTHORN_IO_SAMPLES=1 each. GRIDTHORN_IO_HOLD_MS=500
holds before setup and after cleanup. A hidden Start-Process monitor refreshes
every 10 ms, records the pre-work baseline, sampled maximum PrivateMemorySize64
and OS cumulative PeakWorkingSet64, then checks exit 0. These unprofiled runs are
excluded from CPU percentiles. Short private-memory peaks may be missed; process
pages/allocator retention are distinct from requested heap. The executable names,
baselines, peaks and reproduction monitor are retained in target/io-process-after.csv
and target/io-process-monitor.ps1. The following first-call spans include monitoring
perturbation and two observations only:

| Selected fresh process | First-call duration ms |
| --- | --- |
| Register 512 x 64 KiB raw, fanout | 62.640–119.602 |
| Register sixteen 256x256 PNG, diamond | 5.991–7.644 |
| Capture / serialize / parse 10000 mixed entities | 13.896–17.174 / 77.606–90.113 / 116.022–123.400 |
| Save / load document, 262144 values | 22.895–23.869 / 7.273–8.848 |
| Save / load file, same root | 26.672–28.972 / 7.923–8.724 |

These are first calls within the complete configured workflow. Save file load
follows document load and validation; isolated operations can warm code/data.
They are not independent process-startup-to-load or untouched-storage benchmarks.

### Assets: share unchanged source snapshots; keep all-file polling explicit

Raw fixtures have 64/512 files of 65536 bytes; PPM/PNG fixtures have sixteen
256x256 uniform-color images (compressible PNG, not a realistic art corpus).
Diamond edges target the next two IDs, fanout targets the final
leaf, disconnected fixtures group 32 IDs around separate leaves. Editing the last
leaf invalidates the full connected graph or only its disconnected group. The
expected dependency-first/lexical order is asserted exactly. Missing final files
after another edit reject both synchronous and worker batches without publishing;
invalid images reject decoding; restoration retries successfully. Held source
snapshots remain immutable, duplicate requests are rejected and workers shut down.

Isolated read_all retains file buffers; decode_all uses the actual texture decoder
on those buffers. Registration includes read/decode/storage; graph validation is
separate. sync_scan includes real reads/comparison/invalidation/decode/commit.
worker_prepare includes worker scheduling, scan, transport and up to 1 ms polling
sleep. Publication uses the same private method as poll with an already-ready
reply, including release of the prior store; it excludes waiting/read/decode.
Measurements of isolated phases are attribution examples, not additive accounting.

Previously a scan retained every read Vec and copied all affected source bytes,
including unchanged dependents. It now releases equal read buffers immediately,
shares committed Arc bytes, and prepares only genuinely changed source allocations.
Affected textures still re-decode; commit/rollback and ordering remain atomic.
The new regression checks allocation identity for raw dependents and immutability
of changed old bytes; existing late-decode/read rollback and worker tests remain.

| 512 x 64 KiB raw, changed fanout | Before median ms | After median ms | Before p95 ms | After p95 ms |
| --- | --- | --- | --- | --- |
| Synchronous scan | 54.205–55.351 | 36.268–40.800 | 58.936–60.499 | 39.761–55.779 |
| Ready publication | 0.483–0.525 | 0.124–0.152 | 4.209–4.376 | 0.149–0.287 |

Final diamond scan p95 is 41.863–51.219 ms; disconnected scan 42.060–52.859 ms
despite only 32 affected IDs: all files are still read. Fanout worker preparation
p95 is 37.903–55.576 ms, with 65.208 ms maximum. Final sixteen-image fanout PNG
scan p95 is 2.393–2.508 ms; PPM 17.109–29.019 ms, maximum 39.472 ms. Publication
p95 is 0.034–0.039 ms for PNG and 0.552–0.593 ms for PPM. No uniform codec or
publication speedup is claimed. At 512 raw sources the isolated read_all median
is tens of milliseconds; moving work to the worker does not reduce storage work.

| New requested bytes in selected phase | Before peak / retained | After peak / retained |
| --- | --- | --- |
| Raw changed synchronous scan | 67259136 / 33582080 | 209564 / 85008 |
| Raw worker preparation | 67259392 / 33823532 | 326460 / 326460 |
| Raw missing-file rejection | 33543456 / 178 | 178046 / 178 |
| PNG diamond decode_all | — | 4477676 / 4194944 |
| PNG diamond changed scan | — | 4483982 / 4197096 |
| Ready publication | 0 / 0 | 0 / 0 |

Raw scan total allocated bytes fall 67610472 → 34106008; every file is still read.
Publication allocates nothing in these windows but can free pre-existing data.
The raw whole-workflow peak falls 135296385 → 67537738–67537742 bytes (129.03 →
64.41 MiB); PNG workflow peak is 12941469 bytes (12.34 MiB). Registration still
retains the full source set. Disposition: measured snapshot duplication is fixed;
keep background scanning for larger sets and frame-boundary publication. Native
watching, custom derived loaders, larger textures/graphs and concurrent external
writes remain outside this supported performance envelope, not assumed cheap.

### Scenes: TOML dominates; use an explicit load boundary

1000/10000 scene-owned entities have registered positive-u64 Health and a
multilingual String Label; a persistent entity and global resource survive.
The 10000-entity document is 2907967 UTF-8 bytes. capture, serializer-only
toml::to_string_pretty, validated encoding, parsing, prepare and commit are separate.
Caller std::fs write/read is buffered, not engine durable scene I/O; the scene
service has no file operations. Invalid engine metadata, last-entity constructor,
unknown final type and malformed tail TOML reject without altering captured live
state. Commit includes removal/insertion, not presentation-resource reconstruction.

| 10000 mixed entities | Median ms | p95 ms | New phase peak bytes |
| --- | --- | --- | --- |
| Capture | 12.016–12.678 | 15.356–15.643 | 15892167 |
| Serializer only | 67.978–70.216 | 78.752–80.940 | 25839975 |
| Validated encode | 70.588–74.153 | 88.077–99.847 | 25839975 |
| Caller buffered write / read | 1.187–1.307 / 7.776–7.994 | 1.518–1.554 / 8.403–9.279 | 138 / 2908105 |
| Parse | 90.364–94.355 | 95.279–103.851 | 133019208 |
| Prepare | 10.741–10.900 | 11.905–12.654 | 1339101 |
| Commit | 4.553–5.005 | 5.101–5.767 | 1089076 |

Early engine rejection p95 is 0.003–0.004 ms. Late constructor/type rejection
p95 is 11.834–13.259 ms. Malformed-tail parse rejection p95 is 84.653–88.961 ms
and its new heap peak is 135927176 bytes, slightly larger than successful parse;
returned error retains only 200 newly allocated bytes. Successful parse allocates
173939289 total bytes and retains 15761873 bytes. Workflow peak is 215822079 bytes
(205.82 MiB), including multiple captured/parsed/error documents deliberately held
by the harness. Disposition: no registry/ECS rewrite is justified by this split;
TOML parsing/serialization is the main cost and cannot fit a 16.67 ms frame here.
Keep scalar format and explicit prepare/commit boundaries; richer schemas, huge
registries and game loading screens need their own workloads. No async loader,
format migration or hostile-input memory budget is introduced by this review.

### Saves: remove root copies; retain durable replacement and game-owned codec

Roots have 1024/262144 u64 values, 4096 pending commands and 257 named RNG streams.
The large canonical document is 1759736 bytes. The representative game codec
creates per-number Strings and joins comma-separated values; this is explicitly
game-owned work. Snapshot cloning, isolated codec encode/decode, TOML envelope,
actual create-new/write/sync/rename and file reads are attributed separately.
TimedCodec also observes codec work inside file save/load. Exact canonical state,
commands/RNG metadata and file bytes are checked. Errors cover incompatible
metadata before decode, decoder/encoder rejection, missing path, replacement
onto a directory with temporary cleanup, invalid UTF-8 and 16 MiB+1 rejection.

Load previously cloned live state for metadata and cloned decoded state during
borrowed restoration. It now checks required-state presence/identity without a
clone and consumes the validated snapshot internally. Public borrowed restore
still clones independently. A clone-count regression covers success and metadata
failure; regular continuation, compatibility, shutdown and rollback tests pass.

| Large root operation after fix | Median ms | p95 ms |
| --- | --- | --- |
| Game codec encode / decode | 17.274–17.349 / 4.030–4.037 | 18.268–19.557 / 4.399–4.587 |
| Envelope encode / parse | 4.387–4.455 / 2.729–2.759 | 4.968–5.412 / 3.210–3.240 |
| Durable replacement / isolated file read | 3.572–3.594 / 5.706–6.030 | 3.910–4.013 / 6.453–6.968 |
| Save document / file | 22.224–22.526 / 25.726–26.030 | 23.339–24.054 / 28.061–55.366 |
| Load document / file | 7.051–7.226 / 8.060–8.309 | 7.999–8.504 / 8.771–10.073 |

Before load-document median is 7.585–7.978 ms and file-load 8.432–8.930 ms:
the improvement is modest; no save-side speedup is claimed. File-save maximum
137.875 ms and replacement maximum 24.954 ms show synchronous I/O tails; durability
is preserved. Isolated phases run at different points/cache states and do not sum
to total save/load time. Game codec encode dominates the representative save.

Load-document total allocated bytes fall 21404784 → 17158475; file-load
25649561 → 21352002. New peak bytes remain 10985280 / 13082432 respectively:
envelope parsing/read storage sets the peak, not the removed root copies.
Snapshot clone requests 2147678 bytes; codec encode peak 9509875, envelope encode
7051523, save-document peak 12666818. Durable replacement itself peaks at 438 new
bytes. Incompatible-metadata rejection peaks at 10985304 bytes because parsing
precedes compatibility checks. Oversized file rejection p95 is 18.596–19.921 ms,
reads no more than 16 MiB+1 but Vec growth peaks at 33554462 new requested bytes.
The limit is a consumed-byte bound, not a heap-capacity bound. Whole-workflow
peak remains 61583811 bytes after versus 61583817 before (58.73 MiB): the retained
fixtures/oversized-file exercise dominates. No peak-memory reduction is claimed.
Disposition: unnecessary root copies are fixed; coherent save capture still clones,
codec ownership remains with the game, and synchronous loads/replacement remain
explicit between-tick boundaries. More complex roots, physical cold/network
storage and non-Windows replacement/durability need deployment validation.

### Whole-process observations and gate result

| Final selected workflow, two fresh processes | Sampled private peak MiB | Peak working set MiB |
| --- | --- | --- |
| Raw 512 fanout | 47.34–58.43 | 69.13–69.18 |
| PNG 16 diamond | 9.29–13.60 | 17.49–17.51 |
| Scene mixed 10000 | 169.64–179.22 | 199.79–199.86 |
| Save 262144 | 38.54–43.90 | 61.02–61.02 |

Pre-work private baselines are 0.73–1.25 MiB, resident 4.39–6.68 MiB; full baseline
bytes are in the CSV. No idle-subtracted memory savings or before/after process
comparison is inferred. All eight monitored processes and all release probes exit
0. Logs: target/io-{asset,save}-before-{1,2}.log and -after-{1,2}.log,
io-scene-{1,2}.log, io-{asset,save}-heap-before-1.log,
io-{asset,save}-workflow-before.log, io-{asset,save}-{phase,workflow}-after-{1,2}.log,
io-{texture,scene}-{phase,workflow}-{1,2}.log, io-process-*.stdout/stderr.log;
CPU summaries in io-closure-summary.csv. Acquisition/analysis helpers stay in
ignored target output; domain-owned ignored probes remain reproducible source.

The three dispositions close this requested review: measured costs have explicit
ownership, error/rollback behavior is exercised, justified fixes have repeated
before/after evidence, and remaining deployment limits are documented above.
There are no timing assertions in CI and no claim of full Milestone 4.5 completion.

Validation: full ./scripts/verify.ps1 exit 0, including formatting, workspace
check/Clippy/tests, CLI generated-project end-to-end and dependency boundaries
(target/io-closure-full-verify.log). All three public sibling examples exit 0:
asset-reload --smoke verifies publication/rollback/recovery/shutdown,
scene-serialization verifies reconstruction/rollback/migration, world-saving
verifies file replacement and exact continuation to tick 100. Logs are
target/io-closure-{asset-smoke,scene-example,save-example}.log. Normal facade
cargo tree --edges normal excludes DHAT/backtrace/serde_json. Sibling changes
present at start are preserved; no example sources or locks are modified here.

## Native audio and Windows lifecycle disposition — 2026-10-05

Baseline: engine 80e131cab31791c827136b24c2bd42818c194010. This closes the remaining
audio/platform review for the implemented subset with the limits below; it does
not implement Milestone 5 device/power integration. New ignored probes belong to
audio/output/test and app/window/application/test. No dependencies or production
behavior change. The output rustdoc now distinguishes shutdown signalling from
asynchronous native stream release.

### Reproduction and measured ownership

Run alone, without other benchmarks/builds:

```powershell
cargo test -p gridthorn_audio --features native-output --release --locked measure_native_output_lifecycle -- --ignored --nocapture
cargo test -p gridthorn_audio --features native-output --release --locked measure_native_output_release -- --ignored --nocapture
cargo test -p gridthorn_audio --features native-output --release --locked measure_native_loopback_latency -- --ignored --nocapture
cargo test -p gridthorn_app --release --locked measure_native_window_shutdown -- --ignored --nocapture
```

The audio lifecycle probe defaults to 120 cycles and 1 voice. Set process-local
GRIDTHORN_AUDIO_PROBE_CYCLES and GRIDTHORN_AUDIO_PROBE_VOICES to reproduce the
recorded 1/16-voice, 22-cycle and 64-voice, 360-cycle acquisitions, twice per configuration
in fresh processes. Native window probe is Windows-only; audio native probes
require an available device and are excluded from ordinary automated tests.
Failures are explicit, not silently classified as successful headless playback.

Reference: existing Windows/Rust release host; default Speakers (HyperX Cloud
Stinger Core Wireless + 7.1), stereo F32, 48000 Hz, supported buffer 480 frames.
480 frames / 48000 Hz gives a nominal 10 ms buffer duration, not measured acoustic
latency. Kira's existing default backend/configuration and 10 ms control tweens are
unchanged. CPAL output callback timestamps are not exposed by this adapter.

One 80000-frame stereo PCM16 clip at 8000 Hz is decoded outside timings; looping
voices clone that same 10-second clip. Each cycle measures batch process/convert/
submit, then polls until every voice publishes positive playback position,
Paused and Playing respectively. Polls sleep1ms, so these are observed control
completion delays including scheduling/publication, not exact callback timestamps.
Progress timing includes the process call and its diagnostic print. After pause,
two25ms observations require stable position; state and position are not published
atomically. Every cycle stops its looping voices, waits for a separate800frame
natural completion and processes an empty batch to assert no controlled handles
or commands remain. A 100 ms rest and diagnostic drain follow. All PCM/gain values
are zero: real stream/mixer/resampling run, but sound quality and audible output
are not assessed. Final drop holds one actively progressing looping voice.

Two excluded warm-up cycles per process; retained phase counts20 at1/16voices,
358 at64voices. Upper-middle median and nearest-rank p95/p99. Per-cycle stdout
and backend diagnostics affect scheduling; there is no before/after speedup claim.
Six processes exit 0: short runs6.88–6.89s,64voice runs108.42–108.47s test time
(108.54–108.59s launcher observations). No reported or discarded stream errors
in these acquisitions. Logs target/audio-native-{1,16,64}-{1,2}.stdout/stderr.log;
raw/summary CSVs target/audio-native-{summary,memory,memory-summary}.csv.
The ignored target/audio-native-acquire.ps1 samples OS process private/resident
memory every250ms; it is an acquisition helper, not committed automation.

| Phase | 1 voice median / p95 range | 16 voices median / p95 range | 64 voices median / p95 range |
| --- | --- | --- | --- |
| Batch process/submit | 0.300–0.302 / 0.331–0.340ms | 0.359–0.361 / 0.419–0.423ms | 0.578–0.632 / 0.735–0.807ms |
| Observed mixer progress | 16.744–17.026 / 18.271–19.448ms | 16.832–17.276 / 17.413–18.693ms | 16.527–16.675 / 18.533–18.570ms |
| Pause completion | 9.189–9.910 / 10.678–10.695ms | 10.392–10.511 / 10.693–10.753ms | 10.673–10.685 / 10.859–10.994ms |
| Resume completion | 9.151–9.179 / 10.199–10.254ms | 9.187–9.201 / 10.153–10.728ms | 9.182–9.188 / 10.725–10.731ms |

Fresh output initialization21.466–38.416ms across six observations. At64voices
submit p99 is0.816–0.910ms, worst1.148ms; observed progress worst20.208ms,
pause12.265ms, resume11.739ms. These recorded envelopes are not universal device
guarantees or simulation/frame-thread budgets; output should remain outside the
frame's synchronous large-clip decoding/conversion path as in classic_2d.

Backend callback CPU fractions measure renderer.process wall time divided by
the audio buffer duration, excluding on_start_processing and other driver work.
Collected counts601–605 at1/16voices and10776–10778 at64voices mix active,
transition and idle callbacks; they are not a steady64voice percentile.
At64voices p95 fractions0.05772–0.05774, p990.07170–0.07232, maxima
0.09499–0.10146: all retained measurements below the available buffer duration.
The 100-entry CPU ring may omit samples and has no drop counter, so no full-stream
deadline, underrun, scheduling or acoustic guarantee follows from this observation.

After excluding the first1s, each64voice process has406 memory observations.
Private peaks6340608/6287360bytes, resident15147008/15130624bytes. First versus
last10% sample means: private5255373→5324595 and5218304→5203149bytes;
resident14941594→15108710 and14904730→15069184bytes. Private memory is near a
plateau despite23040 looping submissions per process; handle cleanup is asserted
each cycle. Small resident increases are not attributed to a leak or savings.
This is repeated-output process memory over108seconds, including allocator,
test harness, native mixer and driver mappings. It does not isolate heap peaks,
kernel/device buffers or prove stability over hours, unique large clips or device
replacement. The prior mock conversion/retention regressions remain applicable.

### WASAPI software loopback latency

The additional Windows-only probe uses the existing CPAL backend's output-device
input mode (WASAPI loopback), not a microphone or new engine capture API. It emits
a 100 ms, 1000 Hz stereo PCM tone at 8000 Hz, peak3000/32768 and gain0.05. The native
48 kHz stereo F32 loopback callback searches 96-frame windows for coherent 1000 Hz
energy, with magnitude above 0.000025 and above 0.65 times window RMS. Callback
processing stores only the first monotonic detection time; no captured samples,
recordings or other output content are saved. Quiet-output preconditions check
250ms before every Play; the maintainer stopped other output for acquisition.
Silence, DC, another tone and non-finite detector regression cases are tested.

Origin timestamps surround the actual engine batch process/submit and loopback
detection callback. Thus latency includes mixer scheduling, resampling, Windows
output/loopback buffering, callback dispatch and the detector itself. It is an
observed software-loopback delivery delay, not sample-accurate playback timestamps,
an isolated hardware latency or acoustic/headset wireless delay. An overlapping
1000Hz source could contaminate a timed window; the probe requires quiet output
and does not claim to identify arbitrary media. No master volume/device settings
were changed, and no captured audio is retained.

Pilot recordings failed the pre-play guard; the initial absolute-only detector
also accepted broad interference, so a coherence/RMS condition was added instead
of treating false detections as near-zero latency. Both retained processes report
one capture startup underrun/overrun before the one-second warmup finishes; this
is printed, then cleared before measurement. No capture errors occur in measured
cycles. This startup discontinuity is an explicit capture limitation, not a
silent zero-error claim or an error in the earlier output-only acquisitions.

Two processes exit 0 with22iterations each, first two excluded. Twenty retained
observations per run: median28.4126/28.5029ms, p9528.7674/29.6629ms,
worst29.5991/30.2202ms. Each iteration observes natural completion/handle cleanup
and waits100ms after completion before the next pre-play quiet check. Logs
target/audio-loopback-filtered-{1,2}.log and audio-loopback-summary.csv; failed
pilots remain in audio-loopback-1.log and audio-loopback-quiet-1.log. This provides
native software-path latency evidence without claiming acoustic or unique-device
universality. Final guard/feature/regression confirmation is in the checkpoint.

One subsequent confirmation timed out before first tone detection; its log is
target/audio-loopback-final-1.log. No output failure or physical latency can be
deduced from that detector timeout. Added failure diagnostics distinguish voice
state/position and captured tone magnitude from missing detection. Two final
confirmations pass: median 28.6840/28.8856 ms, p95 29.3851/29.3356 ms, worst
30.1191/29.3521 ms. Logs target/audio-loopback-diagnostic{,-2}.log and
audio-loopback-final-summary.csv. The final detector scans every complete window
to retain peak magnitude for errors rather than stopping on the first match.
The intermittent timeout's cause is not established; capture reproducibility is
an explicit follow-up, and successful samples do not establish a universal or
failure-free latency budget. All final capture processes still report the
excluded startup discontinuity; it is not hidden by their successful exit codes.

### Deferred release, manual sleep and native window shutdown

The release probe submits a silent looping sound through the same manager and
keeps only a Weak reference to converted frames. After real mixer progress it
drops OutputBackend and polls until frames are no longer owned by the native
renderer. A surviving sound handle cannot retain those frames. Two fresh-process
runs of22iterations, two excluded, give drop-signal median2us (p953.4/2.4us),
frame-release median487.528/486.465ms, p95490.785/491.189ms, worst491.966ms.
Logs target/audio-native-release-{1,2}.log and release-summary.csv. Source
attribution: backend stop sets a flag; its detached stream-manager thread checks
every500ms. Drop returning does not join that thread. Frame ownership release
is observable cleanup evidence, not an exclusive-device-release timestamp or
upper bound under stalled scheduling. Calling stop immediately before dropping
likewise does not synchronously acknowledge audible silence. Do not promise
instant device switching or bounded synchronous native teardown.

The maintainer performed and confirmed a Windows sleep/wake cycle while the same
probe process held one Paused voice. The manual gate is optional: set
GRIDTHORN_AUDIO_RESUME_GATE to an absent file path, wait for manual_sleep_ready,
sleep/wake Windows, then create that file. No timer is interpreted as approval or
proof of a power event. The recorded process retained the exact paused position,
resumed Playing in4.536ms, confirmed subsequent position progress and exited0.
Its44.09s total includes manual waiting; it is not OS resume latency. Logs
target/audio-manual-sleep.{stdout,stderr}.log. The OS event query did not
corroborate the precise interval; sleep is maintainer-reported manual evidence.
This tests explicit audio suspend/resume across that cycle, not automatic hooks,
device removal/replacement, audio quality or recovery from a broken stream.

Three fresh-process native window probes create a 1000×800 physical-pixel window with real GPU
presentation, run 120 idle callbacks, request normal event-loop exit, execute 1024
shutdown systems exactly once despite a second shutdown call, then release the
window/renderer/runtime. Event-loop/window/GPU initialization476.488–571.693ms;
shutdown schedule including idempotence check50.3–56.3us; finish plus resource
drop38.583–42.347ms. Logs target/platform-native-{1,2,3}.log, all exit 0. Drop
includes native window/GPU/driver operations, not physical GPU-memory reclamation;
event-loop consumption occurs before the finish timer. This extends the earlier
synthetic callback timings without claiming a percentile from three samples.

### Domain conclusions and concrete follow-ups

Audio: measured and acceptable for the recorded bounded silent-output workload;
no new production fix is justified. Existing repeated-clip conversion reuse and
completed-handle cleanup have regression/before-after evidence. The example's
32-slot lossy effect queue and reliable coalesced pause mailbox remain bounded,
with progress required and no delivery-latency guarantee. Prior headless worker
timings must not be added to these native timings to invent end-to-end latency.
Follow-up with suitable hardware: acoustic/loopback request-to-output latency,
audible multi-voice quality, hours-long unique-clip playback, memory during device
loss/change and output switching. Those are explicit deployment limits.

Software loopback provides native request-to-detection latency for this endpoint;
hardware/acoustic latency remains a distinct follow-up. The capture startup
discontinuity does not justify adding a general capture/recovery subsystem here.

Platform lifecycle: native orderly shutdown and maintainer-reported explicit
audio sleep/wake are measured; automatic Windows power-to-runtime/audio integration
is not implemented. The locked winit Windows backend emits Resumed at initialization
but does not map power broadcasts to Suspended/Resumed. Runtime hooks reset time
and cancel input only when invoked; audio is caller-owned. Sleep elapsed time
must not be assumed to reset automatically through those hooks. Milestone 5 owns
power-event integration, timer/input coordination, device-error reporting and
recovery, repeated sleep/device-change tests and cross-platform coverage. The
500ms asynchronous native teardown is a recorded backend limit for that work,
not a requirement to add an unrelated worker/backend subsystem in Milestone 4.5.

The review disposition is closed within these envelopes and explicit follow-ups;
Milestone 4.5 and its other gates remain open. Final ./scripts/verify.ps1 exits 0
(target/audio-platform-full-verify-final.log): formatting, workspace check/Clippy/
tests, generated-project CLI end-to-end, dependency boundaries and whitespace.
Explicit native-feature Clippy passes; audio tests: 13 passed, 5 manual probes
ignored, 0 failed. Public classic_2d native/headless smokes exit 0 without audio
failure diagnostics, and its saturated/idle pause-worker regressions pass (2 tests,
1 manual probe ignored). Logs target/audio-platform-feature-{clippy,tests}-final.log
and audio-platform-classic-{native-smoke,headless-smoke,audio-tests}.log. No sibling
sources or manifests were changed. Exact intermediate failures and acquisition
details remain in the checkpoint and ignored target logs.
