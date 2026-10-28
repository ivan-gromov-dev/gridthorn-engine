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
committed text: the separate text-session stream below carries insertable text.
Modifiers arrive as ordered aggregate changes; left/right distinctions are physical
keys. No caps-lock/num-lock toggle state is promised. Aggregate window keys remain
separate from the provisional raw-device selection contract below.

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
native cursor capture; [UI ownership, hit testing and explicit input consumption](UI.md)
are now implemented separately. [Controller discovery, state, dead zones and feedback](CONTROLLERS.md) are now implemented provisionally.

## Unicode text sessions, IME and clipboard

Implemented provisionally on 2026-10-03. `TextInput` and `Clipboard` resources
are installed before native runtime startup, alongside `PointerCapture`.
Text delivery is disabled by default. `TextInput::start(ImeCursorArea)` opens or
updates a session; `stop()` closes it. The last request in a frame wins. Requests
are applied after that frame and `TextInputChanged { active, error }` arrives in
the next snapshot. `active` describes engine text delivery, not a guarantee that
an OS input method is available. `TextInputError` distinguishes unfocused windows
and invalid cursor rectangles; rejected requests preserve the current session.
The anchor uses finite physical window-pixel coordinates and nonnegative extents;
games update it after caret movement, scrolling or DPI changes. Backend positioning
support varies (X11 supports position only); unsupported IME environments cannot
be inferred from a successful session request.

`InputEvent::Text(TextInputEvent)` is separate from `Key` and `Keyboard`:

- `Commit(String)` inserts Unicode text once. Keyboard text comes from the
  backend text payload, never from a physical key or logical character. Only
  real key presses, including repeats, deliver it. Releases, synthetic keys,
  control characters and Control/Super shortcut chords are excluded. Control+Alt
  is permitted for AltGr text; games still own shortcut policy.
- `Composition { text, cursor }` replaces uncommitted preedit. Cursor endpoints
  are UTF-8 byte offsets, potentially a reversed selection; `None` means the
  native IME did not provide a visible cursor. Invalid native offsets become
  `None`. `InputState::composition()` retains preedit across frame snapshots.
- `CompositionCancelled` discards preedit without inserting it. Empty native
  preedit clears pending composition, including the native clear immediately
  before a commit: cancellation of preedit does not forbid a following commit.
  Commit, IME disable, session stop, focus loss and suspend also clear preedit.
- `ImeEnabled`/`ImeDisabled` report OS composition lifecycle. While IME is
  enabled, keyboard text payloads are suppressed so composition input cannot
  produce duplicate commits; physical events remain independent.

Focus loss and suspend close the session, cancel preedit, clear modifiers and
release held gameplay inputs. Late IME commits are ignored after closure.
Focus return does not reopen text input: the game restores its intended text
owner and explicitly requests a new session. Native cancellation precedes
`FocusLost`; headless `InputBuffer` injection also cancels outstanding preedit.
All text events are presentation data; fixed simulation consumes game commands.
The raw input boundary performs no normalization, field validation or consumption.
[UI routing](UI.md) now supplies grapheme navigation, selection editing and explicit
world-input consumption. [Unicode font rendering](TEXT.md) is now
implemented provisionally with an independent presentation service.

`Clipboard::read(id)` and `write(id, text)` queue ordered one-shot operations.
Caller-owned `u64` identities correlate `InputEvent::Clipboard(ClipboardResponse)`
with requests; callers should distinguish outstanding operations. A successful
read returns `Ok(Some(String))`; a successful write returns `Ok(None)`.
`ClipboardError` reports `Unfocused`, `NoText` or a contextual native `Platform`
failure. Errors do not terminate the application. Requests execute after the
current frame on the event-loop thread and replies arrive in the next snapshot.
Unfocused requests fail without accessing the OS. Clipboard access is independent
of text-session activity; there is no implicit copy/paste shortcut or text commit.
The backend initializes lazily and stays alive until shutdown for Linux clipboard
ownership. Text is accepted as UTF-8 without normalization. Rich formats,
clipboard preservation across process shutdown on ownership-based platforms,
Linux primary selection and native Wayland data-control are outside this subset;
Linux uses X11/XWayland. Native calls are synchronous; latency, allocation and
binary-size measurements are deferred.

