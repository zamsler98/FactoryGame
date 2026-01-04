use game_core::*;

#[test]
fn single_tile_place_and_remove() {
    let mut g = TileGrid::new(10, 10);
    let spec = BuildingSpec {
        spec_id: 1,
        size: Size2 { w: 1, h: 1 },
    };
    let origin = TilePos { x: 2, y: 3 };
    assert!(g.can_place(&spec, origin, Rotation::R0));
    let id = g.place(&spec, origin, Rotation::R0).expect("place failed");
    assert_eq!(g.tile_occupant(origin), Some(id));
    let removed = g.remove(id).expect("remove failed");
    assert_eq!(removed.id, id);
    assert_eq!(g.tile_occupant(origin), None);
}

#[test]
fn multi_tile_place_and_overlap() {
    let mut g = TileGrid::new(10, 10);
    let spec = BuildingSpec {
        spec_id: 3,
        size: Size2 { w: 3, h: 3 },
    };
    let origin = TilePos { x: 5, y: 5 };
    assert!(g.can_place(&spec, origin, Rotation::R0));
    let id = g.place(&spec, origin, Rotation::R0).expect("place failed");
    // overlapping placement should fail
    let origin2 = TilePos { x: 6, y: 6 };
    assert!(!g.can_place(&spec, origin2, Rotation::R0));
    match g.place(&spec, origin2, Rotation::R0) {
        Err(PlacementError::Occupied) => {}
        other => panic!("expected Occupied, got {:?}", other),
    }
    // remove and then placing should succeed
    g.remove(id);
    assert!(g.can_place(&spec, origin2, Rotation::R0));
}

#[test]
fn rotation_bounds() {
    let g = TileGrid::new(5, 5);
    let spec = BuildingSpec {
        spec_id: 3,
        size: Size2 { w: 3, h: 2 },
    };
    // placing at (3,0) with R0 should be out of bounds (width 5, 3+3>5)
    let oob = TilePos { x: 3, y: 0 };
    assert!(!g.can_place(&spec, oob, Rotation::R0));
    // rotated 90 swaps sizes; placing at (3,0) might fit depending on sizes
    assert!(!g.can_place(&spec, oob, Rotation::R90));
}
