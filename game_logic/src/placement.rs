use game_core::{BuildingKind, InstanceId, PlacementError, Rotation, TileGrid, TilePos, World};

/// Place a building through the `World` so the factory simulation registers
/// its runtime state alongside the grid footprint.
pub fn try_place_building(
    world: &mut World,
    kind: BuildingKind,
    origin: TilePos,
    rot: Rotation,
) -> Result<InstanceId, PlacementError> {
    world.place_building(kind, origin, rot)
}

// A minimal snapshot type for the renderer
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
