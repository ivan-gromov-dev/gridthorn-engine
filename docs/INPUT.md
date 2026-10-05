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
native cursor capture; [UI ownership, hit testing and explicit input consumption](UI.md)
are now implemented separately. Gamepads remain Milestone 5 work.

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

## Desktop pointer example and validation scope

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
