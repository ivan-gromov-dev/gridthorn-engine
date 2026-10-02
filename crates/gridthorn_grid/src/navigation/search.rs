use std::collections::{BTreeMap, BTreeSet};

use super::{NavigationBounds, NavigationError};
use crate::GridCell;

/// Terminal search outcome; exhausting a budget does not prove unreachability.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PathStatus {
    /// A minimum-cost route was settled.
    Found,
    /// The reachable frontier was exhausted.
    Unreachable,
    /// Work stopped before settling the next cell.
    BudgetExceeded,
}

/// Immutable query result and projection-independent diagnostic visualization data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PathSearch {
    /// Reason the search stopped.
    pub status: PathStatus,
    /// Start-to-goal cells, including endpoints; empty unless found.
    pub path: Vec<GridCell>,
    /// Entry-cost sum excluding the start; present only for a found route.
    pub cost: Option<u64>,
    /// Settled cells and minimum costs, in deterministic expansion order.
    pub visited: Vec<(GridCell, u64)>,
    /// Discovered unsettled cells and tentative costs, sorted by cost then cell.
    pub frontier: Vec<(GridCell, u64)>,
}

/// Searches four orthogonal neighbors with deterministic Dijkstra ordering.
///
/// `entry_cost` returns `None` for blocked cells or a positive `u32` cost.
/// It must describe an immutable terrain/occupancy snapshot for the whole query.
/// Neighbors and equal-cost ties use `(column, row)` order. Equal-cost routes
/// retain the first predecessor. At most `max_visited` cells are settled, with
/// at most four neighbor probes per expansion plus endpoint validation.
/// No terrain, occupancy, world, or presentation data is mutated.
///
/// # Errors
/// Rejects invalid endpoints, encountered zero costs, and accumulated cost overflow.
pub fn search_path(
    bounds: NavigationBounds,
    start: GridCell,
    goal: GridCell,
    max_visited: usize,
    entry_cost: impl Fn(GridCell) -> Option<u32>,
) -> Result<PathSearch, NavigationError> {
    for cell in [start, goal] {
        if !bounds.contains(cell) {
            return Err(NavigationError::OutsideBounds(cell));
        }
        if terrain_cost(cell, &entry_cost)?.is_none() {
            return Err(NavigationError::BlockedEndpoint(cell));
        }
    }
    let mut queue = BTreeSet::from([(0_u64, start)]);
    let mut costs = BTreeMap::from([(start, 0_u64)]);
    let mut parents = BTreeMap::new();
    let mut result = PathSearch {
        status: PathStatus::Unreachable,
        path: Vec::new(),
        cost: None,
        visited: Vec::new(),
        frontier: Vec::new(),
    };
    while let Some(&(cost, cell)) = queue.first() {
        if result.visited.len() == max_visited {
            result.status = PathStatus::BudgetExceeded;
            break;
        }
        queue.pop_first();
        result.visited.push((cell, cost));
        if cell == goal {
            result.status = PathStatus::Found;
            result.cost = Some(cost);
            let mut cursor = goal;
            result.path.push(cursor);
            while let Some(&parent) = parents.get(&cursor) {
                result.path.push(parent);
                cursor = parent;
            }
            result.path.reverse();
            break;
        }
        for neighbor in neighbors(cell).filter(|neighbor| bounds.contains(*neighbor)) {
            let Some(step) = terrain_cost(neighbor, &entry_cost)? else {
                continue;
            };
            let candidate = cost
                .checked_add(u64::from(step))
                .ok_or(NavigationError::CostOverflow(neighbor))?;
            if costs.get(&neighbor).is_some_and(|old| *old <= candidate) {
                continue;
            }
            if let Some(old) = costs.insert(neighbor, candidate) {
                queue.remove(&(old, neighbor));
            }
            parents.insert(neighbor, cell);
            queue.insert((candidate, neighbor));
        }
    }
    result.frontier = queue.into_iter().map(|(cost, cell)| (cell, cost)).collect();
    Ok(result)
}

fn terrain_cost(
    cell: GridCell,
    query: &impl Fn(GridCell) -> Option<u32>,
) -> Result<Option<u32>, NavigationError> {
    match query(cell) {
        Some(0) => Err(NavigationError::ZeroCost(cell)),
        cost => Ok(cost),
    }
}

fn neighbors(cell: GridCell) -> impl Iterator<Item = GridCell> {
    [(-1, 0), (0, -1), (0, 1), (1, 0)]
        .into_iter()
        .filter_map(move |(column, row)| {
            Some(GridCell::new(
                cell.column.checked_add(column)?,
                cell.row.checked_add(row)?,
            ))
        })
}
