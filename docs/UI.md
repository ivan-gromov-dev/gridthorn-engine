# Provisional UI composition and controls

Implemented on 2026-10-03 through `gridthorn::ui`. These presentation APIs are
pre-1.0 and provisional. `gridthorn_app` owns the reusable composition service;
the facade only re-exports it. No new dependency edge or backend was added.

## Composition and sizing

`UiNode` owns a stable caller-chosen `UiNodeId`, control, style, visual state,
requested scroll offset and ordered children. Only panels own child nodes.
`UiTree::new` validates the complete tree; `replace` atomically replaces it.
Invalid IDs, sizes, control data and commands return `UiCompositionError`.
`set_style(id, style)` atomically replaces one node's validated style.
Node reads are immutable. Change values with explicit commands or replace the
composition; layouts never silently follow subsequent edits.

`UiTree::layout(logical_viewport, dpi, font_service)` measures, arranges and
prepares an immutable `UiLayout`. Placements retain border/content boxes,
effective ancestor clips, content extents and clamped scroll offsets. Submit
`layout.into_primitives()` through `RenderFrame::with_ui`. Recompute after a
viewport, theme, content, control or DPI change. A zero viewport produces no
paint; no window, GPU, world or simulation is required for layout.

Arrangement reuses asset-font measurements for identical text and effective
wrapping width within that pass. One immutable theme/font service owns the pass;
the cache is discarded before layout returns and stores at most 1024 entries
with at most 1 MiB of copied UTF-8 keys (excluding map metadata).
Further measurements still execute normally after the limit. Bitmap measurement,
paint and editing geometry retain their existing behavior.

Router layout prepares field geometry once for hit testing, native text anchors
and focused caret/selection decoration. Active preedit still prepares its separate
composition text; changed field content, placement or theme requires fresh layout.

`UiLength` supports intrinsic `Auto`, fixed `Pixels`, parent-relative `Fraction`
(0–1) and `Fill`. Minimum/maximum sizes include padding. Fractions resolve
against available parent content, even in auto parents; this is not a cyclic
percentage constraint solver. Auto overflow remains available for scrolling.
Padding uses left/top/right/bottom logical pixels. Children compose as:

| Flow | Placement |
| --- | --- |
| Overlay | Children share the inner box; both anchors apply |
| Row | Left-to-right declaration order; vertical anchor applies |
| Column | Top-to-bottom declaration order; horizontal anchor applies |

Rows/columns subtract non-fill children and gaps, then assign equal remaining
shares to fill children. Each share obeys its own min/max; unused space after a
maximum clamp is not redistributed. Minimum sizes may overflow a small viewport.
Offsets apply after anchoring and do not alter flow advance. Declaration order
defines painter order and reverse hit-test priority.

## Clipping and scrolling

`clip` constrains control content and descendants to the content box. `scroll`
implies that clip and applies a requested nonnegative two-axis offset, clamped
to the current extent minus the viewport. Requested offsets remain in the tree;
the layout exposes effective offsets so callers can build scrollbars or map wheel
input. Backgrounds remain stationary. Lists expose their entire row extent.
Scrollable overlay children should use start anchors when content overflows;
negative anchored overflow is not reachable with nonnegative scrolling.

The renderer's `UiPrimitive::Clipped` supports nested physical-pixel rectangles
around ordered rectangles, bitmap text and asset-font text. Geometry clips every
colored rectangle/sample before GPU submission. Parent clips intersect child
clips without reordering paint. Empty/disjoint intersections produce no visible
geometry. Rectangular clipping has no texture-mask, rounded-corner or rotation
contract. Fully clipped geometry may remain as degenerate triangles.

## Styling and reusable controls

`UiTheme` supplies normal/hovered/pressed/disabled surfaces, accent, foreground,
list row height and text metrics. Nodes can override foreground/background.
`UiVisualState` can be supplied explicitly. `UiRouter` derives hover/focus/held
visuals for enabled interactive nodes; disabled state remains caller-owned.
Per-control values are owned by the tree.

