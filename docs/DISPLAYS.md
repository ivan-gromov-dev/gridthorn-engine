# On-demand desktop displays

The provisional `gridthorn::display` API provides engine-owned display data and
explicit monitor selection. Native queries run only when a game requests them;
ordinary applications do not enumerate monitors or modes in the background.

## Game-owned requests

`WindowedApplication` creates an empty `Displays` resource before `Startup`, with
`Unavailable` availability and revision zero. Opening a graphics-settings page
can call `Displays::request_refresh`. The runner performs one native query after
Startup or the current frame and delivers the result before the next frame.
Repeated refresh requests before dispatch coalesce. `revision()` increments on
every completed query, including unchanged results, so a page can distinguish a
fresh response from a retained snapshot. `Available` means a query completed;
it does not mean monitoring is enabled. An empty native inventory is valid.

```rust
use gridthorn::display::Displays;
use gridthorn::WorldAccess;

fn open_graphics_settings(world: &mut WorldAccess<'_>) {
    world.update_resource(Displays::request_refresh);
}
```

A custom `WindowLifecycle` uses `WindowControl::refresh_displays()` and receives
owned data through `displays_changed`. No inventory is supplied automatically at
startup, resume, DPI changes, idle or redraw. Headless execution does not create
the native service; a manually inserted default resource stays unavailable.

Queries run on the native event-loop thread. Snapshot data is owned and
`Send + Sync` and contains no backend handles. Display data is presentation state,
not authoritative simulation state or serialized world-save content.

For size, policy and supported fullscreen requests, use [WINDOWS.md](WINDOWS.md).
The legacy selection below remains a windowed-placement operation.

## Monitor selection

After enumeration, call `Displays::select_monitor(id)` with a returned ID. The
runner revalidates the native inventory once, retires missing identities and
centers the existing window on the selected monitor. Selecting the monitor moves
the renderer's existing window/surface; it does not select a graphics adapter,
recreate the renderer or change fullscreen mode. A maximized window may be
restored by native placement. Window resizing and effective DPI still flow through
the existing window/renderer lifecycle. Desktop bounds, rather than an OS work
area excluding taskbars, define the centering operation.

`selection()` reports `MonitorSelection::Pending`, `Applied { monitor }` or
`Failed { monitor, error }`. Success requires the OS to report the requested
monitor as the window's current monitor. While placement is pending, the runner
checks the window monitor at idle boundaries, without enumerating monitors or
modes. Checks stop on confirmation or after a two-second confirmation deadline.
A blocked event loop or OS suspension can delay feedback; this is not a frame
budget. The latest selection request supersedes any pending placement, and the
last request queued before dispatch wins. Refresh plus selection in one dispatch
shares a single inventory query. Feedback persists until the next selection.

`MonitorSelectionError` distinguishes unavailable/foreign/retired IDs, unsupported
position queries, unsupported fullscreen placement and unconfirmed placement.
Failure keeps the application running and does not imply rollback: the OS can
have moved the window elsewhere or applied a late movement. Games own retry,
confirmation/revert and persistence policies. Fullscreen selection is deferred to
the remaining window-controls increment. Wayland does not support this windowed
positioning operation and returns typed `PositionUnavailable` feedback.

`active()` is the window monitor at the latest inventory query or confirmed
placement. A manual move, disconnection or later OS correction requires a new
explicit refresh to update this retained observation. Custom lifecycle hooks use
`WindowControl::select_monitor()` and `monitor_selection_changed` feedback.

## Identity and change observations

`MonitorId` is opaque and scoped to one native runner and one observed connection.
It remains unchanged when the backend identity is present in consecutive queries,
including property changes or enumeration reorderings. An observed absence retires
the ID and produces `Disconnected`; an observed reconnect receives a new ID.
Foreign runner IDs never resolve. Names/positions are not unique identities and
IDs must not be persisted across runs or treated as hardware serial numbers.

Explicit queries compare against the previous inventory. Initial monitors produce
`Connected`; changed names, resolutions, positions, refresh rates, scale or modes
produce `Changed`; disappeared identities produce `Disconnected`. Primary changes
produce `PrimaryChanged`. Within one query, connected/changed events follow
backend observation order, disconnections follow previous ID order and primary
changes come last. Inventories are ordered by ID. Changes accumulate until a
runtime frame executes, are visible for that frame and are then cleared.

There are no automatic hotplug notifications. A settings page can refresh on open,
on its Refresh button or at a frequency chosen by the game. Transitions entirely
between explicit queries can be missed; native identity reuse without an observed
absence cannot be distinguished from a property change. The OS can also change
hardware during enumeration: a snapshot is not an atomic OS transaction. Missing
optional metadata is reported as absence; the backend iterator provides no
per-query native error detail.

## Resolution, refresh and DPI

`MonitorInfo.resolution` and `position` use desktop physical pixels, including
negative origins. `DisplayMode` contains an advertised fullscreen resolution,
refresh in millihertz and backend-reported bit depth. Modes are sorted by
resolution/refresh/bit depth and deduplicated. A mode's zero refresh means unknown;
zero current desktop refresh becomes `None`. An empty mode list means the backend
did not advertise modes, and enumeration does not guarantee mode activation.

`scale_factor` is OS DPI scaling in physical pixels per logical pixel, not measured
physical panel DPI. Rendering/layout uses `WindowScaleFactor`, which can differ
from monitor scale, particularly on Wayland. No physical-DPI API is provided by
the selected backend.

## Validation and performance limits

The sibling [settings example](../../gridthorn-examples/desktop-displays/README.md)
uses the public facade and retained UI controls. Its Graphics page requests a
single inventory on entry or Refresh, stages the selection and applies it only
on the Apply button. Audio/Controls are clearly marked future sections. Unchanged
frames reuse prepared layout and presentation resources.

Domain tests cover query coalescing, no implicit follow-up requests, startup
requests, persistent feedback, one-frame changes, identity retention/retirement,
primary changes, mode normalization, placement confirmation/deadline and bounded
centering math. The Windows native test checks no unsolicited inventory before
or during idle, one query on demand, one selection revalidation and confirmed
placement. The expanded GPU settings smoke uses the UI router for supported window controls and verifies that idle does not trigger new queries. See [WINDOWS.md](WINDOWS.md).

| Platform | Acceptance and limits |
| --- | --- |
| Windows | Native query/placement test and GPU settings smoke on a two-monitor desktop. Real cable removal and non-unit native DPI remain manual acceptance. |
| Linux X11 | Backend-supported queries/positioning; native acceptance deferred. Backend environment settings can override scale. |
| Linux Wayland | Queries supported; primary is unavailable and monitor/window scale can differ. Windowed positioning unsupported. Native acceptance deferred. |
| macOS | Backend-supported queries/positioning; native acceptance deferred. |
| Headless | No automatic inventory or platform queries. UI layout/navigation can be tested without native devices. |

Without requests, the display service has no allocated catalog, native queries,
mode enumeration, timer, snapshot copying or notification traffic. Runtime checks
an empty request mailbox once per frame; this is small constant work, not a claim
of literally zero CPU overhead. Native selection confirmation also returns
immediately when no placement is pending. Inventory allocations and mode-list
copies occur only on explicit queries. Native queries are synchronous and can
still delay the requesting frame. Timing/allocation and binary-size measurements
remain deferred; no whole-engine frame-budget guarantee is made.
