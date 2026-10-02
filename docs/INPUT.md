# Provisional desktop input contract

Implemented on 2026-10-02 for the first Milestone 4 increment. These APIs are
provisional. The SDK owns all public types; no windowing backend types escape.

## Keyboard and frame boundaries

`InputEvent::Key(KeyboardEvent)` carries a standardized or native physical key,
layout-dependent logical character/named/dead/native key, location, digital state,
OS repeat flag and platform synthetic flag. All currently mapped desktop physical
and named keys are converted explicitly, including function keys, keypad keys,
left/right modifiers, international positions and media keys. Unknown native
codes preserve their platform namespace and value; they are not portable bindings.
Future unmapped standardized backend values become `Unidentified`.

Logical characters describe shortcuts under the active layout. They are **not**
committed text: Unicode text editing, IME and clipboard are the next increment.
Modifiers arrive as ordered aggregate changes; left/right distinctions are physical
keys. No caps-lock/num-lock toggle state or per-keyboard device selection is promised.

`InputBuffer` applies events in arrival order. `snapshot()` emits an immutable
`InputState` whose `events()` retains every event, including repeats and multiple
transitions of the same key in one frame. The next snapshot clears the event list
and edge sets, retaining held keys, buttons, modifiers and the cursor position.
`key_down`, `key_just_pressed` and `key_just_released` remain available. Repeat
presses maintain held state without generating new press edges. A short tap can
produce both press and release edges in one frame; use ordered events when order
matters. `physical_key_down` also accepts native unidentified identities.
The older `InputEvent::Keyboard` remains a physical-only injection convenience.

The native runner publishes the snapshot before `Input`. Systems convert input
to game commands there; fixed ticks never consume raw platform events directly.
Events are frame-scoped presentation data and are not serialized or replayed.

## Pointer and wheel

Wheel events preserve horizontal/vertical deltas as either lines or physical
pixels and preserve started/moved/ended/cancelled gesture phases. Games choose
scroll sensitivity and direction. Line and pixel units are never silently added.
Cursor positions use physical window pixels with a top-left origin. Relative
`PointerMotion` is delivered from native device motion while the window is focused
and capture is effective; it is displacement, not an absolute window position.

`PointerCapture` is a runtime resource installed before startup. A system calls
`request(None | Confined | Locked)`. The last request in a frame wins; native
application occurs after the frame, with `PointerCaptureChanged` feedback in the
next snapshot. Custom window lifecycle adapters can use
`WindowControl::set_pointer_capture`. Feedback reports requested and effective
modes separately and returns a typed `PointerCaptureError` for an unfocused request
or native rejection. Failed requests retain the previous effective mode. There is
no silent fallback between confinement and locking and no automatic cursor hiding.
Mode availability depends on the native backend, desktop session and permissions.

`CursorLeft` clears position while uncaptured, but retains the last position while
captured. Focus loss and suspend cancel capture, clear modifiers and all held keys
and buttons, record release edges and clear the cursor position. Cancellation
requests native release and reports its result. Capture does not automatically
resume on focus return. Focus loss is an explicit ordered cancellation event;
consumers must not interpret its release edges as a completed click. This is
native cursor capture; UI ownership, hit testing and input consumption remain
later Milestone 4 work. Gamepads remain Milestone 5 work.

## Example and validation scope

The sibling `desktop-input` example uses the public facade:

```sh
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_desktop_input -- --headless
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_desktop_input
```

The native monitor runs with `WindowedApplication::without_renderer()` and prints every event. F1 requests confinement, F2 locking,
F3 release and Escape exits. `--smoke` requests confinement, locking and release, waits for each feedback event, then exits.
The headless mode validates layout-dependent Cyrillic identity, repeats, ordered
press/release, pixel wheel events, modifier cancellation and one-shot requests.
Engine tests additionally exercise every physical key conversion, logical/dead/
native keys, capture failure rollback and unfocused rejection, held-state cleanup,
frame boundaries and wheel units/phases.

Windows compilation, automated contract tests and native capture smoke are local
validation. Windows native smoke applied confinement, locking and release successfully. Interactive
keyboard layouts and hardware wheel gestures require manual device testing. Linux/macOS native execution,
large event burst performance, allocation/memory and binary-size measurements
are explicitly deferred; no universal platform or device validation is claimed.

The public monitor supports `--gpu`; `--gpu --smoke-immediate` is the regression
case that exits at the first frame boundary without presenting. Five consecutive
runs passed after the fix; native GPU capture smoke, textured-sprite smoke and
window resize/minimize/restore smoke also passed. The backend's internal reason
for raising the native exception is not established; the engine no longer creates
the unused swapchains that triggered the reproduced failure.
