use crate::{ChunkSize, GridCell, TileLayerId, TileMap, TileMapError};

#[test]
fn signed_chunk_boundaries_and_extreme_cells() {
    let size = ChunkSize::new(16, 8).unwrap();
    for (cell, chunk, local) in [
        ((0, 0), (0, 0), (0, 0)),
        ((15, 7), (0, 0), (15, 7)),
        ((16, 8), (1, 1), (0, 0)),
        ((-1, -1), (-1, -1), (15, 7)),
        ((-16, -8), (-1, -1), (0, 0)),
        ((-17, -9), (-2, -2), (15, 7)),
        ((i32::MIN, i32::MAX), (-134_217_728, 268_435_455), (0, 7)),
    ] {
        let cell = GridCell::new(cell.0, cell.1);
        assert_eq!(
            size.locate(cell),
            (
                GridCell::new(chunk.0, chunk.1),
                GridCell::new(local.0, local.1)
            )
        );
    }
    let odd = ChunkSize::new(3, 7).unwrap();
    let mut map = TileMap::new(odd);
    map.add_layer(TileLayerId(0)).unwrap();
    for cell in [
        GridCell::new(i32::MIN, i32::MAX),
        GridCell::new(i32::MAX, i32::MIN),
    ] {
        map.set_tile(TileLayerId(0), cell, Some(cell)).unwrap();
        assert_eq!(map.tile(TileLayerId(0), cell), Some(&cell));
    }
    assert_eq!(map.layer(TileLayerId(0)).unwrap().tiles().count(), 2);
}

#[test]
fn replacement_removal_and_layer_validation_preserve_data() {
    assert_eq!(ChunkSize::new(0, 1), Err(TileMapError::InvalidChunkSize));
    assert_eq!(ChunkSize::new(1, -1), Err(TileMapError::InvalidChunkSize));
    let id = TileLayerId(-3);
    let cell = GridCell::new(-1, 0);
    let mut map = TileMap::new(ChunkSize::new(4, 4).unwrap());
    assert_eq!(
        map.set_tile(id, cell, Some(1)),
        Err(TileMapError::MissingLayer(id))
    );
    map.add_layer(id).unwrap();
    assert_eq!(map.set_tile(id, cell, Some(1)).unwrap(), None);
    assert_eq!(map.add_layer(id), Err(TileMapError::DuplicateLayer(id)));
    assert_eq!(map.set_tile(id, cell, Some(2)).unwrap(), Some(1));
    assert_eq!(
        map.layer(id).unwrap().chunks().collect::<Vec<_>>(),
        vec![(GridCell::new(-1, 0), 1)]
    );
    assert_eq!(
        map.set_tile(id, GridCell::new(100, 100), None).unwrap(),
        None
    );
    assert_eq!(map.set_tile(id, cell, None).unwrap(), Some(2));
    assert_eq!(map.layer(id).unwrap().chunks().count(), 0);
    map.set_tile(id, cell, Some(7)).unwrap();
    let removed = map.remove_layer(id).unwrap();
    assert_eq!(removed.tiles().collect::<Vec<_>>(), vec![(cell, &7)]);
    assert!(map.layer(id).is_none());
}

#[test]
fn iteration_is_independent_of_insertion_order() {
    let build = |reverse: bool| {
        let mut map = TileMap::new(ChunkSize::new(2, 2).unwrap());
        let mut ids = vec![TileLayerId(5), TileLayerId(-2), TileLayerId(1)];
        let mut cells = vec![
            GridCell::new(3, -1),
            GridCell::new(-1, 0),
            GridCell::new(0, 1),
            GridCell::new(0, 0),
        ];
        if reverse {
            ids.reverse();
            cells.reverse();
        }
        for id in ids {
            map.add_layer(id).unwrap();
            for cell in &cells {
                map.set_tile(id, *cell, Some(*cell)).unwrap();
            }
        }
        map
    };
    let a = build(false);
    let b = build(true);
    assert_eq!(
        a.layers().map(|(id, _)| id).collect::<Vec<_>>(),
        vec![TileLayerId(-2), TileLayerId(1), TileLayerId(5)]
    );
    for ((id_a, layer_a), (id_b, layer_b)) in a.layers().zip(b.layers()) {
        assert_eq!(id_a, id_b);
        assert_eq!(
            layer_a.tiles().collect::<Vec<_>>(),
            layer_b.tiles().collect::<Vec<_>>()
        );
        assert_eq!(
            layer_a.chunks().collect::<Vec<_>>(),
            layer_b.chunks().collect::<Vec<_>>()
        );
    }
}

#[test]
fn chunk_queries_and_partial_removal_are_local() {
    let id = TileLayerId(0);
    let mut map = TileMap::new(ChunkSize::new(4, 4).unwrap());
    map.add_layer(id).unwrap();
    for cell in [
        GridCell::new(-1, -1),
        GridCell::new(-2, -1),
        GridCell::new(0, 0),
    ] {
        map.set_tile(id, cell, Some(cell)).unwrap();
    }
    let layer = map.layer(id).unwrap();
    assert_eq!(
        layer
            .chunk_tiles(GridCell::new(-1, -1))
            .map(|(cell, _)| cell)
            .collect::<Vec<_>>(),
        vec![GridCell::new(-2, -1), GridCell::new(-1, -1)]
    );
    assert_eq!(layer.chunk_tiles(GridCell::new(9, 9)).count(), 0);
    map.set_tile(id, GridCell::new(-1, -1), None).unwrap();
    assert_eq!(
        map.layer(id).unwrap().chunks().collect::<Vec<_>>(),
        vec![(GridCell::new(-1, -1), 1), (GridCell::new(0, 0), 1)]
    );
    assert_eq!(
        map.tile(id, GridCell::new(-2, -1)),
        Some(&GridCell::new(-2, -1))
    );
}