Custom lifecycle adapters use `WindowControl::set_text_input` and `clipboard`.
Headless adapters can drain `TextInput::take_request` / `Clipboard::take_requests`
and inject the same engine-owned feedback; no OS clipboard is initialized by
the input contract crate.

The sibling `text-input` example uses only the public facade:

```sh
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_text_input -- --headless
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_text_input
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_text_input -- --smoke
```

The native monitor prints committed text and preedit to the terminal; the separate
`multilingual-text` example demonstrates asset-font rendering. F4 opens/updates the session, F5 closes it, Control/Super+C
copies its game-owned document, Control/Super+V appends clipboard text, and Escape
exits outside preedit. The example explicitly reopens text on focus return.
`--smoke` checks native session activation and, when original clipboard text can
be read, a Unicode write/read round-trip followed by restoration. If original
text is unavailable it skips writes. The headless mode exercises Cyrillic,
Japanese preedit/commit, combining marks, emoji sequences and focus cancellation.
Domain/runtime tests additionally cover IME disable, explicit stop, delayed
commits, repeats, shortcut/synthetic suppression, cursor validation, request
draining, clipboard correlation/errors and suspend/resume.

Windows native clipboard and IME behavior has maintainer manual acceptance for
this subset. Automated injected text events do not prove OS language behavior;
Linux/macOS native IME/clipboard and platform candidate-window geometry remain
unvalidated. Backend semantics follow [winit IME](https://docs.rs/winit/0.30.13/winit/event/enum.Ime.html)
and [arboard clipboard](https://docs.rs/arboard/3.6.1/arboard/struct.Clipboard.html).

## Input capabilities, discovery and source selection

Implemented provisionally for Milestone 5. `gridthorn::input_devices` exposes
engine-owned `InputCapabilities`, `InputSource`, connection state and typed
selection errors. The native runner publishes `InputEvent::Capabilities` before
`Startup`; `InputState::input_capabilities()` retains it across frames. Headless
buffers start with unreported capabilities (`DeviceDiscovery::Unknown`); custom
adapters can inject the same capability event. Availability describes an input
stream, not a connected hardware count or a guarantee of OS permission.

Aggregate `Keyboard` and `Mouse` represent the window's virtual input streams.
They do not identify individual physical peripherals. Native Windows additionally
reports `DeviceDiscovery::Observed`: raw connections appear on OS arrival or the
first focused raw key, button or motion event while observation is enabled. `InputState::devices()` is this
observed list, **not a complete startup inventory**. A keyboard or mouse already
connected before startup may require input to appear. There is no OS polling,
manufacturer/model lookup or stable hardware serial number in this subset.

Raw observation is disabled by default. `RawInput` is installed before `Startup`;
call `RawInput::request(true)` to start continuous discovery/input or `request(false)`
to stop it. The last request before dispatch wins. Requests apply after the current
frame; `InputEvent::RawInputChanged { enabled, error }` reports the applied state
in the next snapshot, and `InputState::raw_input_enabled()` retains that state.
Capabilities describe support independently of whether observation is active.
Unsupported enable requests report `enabled: false` with the typed `Unsupported`
error. Repeated enable requests acknowledge the current state without clearing it.

While off, the native adapter does not create per-device events or retain raw
connections. Stopping clears the observed inventory and held raw state and marks
active raw actions `cancelled`, not normally released. Re-enabling starts an empty
observation session; newly observed sources get new IDs and old bindings do not
silently reconnect. Focus loss/suspend cancel held raw input but do not disable
an explicitly active observation session. Games should keep observation active
while using individual device bindings, and stop it when no longer needed.

Aggregate keyboard/mouse input and captured aggregate `PointerMotion` are independent
of this gate. The underlying window backend may still listen to native device
events for cursor capture; disabling observation does not globally disable its
raw-event callback. No periodic device enumeration is introduced. Custom adapters
drain `RawInput::take_request` and inject applied-state feedback; raw state events
received while inactive do not populate the snapshot inventory.

Each raw connection gets an engine-owned `InputDeviceId`, unique within this
runner's lifetime. Removal invalidates it, clears its held state and emits
`DeviceEvent::Disconnected`; reconnect receives a new identity even if the backend
reuses its handle. Do not persist connection IDs or mix inventories from different
runners. `DeviceState` observation flags accumulate keyboard, raw-button and
relative-motion evidence; absence of a flag means unobserved, not unsupported.
Unknown/disconnected input identities do not recreate state without `Connected`.

Raw input is a separate stream: physical keys carry no logical layout/text/IME
meaning; raw button numbers are backend-specific and must not be cast to
`MouseButton`. `DeviceEvent::Motion` uses backend-relative units and is delivered
only while focused, without requiring capture. It is independent of captured
aggregate `PointerMotion`. Raw wheel, absolute raw coordinates, logical per-device
keys and per-device text sessions are outside this subset. Raw input never changes
aggregate key/button state or OS cursor/text ownership. Focus loss and suspend
cancel all held raw state while retaining the observed inventory.

`InputSource::validate(&input)` validates aggregate, raw or controller selection
against the current snapshot. It returns `DeviceSelectionError::Unsupported` for
raw selection on unsupported adapters, `Disabled` while observation is off, or contextual `Unavailable(source)` for
an unreported/disconnected source. Selection is a game-owned binding filter,
not an OS device-routing request. No implicit replacement device is selected.
Controller discovery continues to require explicit `ControllerPolling` requests;
this increment introduces no controller startup or per-frame polling.

| Adapter | Aggregate keyboard/mouse | Individual raw selection | Discovery |
| --- | --- | --- | --- |
| Windows native | Available | Observed connection IDs, explicitly enabled | Arrival/first input; removal events while active |
| Linux X11/Wayland native | Available | Explicitly unsupported by this adapter increment | Aggregate only |
| macOS native | Available | Explicitly unsupported by this adapter increment | Aggregate only |
| Headless/custom | Unreported until injected | Defined by injected capabilities/events | No OS access |

Unsupported here describes the engine adapter, not an assertion that the OS has
no lower-level device APIs. Linux/macOS raw adapters, complete initial hardware
enumeration and actual two-keyboard/two-mouse/hotplug hardware acceptance remain
unvalidated or outside this provisional subset. Native identifiers may represent
virtual or remote sources; see the upstream [winit device identity contract](https://docs.rs/winit/0.30.13/winit/event/struct.DeviceId.html).

## Configurable digital actions

`gridthorn::actions` provides caller-owned `ActionBindings<A>` and an independent
`ActionMapper<A>` per input consumer. Action keys are game-defined `Ord + Clone`
values. `set` replaces an action's alternatives; `remove` removes it; an empty
alternative list disables it. No game action names, settings storage format or
automatic schedule behavior is imposed.

`ActionBinding` accepts aggregate physical keys (including native identities),
aggregate mouse buttons, a physical key/raw button on a selected raw connection,
or a standardized button on a selected controller connection. `BindingInput::source`
returns the corresponding source for validation. Disconnected bindings stay inert;
they do not migrate to a different device. `with_modifiers` requires an exact
aggregate modifier combination; the default accepts any modifiers. Even for raw
bindings, these modifiers come from the window stream, not the selected keyboard.

Call `mapper.evaluate(&bindings, &input)` once per host frame during `Input`.
It applies ordered digital events, preserving short taps and modifier changes,
then reconciles held state with the final snapshot. Alternatives are ORed: releasing
one does not release an action while another is held. Duplicate presses/repeats
do not retrigger active actions. `ActionFrame::get` reports `down`, `just_pressed`,
`just_released` and `cancelled`; a short tap can set both normal edges. Focus loss
or removal of the last active source sets cancellation instead of a normal release.
If another alternative still holds the action, removal does not cancel it.

Configuration changes apply against held input on the next evaluation, producing
normal edges as appropriate. Removing a held action produces its final release;
later lookups are neutral. A mapper first attached to an already-held snapshot
can recover its held action. Skipped snapshots lose their intermediate taps; do
not evaluate the same snapshot twice, since its event history would be replayed.
Empty frames clear result edges while preserving held actions. Text, wheel,
controller axes and pointer motion are not digital action triggers. Analog
mapping/dead zones remain game-owned using the existing controller primitives.

These are presentation results, not serialized simulation input. Convert actions
to game commands in `Input` before fixed ticks. UI consumption remains explicit:
pass a consistently filtered input buffer to the mapper (including focus and
disconnect cancellation), or gate game-command generation using the UI's event
and continuous-input ownership results. Filtering only events while retaining an
unfiltered held snapshot would reactivate inputs during final reconciliation.

Domain tests cover independent raw state, capability persistence, selection errors,
ordered taps/repeats, alternatives, exact modifiers, rebinding/removal, raw/controller
selection and cancellation. Runtime/facade checks cover pre-startup capabilities,
inventory before Input and one action edge before multiple fixed ticks. Adapter
injection checks cover arrival, first-event discovery, focus filtering, raw button
numbers, removal and handle reuse. Injected tests do not prove physical separation.

Validation of the opt-in follow-up on 2026-10-28: full Windows repository
verification passed (440 tests; 52 explicitly ignored probes), including formatting,
Clippy and dependency boundaries. The native raw-observation lifecycle probe was
also run separately and passed. The sibling desktop-input example passed three
tests, Clippy, headless and `--smoke-raw`: raw observation started off, explicit
enable reported five connections and explicit stop cleared the inventory.
Earlier capture smoke reported focused raw motion and successful confinement,
locking and release; the maintainer accepted the initial manual workflow before
the opt-in follow-up. These checks do not establish a complete two-device/hotplug
hardware matrix or Linux/macOS native validation.

## Desktop pointer example and validation scope

The sibling `desktop-input` example uses the public facade:

```sh
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_desktop_input -- --headless
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_desktop_input
```

The native monitor runs with `WindowedApplication::without_renderer()` and prints every event. F1 requests confinement, F2 locking,
F3 release and Escape exits. F4 selects aggregate keyboard/mouse and stops raw
observation. The first F5 enables raw observation; use a device, then press F5
again to cycle observed connections. F6 rebinds the next key/button on the selected source. Space
or the left aggregate mouse button activates the default action; raw selection
defaults to Space/raw button 0. Selection loss stays visible without fallback.
`--smoke` asserts native capability publication, requests confinement, locking and
release, waits for each feedback event, then exits.
The headless mode validates layout-dependent Cyrillic identity, repeats, ordered
press/release, pixel wheel events, modifier cancellation and one-shot requests.
Engine tests additionally exercise every physical key conversion, logical/dead/
native keys, capture failure rollback and unfocused rejection, held-state cleanup,
frame boundaries and wheel units/phases. The headless mode additionally exercises
raw selection isolation, rebinding, disconnection and aggregate action taps.
`--smoke-raw` separately validates default-off state, explicit enable feedback and
stop/empty-inventory feedback through the native runtime without requiring focus.

Windows compilation, automated contract tests and native capture smoke are local
validation. Windows native smoke applied confinement, locking and release successfully. Interactive
keyboard layouts and hardware wheel gestures require manual device testing. Linux/macOS native execution,
native delivery latency and cross-platform measurements
remain unvalidated; no universal platform or device validation is claimed.

## Input burst performance envelope

`InputBuffer` queues owned events without a full event clone and transfers the event queue into each
snapshot; held/preedit state remains independent. A replacement queue reserves
space for the preceding frame's event count. Empty frames do not retain a historical
queue-capacity high-water mark. Snapshots and their clones remain caller-owned;
retaining them can grow application memory. No event truncation or coalescing is
introduced. Larger sustained queues, heap peaks, many distinct held keys and native
OS/clipboard delivery latency remain unmeasured. See [PERFORMANCE.md](PERFORMANCE.md) for the measured burst envelope.