| Control | Content and command behavior |
| --- | --- |
| Label | Display-only UTF-8 caption |
| Button | Caption; `Activate` returns `UiEffect::Activated` |
| Toggle | Caption/check indicator; activate inverts, or set checked explicitly |
| Slider | Finite continuous range, colored track and thumb; finite set values clamp |
| List | Ordered rows, fixed themed height and optional single selection; invalid indices reject |
| TextField | UTF-8 committed value or placeholder; explicit value commands plus routed grapheme editing/selection |

Value changes return `UiEffect::Changed`; identical values return `None`.
Visual and scroll commands return `None` because they do not activate or change
a control value, even when the prepared presentation changes.
Disabled controls ignore value/activation commands. Visual and scroll commands
remain available. Invalid commands preserve all existing state. Text values are
limited to 65536 bytes; list lengths and total node counts to 4096, and tree depth
to 64 levels. Metrics/extents are bounded to 65536 logical pixels, positions to
one million logical pixels, and DPI to (0, 8]. Font-service errors retain their
original context. Asset-font theme validation occurs when layout calls the text
service, including missing family, invalid font metrics and missing service.

Supply `UiTheme::text` and a `TextSystem` for multilingual shaping, wrapping,
fallback and DPI rasterization under the [text contract](TEXT.md). Node width
controls paragraph wrapping; theme paragraph width is replaced by content width.
Lists clip captions to individual rows. The default bitmap theme is for ASCII
diagnostics: it preserves authored newlines, has no automatic wrapping, and uses
the existing unsupported-character replacement glyph. Font assets are game-owned.

## Input and authoritative state

Implemented provisionally on 2026-10-03. `UiRouter::new(first_clipboard_id)` owns
presentation focus, pointer capture, key/button ownership, modifiers and a focused
field editor. Call `route(tree, layout, input)` during `Input`, before mapping world
commands; `route_events` provides the same contract for injected ordered events.
Both leave the raw snapshot unchanged. A rejected batch preserves the tree/router
and returns no platform side effects. Successful effects retain event order.

`UiRoute::world_events` contains only unconsumed events; `consumed` contains their
original stream indices. Map only remaining events into one-shot world commands.
For continuous bindings using held state in the original `InputState`, also obey
`keyboard_blocked` and `pointer_blocked`. Focus blocks keyboard bindings; UI-owned
held keys remain blocked through their release after Escape. Hover/capture and
UI-owned held pointer buttons block pointer bindings. Consumption is explicit:
the service never mutates gameplay input or enqueues authoritative commands.
`FocusLost` remains available to the game as a cancellation signal.

The game inspects `(UiNodeId, UiEffect)` values and enqueues `GameCommand` if an
action changes authoritative state. UI stays active while fixed simulation is
paused. The older independent mouse `UiButton` remains available.

## Hit testing, focus and navigation

`UiRouter::hit_test` takes logical pixels. Routed physical cursor positions convert
using the supplied layout's DPI. Hit tests use half-open border boxes intersected
with ancestor clips, in reverse painter order. Panels and labels pass through;
disabled interactive controls block pointer events without accepting focus or
activation. Focus candidates require a nonempty visible box and an enabled control.
Clicks outside interactive controls clear focus and pass to the world.

An inside primary press focuses and captures its control. Buttons/toggles activate
once on an inside release; release outside or after `CursorLeft` cancels activation.
Slider dragging clamps even outside its box; text dragging extends selection.
The release of a UI-owned button is consumed even outside. Local capture does not
request native confinement/locking. Native focus loss, disabling/removing the owner
or `UiNavigation::Cancel` cancels capture; focus loss also clears held ownership.
Replacing a focused text field with another control kind closes its text session.

Wheel input goes to the top hit control's nearest scrollable ancestor, or a hit
scrolling panel when no interactive control is present. Overlay controls prevent
scrolling unrelated controls underneath. Line deltas use themed row height; pixel
deltas divide by DPI. Positive deltas reduce offsets; both axes clamp to extents.
Multiple events accumulate in arrival order. Recompute layout after scrolling.

