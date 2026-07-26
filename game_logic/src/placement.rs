use game_core::{
    BuildingKind, InstanceId, PlaceError, ResourceKind, Rotation, TileGrid, TilePos, World,
};

/// Place a building through the `World` so the factory simulation registers
/// its runtime state and the player's inventory is charged for it.
pub fn try_place_building(
    world: &mut World,
    kind: BuildingKind,
    origin: TilePos,
    rot: Rotation,
) -> Result<InstanceId, PlaceError> {
    world.place_building(kind, origin, rot)
}

/// Mine (remove) whatever building sits on `tile`, returning it to inventory.
pub fn mine_at(world: &mut World, tile: TilePos) -> bool {
    if let Some(id) = world.tile_grid.tile_occupant(tile) {
        world.remove_building(id).is_some()
    } else {
        false
    }
}

// A minimal snapshot type for the renderer.
pub struct TileGridSnapshot {
    pub width: usize,
    pub height: usize,
    pub instances: Vec<game_core::BuildingInstance>,
}

pub fn grid_snapshot(grid: &TileGrid) -> TileGridSnapshot {
    TileGridSnapshot {
        width: grid.width,
        height: grid.height,
        instances: grid.instances.values().cloned().collect(),
    }
}

/// A resource patch tile to tint under the grid.
pub struct ResourceView {
    pub pos: TilePos,
    pub kind: ResourceKind,
    pub amount: u32,
}

pub fn resource_snapshot(world: &World) -> Vec<ResourceView> {
    world
        .resources
        .patches()
        .map(|(pos, p)| ResourceView {
            pos,
            kind: p.kind,
            amount: p.amount,
        })
        .collect()
}
