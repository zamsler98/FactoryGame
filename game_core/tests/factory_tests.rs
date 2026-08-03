use game_core::*;

fn tick_n(world: &mut World, n: u32, dt: f32) {
    for _ in 0..n {
        world.update(dt);
    }
}

fn place(world: &mut World, kind: BuildingKind, x: i32, y: i32, rot: Rotation) -> InstanceId {
    world
        .place_building(kind, TilePos { x, y }, rot)
        .expect("placement failed")
}

#[test]
fn miner_mines_patch_and_outputs_onto_belt() {
    let mut w = World::new(); // starting field has ore near the origin
                              // Find a coal tile from the generated field to place the miner on.
    let (pos, _) = w
        .resources
        .patches()
        .find(|(_, p)| p.kind == ResourceKind::Coal)
        .expect("coal patch exists");
    place(&mut w, BuildingKind::Miner, pos.x, pos.y, Rotation::R0);
    let belt = place(&mut w, BuildingKind::Belt, pos.x + 1, pos.y, Rotation::R0);

    tick_n(&mut w, 30, 0.05); // >1s mining + handoff

    match w.factory.state(belt) {
        Some(BuildingState::Belt { item: Some(item) }) => {
            assert_eq!(item.kind, ItemKind::Coal)
        }
        other => panic!("expected coal on belt, got {other:?}"),
    }
}

#[test]
fn miner_requires_resource_patch() {
    let mut w = World::empty();
    let err = w
        .place_building(
            BuildingKind::Miner,
            TilePos { x: 500, y: 500 },
            Rotation::R0,
        )
        .expect_err("should fail off-patch");
    assert!(matches!(err, PlaceError::MissingResource));
}

#[test]
fn buildings_place_freely_without_inventory() {
    // No inventory/build cost: any number of buildings can be placed.
    let mut w = World::empty();
    place(&mut w, BuildingKind::Chest, 10, 10, Rotation::R0);
    place(&mut w, BuildingKind::Chest, 11, 10, Rotation::R0);
    place(&mut w, BuildingKind::Assembler, 12, 10, Rotation::R0);
    assert!(w
        .tile_grid
        .tile_occupant(TilePos { x: 11, y: 10 })
        .is_some());
}

#[test]
fn belt_carries_item_between_belts() {
    let mut w = World::empty();
    let a = place(&mut w, BuildingKind::Belt, 10, 10, Rotation::R0);
    let b = place(&mut w, BuildingKind::Belt, 11, 10, Rotation::R0);
    // Seed an item onto belt a directly.
    if let Some(BuildingState::Belt { item }) = w.factory.state_mut(a) {
        *item = Some(ConveyorItem {
            kind: ItemKind::IronPlate,
            progress: 0.0,
        });
    }
    tick_n(&mut w, 40, 0.05);
    assert!(matches!(
        w.factory.state(b),
        Some(BuildingState::Belt { item: Some(_) })
    ));
}

#[test]
fn belt_does_not_auto_load_furnace() {
    let mut w = World::empty();
    let belt = place(&mut w, BuildingKind::Belt, 10, 10, Rotation::R0);
    let furnace = place(&mut w, BuildingKind::Furnace, 11, 10, Rotation::R0);
    if let Some(BuildingState::Belt { item }) = w.factory.state_mut(belt) {
        *item = Some(ConveyorItem {
            kind: ItemKind::IronOre,
            progress: 0.0,
        });
    }
    tick_n(&mut w, 40, 0.05);
    // Belt still holds the ore; furnace was never loaded (needs an inserter).
    assert!(matches!(
        w.factory.state(belt),
        Some(BuildingState::Belt { item: Some(_) })
    ));
    match w.factory.state(furnace) {
        Some(BuildingState::Furnace { input, .. }) => assert!(input.is_none()),
        other => panic!("expected furnace, got {other:?}"),
    }
}