Tab/Shift+Tab wraps declaration-order focus, skipping disabled and fully clipped
controls. Enter/Space activates buttons/toggles on a nonrepeat press. Arrows choose
spatial focus, with declaration order resolving equal distances; sliders adjust
by one percent of their range with Left/Right, and lists move one row with Up/Down.
Escape clears focus. `navigate` accepts the same `UiNavigation` commands from a
game/controller adapter. Controller discovery and native buttons/axes remain
Milestone 5; these are device-independent navigation hooks.

## Context menus, popups and dialogs

Implemented provisionally on 2026-10-03. `UiLayer` configures `modal`,
`dismiss_escape` and `dismiss_outside`; all default to false. Roots are direct
child panels with globally unique IDs. Use an overlay tree root and position
panels explicitly. Menu items and dialog content use existing controls;
trigger positioning, secondary-click opening and action-driven closing are
application policy.

Register initially closed panels with `UiRouter::register_layer` before first
presentation. `open_layer` opens/reopens a panel above existing layers, remembers
focus, cancels capture and focuses its first enabled visible control. Supply a
fresh **tree** layout when opening: router layouts omit closed panels. Invalid
or duplicate opens reject without mutation. Use `UiRouter::layout` for rendering
and routing: it omits closed registered panels and paints open panels in opening
order, preserving subtree order. Recompute after layer changes. Hidden panels
still participate in tree measurement, so overlay composition is recommended.
Roots must remain direct child panels on replacement.

The highest modal layer and layers above it form the active input/focus scope.
Tab and spatial navigation cannot reach underlying controls. Empty modal space
blocks pointer/keyboard events and continuous world bindings even without a
focusable control. Nonmodal layers allow interaction outside their root boxes;
their clipped backgrounds block underlying pointer targets and scrolling.
`hit_test_layers` exposes the same stack-aware picking for caller-owned queries;
the older static `hit_test` remains a single-tree query.
Held key/button ownership survives closing through release. Platform feedback
remains routable; native `FocusLost` remains a world cancellation signal and
clears focus/capture without dismissing layers.

Only the top layer handles dismissal. A nonrepeat Escape press closes it when
enabled; otherwise Escape is consumed and focus remains. IME preedit cancels
before dismissal through the existing native stop handshake.
`UiNavigation::Cancel` follows the same policy. Outside dismissal uses any
pointer-button press outside the clipped root box, consumes that press and its
release, and never activates underlying controls. Each press closes at most one
layer. `UiRoute::dismissed` reports roots in event order. `close_layer` closes
the top layer regardless of policy. Closing restores prior focus only when still
enabled, visible and inside the surviving modal scope; otherwise focus clears.
Removed top roots reconcile on routing. Forward platform requests from opening
and closing just as from routing, including text-session stop/start operations.

The public `composed-controls --layers --headless` workflow covers nested modal
and context panels, DPI, painter order, dismissal, blocking and focus restoration.
`--layers --smoke` renders both through the native runtime; interactive `--layers`
supports Escape/outside-click closing. Domain tests also cover invalid opens,
disabled restoration targets, hidden panels, background blocking, held releases
and IME cancellation priority. Large-stack allocation/latency, Linux/macOS native
behavior, accessibility, automatic trigger placement, menu-specific arrow/submenu
semantics and automatic layer enter/exit choreography remain unmeasured or deferred.

## Presentation animation

Implemented provisionally on 2026-10-03 through `gridthorn::ui`. `UiTween<N>`
interpolates finite scalar/vector channels with `Linear`, quadratic `EaseIn` /
`EaseOut`, or cubic `SmoothStep` easing. Easing is bounded and monotonic, with
exact endpoints and no overshoot. Caller-supplied `Duration` advances time;
zero duration immediately samples the destination and large deltas clamp there.
Intermediates use double precision to avoid overflow between finite f32 endpoints.

`UiTransition` applies one `UiProperty` to a stable `UiNodeId`: logical-pixel
`Offset`, fixed pixel `Size`, linear RGBA `Foreground` / `Background`, or requested
`Scroll`. Supply both endpoints explicitly, including colors resolved by game
policy from its theme/visual state. Size replaces auto/fraction/fill policies;
layout still applies min/max constraints. Scroll requires a scroll-enabled node
and layout clamps the requested offset to its content extent. Geometry endpoints
obey existing 65536-pixel bounds (offsets may be negative); RGBA channels must be
finite and between zero and one. Color alpha affects only that override, not a
whole subtree's opacity. Other style fields and control values are preserved.

