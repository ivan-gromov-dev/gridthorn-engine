use crate::{GridCell, GridFootprint, PlacementError};

#[test]
fn validates_and_orders_arbitrary_offsets() {
    assert_eq!(GridFootprint::new([]), Err(PlacementError::EmptyFootprint));
    let origin = GridCell::new(0, 0);
    assert_eq!(
        GridFootprint::new([origin, origin]),
        Err(PlacementError::DuplicateOffset(origin))
    );
    let footprint = GridFootprint::new([GridCell::new(2, 1), GridCell::new(-1, 0)]).unwrap();
    assert_eq!(
        footprint.cells_at(GridCell::new(-5, 3)).unwrap(),
        vec![GridCell::new(-6, 3), GridCell::new(-3, 4)]
    );
    assert_eq!(
        footprint.offsets().collect::<Vec<_>>(),
        vec![GridCell::new(-1, 0), GridCell::new(2, 1)]
    );
}

#[test]
fn checks_both_axes_at_signed_limits() {
    for (anchor, offset) in [
        (GridCell::new(i32::MAX, 0), GridCell::new(1, 0)),
        (GridCell::new(i32::MIN, 0), GridCell::new(-1, 0)),
        (GridCell::new(0, i32::MAX), GridCell::new(0, 1)),
        (GridCell::new(0, i32::MIN), GridCell::new(0, -1)),
    ] {
        assert_eq!(
            GridFootprint::new([offset]).unwrap().cells_at(anchor),
            Err(PlacementError::CellOverflow { anchor, offset })
        );
    }
    let extreme = GridCell::new(i32::MIN, i32::MAX);
    assert_eq!(
        GridFootprint::single_cell().cells_at(extreme).unwrap(),
        vec![extreme]
    );
}
