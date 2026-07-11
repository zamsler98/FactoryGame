//! game_core: pure game state.
//! This crate must not depend on Macroquad or any platform APIs.
//! It contains the world and deterministic update functions.

mod building;
mod factory;
mod grid;
mod item;
mod recipe;
pub use building::*;
pub use factory::*;
pub use grid::*;
pub use item::*;
pub use recipe::*;

/// World container holding the tile grid and the factory simulation.
pub struct World {
    pub tile_grid: TileGrid,
    pub factory: Factory,
}

impl World {
    /// Create an empty world.
    pub fn new() -> Self {
        Self {
            tile_grid: TileGrid::new(1000, 1000),
            factory: Factory::new(),
        }
    }

    /// Place a building on the grid and register its factory state.
    /// Always use this (not `TileGrid::place` directly) so the simulation
    /// knows about the building.
    pub fn place_building(
        &mut self,
        kind: BuildingKind,
        origin: TilePos,
        rot: Rotation,
    ) -> Result<InstanceId, PlacementError> {
        let id = self.tile_grid.place(&kind.spec(), origin, rot)?;
        if let Some(inst) = self.tile_grid.instances.get(&id) {
            self.factory.on_building_placed(inst);
        }
        Ok(id)
    }

    /// Remove a building and its factory state (items it held are lost).
    pub fn remove_building(&mut self, id: InstanceId) -> Option<BuildingInstance> {
        let inst = self.tile_grid.remove(id)?;
        self.factory.on_building_removed(id);
        Some(inst)
    }

    /// Advance the factory simulation by `dt` seconds.
    pub fn update_factory(&mut self, dt: f32) {
        self.factory.tick(&self.tile_grid, dt);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
