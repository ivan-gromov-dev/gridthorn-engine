use crate::{GridCell, GridError, GridPoint};

#[derive(Clone, Copy, Debug)]
enum Layout {
    Square,
    Isometric,
}

/// Validated square or diamond-isometric presentation transform.
///
/// The origin projects grid vertex `(0, 0)`. Cells occupy half-open grid
/// intervals `[column, column + 1) × [row, row + 1)` in both layouts.
/// Inverse conversion uses floor, including for negative coordinates.
/// This value owns no platform resources and is `Send + Sync`.
#[derive(Clone, Copy, Debug)]
pub struct GridProjection {
    layout: Layout,
    width: f64,
    height: f64,
    origin: GridPoint,
}

impl GridProjection {
    /// Creates an axis-aligned projection with square cells of the given size.
    ///
    /// # Errors
    /// Returns an error for non-positive/non-finite size or non-finite origin.
    pub fn square(size: f64, origin: GridPoint) -> Result<Self, GridError> {
        Self::new(Layout::Square, size, size, origin)
    }

    /// Creates a diamond projection with the given full tile width and height.
    /// Column advances down-right; row advances down-left. The origin is the
    /// top vertex of cell `(0, 0)`, rather than its center.
    ///
    /// # Errors
    /// Returns an error for invalid dimensions or a non-finite origin.
    pub fn isometric(width: f64, height: f64, origin: GridPoint) -> Result<Self, GridError> {
        Self::new(Layout::Isometric, width, height, origin)
    }

    fn new(layout: Layout, width: f64, height: f64, origin: GridPoint) -> Result<Self, GridError> {
        if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
            return Err(GridError::InvalidDimensions);
        }
        finite(origin)?;
        Ok(Self {
            layout,
            width,
            height,
            origin,
        })
    }

    /// Projects the cell's grid vertex at its integer column and row.
    ///
    /// # Errors
    /// Returns an error if the projected point overflows to a non-finite value.
    pub fn cell_vertex(self, cell: GridCell) -> Result<GridPoint, GridError> {
        self.project(f64::from(cell.column), f64::from(cell.row))
    }

    /// Projects the center at `(column + 0.5, row + 0.5)`.
    ///
    /// # Errors
    /// Returns an error if the projected point is non-finite.
    pub fn cell_center(self, cell: GridCell) -> Result<GridPoint, GridError> {
        self.project(f64::from(cell.column) + 0.5, f64::from(cell.row) + 0.5)
    }

    fn project(self, column: f64, row: f64) -> Result<GridPoint, GridError> {
        let (x, y) = match self.layout {
            Layout::Square => (column * self.width, row * self.height),
            Layout::Isometric => (
                (column - row) * 0.5 * self.width,
                column.midpoint(row) * self.height,
            ),
        };
        let point = GridPoint::new(self.origin.x + x, self.origin.y + y);
        finite(point)?;
        Ok(point)
    }

    /// Finds the cell containing a projected point, without map bounds or camera transforms.
    /// Exact grid edges belong to the cell on the positive side. Floating-point
    /// rounding near edges is not corrected with an implicit epsilon.
    ///
    /// # Errors
    /// Rejects non-finite points/results and cells outside the `i32` range.
    pub fn cell_at(self, point: GridPoint) -> Result<GridCell, GridError> {
        finite(point)?;
        let x = (point.x - self.origin.x) / self.width;
        let y = (point.y - self.origin.y) / self.height;
        let (column, row) = match self.layout {
            Layout::Square => (x, y),
            Layout::Isometric => (x + y, y - x),
        };
        Ok(GridCell::new(cell_axis(column)?, cell_axis(row)?))
    }
}

fn finite(point: GridPoint) -> Result<(), GridError> {
    if point.x.is_finite() && point.y.is_finite() {
        Ok(())
    } else {
        Err(GridError::NonFinite)
    }
}

#[allow(clippy::cast_possible_truncation)]
fn cell_axis(value: f64) -> Result<i32, GridError> {
    if !value.is_finite() {
        return Err(GridError::NonFinite);
    }
    let floor = value.floor();
    if floor < f64::from(i32::MIN) || floor > f64::from(i32::MAX) {
        return Err(GridError::CellOutOfRange);
    }
    Ok(floor as i32)
}

#[cfg(test)]
mod test;
