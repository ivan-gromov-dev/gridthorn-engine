# Milestone 4.5 performance review

Started on 2026-10-03 after the maintainer reported poor native performance in
the multilingual workbench, including after example-side layout reuse. This is
an engine-wide review of implemented capabilities, with measured, focused fixes.
Milestone 4 remains open until this review and its own native acceptance finish.

The first increment adds a public-API CPU workload to the sibling workbench and
records debug/release samples and confirmed duplicate paint in
[the active checkpoint](work-in-progress/milestone-4-5.md). Reference hardware,
display/power metadata and whole-engine CPU/GPU/present acceptance remain outstanding.

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

`GRIDTHORN_UI_PERFORMANCE` enables router-owned layout diagnostics. Up to 240
successful layout calls are stored; `ui_layout` rows print when the final cloned
router releases the shared collector. Arrangement includes layer ordering;
text geometry and paint are separate phases. Focused-decoration time is nested
within paint and includes focused field geometry, selection/caret and preedit.
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

`GRIDTHORN_WINDOW_PERFORMANCE` records up to 240 preparation and 240 redraw
callbacks, independently, and prints `window_cpu,phase,sample,elapsed_us` at
shutdown. Preparation includes lifecycle idle (runtime schedules), render-frame
extraction, platform control application and redraw request. Redraw measures the
renderer call. These are wall-clock CPU callback durations, including any blocking
inside those calls, not thread CPU usage. They exclude event-loop waiting, native
input callbacks, initialization and shutdown. Phase indices are independent and
must not be treated as paired frame IDs or added as a whole-frame percentile.
Disabled collection takes no timestamps and stores no samples. Native sample
counts do not establish actual displayed-frame timing.

A preliminary Windows debug probe measured one workbench router layout at about
105 ms at DPI 1 and 119 ms at DPI 2; bitmap layout took about 0.2 ms. Empty routing
took 0.03–0.06 ms. These are diagnostic samples, not release baselines or measured
native frame times. The example now reuses unchanged layout, but the maintainer
still observes poor performance. The remaining CPU/GPU split is unmeasured.

Code inspection identified repeated text preparation, painting before hidden-layer
filtering, and raster text represented as many colored spans/rectangles. Their
relative costs must be measured before choosing an implementation. An atlas,
layout cache or retained GPU buffer is a candidate, not a committed design.

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
