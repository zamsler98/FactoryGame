use game_core::*;

fn tick_n(world: &mut World, n: u32, dt: f32) {
    for _ in 0..n {
        world.update(dt);
    }
}

/// Give the player plenty of every building item so placement never fails on
/// inventory in these focused simulation tests.
fn stock(world: &mut World) {
    for kind in BuildingKind::ALL {
        world.inventory.add(kind.item(), 100);
    }
    world.inventory.add(ItemKind::Coal, 100);
}

fn place(world: &mut World, kind: BuildingKind, x: i32, y: i32, rot: Rotation) -> InstanceId {
    world
        .place_building(kind, TilePos { x, y }, rot)
        .expect("placement failed")
}

fn fuel_miner(world: &mut World, id: InstanceId) {
    if let Some(BuildingState::Miner { fuel, .. }) = world.factory.state_mut(id) {
        *fuel = 5;
    }
}

#[test]
fn miner_mines_patch_and_outputs_onto_belt() {
    let mut w = World::new(); // starting field has ore near the origin
    stock(&mut w);
    // Find a coal tile from the generated field to place the miner on.
    let (pos, _) = w
        .resources
        .patches()
        .find(|(_, p)| p.kind == ResourceKind::Coal)
        .expect("coal patch exists");
    let miner = place(&mut w, BuildingKind::Miner, pos.x, pos.y, Rotation::R0);
    fuel_miner(&mut w, miner);
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
fn miner_without_fuel_does_nothing() {
    let mut w = World::new();
    stock(&mut w);
    let (pos, _) = w
        .resources
        .patches()
        .find(|(_, p)| p.kind == ResourceKind::IronOre)
        .expect("iron patch exists");
    let miner = place(&mut w, BuildingKind::Miner, pos.x, pos.y, Rotation::R0);
    // no fuel added
    tick_n(&mut w, 60, 0.05);
    match w.factory.state(miner) {
        Some(BuildingState::Miner {
            output, progress, ..
        }) => {
            assert!(output.is_none());
            assert_eq!(*progress, 0.0, "no fuel means no mining progress");
        }
        other => panic!("expected idle miner, got {other:?}"),
    }
}

#[test]
fn miner_requires_resource_patch() {
    let mut w = World::empty();
    stock(&mut w);
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
fn belt_carries_item_between_belts() {
    let mut w = World::empty();
    stock(&mut w);
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
    stock(&mut w);
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
    stock(&mut w);
    let belt = place(&mut w, BuildingKind::Belt, 10, 10, Rotation::R0);
    // Inserter to the "north" of the belt, facing up (R270 = -y), grabbing from
    // the belt below it (back = +y) and dropping onto the chest above it.
    let inserter = place(&mut w, BuildingKind::Inserter, 10, 9, Rotation::R270);
    let chest = place(&mut w, BuildingKind::Chest, 10, 8, Rotation::R270);
    if let Some(BuildingState::Belt { item }) = w.factory.state_mut(belt) {
        *item = Some(ConveyorItem {
            kind: ItemKind::IronPlate,
            progress: 1.0,
        });
    }
    tick_n(&mut w, 40, 0.05); // > swing time
    let _ = inserter;
    match w.factory.state(chest) {
        Some(BuildingState::Chest { items }) => {
            assert_eq!(items.get(&ItemKind::IronPlate).copied(), Some(1));
        }
        other => panic!("expected chest with a plate, got {other:?}"),
    }
}

#[test]
fn furnace_smelts_ore_to_plate_with_fuel() {
    let mut w = World::empty();
    stock(&mut w);
    let furnace = place(&mut w, BuildingKind::Furnace, 10, 10, Rotation::R0);
    if let Some(BuildingState::Furnace { fuel, input, .. }) = w.factory.state_mut(furnace) {
        *fuel = 5;
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
fn furnace_without_fuel_does_not_smelt() {
    let mut w = World::empty();
    stock(&mut w);
    let furnace = place(&mut w, BuildingKind::Furnace, 10, 10, Rotation::R0);
    if let Some(BuildingState::Furnace { input, .. }) = w.factory.state_mut(furnace) {
        *input = Some((ItemKind::IronOre, 3));
    }
    tick_n(&mut w, 100, 0.05);
    assert_eq!(w.factory.produced.get(&ItemKind::IronPlate).copied(), None);
}

#[test]
fn assembler_crafts_gears_from_plates() {
    let mut w = World::empty();
    stock(&mut w);
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
    // Miner (on iron) -> belt -> inserter -> furnace (fueled) produces plates.
    let mut w = World::new();
    stock(&mut w);
    let (pos, _) = w
        .resources
        .patches()
        .find(|(_, p)| p.kind == ResourceKind::IronOre)
        .expect("iron patch");
    let miner = place(&mut w, BuildingKind::Miner, pos.x, pos.y, Rotation::R0);
    fuel_miner(&mut w, miner);
    place(&mut w, BuildingKind::Belt, pos.x + 1, pos.y, Rotation::R0);
    // Inserter above the belt end pulling from it into a fueled furnace.
    let ins = place(
        &mut w,
        BuildingKind::Inserter,
        pos.x + 1,
        pos.y - 1,
        Rotation::R270,
    );
    let _ = ins;
    let furnace = place(
        &mut w,
        BuildingKind::Furnace,
        pos.x + 1,
        pos.y - 2,
        Rotation::R0,
    );
    if let Some(BuildingState::Furnace { fuel, .. }) = w.factory.state_mut(furnace) {
        *fuel = 5;
    }
    tick_n(&mut w, 200, 0.05); // 10s
    let plates = w.factory.produced.get(&ItemKind::IronPlate).copied();
    assert!(
        plates.unwrap_or(0) >= 1,
        "expected plates from full chain, got {plates:?}"
    );
}

#[test]
fn placing_consumes_inventory_and_mining_returns_it() {
    let mut w = World::empty();
    w.inventory.add(ItemKind::WoodenChest, 1);
    assert_eq!(w.inventory.count(ItemKind::WoodenChest), 1);
    let chest = place(&mut w, BuildingKind::Chest, 10, 10, Rotation::R0);
    assert_eq!(w.inventory.count(ItemKind::WoodenChest), 0);
    // Can't place a second: out of items.
    assert!(matches!(
        w.place_building(BuildingKind::Chest, TilePos { x: 11, y: 10 }, Rotation::R0),
        Err(PlaceError::NoItem)
    ));
    w.remove_building(chest);
    assert_eq!(w.inventory.count(ItemKind::WoodenChest), 1);
}

#[test]
fn mining_a_chest_returns_its_contents() {
    let mut w = World::empty();
    w.inventory.add(ItemKind::WoodenChest, 1);
    let chest = place(&mut w, BuildingKind::Chest, 10, 10, Rotation::R0);
    if let Some(BuildingState::Chest { items }) = w.factory.state_mut(chest) {
        items.insert(ItemKind::IronPlate, 7);
    }
    w.remove_building(chest);
    assert_eq!(w.inventory.count(ItemKind::IronPlate), 7);
}

#[test]
fn hand_crafting_consumes_and_produces() {
    let mut w = World::empty();
    w.inventory.add(ItemKind::IronPlate, 2);
    w.queue_craft(4); // Iron Gear Wheel: 2 plate -> 1 gear
    tick_n(&mut w, 20, 0.05); // > 0.5s
    assert_eq!(w.inventory.count(ItemKind::IronGearWheel), 1);
    assert_eq!(w.inventory.count(ItemKind::IronPlate), 0);
}

#[test]
fn hand_crafting_without_ingredients_is_dropped() {
    let mut w = World::empty();
    w.queue_craft(4);
    tick_n(&mut w, 20, 0.05);
    assert_eq!(w.inventory.count(ItemKind::IronGearWheel), 0);
    assert!(w.crafting.is_empty());
}

#[test]
fn removing_building_clears_state() {
    let mut w = World::empty();
    stock(&mut w);
    let belt = place(&mut w, BuildingKind::Belt, 11, 0, Rotation::R0);
    assert!(w.factory.state(belt).is_some());
    w.remove_building(belt);
    assert!(w.factory.state(belt).is_none());
    assert!(w.tile_grid.tile_occupant(TilePos { x: 11, y: 0 }).is_none());
}
