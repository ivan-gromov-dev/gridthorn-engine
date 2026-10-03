# Provisional multilingual text

Implemented on 2026-10-03 for horizontal UTF-8 presentation. The public SDK exposes
`FontAsset`, `TextSystem`, `TextStyle`, `TextLayout` and `RasterText`. Backend types
stay private. The existing `TextLabel` retains its 5×7 diagnostic behavior.

## Font ownership and fallback

`FontAsset::load` / `from_bytes` validate OpenType/TrueType font data and collections,
retain shared immutable bytes and report family names. Files are limited to 32 MiB.
Loading and validation do not initialize a GPU or consult installed system fonts.
Font discovery, bundled default fonts and automatic decoded-font hot reload are
not provided. A raw font source can use the existing `AssetStore`/`AssetReloader`;
the caller validates changed bytes and replaces its text service at a frame boundary.
Old raster snapshots retain their last-good draw data independently.

Construct one `TextSystem` with an explicit locale and all game-owned font assets.
The selected primary family must exist; unknown names return `TextError`.
Contextual script fallback searches only the supplied faces using the backend's
script/locale matching policy. Asset order is not a priority list. Supply the fonts
needed by the game's supported languages and inspect `TextLayout::missing_glyphs`
and resolved glyph families. Unsupported coverage draws the selected font's
`.notdef` glyph and reports glyph index zero; it is not silently replaced with `?`.
Fallback preferences beyond primary family and locale remain a future extension.

## Layout, measurement and drawing

`TextSystem::layout` uses advanced Unicode shaping: contextual glyph substitution,
ligatures, mark positioning, Unicode bidirectional paragraph analysis and visual
reordering. Content remains UTF-8 in logical order; diagnostic glyph cluster ranges
are UTF-8 byte offsets within the original paragraph, with paragraph indices on
visual lines. These diagnostics are not a text-editing/caret API.

`TextStyle` specifies a family, logical font size, line height, optional width,
wrapping and alignment. Default alignment follows the paragraph's base direction.
Word, glyph-cluster, word-or-glyph and authored-break-only wrapping are available.
Explicit line breaks and empty lines retain their vertical space. Word-only mode
can overflow on long words; a glyph cluster can exceed very narrow widths.
Measurement reports maximum line advance and total line-box height in logical
pixels. It is not an ink bounding box; overhangs are retained when drawing.

`rasterize(layout, scale_factor, color)` creates an antialiased immutable snapshot.
Submit `snapshot.at([x, y])?.into()` in `RenderFrame::with_ui`; placement is logical,
then converted to physical pixels using the snapshot's DPI. Text follows ordered
UI primitives above world sprites. Pixels outside the surface are culled; custom
UI clipping/scissors belong to the later layout milestone. Adjacent identical
raster samples merge into horizontal spans through the existing colored pipeline.
Alpha coverage multiplies the caller's color alpha. Logical layout is independent
of raster scale: do not multiply the layout width or font size by DPI.

`WindowScaleFactor` is published before `Startup` and on native DPI changes,
separately from physical `WindowViewport`. Re-rasterize when it changes. If logical
available width changes, lay out again as well. Raster snapshots do not secretly
update on DPI changes. Layouts belong to their originating service; another service
rejects them. Rebuild layout after replacing the font database. Raster snapshots
are cloneable, `Send + Sync`, and remain valid after their service is dropped.
The mutable service serializes cache operations and offers `clear_raster_cache`.

Text requests are limited to 65536 UTF-8 bytes, font size and line height to 1024
logical pixels, layout extent to 65536 logical pixels and DPI scale to `(0, 8]`.
Rasterization rejects work exceeding one million glyph-image samples. Failures
return typed errors without replacing the caller's previous snapshot.

## Public usage

The sibling [multilingual-text example](../../gridthorn-examples/multilingual-text/README.md)
loads licensed Noto fonts, displays Russian, mixed Arabic/Latin/numbers, Japanese,
combining marks and ligatures, and re-prepares text on resize/DPI changes:

```console
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_multilingual_text
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_multilingual_text -- --headless
cargo run --manifest-path ../gridthorn-examples/Cargo.toml -p gridthorn_example_multilingual_text -- --smoke
```

## Evidence and limitations

Domain tests with bundled font fixtures cover Cyrillic, Japanese fallback, Arabic
contextual forms and mixed bidi, combining clusters, ligatures, missing glyphs,
alignment, wrapping, empty lines, DPI re-rasterization, tint alpha, cache rebuilding,
foreign-layout rejection, invalid requests and ordered screen geometry. Tests
also cover bidi formatting controls and raster-budget rollback. Application
tests cover initial/change DPI publication. The headless public example exercises
1×, 1.25×, 1.5× and 2× rasterization without a window or GPU.

Language coverage is limited by authored fonts, not a promise that all scripts
and font formats are validated. Vertical text, rich styled spans, text editing,
selection, emoji coverage/palette behavior and user-configurable fallback priority
remain deferred. Windows native rendering is exercised by the smoke example;
real multi-monitor DPI transitions, Linux/macOS native rendering and IME-driven
visual editing remain unvalidated. Performance, binary size, large font databases,
cache memory and GPU atlas optimization remain under Milestone 4.5 review. Native
CPU samples and renderer snapshot/buffer reuse are recorded in
[the performance checkpoint](work-in-progress/milestone-4-5.md). Unchanged shared
raster snapshots reuse colored/UI geometry and an immutable GPU vertex buffer.
Dirty geometry reuses CPU vector capacity; one high-water vector and one input
snapshot remain until surface reconfiguration or shutdown. Changed raster storage,
placement, DPI, clipping/order, camera, colored sprites and overlay invalidate it.
GPU execution and actual presented intervals remain unmeasured. The span renderer
is a correctness foundation for modest UI text, not a measured high-throughput
text renderer. [ADR 0004](adr/0004-multilingual-text.md) records the provisional backend.
