//! Crafting recipes. To add a new smelter recipe, extend `SMELTER_RECIPES`.

use crate::ItemKind;

#[derive(Clone, Copy, Debug)]
pub struct Recipe {
    pub input: ItemKind,
    pub output: ItemKind,
    /// Seconds to craft one output item.
    pub duration: f32,
}

pub const SMELTER_RECIPES: &[Recipe] = &[Recipe {
    input: ItemKind::IronOre,
    output: ItemKind::IronIngot,
    duration: 2.0,
}];

pub fn smelter_recipe_for(input: ItemKind) -> Option<&'static Recipe> {
    SMELTER_RECIPES.iter().find(|r| r.input == input)
}
