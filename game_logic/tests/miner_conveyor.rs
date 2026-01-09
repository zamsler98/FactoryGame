use game_core::World;
use game_core::{BuildingSpec, Rotation, Size2, TilePos};
use game_logic::{register_example_buildings, update_world, CONVEYOR_SPEED, TILE_SIZE};

#[test]
fn miner_spawns_and_conveyor_moves_item() {
    let mut world = World::new();
    // place miner at (1,1) rotation R0 (faces right)
    let miner_spec = BuildingSpec {
        spec_id: 2,
        size: Size2 { w: 1, h: 1 },
    };
    world
        .tile_grid
        .place(&miner_spec, TilePos { x: 1, y: 1 }, Rotation::R0)
        .unwrap();
    // place conveyor at (2,1) facing right
    let conv_spec = BuildingSpec {
        spec_id: 1,
        size: Size2 { w: 1, h: 1 },
    };
    world
        .tile_grid
        .place(&conv_spec, TilePos { x: 2, y: 1 }, Rotation::R0)
        .unwrap();

    // register logic state from grid
    register_example_buildings(&world);

    // simulate 1.1 seconds in small steps
    let dt = 0.1f32;
    for _ in 0..20 {
        update_world(&mut world, &game_logic::InputFrame::default(), dt);
        if !world.items.is_empty() {
            break;
        }
    }
    assert!(
        !world.items.is_empty(),
        "Miner did not spawn any items after 2 seconds"
    );

    // After spawn, make one update and check item's velocity was set by conveyor
    // If the item is spawned on the conveyor tile center, its velocity should equal conveyor speed in x
    let it = world.items.first().expect("no items present");
    // compute tile under item
    let tx = (it.transform.x / TILE_SIZE).floor() as i32;
    let ty = (it.transform.y / TILE_SIZE).floor() as i32;
    assert_eq!(tx, 2, "Item spawned in wrong tile: expected 2, got {}", tx);
    assert_eq!(ty, 1, "Item spawned in wrong tile: expected 1, got {}", ty);

    // run one more update and expect velocity to be set
    update_world(&mut world, &game_logic::InputFrame::default(), dt);
    let it = &world.items[0];
    assert!(
        (it.velocity.vx - CONVEYOR_SPEED).abs() < 1e-3,
        "Item velocity not set by conveyor: {}",
        it.velocity.vx
    );
}
