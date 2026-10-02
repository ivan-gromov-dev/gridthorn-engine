# Provisional grid coordinate contract

Milestone 3 starts with `gridthorn::grid`, enabled by the `grid` Cargo feature.
The dependency-free-of-platform `gridthorn_grid` subsystem owns the implementation;
the SDK facade is the application entry point. It is an optional standard grid
module with no lifecycle registration yet, rather than a core world service.
No third-party math types cross this API. All values are `Send + Sync`.

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
measurements are explicitly deferred. Tilemaps, layers, chunks, camera-aware
picking, placement, navigation, elevation, and plugin registration remain planned.
