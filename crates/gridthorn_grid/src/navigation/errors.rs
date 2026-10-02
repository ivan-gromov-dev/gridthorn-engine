use crate::GridCell;
use thiserror::Error;

/// Invalid navigation query or terrain cost.
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum NavigationError {
    /// Inclusive minimum must not exceed maximum on either axis.
    #[error("invalid navigation bounds {min:?}..={max:?}")]
    InvalidBounds { min: GridCell, max: GridCell },
    /// Both endpoints must belong to the search rectangle.
    #[error("navigation endpoint {0:?} is outside search bounds")]
    OutsideBounds(GridCell),
    /// Both endpoints must be traversable.
    #[error("navigation endpoint {0:?} is blocked")]
    BlockedEndpoint(GridCell),
    /// Traversable terrain requires strictly positive costs.
    #[error("navigation cell {0:?} has zero entry cost")]
    ZeroCost(GridCell),
    /// Accumulated path cost exceeds the supported integer range.
    #[error("navigation path cost overflows at {0:?}")]
    CostOverflow(GridCell),
}
