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
UI clipping is supplied by composition or ordered `UiPrimitive::Clipped` groups.
Adjacent identical
raster samples merge into horizontal spans through the existing colored pipeline.
Alpha coverage multiplies the caller's color alpha. Logical layout is independent
of raster scale: do not multiply the layout width or font size by DPI.

`rasterize_clipped(layout, scale_factor, color, clip)` avoids constructing draw
spans for glyph images wholly outside a physical-pixel `UiRect` relative to the
layout origin. Its rectangle color is ignored. Measurement remains unchanged;
partially intersecting glyphs and a conservative two-pixel edge margin remain
intact. Submit the result within the same clip for exact edges; this operation
does not crop the snapshot to the rectangle. UI labels and focused preedit use
their effective ancestor/content clip through this path. All glyph images still
count toward the same one-million-sample guard, including invisible glyphs.
Foreign layouts, invalid DPI, glyph failures and oversized requests retain the
normal errors and last-good snapshot behavior. Glyph image lookup/raster cache
population, full shaping and editing geometry are still performed.

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

## Limits and storage lifetimes

Coverage depends on authored fonts. Vertical text, rich styled spans and broad
emoji/palette coverage are deferred. UI text editing is a separate [UI contract](UI.md).
Windows native rendering/IME has manual acceptance; Linux/macOS rendering and
multi-monitor DPI transitions remain unvalidated. [PERFORMANCE.md](PERFORMANCE.md)
describes timing envelopes without an engine-wide frame guarantee.

Unchanged shared raster snapshots reuse colored/UI geometry and an immutable GPU
vertex buffer. Dirty geometry reuses CPU vector capacity. One high-water vector
and input snapshot remain until surface reconfiguration or shutdown. Changed
raster storage, placement, DPI, clipping/order, camera, sprites or overlay invalidate reuse.

Each text service retains an LRU of at most 64 shaped layouts keyed by complete
text/style. Logical layout reuse is independent of raster DPI/color. Cached and
returned layouts share immutable shaping/diagnostic storage; eviction does not
invalidate returned layouts. The cache belongs to that service's fonts/locale;
replacing the service after font reload starts fresh. Retention caps are 256 KiB
of copied keys, 16384 diagnostic glyphs and 4096 lines. Oversized entries are not
retained. Opaque backend/font buffers and caller-held layouts are outside these caps.
`clear_raster_cache` clears raster data while retaining shaped layouts.

Glyph-relative tinted spans are reused across raster calls, capped at 128 entries
and 65536 retained spans. Keys include backend font/glyph identity, physical size
and fractional positioning. Changing tint discards the previous set;
`clear_raster_cache` clears backend images and spans. Saturated entries use direct
sampling; saturated spans do not retain the new glyph. Scratch storage is released
before snapshot construction. The one-million image-sample work limit still applies.
Map metadata, vector capacity and opaque backend caches are outside the count cap.
Failed raster requests discard the active span set and preserve previous snapshots.

Raster snapshot clones share an immutable construction vector, including spare
capacity, until the last clone drops. Clearing service caches does not release live
snapshots. Unique long Japanese fallback shaping can exceed a frame budget; warm
matching layouts avoid shaping. Large DPI-2 requests can reject `TooLarge`.

The multilingual-text sibling example supplies headless shaping/rasterization
checks and a native smoke path. Opt-in `GRIDTHORN_TEXT_PERFORMANCE` collects bounded
successful layout/raster timings, call totals and sizes; GPU timestamps describe
render-pass execution only. These diagnostics are not precise whole-process CPU
or displayed-frame latency measurements.
