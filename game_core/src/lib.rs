//! game_core: pure game state.
//! This crate must not depend on Macroquad or any platform APIs.
//! It contains the world and deterministic update functions.

mod building;
mod factory;
mod grid;
mod item;
mod recipe;
mod resource;
pub use building::*;
pub use factory::*;
pub use grid::*;
pub use item::*;
pub use recipe::*;
pub use resource::*;

/// Why a building could not be placed.
#[derive(Debug)]
pub enum PlaceError {
    Grid(PlacementError),
    /// The building must sit on a resource patch and the tile has none.
    MissingResource,
}

/// World container: the tile grid, the resource field under it, and the factory
/// simulation. Buildings are placed freely (no inventory or build cost).
pub struct World {
    pub tile_grid: TileGrid,
    pub resources: ResourceLayer,
    pub factory: Factory,
}

impl World {
    /// Create a world with generated ore patches.
    pub fn new() -> Self {
        Self {
            tile_grid: TileGrid::new(1000, 1000),
            resources: ResourceLayer::generate_starting_field(),
            factory: Factory::new(),
        }
    }

    /// A bare world with no ore (for deterministic unit tests that set up
    /// their own scenario).
    pub fn empty() -> Self {
        Self {
            tile_grid: TileGrid::new(1000, 1000),
            resources: ResourceLayer::new(),
            factory: Factory::new(),
        }
    }

    /// Place a building. Miners must be placed on a resource patch. On success
    /// the factory simulation registers the new building's runtime state.
    pub fn place_building(
        &mut self,
        kind: BuildingKind,
        origin: TilePos,
        rot: Rotation,
    ) -> Result<InstanceId, PlaceError> {
        if kind.requires_resource() && self.resources.get(origin).is_none() {
            return Err(PlaceError::MissingResource);
        }
        if !self.tile_grid.can_place(&kind.spec(), origin, rot) {
            // Surface the specific grid reason.
            let err = self
                .tile_grid
                .place(&kind.spec(), origin, rot)
                .expect_err("can_place was false");
            return Err(PlaceError::Grid(err));
        }
        let id = self
            .tile_grid
            .place(&kind.spec(), origin, rot)
            .map_err(PlaceError::Grid)?;
        if let Some(inst) = self.tile_grid.instances.get(&id) {
            self.factory.on_building_placed(inst);
        }
        Ok(id)
    }

    /// Mine (remove) a building: it is deleted along with anything it held.
    pub fn remove_building(&mut self, id: InstanceId) -> Option<BuildingInstance> {
        let inst = self.tile_grid.remove(id)?;
        self.factory.on_building_removed(id);
        Some(inst)
    }

    /// Set (or clear) the recipe an assembler runs. Ignored for other kinds.
    pub fn set_assembler_recipe(&mut self, id: InstanceId, recipe: Option<RecipeId>) {
        if let Some(BuildingState::Assembler {
            recipe: sel,
            inputs,
            craft,
            ..
        }) = self.factory.state_mut(id)
        {
            // Changing the recipe discards any buffered inputs and cancels the
            // in-progress craft, matching Factorio's behavior.
            if *sel != recipe {
                inputs.clear();
                *craft = None;
                *sel = recipe;
            }
        }
    }

    /// Advance the whole game by `dt` seconds.
    pub fn update(&mut self, dt: f32) {
        self.factory.tick(&self.tile_grid, &mut self.resources, dt);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
