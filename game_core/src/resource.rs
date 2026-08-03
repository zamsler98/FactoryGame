//! The resource layer: finite ore patches sitting under the tile grid.
//!
//! A burner mining drill must be placed on a tile that has a patch; it mines
//! the patch's item and decrements the remaining amount until the patch is
//! exhausted. Patches are generated deterministically at world creation so the
//! sim stays reproducible and headless-testable.

use std::collections::HashMap;

use crate::{ItemKind, TilePos};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceKind {
    IronOre,
    CopperOre,
    Coal,
    Stone,
}

impl ResourceKind {
    /// The item a drill produces from this resource.
    pub fn mined_item(self) -> ItemKind {
        match self {
            ResourceKind::IronOre => ItemKind::IronOre,
            ResourceKind::CopperOre => ItemKind::CopperOre,
            ResourceKind::Coal => ItemKind::Coal,
            ResourceKind::Stone => ItemKind::Stone,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            ResourceKind::IronOre => "Iron Ore",
            ResourceKind::CopperOre => "Copper Ore",
            ResourceKind::Coal => "Coal",
            ResourceKind::Stone => "Stone",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ResourcePatch {
    pub kind: ResourceKind,
    /// Units of ore remaining; each mine op removes one.
    pub amount: u32,
}

#[derive(Default)]
pub struct ResourceLayer {
    tiles: HashMap<TilePos, ResourcePatch>,
}

impl ResourceLayer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate the starting ore field: a handful of circular blobs near the
    /// origin so the initial camera view (top-left of the grid) sees them.
    pub fn generate_starting_field() -> Self {
        let mut layer = Self::new();
        // (center_x, center_y, radius, kind, per-tile amount)
        let blobs = [
            (6, 6, 3, ResourceKind::Coal, 1500),
            (16, 5, 4, ResourceKind::IronOre, 2000),
            (6, 17, 3, ResourceKind::CopperOre, 1500),
            (18, 17, 3, ResourceKind::Stone, 1200),
            (30, 10, 4, ResourceKind::IronOre, 2500),
            (30, 24, 3, ResourceKind::Coal, 2000),
        ];
        for (cx, cy, r, kind, amount) in blobs {
            layer.add_blob(cx, cy, r, kind, amount);
        }
        layer
    }

    fn add_blob(&mut self, cx: i32, cy: i32, r: i32, kind: ResourceKind, amount: u32) {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let pos = TilePos {
                        x: cx + dx,
                        y: cy + dy,
                    };
                    self.tiles.insert(pos, ResourcePatch { kind, amount });
                }
            }
        }
    }

    pub fn get(&self, pos: TilePos) -> Option<ResourcePatch> {
        self.tiles.get(&pos).copied()
    }

    /// Iterate over every patch tile (for rendering).
    pub fn patches(&self) -> impl Iterator<Item = (TilePos, ResourcePatch)> + '_ {
        self.tiles.iter().map(|(p, patch)| (*p, *patch))
    }

    /// Mine one unit from the patch at `pos`, returning the item produced.
    /// Removes the patch once exhausted. Returns `None` if there is no patch.
    pub fn mine(&mut self, pos: TilePos) -> Option<ItemKind> {
        let patch = self.tiles.get_mut(&pos)?;
        let item = patch.kind.mined_item();
        patch.amount -= 1;
        if patch.amount == 0 {
            self.tiles.remove(&pos);
        }
        Some(item)
    }
}
