//! Read-only view structs the renderer consumes each frame. This keeps
//! `game_app` decoupled from simulation internals: it draws whatever the
//! snapshot says, nothing more.

use game_core::{BuildingKind, BuildingState, ItemKind, TilePos, World};

/// An item to draw. `x`/`y` are in fractional tile units (center of the item).
pub struct ItemView {
    pub x: f32,
    pub y: f32,
    pub kind: ItemKind,
}

/// A machine with runtime status worth drawing (progress bar, buffer counts).
pub struct MachineView {
    pub origin: TilePos,
    pub kind: BuildingKind,
    /// 0.0..1.0 while the machine is working.
    pub progress: Option<f32>,
    pub input_count: u32,
    pub output_count: u32,
}

pub struct FactorySnapshot {
    pub items: Vec<ItemView>,
    pub machines: Vec<MachineView>,
    /// Total items produced by machines, sorted by item name.
    pub produced: Vec<(ItemKind, u64)>,
}

pub fn factory_snapshot(world: &World) -> FactorySnapshot {
    let mut items = Vec::new();
    let mut machines = Vec::new();

    for (id, state) in world.factory.states() {
        let Some(inst) = world.tile_grid.instances.get(&id) else {
            continue;
        };
        let ox = inst.origin.x as f32;
        let oy = inst.origin.y as f32;
        match state {
            BuildingState::Conveyor { item } => {
                if let Some(item) = item {
                    let (dx, dy) = inst.rotation.dir();
                    // progress 0 => entry edge, 1 => exit edge of the tile
                    let t = item.progress - 0.5;
                    items.push(ItemView {
                        x: ox + 0.5 + dx as f32 * t,
                        y: oy + 0.5 + dy as f32 * t,
                        kind: item.kind,
                    });
                }
            }
            BuildingState::Miner { progress, output } => {
                if let Some(kind) = output {
                    items.push(ItemView {
                        x: ox + 0.5,
                        y: oy + 0.5,
                        kind: *kind,
                    });
                }
                machines.push(MachineView {
                    origin: inst.origin,
                    kind: BuildingKind::Miner,
                    progress: output.is_none().then_some(*progress),
                    input_count: 0,
                    output_count: u32::from(output.is_some()),
                });
            }
            BuildingState::Smelter {
                input,
                craft,
                output,
            } => {
                machines.push(MachineView {
                    origin: inst.origin,
                    kind: BuildingKind::Smelter,
                    progress: craft.as_ref().map(|c| c.progress),
                    input_count: input.map_or(0, |(_, n)| n),
                    output_count: output.map_or(0, |(_, n)| n),
                });
            }
        }
    }

    let mut produced: Vec<(ItemKind, u64)> = world
        .factory
        .produced
        .iter()
        .map(|(k, n)| (*k, *n))
        .collect();
    produced.sort_by_key(|(k, _)| k.name());

    FactorySnapshot {
        items,
        machines,
        produced,
    }
}
