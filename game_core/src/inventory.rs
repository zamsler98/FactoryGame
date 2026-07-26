//! The player's inventory and hand-crafting queue.
//!
//! Placing a building consumes its item from the inventory; mining one returns
//! it. Hand-crafting consumes ingredients up front, runs one craft at a time
//! through a queue, and deposits the outputs when each finishes.

use std::collections::HashMap;

use crate::{recipe, ItemKind, RecipeId};

#[derive(Default)]
pub struct Inventory {
    items: HashMap<ItemKind, u32>,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    /// The starting kit: enough to bootstrap the first mining -> smelting loop.
    pub fn starting_kit() -> Self {
        let mut inv = Self::new();
        inv.add(ItemKind::BurnerMiningDrill, 2);
        inv.add(ItemKind::StoneFurnace, 2);
        inv.add(ItemKind::TransportBelt, 20);
        inv.add(ItemKind::Inserter, 6);
        inv.add(ItemKind::WoodenChest, 2);
        inv.add(ItemKind::AssemblingMachine, 1);
        inv.add(ItemKind::Coal, 20);
        inv
    }

    pub fn count(&self, item: ItemKind) -> u32 {
        self.items.get(&item).copied().unwrap_or(0)
    }

    pub fn add(&mut self, item: ItemKind, n: u32) {
        if n > 0 {
            *self.items.entry(item).or_insert(0) += n;
        }
    }

    /// Remove `n` of `item` if at least that many are present. Returns whether
    /// the removal happened.
    pub fn remove(&mut self, item: ItemKind, n: u32) -> bool {
        let have = self.count(item);
        if have < n {
            return false;
        }
        if have == n {
            self.items.remove(&item);
        } else {
            self.items.insert(item, have - n);
        }
        true
    }

    /// Non-empty stacks, in `ItemKind::ALL` order (stable for display).
    pub fn stacks(&self) -> Vec<(ItemKind, u32)> {
        ItemKind::ALL
            .iter()
            .filter_map(|k| {
                let n = self.count(*k);
                (n > 0).then_some((*k, n))
            })
            .collect()
    }

    fn has_ingredients(&self, id: RecipeId) -> bool {
        let Some(r) = recipe(id) else {
            return false;
        };
        r.inputs.iter().all(|i| self.count(i.item) >= i.count)
    }

    fn consume_ingredients(&mut self, id: RecipeId) -> bool {
        if !self.has_ingredients(id) {
            return false;
        }
        let r = recipe(id).expect("checked above");
        for i in r.inputs {
            self.remove(i.item, i.count);
        }
        true
    }
}

/// A single hand-craft in progress.
#[derive(Clone, Copy, Debug)]
pub struct ActiveCraft {
    pub recipe: RecipeId,
    /// 0.0..1.0
    pub progress: f32,
}

/// The player's hand-crafting queue: at most one active craft, the rest wait.
#[derive(Default)]
pub struct CraftQueue {
    active: Option<ActiveCraft>,
    pending: Vec<RecipeId>,
}

impl CraftQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn active(&self) -> Option<ActiveCraft> {
        self.active
    }

    pub fn pending(&self) -> &[RecipeId] {
        &self.pending
    }

    pub fn is_empty(&self) -> bool {
        self.active.is_none() && self.pending.is_empty()
    }

    /// Queue a hand-craftable recipe.
    pub fn enqueue(&mut self, id: RecipeId) {
        if recipe(id).is_some_and(|r| r.hand_craftable) {
            self.pending.push(id);
        }
    }

    /// Advance crafting by `dt` seconds, drawing from and depositing into
    /// `inv`. Ingredients are consumed when a craft starts; outputs are added
    /// when it finishes.
    pub fn update(&mut self, inv: &mut Inventory, dt: f32) {
        // Start the next craft if idle.
        if self.active.is_none() {
            while let Some(id) = (!self.pending.is_empty()).then(|| self.pending.remove(0)) {
                if inv.consume_ingredients(id) {
                    self.active = Some(ActiveCraft {
                        recipe: id,
                        progress: 0.0,
                    });
                    break;
                }
                // Not enough ingredients: drop this entry and try the next.
            }
        }

        let Some(active) = self.active.as_mut() else {
            return;
        };
        let Some(r) = recipe(active.recipe) else {
            self.active = None;
            return;
        };
        active.progress += dt / r.duration;
        if active.progress >= 1.0 {
            for out in r.outputs {
                inv.add(out.item, out.count);
            }
            self.active = None;
        }
    }
}
