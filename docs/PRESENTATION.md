# Provisional presentation controls

Implemented on 2026-10-06. `gridthorn::presentation` exposes engine-owned
`PresentMode`, `FrameRateLimit`, `PresentationConfig`, `PresentationSettings`,
`PresentationState`, `PresentationOperation` and `PresentationError`. No new
dependencies are introduced. These APIs remain provisional.

## Surface policy and capabilities

`PresentationSettings::state().supported_modes` reports explicit policies for
the initialized surface/adapter pair. Capabilities are queried at initialization
and revalidated on a mode request and before native configuration. There is no
periodic adapter or monitor enumeration. Renderer-free windows report an empty
list and requests fail with `Unavailable`. Headless runtimes do not install or
execute this service; games may insert a default mailbox for their own tests.

| Mode | Queue behavior | Synchronization and tearing |
| --- | --- | --- |
| `Fifo` | Ordered frame queue | VSync; no tearing |
| `FifoRelaxed` | Ordered queue with immediate late frames | Adaptive VSync; late frames may tear |
| `Immediate` | Immediate swap | VSync off; tearing possible |
| `Mailbox` | Replace the queued frame with the latest | Display at vertical blank; no tearing |

The default is `Fifo` with no software cap. Optional modes appear only when the
backend reports support. Automatic backend policies are excluded: a request for
`Immediate` must not silently fall back to FIFO. Capabilities are specific to
the active window and adapter; they are not a universal OS/hardware guarantee.
Driver/compositor behavior, variable refresh and actual scan-out timing remain
outside this contract.

## Requests and observed state

Submit a complete configuration from Startup or a frame schedule:

```rust
use gridthorn::presentation::{
    FrameRateLimit, PresentMode, PresentationConfig, PresentationSettings,
};

let config = PresentationConfig {
    present_mode: PresentMode::Fifo,
    frame_rate_limit: Some(FrameRateLimit::new(90)?),
};
let mut settings = PresentationSettings::default();
let request_id = settings.request(config)?;
```

In a running window, mutate the installed world resource rather than a separate
local mailbox. Each request receives a checked u64 identifier. The last queued
request wins; the mailbox dispatches it once on the event-loop thread. Feedback
for an older identifier cannot replace newer feedback. Game code owns staged
choices, persistence and fallback preferences.

`Pending` begins when queued, before native validation. After acceptance the
observed cap becomes active while surface configuration may still be deferred;
inspect state rather than treating Pending as proof of application.
`Applied` means the requested mode has
been configured and the cap accepted. An unchanged, already configured mode
can acknowledge a cap change immediately. `Failed` contains `Unavailable` or
`UnsupportedMode` without changing the previous requested mode or cap. Invalid
FPS values and exhausted request IDs fail synchronously before queuing.

`PresentationState::applied_mode` is the last successfully configured mode,
separate from the game request. It is absent before the first configuration
and at zero size. Nonzero resize, mode changes and outdated-surface recovery
defer configuration until the next renderable acquisition. Occlusion and zero
size never configure or acquire a swapchain. Pending requests can therefore
remain pending until the window becomes renderable. GPU/device/surface failures
retain the existing contextual `ApplicationError` shutdown contract; this
increment does not add device-loss or surface recreation recovery.

Low-level `WindowControl::configure_presentation` and `WindowLifecycle`
state/operation callbacks expose the same native workflow. Direct renderer
clients can query `SurfaceRenderer::present_modes`, request a policy through
`set_present_mode` and inspect `applied_present_mode`. Renderer clients own their
own scheduling; application FPS caps do not affect direct `render()` calls.

## FPS caps and simulation

`FrameRateLimit::new` accepts integer FPS from 1 through 1,000,000,000. Zero is
invalid; `None` means uncapped. The software limiter spaces redraw attempts
using a monotonic clock and a rounded-up nanosecond interval. Both requested
redraws and unsolicited OS redraw events pass through the same gate. Slow or
skipped acquisitions consume an attempt; late frames do not create catch-up
render bursts. Changing a cap uses the last attempt time, and resume resets
that deadline. There is no spin-wait or thread sleep.

The event loop waits until the earliest application wake or redraw deadline.
`WindowedApplication` also schedules runtime wakes at its configured fixed-step
interval, so a low FPS cap does not defer all simulation work until redraw.
Input, Update and presentation extraction can run between displayed frames.
Custom lifecycle clients own their simulation wakes through `wake_at`.
Platform Suspend stops runtime preparation and redraws; Resume resets the
presentation deadline and excludes suspended time from the runtime timer.

The cap is an upper bound on redraw attempts, not a promised output FPS. VSync,
GPU load, acquisition delays, OS scheduling and display refresh can produce
fewer frames. The existing single event-loop thread can still be blocked by
native/GPU work. Fixed tick duration, simulation speed, catch-up limits and
backlog retention are unchanged; no tick is dropped or stretched to match
presentation. A very expensive host frame still emits the existing simulation
overload diagnostic. The cap neither selects nor modifies monitor refresh.
`FrameTiming::frame_elapsed` measures runtime host updates, not scan-out or
necessarily the interval between displayed frames.

## Verification and platform limits

The sibling `desktop-displays` settings menu uses only public facade APIs:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_desktop_displays --locked
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_desktop_displays --locked -- --presentation-smoke --auto-adapter
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_desktop_displays --locked -- --headless
```

The Graphics tab offers VSync on/off, explicit supported modes and
uncapped/30/60/90/144/1 FPS. Choices are staged. The presentation Apply button
only requests VSync/FPS; the general Apply button also requests window settings.
These are independent operations with separate feedback, not an atomic native
transaction. Reset restores staged choices from observed state. Unsupported
VSync toggles are disabled, with no silent fallback. Observed mode/cap and
feedback remain separate from staged choices. Monitor refresh is selected only
for exclusive fullscreen. Presentation smoke routes actual UI clicks for every
advertised mode at low/high caps and uncapped, tests reset, and checks continued
fixed simulation without changing or re-querying monitors. Headless checks and
domain tests validate menu routing and capability-gated controls without a GPU.

Domain tests cover cap validation/rounding, early and late frames, cap changes,
resume reset, fallback exclusion, unsupported policies, coalescing, stale
feedback, renderer-free diagnostics and fixed-tick independence. The ignored
Windows native test exercises real switching, deferred zero-size/occlusion
configuration, restoration, cap removal and redraw spacing under frequent
wake events:

```console
cargo test -p gridthorn_app switches_supported_native_modes_and_caps_redraws --locked -- --ignored --nocapture --test-threads=1
```

Windows native acceptance on 2026-10-06 reported all four explicit modes on the
available surface. A 30 FPS cap produced 14 attempts over 641 ms with the
minimum interval preserved. This is an upper-bound check, not a frame-budget or
physical refresh measurement. Linux/macOS native validation, other adapters,
compositors, VRR and real monitor disconnection remain untested/deferred.

Logical Suspend/Resume hooks were injected in the native test; an actual OS
sleep/wake cycle was not exercised for these controls.

On the same date, `desktop-displays --presentation-smoke --auto-adapter` passed
on the RTX 3070/Vulkan through routed UI controls: VSync off/on, all four modes
at 1/30/90 FPS and uncapped, observed feedback and Reset. It completed with
1266 fixed ticks at the default 16.666667 ms interval, and confirmed unchanged
window/display state and monitor inventory revision. Its 13 package tests,
Clippy and headless font/layout/cache checks passed. The separate existing
window smoke passed with ordinary desktop permissions; restricted-token
exclusive rejection still has the limitation documented in [WINDOWS.md](WINDOWS.md).
