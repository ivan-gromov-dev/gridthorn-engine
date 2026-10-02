# Provisional grid and tilemap contract

Milestone 3 starts with `gridthorn::grid`, enabled by the `grid` Cargo feature.
The dependency-free-of-platform `gridthorn_grid` subsystem owns the implementation;
the SDK facade is the application entry point. It is an optional standard grid
module with no lifecycle registration yet, rather than a core world service.
No third-party math types cross this API. Coordinate/view values are `Send + Sync`;
tilemap storage is `Send`/`Sync` when its caller-owned tile data is.

`GridCell` stores signed `i32` column and row and is suitable for authoritative
cell identity. It has no map bounds, occupancy, elevation, or entity ownership.
`GridPoint` stores `f64` presentation coordinates; these must not become
authoritative simulation state. This slice does not introduce authoritative
floating-point arithmetic or change the determinism contract.

`GridProjection::square(size, origin)` maps `(column, row)` to
`origin + (column * size, row * size)`. `isometric(width, height, origin)` uses
full diamond dimensions and maps to
`origin + ((column - row) * width / 2, (column + row) * height / 2)`.
Positive presentation Y is downward. Column moves down-right and row down-left
in isometric space. The origin is the grid vertex of cell `(0, 0)`;
its isometric center is `origin + (0, height / 2)`.

`cell_vertex` projects integer grid coordinates. `cell_center` projects the
grid coordinates plus `(0.5, 0.5)`. `cell_at` inverts the transform and floors
each axis, so negative positions select negative cells. Cells occupy half-open
intervals on each grid axis: exact shared edges belong to the positive side.
There is no implicit epsilon. Floating-point rounding can affect positions
near boundaries; callers should prefer centers for round-trip identification.
Extreme origins or dimension ratios can lose precision even for finite results.

Typed `GridError` values reject non-finite origins/points/results, non-positive
or non-finite dimensions, and inverse cells outside the `i32` range. Projection
does not clip to a viewport or map. Callers remove camera transforms before
inverse conversion; this is a mathematical coordinate query, not tilemap picking.

The sibling `grid-coordinates` example compiles and runs through the facade.
Tests cover signed center round trips, extreme cell identities at ordinary
dimensions, axis orientation, exact edges, invalid inputs, and overflow.
Performance, presentation precision at large world scales, and cross-platform
measurements are explicitly deferred. Navigation, elevation, and
plugin registration remain planned.

## Sparse tilemaps, layers, and chunks

The same opt-in `grid` feature exports provisional `TileMap<T>`, `TileLayerId`,
`TileLayer<T>`, `ChunkSize`, `GridView`, `TilePick`, and `TileMapError` APIs. No
additional dependency or platform boundary is introduced. These are data/query
services without implicit lifecycle registration. Authoritative map edits belong
at a fixed-tick boundary or an explicit load/reset boundary; presentation queries
borrow tiles without mutating them.

`TileMap<T>` is unbounded over signed `i32` cells and stores caller-owned tile
values, not an SDK-imposed atlas or renderer handle. Empty cells allocate no
storage. `ChunkSize::new(width, height)` validates strictly positive `i32` cell
counts independently of projection dimensions. `locate(cell)` uses Euclidean
division: with 16-wide chunks, column -1 belongs to chunk -1 at local column 15.
Extreme signed cells also work for non-power-of-two chunk dimensions.

Layers must be registered with `add_layer`. Their signed IDs define bottom-to-top
order: larger IDs are above smaller ones, independently of registration order.
Visibility and picking eligibility default to true and can change independently.
`set_tile(id, cell, Some(value))` inserts/replaces and returns the previous value;
`None` removes a tile and releases the last-empty chunk. Duplicate layers and
mutations targeting missing layers return typed errors without changing data.
`tile` returns `None` for empty cells or missing layers. `remove_layer` returns
the removed storage; layer IDs can subsequently be reused.

`layers()` iterates by ID. `chunks()` reports occupied chunk identities and tile
counts in `(column, row)` order. `tiles()` iterates by chunk then local cell;
`chunk_tiles(chunk)` reads one chunk, yielding an empty iterator if absent.
Both return global cell identities. Ordered tree storage makes these traversals
independent of insertion order. Storage order is not isometric sprite depth order.
Generic payloads retain the caller's own determinism and serialization obligations.

## Occupied-cell picking

`pick_world(projection, point)` first resolves the coordinate contract's half-open
cell, then returns a borrowed `TilePick { layer, cell, tile }` from the highest
visible, pickable layer with an occupied cell. Empty, hidden, and unpickable
layers allow pass-through. An empty map/cell returns `None`; invalid coordinates
return a contextual `TileMapError::Projection`. Projection origin places the map;
all layers share that projection and have no independent offsets/elevation.

