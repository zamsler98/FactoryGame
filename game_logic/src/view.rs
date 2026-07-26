//! Read-only view structs the renderer consumes each frame. This keeps
//! `game_app` decoupled from simulation internals: it draws whatever the
//! snapshot says, nothing more.

use game_core::{recipe, BuildingKind, BuildingState, ItemKind, TilePos, World};

/// An item to draw. `x`/`y` are in fractional tile units (center of the item).
pub struct ItemView {
    pub x: f32,
    pub y: f32,
    pub kind: ItemKind,
}

/// A machine with runtime status worth drawing.
pub struct MachineView {
    pub origin: TilePos,
    pub kind: BuildingKind,
    /// 0.0..1.0 while the machine is actively working.
    pub progress: Option<f32>,
    /// True if this is a burner that currently has fuel to run on.
    pub fueled: bool,
    /// Whether this building is a burner at all (draws a fuel pip).
    pub is_burner: bool,
    /// Small overlay text (buffer counts / selected recipe).
    pub label: Option<String>,
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
        let (dx, dy) = inst.rotation.dir();
        match state {
            BuildingState::Belt { item } => {
                if let Some(item) = item {
                    let t = item.progress - 0.5;
                    items.push(ItemView {
                        x: ox + 0.5 + dx as f32 * t,
                        y: oy + 0.5 + dy as f32 * t,
                        kind: item.kind,
                    });
                }
            }
            BuildingState::Miner {
                fuel,
                burn,
                progress,
                output,
            } => {
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
                    fueled: *burn > 0.0 || *fuel > 0,
                    is_burner: true,
                    label: None,
                });
            }
            BuildingState::Furnace {
                fuel,
                burn,
                input,
                craft,
                output,
            } => {
                let in_n = input.map_or(0, |(_, n)| n);
                let out_n = output.map_or(0, |(_, n)| n);
                machines.push(MachineView {
                    origin: inst.origin,
                    kind: BuildingKind::Furnace,
                    progress: craft.as_ref().map(|c| c.progress),
                    fueled: *burn > 0.0 || *fuel > 0,
                    is_burner: true,
                    label: Some(format!("{in_n}/{out_n}")),
                });
            }
            BuildingState::Inserter { holding, progress } => {
                if let Some(kind) = holding {
                    // Swing from the tile behind to the tile in front.
                    let t = *progress; // 0 at pickup .. 1 at drop
                    let sx = ox + 0.5 - dx as f32 * (1.0 - t);
                    let sy = oy + 0.5 - dy as f32 * (1.0 - t);
                    items.push(ItemView {
                        x: sx,
                        y: sy,
                        kind: *kind,
                    });
                }
            }
            BuildingState::Assembler {
                recipe: sel,
                craft,
                outputs,
                ..
            } => {
                let out_n: u32 = outputs.values().sum();
                let label = sel
                    .and_then(recipe)
                    .map(|r| short_name(r.name))
                    .unwrap_or_else(|| "—".to_string());
                machines.push(MachineView {
                    origin: inst.origin,
                    kind: BuildingKind::Assembler,
                    progress: craft.as_ref().map(|c| c.progress),
                    fueled: true,
                    is_burner: false,
                    label: Some(format!("{label} {out_n}")),
                });
            }
            BuildingState::Chest { items: stored } => {
                let total: u32 = stored.values().sum();
                machines.push(MachineView {
                    origin: inst.origin,
                    kind: BuildingKind::Chest,
                    progress: None,
                    fueled: true,
                    is_burner: false,
                    label: (total > 0).then(|| total.to_string()),
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

/// A very short tag for a recipe name (first letters), for the tiny overlay.
fn short_name(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|w| w.chars().next())
        .take(3)
        .collect()
}
