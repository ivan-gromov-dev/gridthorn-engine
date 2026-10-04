use crate::{GridCell, NavigationBounds, PathStatus, search_path};

#[test]
fn open_grid_ties_preserve_exact_expansion_route_and_frontier() {
    let bounds = NavigationBounds::new(GridCell::default(), GridCell::new(3, 3)).unwrap();
    let mut expected = (0..4)
        .flat_map(|column| {
            (0..4).map(move |row| {
                (
                    GridCell::new(column, row),
                    u64::try_from(column + row).unwrap(),
                )
            })
        })
        .collect::<Vec<_>>();
    expected.sort_by_key(|(cell, cost)| (*cost, *cell));
    for budget in [0, 1, 3, 8, 16] {
        let result = search_path(
            bounds,
            GridCell::default(),
            GridCell::new(3, 3),
            budget,
            |_| Some(1),
        )
        .unwrap();
        assert_eq!(result.visited, expected[..budget]);
        let settled = result
            .visited
            .iter()
            .map(|(cell, _)| *cell)
            .collect::<std::collections::BTreeSet<_>>();
        let mut discovered = std::collections::BTreeSet::from([GridCell::default()]);
        for cell in &settled {
            for (column, row) in [(-1, 0), (0, -1), (0, 1), (1, 0)] {
                let neighbor = GridCell::new(cell.column + column, cell.row + row);
                if bounds.contains(neighbor) {
                    discovered.insert(neighbor);
                }
            }
        }
        let mut frontier = discovered
            .difference(&settled)
            .map(|cell| (*cell, u64::try_from(cell.column + cell.row).unwrap()))
            .collect::<Vec<_>>();
        frontier.sort_by_key(|(cell, cost)| (*cost, *cell));
        assert_eq!(result.frontier, frontier);
        if budget == 16 {
            assert_eq!(
                result.path,
                [(0, 0), (0, 1), (0, 2), (0, 3), (1, 3), (2, 3), (3, 3)]
                    .map(|(column, row)| GridCell::new(column, row))
            );
        } else {
            assert_eq!(result.status, PathStatus::BudgetExceeded);
        }
    }
}

#[test]
fn weighted_route_avoids_expensive_direct_cell_and_is_repeatable() {
    let bounds = NavigationBounds::new(GridCell::new(-1, -1), GridCell::new(1, 1)).unwrap();
    let terrain = |cell| Some(if cell == GridCell::default() { 20 } else { 1 });
    let search = || {
        search_path(
            bounds,
            GridCell::new(-1, 0),
            GridCell::new(1, 0),
            20,
            terrain,
        )
        .unwrap()
    };
    let result = search();
    assert_eq!(result, search());
    assert_eq!(result.status, PathStatus::Found);
    assert_eq!(result.cost, Some(4));
    assert_eq!(
        result.path,
        vec![
            GridCell::new(-1, 0),
            GridCell::new(-1, -1),
            GridCell::new(0, -1),
            GridCell::new(1, -1),
            GridCell::new(1, 0)
        ]
    );
    assert!(!result.path.contains(&GridCell::default()));
}

#[test]
fn unreachable_and_budget_exhaustion_have_distinct_diagnostics() {
    let bounds = NavigationBounds::new(GridCell::new(0, 0), GridCell::new(2, 0)).unwrap();
    let blocked = search_path(
        bounds,
        GridCell::new(0, 0),
        GridCell::new(2, 0),
        10,
        |cell| (cell.column != 1).then_some(1),
    )
    .unwrap();
    assert_eq!(blocked.status, PathStatus::Unreachable);
    assert_eq!(blocked.visited, vec![(GridCell::default(), 0)]);
    assert_eq!(blocked.frontier, []);
    assert_eq!(blocked.path, []);
    assert_eq!(blocked.cost, None);
    let limited = search_path(bounds, GridCell::default(), GridCell::new(2, 0), 1, |_| {
        Some(1)
    })
    .unwrap();
    assert_eq!(limited.status, PathStatus::BudgetExceeded);
    assert_eq!(limited.frontier, vec![(GridCell::new(1, 0), 1)]);
    let zero = search_path(bounds, GridCell::default(), GridCell::default(), 0, |_| {
        Some(1)
    })
    .unwrap();
    assert_eq!(zero.status, PathStatus::BudgetExceeded);
    let same = search_path(bounds, GridCell::default(), GridCell::default(), 1, |_| {
        Some(1)
    })
    .unwrap();
    assert_eq!(same.path, vec![GridCell::default()]);
    assert_eq!(same.cost, Some(0));
}

#[test]
fn coordinate_limits_do_not_wrap() {
    for column in [i32::MIN, i32::MAX] {
        let start = GridCell::new(column, i32::MAX - 1);
        let goal = GridCell::new(column, i32::MAX);
        let bounds = NavigationBounds::new(start, goal).unwrap();
        let result = search_path(bounds, start, goal, 2, |_| Some(u32::MAX)).unwrap();
        assert_eq!(result.path, vec![start, goal]);
        assert_eq!(result.cost, Some(u64::from(u32::MAX)));
    }
}