Constructing a transition does not edit the tree. `advance(tree, Duration::ZERO)`
installs the start or zero-duration destination; subsequent calls return whether
it is finished. Retarget from the current sample without a positional jump;
the new duration and easing restart. Local pause/resume ignores paused deltas
and preserves pause through retargeting. Drop a transition to cancel at its last
applied value. Completed transitions reapply their exact endpoint until removed.
Invalid endpoints/kinds reject without changing animation state. Unknown nodes
or invalid destination operations preserve the tree and elapsed animation time,
with node-specific `UiAnimationError` diagnostics. External style edits are not
read back into retargeting; the caller owns conflicting writes and ordering.

Advance during `Update` using `FrameTiming::frame_elapsed()`, the **unscaled host
duration**, including paused simulation frames. Do not use fixed-tick duration,
tick index or simulation-scaled time. Prepare a fresh layout after applying
samples, before painting or routing; refresh text-session anchors through an
empty routed batch when focused text geometry moved. `run_frame(fixed_steps)`
supplies no elapsed host time; use `run_timed_frame` or an explicitly injected
presentation delta for headless animation. Tweens do not read an ambient clock,
modify simulation controls, enqueue commands or require a window/GPU.

Games own enter/exit choreography: open a layer before animating entry, keep it
open while animating exit, then explicitly close it. Focus, modal blocking and
dismissal continue to follow the router's layer contract throughout; no hidden
delayed close or animation-driven focus changes occur. Keyframe timelines,
repeat/yoyo playback, springs, transforms, subtree opacity and automatic
style-state transitions remain deferred.

Domain tests cover easing, time partitioning, zero/large deltas, finite extremes,
pause, interruption, property application, validation and atomic failure. The
public facade lifecycle test animates during simulation pause and speed changes
and prepares a render frame. `composed-controls --animations --headless` covers
control and panel transitions, background alpha, interruption and DPI 1/2;
`--animations --smoke` and `--layers --animations --smoke` exercise native rendering.
Both Windows workflows passed 120-frame lifecycle smoke on 2026-10-03. This proves
native submission/shutdown, not visual acceptance, interactive device behavior or
Linux/macOS support. Per-frame layout allocation and animation performance remain
unmeasured.

## Text editing and platform integration

Focused fields consume keyboard/text events separately. Committed text comes only
from `TextInputEvent::Commit`; logical characters never become inserted text.
Left/Right and Backspace/Delete operate on extended Unicode grapheme clusters,
including combining sequences and joined emoji. Shift extends selection; Up/Down
uses laid-out line boxes; Home/End selects document endpoints. Horizontal movement
is logical byte order, including bidi text. `UiSelection` retains anchor/caret byte
offsets and validates grapheme boundaries. `select` rejects invalid selections.
Insertion replaces selection; new grapheme boundaries are recomputed after merging.
External value commands reset stale selection to the new end on the next route.
`PopText` remains the legacy scalar command; the router uses grapheme operations.

Control/Super+A selects all; C copies, X cuts and V pastes. Alt excludes these
shortcuts to preserve AltGr. Reserve a unique increasing clipboard-ID range for
each router. Forward `UiPlatformRequest::Clipboard` to `Clipboard::read/write` and
route its feedback. Cut deletes only after successful write. Paste/cut replies
apply only while focus, selected range and committed value still match the request;
unrelated replies remain world events. Matched failures are returned in
`UiRoute::clipboard`, preserving content. Only the latest pending edit operation
is accepted; older replies are available to the caller without modifying text.

IME preedit is separate from the committed value. `preedit` and
`composition_cursor` expose text and validated native byte endpoints. Preedit
blocks editing/navigation keys; Escape cancels composition before clearing focus.
Cancellation followed by commit in the same batch still commits once. Focus loss
cancels preedit and text ownership. Native session/clipboard errors remain explicit
input feedback under [INPUT.md](INPUT.md).

