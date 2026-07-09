//! game_core: pure game state.
//! This crate must not depend on Macroquad or any platform APIs.
//! It contains the world and deterministic update functions.

mod grid;
pub use grid::*;

/// World container holding the tile grid.
pub struct World {
    pub tile_grid: TileGrid,
}

impl World {
    /// Create an empty world.
    pub fn new() -> Self {
        Self {
            tile_grid: TileGrid::new(1000, 1000),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
