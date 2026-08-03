//! Building catalog: every placeable building kind and its static spec.
//!
//! Every building is also an inventory item (the thing you place and get back
//! when you mine it) — see `ItemKind` and the `item`/`from_item` map below.
//!
//! To add a new building: add a variant, extend `ALL`, the `spec_id` map, and
//! the `item` map, then give it runtime behavior in `factory.rs`.

use crate::{BuildingSpec, ItemKind, Size2};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuildingKind {
    /// Moves items along the direction it faces.
    Belt,
    /// Sits on an ore patch, burns fuel, and mines the ore under it.
    Miner,
    /// Burns fuel and smelts ore into plates/bricks.
    Furnace,
    /// Grabs an item from the tile behind it and drops it on the tile ahead.
    Inserter,
    /// Crafts a selected multi-ingredient recipe.
    Assembler,
    /// Passively stores items; loaded and unloaded by inserters.
    Chest,
}

impl BuildingKind {
    pub const ALL: [BuildingKind; 6] = [
        BuildingKind::Belt,
        BuildingKind::Miner,
        BuildingKind::Furnace,
        BuildingKind::Inserter,
        BuildingKind::Assembler,
        BuildingKind::Chest,
    ];

    /// Stable numeric id stored in `BuildingInstance::spec_id`.
    pub fn spec_id(self) -> u32 {
        match self {
            BuildingKind::Belt => 1,
            BuildingKind::Miner => 2,
            BuildingKind::Furnace => 3,
            BuildingKind::Inserter => 4,
            BuildingKind::Assembler => 5,
            BuildingKind::Chest => 6,
        }
    }

    pub fn from_spec_id(id: u32) -> Option<Self> {
        Self::ALL.iter().copied().find(|k| k.spec_id() == id)
    }

    pub fn name(self) -> &'static str {
        self.item().name()
    }

    /// The inventory item consumed to place this building (and returned when
    /// it is mined).
    pub fn item(self) -> ItemKind {
        match self {
            BuildingKind::Belt => ItemKind::TransportBelt,
            BuildingKind::Miner => ItemKind::BurnerMiningDrill,
            BuildingKind::Furnace => ItemKind::StoneFurnace,
            BuildingKind::Inserter => ItemKind::Inserter,
            BuildingKind::Assembler => ItemKind::AssemblingMachine,
            BuildingKind::Chest => ItemKind::WoodenChest,
        }
    }

    /// The building an item places, if it is a building item.
    pub fn from_item(item: ItemKind) -> Option<Self> {
        Self::ALL.iter().copied().find(|k| k.item() == item)
    }

    /// Whether this building must be placed on top of a resource patch.
    pub fn requires_resource(self) -> bool {
        matches!(self, BuildingKind::Miner)
    }

    pub fn spec(self) -> BuildingSpec {
        BuildingSpec {
            spec_id: self.spec_id(),
            size: Size2 { w: 1, h: 1 },
        }
    }
}
