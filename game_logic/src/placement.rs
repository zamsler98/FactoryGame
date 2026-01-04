use game_core::{BuildingSpec, InstanceId, Rotation, TileGrid, TilePos};

pub fn try_place_building(
    grid: &mut TileGrid,
    spec: &BuildingSpec,
    origin: TilePos,
    rot: Rotation,
) -> Result<InstanceId, game_core::PlacementError> {
    grid.place(spec, origin, rot)
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
