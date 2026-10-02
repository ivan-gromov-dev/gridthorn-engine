use crate::{GridCell, GridFootprint, GridObjectId, PlacementError, PlacementMap};

fn wide() -> GridFootprint {
    GridFootprint::new([GridCell::new(0, 0), GridCell::new(1, 0)]).unwrap()
}

fn snapshot(map: &PlacementMap) -> Vec<(GridObjectId, crate::GridPlacement)> {
    map.objects()
        .map(|(id, placement)| (id, placement.clone()))
        .collect()
}

#[test]
fn placement_queries_all_cells_and_removal_releases_them() {
    let mut map = PlacementMap::new();
    let id = GridObjectId(4);
    map.place(id, GridCell::new(-1, -2), wide()).unwrap();
    assert_eq!(map.object_at(GridCell::new(-1, -2)), Some(id));
    assert_eq!(map.object_at(GridCell::new(0, -2)), Some(id));
    assert_eq!(map.object_at(GridCell::new(1, -2)), None);
    assert_eq!(map.placement(id).unwrap().anchor(), GridCell::new(-1, -2));
    assert_eq!(map.placement(id).unwrap().footprint(), &wide());
    let removed = map.remove(id).unwrap();
    assert_eq!(removed.anchor(), GridCell::new(-1, -2));
    assert_eq!(map.object_at(GridCell::new(0, -2)), None);
    assert_eq!(map.object_at(GridCell::new(-1, -2)), None);
    assert!(map.remove(id).is_none());
    assert!(map.placement(id).is_none());
    assert_eq!(map.objects().len(), 0);
    map.place(id, GridCell::new(-1, -2), wide()).unwrap();
}

#[test]
fn rejected_edits_preserve_objects_and_occupancy() {
    let mut map = PlacementMap::new();
    let id = GridObjectId(1);
    let other = GridObjectId(2);
    map.place(id, GridCell::new(0, 0), wide()).unwrap();
    map.place(other, GridCell::new(3, 0), GridFootprint::single_cell())
        .unwrap();
    let before = snapshot(&map);
    let conflict = PlacementError::Occupied {
        cell: GridCell::new(3, 0),
        object: other,
    };
    assert_eq!(
        map.validate(id, GridCell::new(2, 0), &wide()),
        Err(conflict)
    );
    assert_eq!(map.relocate(id, GridCell::new(2, 0), wide()), Err(conflict));
    assert_eq!(
        map.place(GridObjectId(3), GridCell::new(2, 0), wide()),
        Err(conflict)
    );
    assert_eq!(
        map.place(id, GridCell::new(8, 8), wide()),
        Err(PlacementError::DuplicateObject(id))
    );
    assert_eq!(
        map.relocate(GridObjectId(9), GridCell::new(0, 0), wide()),
        Err(PlacementError::MissingObject(GridObjectId(9)))
    );
    assert!(matches!(
        map.relocate(id, GridCell::new(i32::MAX, 0), wide()),
        Err(PlacementError::CellOverflow { .. })
    ));
    assert!(matches!(
        map.place(GridObjectId(3), GridCell::new(i32::MAX, 0), wide()),
        Err(PlacementError::CellOverflow { .. })
    ));
    assert_eq!(snapshot(&map), before);
    for (cell, expected) in [(0, Some(id)), (1, Some(id)), (2, None), (3, Some(other))] {
        assert_eq!(map.object_at(GridCell::new(cell, 0)), expected);
    }
}

#[test]
fn overlapping_moves_and_shape_changes_are_atomic() {
    let mut map = PlacementMap::new();
    let id = GridObjectId(1);
    map.place(id, GridCell::new(0, 0), wide()).unwrap();
    assert_eq!(
        map.validate(id, GridCell::new(1, 0), &wide()).unwrap(),
        vec![GridCell::new(1, 0), GridCell::new(2, 0)]
    );
    assert_eq!(map.object_at(GridCell::new(2, 0)), None);
    map.relocate(id, GridCell::new(1, 0), wide()).unwrap();
    assert_eq!(map.object_at(GridCell::new(0, 0)), None);
    assert_eq!(map.object_at(GridCell::new(1, 0)), Some(id));
    assert_eq!(map.object_at(GridCell::new(2, 0)), Some(id));
    map.relocate(id, GridCell::new(1, 0), GridFootprint::single_cell())
        .unwrap();
    assert_eq!(map.object_at(GridCell::new(2, 0)), None);
    map.relocate(id, GridCell::new(1, 0), GridFootprint::single_cell())
        .unwrap();
    assert_eq!(map.object_at(GridCell::new(1, 0)), Some(id));
}

#[test]
fn holes_and_independent_spaces_do_not_block_placement() {
    let mut map = PlacementMap::new();
    let id = GridObjectId(1);
    let shape = GridFootprint::new([GridCell::new(-1, 0), GridCell::new(1, 0)]).unwrap();
    map.place(id, GridCell::new(0, 0), shape).unwrap();
    map.place(
        GridObjectId(2),
        GridCell::new(0, 0),
        GridFootprint::single_cell(),
    )
    .unwrap();
    let mut independent = PlacementMap::new();
    independent.place(id, GridCell::new(0, 0), wide()).unwrap();
    assert_eq!(map.object_at(GridCell::new(0, 0)), Some(GridObjectId(2)));
}

#[test]
fn object_iteration_and_conflicts_are_deterministic() {
    for order in [[3, 1, 2], [2, 3, 1]] {
        let mut map = PlacementMap::new();
        for id in order {
            map.place(
                GridObjectId(id),
                GridCell::new(i32::try_from(id).unwrap(), 0),
                GridFootprint::single_cell(),
            )
            .unwrap();
        }
        assert_eq!(
            map.objects().map(|(id, _)| id.0).collect::<Vec<_>>(),
            vec![1, 2, 3]
        );
        let shape = GridFootprint::new([GridCell::new(3, 0), GridCell::new(1, 0)]).unwrap();
        assert_eq!(
            map.validate(GridObjectId(8), GridCell::new(0, 0), &shape),
            Err(PlacementError::Occupied {
                cell: GridCell::new(1, 0),
                object: GridObjectId(1)
            })
        );
    }
}

#[test]
fn overflow_precedes_occupancy_and_extreme_placements_can_be_removed() {
    let mut map = PlacementMap::new();
    let anchor = GridCell::new(i32::MAX, i32::MIN);
    let existing = GridObjectId(1);
    map.place(existing, anchor, GridFootprint::single_cell())
        .unwrap();
    assert_eq!(
        map.validate(GridObjectId(2), anchor, &wide()),
        Err(PlacementError::CellOverflow {
            anchor,
            offset: GridCell::new(1, 0)
        })
    );
    assert_eq!(map.object_at(anchor), Some(existing));
    map.remove(existing).unwrap();
    assert_eq!(map.object_at(anchor), None);
    let shape = GridFootprint::new([GridCell::new(-1, 1), GridCell::new(0, 0)]).unwrap();
    map.place(existing, anchor, shape).unwrap();
    let adjacent = GridCell::new(i32::MAX - 1, i32::MIN + 1);
    assert_eq!(map.object_at(adjacent), Some(existing));
    map.remove(existing).unwrap();
    assert_eq!(map.object_at(adjacent), None);
    assert_eq!(map.object_at(anchor), None);
}
