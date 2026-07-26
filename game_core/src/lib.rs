//! game_core: pure game state.
//! This crate must not depend on Macroquad or any platform APIs.
//! It contains the world and deterministic update functions.

mod building;
mod factory;
mod grid;
mod inventory;
mod item;
mod recipe;
mod resource;
pub use building::*;
pub use factory::*;
pub use grid::*;
pub use inventory::*;
pub use item::*;
pub use recipe::*;
pub use resource::*;

/// Why a building could not be placed.
#[derive(Debug)]
pub enum PlaceError {
    Grid(PlacementError),
    /// The player has none of the building item in their inventory.
    NoItem,
    /// The building must sit on a resource patch and the tile has none.
    MissingResource,
}

/// World container: the tile grid, the resource field under it, the factory
/// simulation, the player's inventory, and their hand-crafting queue.
pub struct World {
    pub tile_grid: TileGrid,
    pub resources: ResourceLayer,
    pub factory: Factory,
    pub inventory: Inventory,
    pub crafting: CraftQueue,
}

impl World {
    /// Create an empty world with generated ore patches and a starting kit.
    pub fn new() -> Self {
        Self {
            tile_grid: TileGrid::new(1000, 1000),
            resources: ResourceLayer::generate_starting_field(),
            factory: Factory::new(),
            inventory: Inventory::starting_kit(),
            crafting: CraftQueue::new(),
        }
    }

    /// A bare world with no ore and an empty inventory (for deterministic
    /// unit tests that set up their own scenario).
    pub fn empty() -> Self {
        Self {
            tile_grid: TileGrid::new(1000, 1000),
            resources: ResourceLayer::new(),
            factory: Factory::new(),
            inventory: Inventory::new(),
            crafting: CraftQueue::new(),
        }
    }

    /// Place a building, consuming its item from the inventory. Miners must be
    /// placed on a resource patch. On success the factory simulation registers
    /// the new building's runtime state.
    pub fn place_building(
        &mut self,
        kind: BuildingKind,
        origin: TilePos,
        rot: Rotation,
    ) -> Result<InstanceId, PlaceError> {
        if self.inventory.count(kind.item()) == 0 {
            return Err(PlaceError::NoItem);
        }
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
        self.inventory.remove(kind.item(), 1);
        if let Some(inst) = self.tile_grid.instances.get(&id) {
            self.factory.on_building_placed(inst);
        }
        Ok(id)
    }

    /// Mine (remove) a building: it returns to the inventory along with any
    /// items it was holding.
    pub fn remove_building(&mut self, id: InstanceId) -> Option<BuildingInstance> {
        let inst = self.tile_grid.remove(id)?;
        if let Some(kind) = BuildingKind::from_spec_id(inst.spec_id) {
            self.inventory.add(kind.item(), 1);
        }
        if let Some(state) = self.factory.on_building_removed(id) {
            for (item, n) in salvage(&state) {
                self.inventory.add(item, n);
            }
        }
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
            // Changing the recipe returns any buffered inputs and cancels the
            // in-progress craft, matching Factorio's behavior.
            if *sel != recipe {
                let salvaged: Vec<(ItemKind, u32)> = inputs.iter().map(|(k, n)| (*k, *n)).collect();
                inputs.clear();
                *craft = None;
                for (item, n) in salvaged {
                    self.inventory.add(item, n);
                }
                *sel = recipe;
            }
        }
    }

    /// Move everything out of a chest into the player's inventory.
    pub fn take_all_from_chest(&mut self, id: InstanceId) {
        if let Some(BuildingState::Chest { items }) = self.factory.state_mut(id) {
            let drained: Vec<(ItemKind, u32)> = items.drain().collect();
            for (item, n) in drained {
                self.inventory.add(item, n);
            }
        }
    }

    /// Queue a hand-craft of `recipe` for the player.
    pub fn queue_craft(&mut self, recipe: RecipeId) {
        self.crafting.enqueue(recipe);
    }

    /// Manually load coal from the inventory into a burner's fuel slot (up to
    /// its cap). Handy for bootstrapping before inserters are set up.
    pub fn add_fuel_from_inventory(&mut self, id: InstanceId) {
        let slot = match self.factory.state_mut(id) {
            Some(BuildingState::Miner { fuel, .. }) => fuel,
            Some(BuildingState::Furnace { fuel, .. }) => fuel,
            _ => return,
        };
        let want = FUEL_CAP.saturating_sub(*slot);
        let have = self.inventory.count(ItemKind::Coal).min(want);
        if have > 0 {
            *slot += have;
            self.inventory.remove(ItemKind::Coal, have);
        }
    }

    /// Advance the whole game by `dt` seconds.
    pub fn update(&mut self, dt: f32) {
        self.factory.tick(&self.tile_grid, &mut self.resources, dt);
        self.crafting.update(&mut self.inventory, dt);
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// Items to return to the inventory when a building holding them is mined.
fn salvage(state: &BuildingState) -> Vec<(ItemKind, u32)> {
    let mut out = Vec::new();
    let mut push = |item: ItemKind, n: u32| {
        if n > 0 {
            out.push((item, n));
        }
    };
    match state {
        BuildingState::Belt { item } => {
            if let Some(it) = item {
                push(it.kind, 1);
            }
        }
        BuildingState::Miner { fuel, output, .. } => {
            push(ItemKind::Coal, *fuel);
            if let Some(o) = output {
                push(*o, 1);
            }
        }
        BuildingState::Furnace {
            fuel,
            input,
            output,
            ..
        } => {
            push(ItemKind::Coal, *fuel);
            if let Some((k, n)) = input {
                push(*k, *n);
            }
            if let Some((k, n)) = output {
                push(*k, *n);
            }
        }
        BuildingState::Inserter { holding, .. } => {
            if let Some(item) = holding {
                push(*item, 1);
            }
        }
        BuildingState::Assembler {
            inputs, outputs, ..
        } => {
            for (k, n) in inputs.iter().chain(outputs.iter()) {
                push(*k, *n);
            }
        }
        BuildingState::Chest { items } => {
            for (k, n) in items {
                push(*k, *n);
            }
        }
    }
    out
}
