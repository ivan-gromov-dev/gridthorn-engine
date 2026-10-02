use super::{TileLayerId, TileMap, TileMapError};
use crate::{GridCell, GridError, GridPoint, GridProjection};

/// Validated orthographic view with top-left screen origin and positive Y down.
/// Pass `Camera2d` center/height and viewport dimensions in the same pixel units
/// as the cursor. This value has no dependency on the renderer or input backend.
#[derive(Clone, Copy, Debug)]
pub struct GridView {
    center: GridPoint,
    extent: f64,
    viewport: GridPoint,
}

impl GridView {
    /// Creates a view from camera center, visible world height, and pixel size.
    ///
    /// # Errors
    /// Rejects a non-finite center or non-positive/non-finite dimensions.
    pub fn new(
        center: GridPoint,
        world_height: f64,
        viewport: GridPoint,
    ) -> Result<Self, TileMapError> {
        finite(center)?;
        if !world_height.is_finite()
            || world_height <= 0.0
            || !viewport.x.is_finite()
            || !viewport.y.is_finite()
            || viewport.x <= 0.0
            || viewport.y <= 0.0
        {
            return Err(TileMapError::InvalidView);
        }
        Ok(Self {
            center,
            extent: world_height,
            viewport,
        })
    }

    /// Converts an inside screen pixel to world units. Outside the half-open
    /// viewport returns `None`; resizing requires constructing a fresh view.
    ///
    /// # Errors
    /// Rejects non-finite input or overflowing conversion results.
    pub fn screen_to_world(self, screen: GridPoint) -> Result<Option<GridPoint>, TileMapError> {
        finite(screen)?;
        if screen.x < 0.0
            || screen.y < 0.0
            || screen.x >= self.viewport.x
            || screen.y >= self.viewport.y
        {
            return Ok(None);
        }
        let scale = self.extent / self.viewport.y;
        let point = GridPoint::new(
            self.center.x + (screen.x - self.viewport.x * 0.5) * scale,
            self.center.y + (screen.y - self.viewport.y * 0.5) * scale,
        );
        finite(point)?;
        Ok(Some(point))
    }
}

/// Borrowed occupied-cell hit; picking never modifies authoritative data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TilePick<'a, T> {
    /// Topmost eligible layer.
    pub layer: TileLayerId,
    /// Global integer cell identity.
    pub cell: GridCell,
    /// Caller-owned tile data.
    pub tile: &'a T,
}

impl<T> TileMap<T> {
    /// Picks the highest visible, pickable occupied layer at a world point.
    /// Empty or excluded layers let the query pass through to lower layers.
    /// This uses grid footprints, not sprite alpha, elevation, or overhangs.
    ///
    /// # Errors
    /// Returns contextual projection errors for invalid or out-of-range points.
    pub fn pick_world(
        &self,
        projection: GridProjection,
        point: GridPoint,
    ) -> Result<Option<TilePick<'_, T>>, TileMapError> {
        let cell = projection.cell_at(point)?;
        Ok(self.layers().rev().find_map(|(id, layer)| {
            if !layer.is_visible() || !layer.is_pickable() {
                return None;
            }
            self.tile(id, cell).map(|tile| TilePick {
                layer: id,
                cell,
                tile,
            })
        }))
    }

    /// Converts a cursor through an orthographic view, then picks a tile.
    /// Outside the viewport or over an empty cell returns `None`.
    ///
    /// # Errors
    /// Returns view conversion or grid projection errors.
    pub fn pick_screen(
        &self,
        projection: GridProjection,
        view: GridView,
        screen: GridPoint,
    ) -> Result<Option<TilePick<'_, T>>, TileMapError> {
        let Some(point) = view.screen_to_world(screen)? else {
            return Ok(None);
        };
        self.pick_world(projection, point)
    }
}

fn finite(point: GridPoint) -> Result<(), TileMapError> {
    if point.x.is_finite() && point.y.is_finite() {
        Ok(())
    } else {
        Err(GridError::NonFinite.into())
    }
}
