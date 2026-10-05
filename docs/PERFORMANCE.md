# Performance and workload limits

Milestone 4.5 is complete for the measured Windows release subset as of
2026-10-05. Measurements describe specific workloads, not API limits or universal
frame deadlines. Engine/game APIs remain provisional.

## Reference conditions

Windows x86_64 MSVC, Rust 1.99.0, Ryzen 5 5600X, RTX 3070/Vulkan, Fifo,
1920×1080/144 Hz display, native DPI 1 and Balanced power plan. Background load,
thermals and live clocks were not controlled. Fresh-process/service and empty
build-target runs do not imply physical-cold storage or uncached downloads.
Synthetic DPI 2 was measured; native DPI-2 quantitative timing was not.

The workbench target is p95 engine CPU/GPU each ≤16.67 ms and warmed p99 displayed
intervals ≤33.33 ms, four locales and 1000×800 logical pixels at DPI 1/2.
Windows native/IME/clipboard and manual DPI acceptance is complete; precise
whole-process CPU and the complete target matrix are unverified. GPU-pass timing
excludes transfers/display; callback wall time is not whole-process CPU.
Intermittent scrolling/long-Japanese display gaps remain. Linux/macOS performance,
physical audio latency, hours-long stability and device-loss recovery are unverified.

## Workload envelopes

The following final-run observations must not be summed into a frame budget.
Games own work admission, content size, history retention and release scheduling.

| Domain | Measured subset and practical limit |
| --- | --- |
| Runtime/world/input | Up to 32 systems/stage, eight catch-up ticks, 100000 persistent entities and 16384-event bursts. Counts do not bound CPU or payload memory. |
| Text/UI/localization | Four scripts/locales, synthetic DPI 1/2, up to 1024 unique fields and 64 overlapping layers. Japanese 1024-field layout p95 is 83–100 ms at DPI 1 and 105–111 ms at DPI 2. Reuse unchanged layouts; no virtualization/occlusion culling is supplied. Large DPI-2 raster requests reject `TooLarge`. |
| Assets | Up to 512×64 KiB raw sources and sixteen 256×256 textures. Polling reads every registered file; decoding/scanning belongs off the frame thread. One outstanding worker request/result; publication can release old storage. |
| Scenes/world saves | Scalar scenes through 10000 entities; typed saves through 262144 u64 values, 4096 queued commands and 257 RNG streams. Serialization and root clones belong at explicit load/save boundaries. A 16 MiB file limit does not bound parsed heap memory. |
| Fixed simulation | 65536-entity, 128-tick catch-up workflow p95 is 15.6–22.8 ms. This spans multiple admitted frames; a tick cap is not a deadline. Queues and accumulated lag require game policy. |
| Navigation/placement | Searches through 512×512; inhabited 256×256 fixtures with eight budget-1024 requests take p95 4.09–4.29 ms. `max_visited` caps work, not wall time. Full searches/map clones may exceed a frame. Placement/storage workloads cover 65536 objects/tiles. |
| Collision | Prepared mixed candidates through 393216 pairs/262144 shapes. Selected batch p95 is about 2.46 ms before the final rerun; candidate generation/storage are excluded. All-pairs is quadratic; no broad phase is supplied. |
| Snapshots/RNG | Up to 16384 agents/32 histories and 1024 named streams. Largest nested workflow requests about 270 MiB at peak and history release p95 is 131–134 ms; 4096 agents/eight histories requests about 23 MiB. Caller-held snapshots retain independent roots. Determinism remains scoped to the documented target/configuration. |
| Audio | Native 1/16/64 voices and repeated output processing. Software loopback p95 is about 29.4 ms; this is callback delivery, not acoustic latency. Native backend release is asynchronous, about 0.5 s. Automatic power/device integration is not supplied. |
| Builds/delivery | Initial empty-target CLI/default-SDK release builds were about 23/162 s; unchanged repeats below 1 s. CLI/generated-game/classic_2d executables about 2.13/8.03/8.72 MiB. Host/cache-specific references, not caps; clean-host prerequisites and packaging need deployment validation. |

Text/cache retention and rejection bounds are specified in [TEXT.md](TEXT.md)
and [UI.md](UI.md); these are stronger contracts than the timing observations.
Allocator-request peaks exclude allocator/backend overhead and are not process
resident memory or a leak-free guarantee.

## Regression measurements

Ordinary domain tests protect correctness, rollback, deterministic ordering,
resource lifetime and bounded caches. They remain required. Ignored release
probes exercise scaling, allocation/I/O phases and native devices; they are
retained for reproducible diagnosis and do not run in normal `cargo test`.
Attribution probes measure different phases/lifetimes and are not redundant
correctness tests. Native/GPU/font-file/heap attribution probes remain manual.

CI runs 14 portable CPU workloads covering 270 configurations: runtime dispatch,
world churn, input publication, bitmap UI routing, localization, fixed time, RNG,
navigation, placement, collision, snapshots, asset reload, scenes and world saves.
It measures the PR base and exact PR head, or the previous push revision and new
revision, on one Linux runner with Rust 1.99.0. It compiles both first, then runs
each probe sequentially in base/head/head/base order, with two warmup batches
excluded. Median and nearest-rank p95 describe operation times or batch means,
not individual-frame latency. Both paired comparisons must exceed 20% and an
absolute 1 µs per reported work unit to emit a GitHub warning. Thresholds are
noise filters, not acceptance budgets; hosted-runner warnings need confirmation.
Timing regressions do not fail the job. Build/probe errors or missing/changed
coverage fail the performance job and preserve logs; `CI Success` continues to
require platform verification and MSRV. Initial pushes without a previous revision
cannot be compared. Artifacts retain metadata, raw logs and all comparisons for 14 days.

[scripts/performance/reference-2026-10-05.json](../scripts/performance/reference-2026-10-05.json)
records two final Windows CPU runs for this subset. It is a historical reference;
CI remeasures both revisions instead of comparing Linux timings with Windows values.
The workload schema records expected configurations and sample counts. Changed
fixtures require explicit schema review; results from different fixtures are not comparable.

Run locally against an existing checkout of the previous revision (Python 3.10+):

```powershell
./scripts/performance.ps1 -Baseline C:/path/to/previous-checkout
```

```console
sh ./scripts/performance.sh --baseline /path/to/previous-checkout
```

Reports default to ignored `target/performance`. Additional manual probes can be
listed with `cargo test -p <crate> --release --locked -- --list --ignored` and run
by exact name with `--ignored --exact --nocapture --test-threads=1`. Use the
required feature/device/font fixtures; disable unrelated `GRIDTHORN_*` diagnostics
for CPU comparisons. Audio output probes require the `native-output` feature.

For limited local disk space, PowerShell accepts `-BuildTarget C:/path/to/target`
and the shell entrypoint accepts `--build-target /path/to/target`. The runner
copies each revision's test binaries before reusing compilation output. CI uses
separate build targets. The comparator's portable measurements do not require
native windows or audio devices.
