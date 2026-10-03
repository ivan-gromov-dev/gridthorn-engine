# Provisional UI composition and controls

Implemented on 2026-10-03 through `gridthorn::ui`. These presentation APIs are
pre-1.0 and provisional. `gridthorn_app` owns the reusable composition service;
the facade only re-exports it. No new dependency edge or backend was added.

## Composition and sizing

`UiNode` owns a stable caller-chosen `UiNodeId`, control, style, visual state,
requested scroll offset and ordered children. Only panels own child nodes.
`UiTree::new` validates the complete tree; `replace` atomically replaces it.
Invalid IDs, sizes, control data and commands return `UiCompositionError`.
Node reads are immutable. Change values with explicit commands or replace the
composition; layouts never silently follow subsequent edits.

`UiTree::layout(logical_viewport, dpi, font_service)` measures, arranges and
prepares an immutable `UiLayout`. Placements retain border/content boxes,
effective ancestor clips, content extents and clamped scroll offsets. Submit
`layout.into_primitives()` through `RenderFrame::with_ui`. Recompute after a
viewport, theme, content, control or DPI change. A zero viewport produces no
paint; no window, GPU, world or simulation is required for layout.

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
defines painter order; layouts do not provide event arbitration.

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
`UiVisualState` is explicit caller input; this increment does not infer hover,
focus or capture from native events. Per-control state is owned by the tree.

| Control | Content and command behavior |
| --- | --- |
| Label | Display-only UTF-8 caption |
| Button | Caption; `Activate` returns `UiEffect::Activated` |
| Toggle | Caption/check indicator; activate inverts, or set checked explicitly |
| Slider | Finite continuous range, colored track and thumb; finite set values clamp |
| List | Ordered rows, fixed themed height and optional single selection; invalid indices reject |
| TextField | UTF-8 committed value or placeholder; replace, append and remove final scalar |

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

`UiCommand` is an explicit presentation boundary. The game maps its own input
into commands, inspects effects and enqueues `GameCommand` values if an action
changes authoritative state. UI stays active when fixed simulation is paused.
Existing independent mouse `UiButton` remains available and unchanged.

Automatic event routing/consumption, hit-test arbitration, keyboard/controller
navigation, focus, pointer capture, IME/clipboard integration, caret/selection,
grapheme-aware editing, modal layers and transitions belong to the subsequent
roadmap items. `PopText` removes one Unicode scalar; it is not a complete editor.
Text fields in this increment provide reusable value/placeholder presentation
and explicit committed-value operations, without promising native editing.

## Evidence and limits

Domain tests cover row fill/padding/gaps, fraction sizing, anchors, resize,
intrinsic content, nested clip/scroll, list extents, zero viewport, atomic
replacement/commands, disabled controls and invalid metrics/ranges/IDs/depth.
Facade tests compose a render frame and validate multilingual wrapping and DPI
invariance with Cyrillic, Arabic, Japanese and combining text. Renderer tests
check clipping geometry and paint order. The sibling
`composed-controls` example presents all six controls through the public facade,
with game-owned keyboard commands and a headless resize/DPI/scroll workflow.

The `--smoke` workflow passed on the available Windows host on 2026-10-03,
creating a native window, submitting 120 frames and shutting down successfully.
This verifies lifecycle execution, not visual or interactive input acceptance.
Linux/macOS rendering, native interactive
language/IME behavior, accessibility, text caret semantics, large-tree performance,
layout caching, virtualization, flex/grid constraint solving, border/radius/shadow
styling and live font reload integration remain unvalidated or deferred.

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_composed_controls --locked -- --headless
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_composed_controls --locked -- --smoke
```
