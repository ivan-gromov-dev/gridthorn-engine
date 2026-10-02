/// An authoritative cell on an unbounded signed grid.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GridCell {
    /// Column, increasing along the grid's first axis.
    pub column: i32,
    /// Row, increasing along the grid's second axis.
    pub row: i32,
}

impl GridCell {
    /// Constructs a cell without imposing map bounds.
    #[must_use]
    pub const fn new(column: i32, row: i32) -> Self {
        Self { column, row }
    }
}

/// A presentation point in caller-selected units, with positive Y downward.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GridPoint {
    /// Horizontal position.
    pub x: f64,
    /// Vertical position.
    pub y: f64,
}

impl GridPoint {
    /// Constructs a point; projection operations validate finite values.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
