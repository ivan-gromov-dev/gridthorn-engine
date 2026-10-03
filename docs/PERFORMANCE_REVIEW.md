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
