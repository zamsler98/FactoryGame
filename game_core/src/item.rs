//! Item definitions.
//!
//! To add a new item to the game, add a variant here, give it a name, and
//! (if machines should craft it) add a recipe in `recipe.rs`. Rendering
//! colors live in `game_app`.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ItemKind {
    IronOre,
    IronIngot,
}

impl ItemKind {
    pub fn name(self) -> &'static str {
        match self {
            ItemKind::IronOre => "Iron Ore",
            ItemKind::IronIngot => "Iron Ingot",
        }
    }
}
