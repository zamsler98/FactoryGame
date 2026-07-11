use game_core::*;

fn tick_n(world: &mut World, n: u32, dt: f32) {
    for _ in 0..n {
        world.update_factory(dt);
    }
}

fn place(world: &mut World, kind: BuildingKind, x: i32, y: i32, rot: Rotation) -> InstanceId {
    world
        .place_building(kind, TilePos { x, y }, rot)
        .expect("placement failed")
}

#[test]
fn miner_outputs_onto_conveyor() {
    let mut w = World::new();
    let miner = place(&mut w, BuildingKind::Miner, 0, 0, Rotation::R0);
    let belt = place(&mut w, BuildingKind::Conveyor, 1, 0, Rotation::R0);

    // 1.25s: mining takes 1.0s, then the ore transfers to the belt.
    tick_n(&mut w, 25, 0.05);

    match w.factory.state(belt) {
        Some(BuildingState::Conveyor { item: Some(item) }) => {
            assert_eq!(item.kind, ItemKind::IronOre)
        }
        other => panic!("expected ore on belt, got {other:?}"),
    }
    match w.factory.state(miner) {
        Some(BuildingState::Miner { output, .. }) => {
            assert!(output.is_none(), "miner should have handed off its ore")
        }
        other => panic!("expected miner state, got {other:?}"),
    }
}

#[test]
fn miner_respects_rotation() {
    let mut w = World::new();
    place(&mut w, BuildingKind::Miner, 5, 5, Rotation::R90); // faces +y
    let below = place(&mut w, BuildingKind::Conveyor, 5, 6, Rotation::R90);
    let beside = place(&mut w, BuildingKind::Conveyor, 6, 5, Rotation::R0);

    tick_n(&mut w, 25, 0.05);

    assert!(matches!(
        w.factory.state(below),
        Some(BuildingState::Conveyor { item: Some(_) })
    ));
    assert!(matches!(
        w.factory.state(beside),
        Some(BuildingState::Conveyor { item: None })
    ));
}

#[test]
fn ore_flows_through_belts_and_smelts() {
    let mut w = World::new();
    place(&mut w, BuildingKind::Miner, 0, 0, Rotation::R0);
    place(&mut w, BuildingKind::Conveyor, 1, 0, Rotation::R0);
    place(&mut w, BuildingKind::Conveyor, 2, 0, Rotation::R0);
    place(&mut w, BuildingKind::Smelter, 3, 0, Rotation::R0);

    // Mine 1s + travel 2 tiles at 2 tiles/s (1s) + smelt 2s = ~4s. Run 6s.
    tick_n(&mut w, 120, 0.05);

    let ingots = w.factory.produced.get(&ItemKind::IronIngot).copied();
    assert!(
        ingots.unwrap_or(0) >= 1,
        "expected at least one ingot, got {ingots:?}"
    );
}

#[test]
fn blocked_conveyor_holds_item_and_miner_waits() {
    let mut w = World::new();
    let miner = place(&mut w, BuildingKind::Miner, 0, 0, Rotation::R0);
    // Belt points at empty ground, so the item can never leave.
    let belt = place(&mut w, BuildingKind::Conveyor, 1, 0, Rotation::R0);

    tick_n(&mut w, 200, 0.05); // 10s

    match w.factory.state(belt) {
        Some(BuildingState::Conveyor { item: Some(item) }) => {
            assert_eq!(item.kind, ItemKind::IronOre);
            assert!(item.progress >= 1.0, "item should be stuck at exit edge");
        }
        other => panic!("expected stuck ore on belt, got {other:?}"),
    }
    // The miner holds one finished ore and pauses; nothing is lost or duplicated.
    match w.factory.state(miner) {
        Some(BuildingState::Miner { output, .. }) => assert_eq!(*output, Some(ItemKind::IronOre)),
        other => panic!("expected miner state, got {other:?}"),
    }
}

#[test]
fn smelter_buffers_cap_out() {
    let mut w = World::new();
    place(&mut w, BuildingKind::Miner, 0, 0, Rotation::R0);
    let smelter = place(&mut w, BuildingKind::Smelter, 1, 0, Rotation::R0);

    // Long run: smelter output is never emptied (it faces empty ground), so
    // input and output buffers must both stop at the cap.
    tick_n(&mut w, 1200, 0.05); // 60s

    match w.factory.state(smelter) {
        Some(BuildingState::Smelter { input, output, .. }) => {
            let (_, in_n) = input.expect("input buffer should be backed up");
            let (out_kind, out_n) = output.expect("output buffer should have ingots");
            assert!(in_n <= SMELTER_STACK_CAP);
            assert_eq!(out_kind, ItemKind::IronIngot);
            assert_eq!(out_n, SMELTER_STACK_CAP);
        }
        other => panic!("expected smelter state, got {other:?}"),
    }
}

#[test]
fn smelter_pushes_output_to_conveyor() {
    let mut w = World::new();
    place(&mut w, BuildingKind::Miner, 0, 0, Rotation::R0);
    place(&mut w, BuildingKind::Smelter, 1, 0, Rotation::R0);
    let out_belt = place(&mut w, BuildingKind::Conveyor, 2, 0, Rotation::R0);

    tick_n(&mut w, 120, 0.05); // 6s

    match w.factory.state(out_belt) {
        Some(BuildingState::Conveyor { item: Some(item) }) => {
            assert_eq!(item.kind, ItemKind::IronIngot)
        }
        other => panic!("expected ingot on output belt, got {other:?}"),
    }
}

#[test]
fn removing_building_clears_state() {
    let mut w = World::new();
    place(&mut w, BuildingKind::Miner, 0, 0, Rotation::R0);
    let belt = place(&mut w, BuildingKind::Conveyor, 1, 0, Rotation::R0);

    tick_n(&mut w, 25, 0.05);
    assert!(w.factory.state(belt).is_some());

    w.remove_building(belt);
    assert!(w.factory.state(belt).is_none());
    assert!(w.tile_grid.tile_occupant(TilePos { x: 1, y: 0 }).is_none());

    // Sim keeps running without the removed building.
    tick_n(&mut w, 25, 0.05);
}
