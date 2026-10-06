# Explicit window controls

The provisional `gridthorn::window` API owns native presentation settings without
exposing backend handles. `WindowedApplication` supplies `WindowSettings` before
Startup. Its capabilities and latest `WindowState` are available before Startup;
headless/default settings are unavailable and have no native state.

## Requests and feedback

Call `WindowSettings::request(WindowRequest)` from a game schedule. A request is a
patch: omitted properties preserve state. The last queued request wins. The
returned runner-local sequence number correlates retained `Pending`, `Applied`
or `Failed` feedback; older results cannot overwrite newer requests. Invalid
empty requests, zero/oversized dimensions, inconsistent min/max bounds and
fullscreen combined with windowed properties are rejected before queuing.

Operations execute on the event-loop thread after Startup or the current frame.
Monitor-targeted operations perform one explicit inventory revalidation and use
connection-scoped IDs from [DISPLAYS.md](DISPLAYS.md). An unavailable monitor or
advertised mode produces a typed failure. Size/policy/explicit-coordinate-only
operations do not enumerate monitors. A custom `WindowLifecycle` can use
`WindowControl::configure_window(id, request)` and receive state/capability and
operation callbacks. The caller owns correlation IDs for that lower-level API.
Mixing legacy monitor selection and a configuration request gives configuration
priority. Superseded in-flight operations report `Superseded`.

`Applied` means the requested observable properties match backend readback.
Confirmation runs only while pending, with a five-second deadline and a two-pixel
outer-position tolerance. Timeout returns `NotApplied` with actual state. Native
setters are asynchronous and do not form an atomic transaction: partial changes
are possible, failures do not roll them back, and the game must consult actual
state. Constraints have no portable readback; `resize_policy` is configured state.
`resizable` is optional native flag readback. Fullscreen classification/mode
readback does not independently prove exclusive hardware scanout.

Windows exclusive fullscreen currently supports only the happy path with the
unmodified upstream winit backend. An OS mode-switch rejection may panic and
terminate the application; it is not converted to `Failed` or `NativeRejected`.
`NativeRejected` remains reserved for future recovery. Inventory validation and
confirmation timeouts still return typed errors, but cannot prevent a native
rejection between validation and activation. No panic-catching workaround is used.
Recovery, applied-state consistency and disconnected-monitor restoration are
explicitly deferred in [Milestone 5](ROADMAP.md). See
[ADR 0002](adr/0002-upstream-fullscreen-happy-path.md).

## Geometry and modes

Sizes and min/max constraints are physical client pixels. Placement is the outer
frame's physical desktop origin (negative coordinates are valid), or centered on
a monitor. `Fixed` prevents user resizing but permits programmatic changes.
Resizable bounds are cleared with `None`. OS/window-manager policy may constrain
requests. Renderer texture/surface limits still apply; this API does not expand
GPU capabilities.

Borderless fullscreen uses the target monitor's full extent and preserves its
desktop video mode. Exclusive requests select an exact advertised
resolution/millihertz-refresh/bit-depth tuple and revalidate it before use.
Fullscreen requests reject windowed size, policy and placement. Switching to
`Windowed` restores the last windowed size/position unless overridden; configured
resizing policy is reapplied after fullscreen bounds are cleared. Ordinary resize
and DPI events continue to drive viewport, layout and surface reconfiguration.
Monitor selection moves the existing rendering window. Initialization adapter
selection and presentation/VSync controls are separate implemented APIs; see
[GRAPHICS_ADAPTERS.md](GRAPHICS_ADAPTERS.md) and [PRESENTATION.md](PRESENTATION.md).

Windows exclusive readback uses current OS refresh rather than echoing the
requested mode. The underlying API reports integer hertz; millihertz units do not
imply fractional precision. Confirmation accepts the documented 59/60 Hz alias
while still requiring matching resolution and bit depth; other rate differences
remain failures. The actual reported rate stays in `WindowState.display_mode`.
See [Microsoft's refresh-rate explanation](https://support.microsoft.com/en-us/topic/screen-refresh-rate-in-windows-does-not-apply-the-user-selected-settings-on-monitors-tvs-that-report-specific-tv-compatible-timings-0a7a6a38-6c6a-2aec-debc-5183a76b9e1d).
Desktop-mode restoration is delegated to upstream winit. The engine exits
exclusive before transferring to another monitor and restores windowed geometry
separately. Recovery after rejected restoration or disconnection is not guaranteed.

| Backend | Supported subset and limitations |
| --- | --- |
| Windows | Size, policy, placement and windowed/borderless/exclusive happy path supported. Native rejection may panic; recovery and restoration failure handling are deferred. |
| Linux X11 | Backend-supported size/policy/placement/borderless/exclusive; native acceptance deferred. Resizable flag readback unavailable; configured constraints remain explicit. |
| Linux Wayland | Size/policy and borderless; placement and exclusive unavailable. Native acceptance deferred. |
| macOS | Backend-supported controls including fullscreen; native acceptance deferred. |
| Headless | Default unavailable service, no native operations. |

## Validation and cost

Domain tests cover request validation/coalescing, stale-feedback rejection,
startup/idle routing, confirmation conditions and timeout observations. Ignored
Windows native success probes cover size/policy, placement, fullscreen, refresh
readback, transfer and restoration; their previous acceptance used the patched
backend and does not establish equivalent upstream behavior. Rejection recovery
coverage is deferred. The sibling
[settings example](../../gridthorn-examples/desktop-displays/README.md) exercises
mode/resolution/refresh transitions through a real GPU window.
Visual acceptance, physical hotplug and non-unit native DPI remain manual.

Idle applications perform only an empty mailbox/pending check; there is no
monitor enumeration, timer, mode-list copying or snapshot cloning from this
service without a request. Native state getters run on relevant window events or
while confirming an operation. An explicit query and setters can stall the
requesting frame. Timing/allocation/binary-size measurements for this increment
remain deferred; no zero-overhead or whole-engine frame-budget claim is made.
Game-owned persistence, confirmation countdown/revert, multiple windows, adapter
migration and unverified backend recovery are outside this increment.
