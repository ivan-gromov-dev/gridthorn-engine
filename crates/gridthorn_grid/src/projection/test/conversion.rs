use crate::{GridCell, GridPoint, GridProjection};

#[test]
fn square_and_isometric_centers_round_trip_across_signed_grid() {
    for projection in [
        GridProjection::square(32.0, GridPoint::new(100.0, -20.0)).unwrap(),
        GridProjection::isometric(64.0, 32.0, GridPoint::new(100.0, -20.0)).unwrap(),
    ] {
        for column in -32..=32 {
            for row in -32..=32 {
                let cell = GridCell::new(column, row);
                assert_eq!(
                    projection.cell_at(projection.cell_center(cell).unwrap()),
                    Ok(cell)
                );
            }
        }
        for cell in [
            GridCell::new(i32::MIN, i32::MAX),
            GridCell::new(i32::MAX, i32::MIN),
        ] {
            assert_eq!(
                projection.cell_at(projection.cell_center(cell).unwrap()),
                Ok(cell)
            );
        }
    }
}

#[test]
fn axes_centers_and_negative_edges_have_explicit_conventions() {
    let iso = GridProjection::isometric(64.0, 32.0, GridPoint::default()).unwrap();
    assert_eq!(
        iso.cell_vertex(GridCell::new(1, 0)).unwrap(),
        GridPoint::new(32.0, 16.0)
    );
    assert_eq!(
        iso.cell_vertex(GridCell::new(0, 1)).unwrap(),
        GridPoint::new(-32.0, 16.0)
    );
    assert_eq!(
        iso.cell_center(GridCell::default()).unwrap(),
        GridPoint::new(0.0, 16.0)
    );
    for projection in [
        iso,
        GridProjection::square(32.0, GridPoint::default()).unwrap(),
    ] {
        let cell = GridCell::new(-1, 2);
        assert_eq!(
            projection.cell_at(projection.cell_vertex(cell).unwrap()),
            Ok(cell)
        );
        assert_eq!(
            projection
                .cell_at(GridPoint::new(-0.25, 0.0))
                .unwrap()
                .column,
            -1
        );
    }
}