`GridView::new(center, world_height, viewport)` accepts an orthographic camera's
world center, visible vertical world extent, and viewport pixel width/height
in a `GridPoint`. Pass `Camera2d::center()` and `viewport_height()` converted to
`f64`; this avoids linking the grid subsystem to rendering. `screen_to_world`
uses `world_height / viewport.y` world units per pixel on both axes, matching
the renderer's aspect ratio and positive-down Y convention. Screen origin is
top-left. Callers must supply cursor and viewport dimensions in the same units
(including DPI conversion and viewport-relative coordinates where needed).
Construct a new view after camera movement, zoom, or viewport resize.

`pick_screen(projection, view, cursor)` converts an inside cursor and performs
the same occupied-cell query. The viewport is half-open: negative positions and
positions on its right/bottom edges return `None`. Non-finite cursor/center,
invalid extents, and non-finite conversion results are errors. There is no
implicit epsilon, clamping, or authoritative floating-point use.

The sibling `tilemap-basics` example compiles and runs through `gridthorn::grid`,
including `Camera2d` parameter conversion, two projections, roof/ground picking,
negative chunks, and reclamation. Tests additionally cover insertion-order
independence, replacement, partial removal, extreme cells, viewport clipping,
camera pan/zoom/aspect changes, and configuration/conversion failures.

This increment implements tile data and geometric cell picking. Dedicated tile
rendering, atlas binding, culling/streaming, chunk dirty tracking, elevation,
sprite-alpha/overhang picking, bounded maps, and tilemap persistence are deferred.
Applications may extract tiles into the existing sprite API and own draw order.
Large-map memory/performance, binary-size, and cross-platform/precision
measurements are explicitly deferred; no scalability or stronger cross-platform
determinism guarantee is claimed. The APIs remain provisional and introduce no
irreversible format or stable plugin decision.

## Grid-based object placement

The same opt-in `grid` feature now exposes provisional `GridFootprint`,
`GridObjectId`, `GridPlacement`, `PlacementMap`, and `PlacementError`. These are
platform-independent integer data/query services, with no new dependencies or
implicit systems. All placement values are `Send + Sync`.

`GridFootprint::new` validates a nonempty set of unique signed offsets relative
to an anchor. Shapes may have holes and negative offsets, and need not contain
the anchor. `single_cell` occupies only the anchor. `offsets` sorts by
`(column, row)` regardless of construction order. `cells_at` translates the whole
shape using checked integer addition and returns ordered cells or a contextual
overflow error. Games can construct rectangles or rotated shapes as offsets;
there is no projection-specific authoritative geometry.

`PlacementMap` stores exclusive sparse occupancy in ordered trees. Each instance
is an independent occupancy space, unrelated to tilemap layer IDs. Caller-owned
`GridObjectId(u64)` values identify objects without binding them to ECS entity
lifetimes. Games own identity allocation, associated entity data, and cleanup.
`objects` iterates by identity; `placement` reads an immutable anchor/footprint;
`object_at` selects through any occupied footprint cell, including non-anchor
cells. Holes are free for other objects.

`validate(id, anchor, footprint)` returns candidate cells without reserving them
and ignores cells owned by that identity, enabling self-overlapping moves.
It validates all coordinate additions before testing occupancy and reports the
first occupied cell in offset order. It does not require an existing identity.
`place` first rejects duplicate identities; `relocate` first rejects missing
identities. Both then validate the complete candidate before committing. Failed
operations preserve all old object and occupancy data. `relocate` can change both
anchor and footprint. `remove` releases every occupied cell and returns the old
placement, or `None` for a missing identity. Removed identities may be reused.

Terrain eligibility, bounded maps, costs, and game-specific adjacency rules are
caller policy: inspect `cells_at`/`validate` results against tile data before
committing. Preview results are not reservations; revalidate game rules at the
fixed tick and let `place`/`relocate` recheck occupancy at commit. Integrate input
with `GridView::screen_to_world` and `GridProjection::cell_at`, then enqueue the
integer anchor as a game command. Picking an empty cell for construction does
not require a tile hit. Square and isometric projections share identical
authoritative placement semantics. Presentation queries do not mutate storage;
edits belong at fixed-tick or explicit load/reset boundaries.

The sibling `grid-placement` public SDK example runs without a GPU and covers
terrain policy, negative anchors, both projections, preview, selection,
conflicting-move rollback, self-overlapping movement, and removal. Focused tests
also cover invalid footprints, both-axis integer limits, duplicate/missing IDs,
shape replacement, holes, identity reuse, independent spaces, deterministic
iteration/conflicts, and unchanged occupancy after rejected edits.

Automatic ECS/scene synchronization, rotation convenience APIs, placement UI,
reservations, multi-object transactions, occupancy masks/elevation, and placement
persistence remain deferred. Memory, large-footprint/map performance, binary-size,
and cross-platform measurements are explicitly deferred. No scalability or
stronger determinism guarantee is claimed; this increment introduces no stable
format or irreversible architecture decision.
