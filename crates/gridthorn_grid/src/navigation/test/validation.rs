use crate::{GridCell, NavigationBounds, NavigationError, search_path};

#[test]
fn invalid_queries_report_context() {
    let start = GridCell::default();
    let goal = GridCell::new(2, 0);
    assert!(matches!(
        NavigationBounds::new(goal, start),
        Err(NavigationError::InvalidBounds { .. })
    ));
    let bounds = NavigationBounds::new(start, goal).unwrap();
    assert_eq!(
        search_path(bounds, GridCell::new(-1, 0), goal, 10, |_| Some(1)),
        Err(NavigationError::OutsideBounds(GridCell::new(-1, 0)))
    );
    assert_eq!(
        search_path(bounds, start, goal, 10, |cell| (cell != goal).then_some(1)),
        Err(NavigationError::BlockedEndpoint(goal))
    );
    assert_eq!(
        search_path(bounds, start, goal, 10, |cell| Some(u32::from(
            cell.column != 1
        ))),
        Err(NavigationError::ZeroCost(GridCell::new(1, 0)))
    );
}
