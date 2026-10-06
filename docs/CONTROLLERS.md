# Provisional controller contract

Implemented on 2026-10-06. Public types live in `gridthorn::controller`; the input
crate owns contracts and snapshots, application orchestration owns the native
adapter. gilrs types stay private. Controllers are presentation input; games map
input to commands before fixed simulation, without serializing native identities.

## Discovery and identity

The native runner performs no automatic controller initialization or polling.
`ControllerPolling` is installed before startup. Call `request_poll()` to initialize
services lazily and collect discovery/input once after the frame on the event-loop
thread. Requests before dispatch coalesce; results arrive in the next `InputState`.
Custom lifecycles use `WindowControl::poll_controllers()`.
`controller_availability()` is `None` until an adapter reports a result, `Some(Ok(()))`
when discovery is operational (including zero devices), or a contextual error.
`ControllerEvent::Ready`, `Connected`, `Disconnected` and `Unavailable` remain in
the ordered input stream. A failed backend leaves desktop input operational; no
automatic initialization retry is promised. Headless runtimes initialize no OS
services; custom adapters inject the same events through `InputBuffer`.

`ControllerId` identifies one connection within a runner session. Reconnecting
creates a new ID; stale feedback cannot target a replacement connection. Two
identical models have distinct connection IDs. Name, optional USB vendor/product,
and the backend's model UUID describe a device; UUID is not a serial number or a
persistent per-unit selection guarantee. Games own saved preference matching and
ambiguity policies. `ControllerInfo` lists mapped buttons/axes and backend rumble
support. Unmapped vendor controls, touchpads, motion sensors, LEDs and batteries
are outside this increment. Capabilities describe mappings, not proof of every
physical feature or successful operation.

## Buttons, axes and frame boundaries

The adapter drains events only on explicit requests, reporting initial inventory
and accumulated hotplug changes. State can become stale between requests.
Input is ordered within polling; no total timestamp
order relative to keyboard/window events is promised. Buttons use physical face
positions, shoulders/triggers, stick clicks, D-pad and menu/system controls.
Sticks range from -1 to 1 with positive Y upward; triggers range from 0 to 1.
Digital trigger thresholds come from backend mappings; analog pressure is separate.
Axis D-pads are mapped to buttons; backend jitter and dead-zone filters are disabled.

`InputState::controllers()` returns connected state in connection-key order.
`controller(id)` exposes info, held/pressed/released buttons and latest axes.
Duplicate presses do not duplicate edges. A complete tap can set both edges in
one frame. Snapshot drains events and edges while retaining held state and axes.
Events from unknown/disconnected IDs cannot recreate devices. Disconnect removes
state immediately; the ordered disconnect event is the cancellation signal.
Nonfinite injected axis values do not update state; finite values are clamped to
the axis range. Original injected events remain in the ordered stream.

Focus loss and suspend neutralize held/analog state while retaining inventory;
native input samples are suppressed while unfocused. Requested discovery works
while unfocused; suspend pauses dispatch until resume. Focus changes discard earlier
queued input without polling. Focus return does not synthesize
held presses: release/repress or a new axis sample is needed. UI consumers clear
ownership on disconnect/focus cancellation. Native poll latency depends on explicit
requests and runtime wake cadence; delivery latency and large device counts remain unmeasured.
Release binary-size, dependency-footprint and native polling CPU measurements are
also deferred; this increment makes no controller performance-budget claim.

## Dead zones and feedback

`DeadZone::new` validates a finite threshold in `[0, 1)`. `axial` rescales a signed
scalar outside the zone; `radial` preserves stick direction and rescales its
magnitude, clamped at one. Nonfinite samples become neutral. Filtering is explicit
and game-owned; raw controller state is not silently filtered.

`ControllerFeedback` is installed before runtime startup. Queue `RumbleRequest`
with caller correlation ID, current connection ID, finite strong/weak magnitudes
in `[0, 1]`, and duration at most 60,000 ms. The queue drains once after a frame;
startup requests are also dispatched. Custom lifecycles use `WindowControl::rumble`.
`ControllerEvent::Feedback` reports validation, disconnected, unsupported, unfocused
or contextual native errors. Success means submitted, not confirmed physical
vibration. Requests are processed in order. A successful request replaces that
device's previous effect; zero duration stops it. Rejected requests retain the
previous effect. Effects expire automatically and are released on disconnect,
focus loss, suspend and shutdown. Feedback is rejected while unfocused.

The supported effect is bounded dual-motor rumble. Backend support varies by
platform/device; macOS rumble is unsupported by gilrs. Linux builds require
pkg-config/libudev development files and device access. Windows uses the backend's
default Windows Gaming Input adapter. See [gilrs platform support](https://docs.rs/gilrs/0.11.2/gilrs/)
and [force feedback](https://docs.rs/gilrs/0.11.2/gilrs/ff/index.html).

## Generic UI navigation

Call `UiRouter::set_controller_navigation(true)` before routing input. It defaults
to disabled so games choose when the UI owns controller navigation. South activates,
East cancels, shoulders cycle, D-pad/left stick move spatially and adjust sliders
or lists through the existing `UiNavigation` contract. Modal scopes, layer dismissal,
IME cancellation and platform requests use the same navigation path as manual
hooks. Mapped press/release events are consumed; duplicate presses do not activate
again. The stick enters at magnitude 0.6 per axis and rearms at 0.3, with no timed
repeat. Opposite directions can switch directly. Disconnect/focus loss clears
button/stick ownership. Unmapped controls and lifecycle/feedback events pass through.
`UiRoute::controller_blocked` tells games to suppress continuous navigation bindings
while this mode is enabled; raw snapshots remain unchanged. Games needing different
bindings can disable it and map their own controls to `navigate`.

## Verification and public example

Domain tests cover independent model identities, ordered taps/frame edges,
disconnect and focus cancellation, malformed samples, dead zones and feedback
validation/queue order. Runtime tests cover startup inventory and one-shot feedback.
UI tests cover opt-in routing, duplicate suppression, stick hysteresis and disconnect.
The sibling [controllers example](../../gridthorn-examples/controllers/README.md)
uses only facade APIs and provides injected headless and native discovery smoke:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_controllers --locked -- --headless
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_controllers --locked -- --smoke
```

The visual example shows held buttons, trigger pressure, stick positions and a live
device selector. Only connected controllers appear; selection falls back to
keyboard/mouse on disconnect. Its tests validate selection identity and indicators.

Windows native backend initialization passed with zero controllers; rendered example
smoke later detected one connected controller. Physical input/rumble was not exercised.
Native discovery smoke validates service initialization even with zero controllers.
Physical button mappings, hotplug and rumble require interactive hardware acceptance;
Linux/macOS execution and hardware coverage remain unvalidated. No universal device
support or physical-feedback confirmation is claimed.

The visual example requests discovery once from its Startup system and provides
an explicit refresh button. While a controller is selected, its UI requests
input every 50 ms for live indicators. Keyboard/mouse mode has no periodic polling.
This is example-owned policy; the engine installs no polling timer. On Windows, gilrs retains its own input-polling thread after initialization.
The request API gates engine event collection; it does not suspend that backend
thread. Eliminating native background polling requires a different backend contract.
