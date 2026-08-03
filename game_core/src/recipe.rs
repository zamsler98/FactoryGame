//! Crafting recipes shared by furnaces, assemblers, and the player's hands.
//!
//! A recipe turns a set of input items into a set of output items over a fixed
//! duration. `category` decides which machine can run it: `Smelting` recipes
//! run in furnaces (auto-selected from the input ore), `Crafting` recipes run
//! in assemblers (the player picks one) and, when `hand_craftable`, in the
//! player's crafting queue.
//!
//! To add a recipe, append to `RECIPES` with a fresh `id`.

use crate::ItemKind;

pub type RecipeId = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CraftCategory {
    /// Runs in a furnace; selected automatically from the ore fed in.
    Smelting,
    /// Runs in an assembler (or the player's hands).
    Crafting,
}

#[derive(Clone, Copy, Debug)]
pub struct Ingredient {
    pub item: ItemKind,
    pub count: u32,
}

/// Shorthand for a static ingredient.
const fn ing(item: ItemKind, count: u32) -> Ingredient {
    Ingredient { item, count }
}

#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub id: RecipeId,
    pub name: &'static str,
    pub category: CraftCategory,
    pub inputs: &'static [Ingredient],
    pub outputs: &'static [Ingredient],
    /// Seconds to complete one craft.
    pub duration: f32,
    /// Whether the player can craft this by hand.
    pub hand_craftable: bool,
}

impl Recipe {
    /// The single primary output item (all current recipes have one).
    pub fn primary_output(&self) -> ItemKind {
        self.outputs[0].item
    }
}

use CraftCategory::{Crafting, Smelting};
use ItemKind::*;

pub const RECIPES: &[Recipe] = &[
    // --- Smelting (furnaces) ---
    Recipe {
        id: 1,
        name: "Iron Plate",
        category: Smelting,
        inputs: &[ing(IronOre, 1)],
        outputs: &[ing(IronPlate, 1)],
        duration: 2.0,
        hand_craftable: false,
    },
    Recipe {
        id: 2,
        name: "Copper Plate",
        category: Smelting,
        inputs: &[ing(CopperOre, 1)],
        outputs: &[ing(CopperPlate, 1)],
        duration: 2.0,
        hand_craftable: false,
    },
    Recipe {
        id: 3,
        name: "Stone Brick",
        category: Smelting,
        inputs: &[ing(Stone, 2)],
        outputs: &[ing(StoneBrick, 1)],
        duration: 3.2,
        hand_craftable: false,
    },
    // --- Crafting (assemblers + hand) ---
    Recipe {
        id: 4,
        name: "Iron Gear Wheel",
        category: Crafting,
        inputs: &[ing(IronPlate, 2)],
        outputs: &[ing(IronGearWheel, 1)],
        duration: 0.5,
        hand_craftable: true,
    },
    Recipe {
        id: 5,
        name: "Copper Cable",
        category: Crafting,
        inputs: &[ing(CopperPlate, 1)],
        outputs: &[ing(CopperCable, 2)],
        duration: 0.5,
        hand_craftable: true,
    },
    Recipe {
        id: 6,
        name: "Electronic Circuit",
        category: Crafting,
        inputs: &[ing(IronPlate, 1), ing(CopperCable, 3)],
        outputs: &[ing(ElectronicCircuit, 1)],
        duration: 0.5,
        hand_craftable: true,
    },
    // --- Crafting: building items ---
    Recipe {
        id: 7,
        name: "Transport Belt",
        category: Crafting,
        inputs: &[ing(IronGearWheel, 1), ing(IronPlate, 1)],
        outputs: &[ing(TransportBelt, 2)],
        duration: 0.5,
        hand_craftable: true,
    },
    Recipe {
        id: 8,
        name: "Stone Furnace",
        category: Crafting,
        inputs: &[ing(Stone, 5)],
        outputs: &[ing(StoneFurnace, 1)],
        duration: 0.5,
        hand_craftable: true,
    },
    Recipe {
        id: 9,
        name: "Burner Mining Drill",
        category: Crafting,
        inputs: &[
            ing(IronGearWheel, 3),
            ing(IronPlate, 3),
            ing(StoneFurnace, 1),
        ],
        outputs: &[ing(BurnerMiningDrill, 1)],
        duration: 2.0,
        hand_craftable: true,
    },
    Recipe {
        id: 10,
        name: "Inserter",
        category: Crafting,
        inputs: &[
            ing(IronGearWheel, 1),
            ing(IronPlate, 1),
            ing(ElectronicCircuit, 1),
        ],
        outputs: &[ing(Inserter, 1)],
        duration: 0.5,
        hand_craftable: true,
    },
    Recipe {
        id: 11,
        name: "Assembling Machine",
        category: Crafting,
        inputs: &[
            ing(ElectronicCircuit, 3),
            ing(IronGearWheel, 5),
            ing(IronPlate, 9),
        ],
        outputs: &[ing(AssemblingMachine, 1)],
        duration: 0.5,
        hand_craftable: true,
    },
    Recipe {
        id: 12,
        name: "Wooden Chest",
        category: Crafting,
        inputs: &[ing(IronPlate, 2)],
        outputs: &[ing(WoodenChest, 1)],
        duration: 0.5,
        hand_craftable: true,
    },
];

pub fn recipe(id: RecipeId) -> Option<&'static Recipe> {
    RECIPES.iter().find(|r| r.id == id)
}

/// The smelting recipe that consumes `input` ore, if any.
pub fn smelting_recipe_for(input: ItemKind) -> Option<&'static Recipe> {
    RECIPES
        .iter()
        .find(|r| r.category == Smelting && r.inputs.iter().any(|i| i.item == input))
}

/// All recipes an assembler can be set to run.
pub fn assembler_recipes() -> impl Iterator<Item = &'static Recipe> {
    RECIPES.iter().filter(|r| r.category == Crafting)
}

/// All recipes the player can queue by hand.
pub fn hand_recipes() -> impl Iterator<Item = &'static Recipe> {
    RECIPES.iter().filter(|r| r.hand_craftable)
}
