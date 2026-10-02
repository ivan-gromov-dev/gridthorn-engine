use crate::{GridCell, GridError, GridPoint, GridProjection};

#[test]
fn rejects_invalid_configuration_and_coordinates() {
    for size in [0.0, -1.0, f64::NAN, f64::INFINITY] {
        assert_eq!(
            GridProjection::square(size, GridPoint::default()).unwrap_err(),
            GridError::InvalidDimensions
        );
        assert_eq!(
            GridProjection::isometric(64.0, size, GridPoint::default()).unwrap_err(),
            GridError::InvalidDimensions
        );
    }
    assert_eq!(
        GridProjection::square(1.0, GridPoint::new(f64::NAN, 0.0)).unwrap_err(),
        GridError::NonFinite
    );
    let projection = GridProjection::square(1.0, GridPoint::default()).unwrap();
    assert_eq!(
        projection.cell_at(GridPoint::new(f64::INFINITY, 0.0)),
        Err(GridError::NonFinite)
    );
    assert_eq!(
        projection.cell_at(GridPoint::new(f64::from(i32::MAX) + 1.0, 0.0)),
        Err(GridError::CellOutOfRange)
    );
    assert_eq!(
        projection.cell_at(GridPoint::new(f64::from(i32::MIN) - 0.5, 0.0)),
        Err(GridError::CellOutOfRange)
    );
    let huge = GridProjection::square(f64::MAX, GridPoint::default()).unwrap();
    assert_eq!(
        huge.cell_vertex(GridCell::new(2, 0)),
        Err(GridError::NonFinite)
    );
    let tiny = GridProjection::square(f64::MIN_POSITIVE, GridPoint::default()).unwrap();
    assert_eq!(
        tiny.cell_at(GridPoint::new(f64::MAX, 0.0)),
        Err(GridError::NonFinite)
    );
}