Forward `UiPlatformRequest::Text` to `TextInput::start/stop` in result order.
The last request in a frame wins. Start carries a physical-pixel caret anchor.
When transferring an existing text session to another field, or cancelling native
preedit with Escape, the router emits Stop and waits for `TextInputChanged` with
`active: false` before reopening. Late commits during that wait are consumed and
discarded. Custom/headless adapters must return the same stop feedback; Start must
not overwrite Stop in that frame and leave the old native composition alive.
After value, viewport, font or scroll changes, prepare a fresh layout and route an
empty batch to refresh focus/anchor requests; stale text geometry never supplies
an anchor for a new value. `UiRouter::layout` prepares control visuals, clipped
selection/caret and underlined preedit, including its native cursor/selection.
Asset fonts use shaped clusters, wrapping and directional glyph coordinates;
ligature-internal graphemes divide the cluster advance evenly. The bitmap fallback
uses its existing scalar advances. Selection paint follows each visual segment.

## Evidence and limits

Domain tests cover row fill/padding/gaps, fraction sizing, anchors, resize,
intrinsic content, nested clip/scroll, list extents, zero viewport, atomic
replacement/commands, disabled controls and invalid metrics/ranges/IDs/depth.
Facade tests compose a render frame and validate multilingual wrapping and DPI
invariance with Cyrillic, Arabic, Japanese and combining text. Renderer tests
check clipping geometry and paint order. The sibling
`composed-controls` example presents all six controls through the public facade,
with routed pointer/keyboard input, text/clipboard forwarding and a headless
resize/DPI/scroll/routing/Unicode-editing workflow. Facade tests execute routing
before fixed work and rendering, and exercise Cyrillic/Arabic/Japanese fields,
combining marks, ligatures and multiline text at DPI 2. Domain tests cover short
clicks, outside/window-exit release, capture, disabled overlays, wheel accumulation,
focus traversal/cancellation, owned key releases, selection replacement, grapheme
merging, IME cancellation/commit, clipboard failures/stale replies and atomic rollback.

The `--smoke` workflow passed on the available Windows host on 2026-10-03,
creating a native window, submitting 120 frames and shutting down successfully.
The routing version also passed the Windows 120-frame native smoke on 2026-10-03.
The nested layer version passed the Windows 120-frame native smoke on 2026-10-03.
This verifies lifecycle execution, not visual or interactive input acceptance.
Linux/macOS rendering, native interactive
language/IME behavior, accessibility, large-tree performance,
layout caching, virtualization, flex/grid constraint solving, border/radius/shadow
styling and live font reload integration remain unvalidated or deferred.
Full editor extensions (undo/redo, word/double-click navigation, bidi visual-arrow
affinity, exact font-provided ligature carets, automatic caret/list reveal and caret
blinking) remain deferred. Ordered modal layers and focus restoration are
implemented provisionally, along with explicit presentation-property transitions.
Routing clones bounded presentation state for atomic failure handling; allocation,
large-field latency and repeated shaping costs have not been measured.

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_composed_controls --locked -- --headless
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_composed_controls --locked -- --smoke
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_composed_controls --locked -- --layers --headless
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_composed_controls --locked -- --layers --smoke
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_composed_controls --locked -- --animations --headless
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_composed_controls --locked -- --animations --smoke
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_composed_controls --locked -- --layers --animations --smoke
```

## Integrated workbench

Router layout now measures/arranges the tree, filters and orders managed layers,
then prepares field geometry and paints once. Closed layers retain their layout
participation but produce neither paint nor editing geometry. Direct tree layout
continues to prepare every authored layer. The first Milestone 4.5 optimization
and CPU before/after samples are recorded in the
[performance checkpoint](work-in-progress/milestone-4-5.md); native frame-budget
acceptance remains outstanding.

The sibling [multilingual-workbench](../../gridthorn-examples/multilingual-workbench/README.md)
combines four Fluent catalogs, multilingual editing, all six controls, reusable modal/
context actions and unscaled presentation transitions in one interactive application.
Its headless and native smoke workflows share the application's tree and actions.
The README records a language/platform matrix and manual native IME acceptance steps;
interactive native acceptance remains outstanding and Milestone 4 stays open.

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_multilingual_workbench --locked -- --headless
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_multilingual_workbench --locked -- --smoke
```
