use crate::{
    ChunkSize, GridCell, GridError, GridPoint, GridProjection, GridView, TileLayerId, TileMap,
    TileMapError,
};

#[test]
fn highest_eligible_occupied_layer_wins_in_both_projections() {
    for projection in [
        GridProjection::square(16.0, GridPoint::default()).unwrap(),
        GridProjection::isometric(64.0, 32.0, GridPoint::new(100.0, 50.0)).unwrap(),
    ] {
        let mut map = TileMap::new(ChunkSize::new(4, 4).unwrap());
        let cell = GridCell::new(-5, 7);
        let point = projection.cell_center(cell).unwrap();
        for id in [TileLayerId(3), TileLayerId(-1), TileLayerId(8)] {
            map.add_layer(id).unwrap();
        }
        map.set_tile(TileLayerId(-1), cell, Some("ground")).unwrap();
        map.set_tile(TileLayerId(3), cell, Some("roof")).unwrap();
        let hit = map.pick_world(projection, point).unwrap().unwrap();
        assert_eq!(
            (hit.layer, hit.cell, *hit.tile),
            (TileLayerId(3), cell, "roof")
        );
        map.layer_mut(TileLayerId(3)).unwrap().set_visible(false);
        assert_eq!(
            map.pick_world(projection, point).unwrap().unwrap().layer,
            TileLayerId(-1)
        );
        map.layer_mut(TileLayerId(3)).unwrap().set_visible(true);
        map.layer_mut(TileLayerId(3)).unwrap().set_pickable(false);
        assert_eq!(
            map.pick_world(projection, point).unwrap().unwrap().layer,
            TileLayerId(-1)
        );
        map.layer_mut(TileLayerId(-1)).unwrap().set_pickable(false);
        assert!(map.pick_world(projection, point).unwrap().is_none());
        assert!(
            map.pick_world(
                projection,
                projection.cell_center(GridCell::new(0, 0)).unwrap()
            )
            .unwrap()
            .is_none()
        );
    }
}

#[test]
fn camera_pan_zoom_aspect_and_viewport_clipping() {
    let mut map = TileMap::new(ChunkSize::new(8, 8).unwrap());
    let id = TileLayerId(0);
    map.add_layer(id).unwrap();
    let cell = GridCell::new(-2, 3);
    map.set_tile(id, cell, Some(42)).unwrap();
    for projection in [
        GridProjection::square(10.0, GridPoint::default()).unwrap(),
        GridProjection::isometric(40.0, 20.0, GridPoint::default()).unwrap(),
    ] {
        let center = projection.cell_center(cell).unwrap();
        for (height, viewport) in [
            (100.0, GridPoint::new(800.0, 400.0)),
            (200.0, GridPoint::new(300.0, 600.0)),
        ] {
            let view = GridView::new(
                GridPoint::new(center.x - 10.0, center.y + 20.0),
                height,
                viewport,
            )
            .unwrap();
            let screen = GridPoint::new(
                viewport.x * 0.5 + 10.0 * viewport.y / height,
                viewport.y * 0.5 - 20.0 * viewport.y / height,
            );
            assert_eq!(view.screen_to_world(screen).unwrap(), Some(center));
            assert_eq!(
                map.pick_screen(projection, view, screen)
                    .unwrap()
                    .unwrap()
                    .cell,
                cell
            );
            for outside in [
                GridPoint::new(-1.0, 0.0),
                GridPoint::new(0.0, -1.0),
                GridPoint::new(viewport.x, 0.0),
                GridPoint::new(0.0, viewport.y),
            ] {
                assert!(
                    map.pick_screen(projection, view, outside)
                        .unwrap()
                        .is_none()
                );
            }
        }
    }
}

#[test]
fn exact_edges_and_invalid_queries() {
    let projection = GridProjection::square(1.0, GridPoint::default()).unwrap();
    let mut map = TileMap::new(ChunkSize::new(2, 2).unwrap());
    map.add_layer(TileLayerId(0)).unwrap();
    map.set_tile(TileLayerId(0), GridCell::new(0, 0), Some(1))
        .unwrap();
    map.set_tile(TileLayerId(0), GridCell::new(1, 0), Some(2))
        .unwrap();
    assert_eq!(
        *map.pick_world(projection, GridPoint::new(1.0, 0.0))
            .unwrap()
            .unwrap()
            .tile,
        2
    );
    assert_eq!(
        map.pick_world(projection, GridPoint::new(f64::NAN, 0.0)),
        Err(TileMapError::Projection(GridError::NonFinite))
    );
    assert_eq!(
        map.pick_world(projection, GridPoint::new(f64::from(i32::MAX) + 1.0, 0.0)),
        Err(TileMapError::Projection(GridError::CellOutOfRange))
    );
    for (height, viewport) in [
        (0.0, GridPoint::new(1.0, 1.0)),
        (1.0, GridPoint::new(0.0, 1.0)),
        (f64::INFINITY, GridPoint::new(1.0, 1.0)),
        (1.0, GridPoint::new(1.0, f64::NAN)),
    ] {
        assert!(matches!(
            GridView::new(GridPoint::default(), height, viewport),
            Err(TileMapError::InvalidView)
        ));
    }
    assert!(matches!(
        GridView::new(GridPoint::new(f64::NAN, 0.0), 1.0, GridPoint::new(1.0, 1.0)),
        Err(TileMapError::Projection(GridError::NonFinite))
    ));
    let view = GridView::new(GridPoint::default(), 1.0, GridPoint::new(1.0, 1.0)).unwrap();
    assert_eq!(
        view.screen_to_world(GridPoint::new(f64::INFINITY, 0.0)),
        Err(TileMapError::Projection(GridError::NonFinite))
    );
    assert!(
        view.screen_to_world(GridPoint::default())
            .unwrap()
            .is_some()
    );
    let huge = GridView::new(GridPoint::default(), f64::MAX, GridPoint::new(4.0, 1.0)).unwrap();
    assert_eq!(
        huge.screen_to_world(GridPoint::default()),
        Err(TileMapError::Projection(GridError::NonFinite))
    );
}