#[test]
fn inserter_moves_item_from_belt_to_chest() {
    let mut w = World::empty();
    let belt = place(&mut w, BuildingKind::Belt, 10, 10, Rotation::R0);
    // Inserter to the "north" of the belt, facing up (R270 = -y), grabbing from
    // the belt below it (back = +y) and dropping onto the chest above it.
    place(&mut w, BuildingKind::Inserter, 10, 9, Rotation::R270);
    let chest = place(&mut w, BuildingKind::Chest, 10, 8, Rotation::R270);
    if let Some(BuildingState::Belt { item }) = w.factory.state_mut(belt) {
        *item = Some(ConveyorItem {
            kind: ItemKind::IronPlate,
            progress: 1.0,
        });
    }
    tick_n(&mut w, 40, 0.05); // > swing time
    match w.factory.state(chest) {
        Some(BuildingState::Chest { items }) => {
            assert_eq!(items.get(&ItemKind::IronPlate).copied(), Some(1));
        }
        other => panic!("expected chest with a plate, got {other:?}"),
    }
}

#[test]
fn furnace_smelts_ore_to_plate() {
    let mut w = World::empty();
    let furnace = place(&mut w, BuildingKind::Furnace, 10, 10, Rotation::R0);
    if let Some(BuildingState::Furnace { input, .. }) = w.factory.state_mut(furnace) {
        *input = Some((ItemKind::IronOre, 3));
    }
    tick_n(&mut w, 60, 0.05); // 3s, > 2s smelt
    let plates = w.factory.produced.get(&ItemKind::IronPlate).copied();
    assert!(
        plates.unwrap_or(0) >= 1,
        "expected an iron plate, got {plates:?}"
    );
}

#[test]
fn assembler_crafts_gears_from_plates() {
    let mut w = World::empty();
    let asm = place(&mut w, BuildingKind::Assembler, 10, 10, Rotation::R0);
    // Recipe 4 = Iron Gear Wheel (2 iron plate -> 1 gear).
    w.set_assembler_recipe(asm, Some(4));
    if let Some(BuildingState::Assembler { inputs, .. }) = w.factory.state_mut(asm) {
        inputs.insert(ItemKind::IronPlate, 10);
    }
    tick_n(&mut w, 40, 0.05);
    match w.factory.state(asm) {
        Some(BuildingState::Assembler { outputs, .. }) => {
            assert!(outputs.get(&ItemKind::IronGearWheel).copied().unwrap_or(0) >= 1);
        }
        other => panic!("expected assembler, got {other:?}"),
    }
}

#[test]
fn full_chain_mines_and_smelts_iron() {
    // Miner (on iron) -> belt -> inserter -> furnace produces plates. No fuel.
    let mut w = World::new();
    let (pos, _) = w
        .resources
        .patches()
        .find(|(_, p)| p.kind == ResourceKind::IronOre)
        .expect("iron patch");
    place(&mut w, BuildingKind::Miner, pos.x, pos.y, Rotation::R0);
    place(&mut w, BuildingKind::Belt, pos.x + 1, pos.y, Rotation::R0);
    // Inserter above the belt end pulling from it into a furnace.
    place(
        &mut w,
        BuildingKind::Inserter,
        pos.x + 1,
        pos.y - 1,
        Rotation::R270,
    );
    place(
        &mut w,
        BuildingKind::Furnace,
        pos.x + 1,
        pos.y - 2,
        Rotation::R0,
    );
    tick_n(&mut w, 200, 0.05); // 10s
    let plates = w.factory.produced.get(&ItemKind::IronPlate).copied();
    assert!(
        plates.unwrap_or(0) >= 1,
        "expected plates from full chain, got {plates:?}"
    );
}

#[test]
fn removing_building_clears_state() {
    let mut w = World::empty();
    let belt = place(&mut w, BuildingKind::Belt, 11, 0, Rotation::R0);
    assert!(w.factory.state(belt).is_some());
    w.remove_building(belt);
    assert!(w.factory.state(belt).is_none());
    assert!(w.tile_grid.tile_occupant(TilePos { x: 11, y: 0 }).is_none());
}
