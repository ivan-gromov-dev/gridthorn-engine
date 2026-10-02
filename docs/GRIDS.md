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
measurements are explicitly deferred. Placement, navigation, elevation, and
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
