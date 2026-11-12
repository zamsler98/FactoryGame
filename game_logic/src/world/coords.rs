//! Distinct coordinate spaces and explicit conversions between them.

use super::{CHUNK_SIZE, CHUNK_TILES, TILE_SIZE};

/// A position in whole world units.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct WorldCoords {
    pub x: i32,
    pub y: i32,
}

/// A global tile index, not an offset within a chunk.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct GridCoords {
    pub x: i32,
    pub y: i32,
}

/// A global chunk index.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkCoords {
    pub x: i32,
    pub y: i32,
}

impl WorldCoords {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// The containing tile, including for negative world positions.
    pub const fn to_grid_coords(self) -> GridCoords {
        GridCoords::new(self.x.div_euclid(TILE_SIZE), self.y.div_euclid(TILE_SIZE))
    }

    /// The containing chunk, including for negative world positions.
    pub const fn to_chunk_coords(self) -> ChunkCoords {
        ChunkCoords::new(self.x.div_euclid(CHUNK_SIZE), self.y.div_euclid(CHUNK_SIZE))
    }
}

impl GridCoords {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// The chunk containing this tile.
    pub const fn to_chunk_coords(self) -> ChunkCoords {
        ChunkCoords::new(
            self.x.div_euclid(CHUNK_TILES),
            self.y.div_euclid(CHUNK_TILES),
        )
    }

    /// The world position of this tile's top-left corner.
    pub const fn world_origin(self) -> WorldCoords {
        WorldCoords::new(self.x * TILE_SIZE, self.y * TILE_SIZE)
    }
}

impl ChunkCoords {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// The global tile index of this chunk's top-left corner.
    pub const fn grid_origin(self) -> GridCoords {
        GridCoords::new(self.x * CHUNK_TILES, self.y * CHUNK_TILES)
    }

    /// The world position of this chunk's top-left corner.
    pub const fn world_origin(self) -> WorldCoords {
        WorldCoords::new(self.x * CHUNK_SIZE, self.y * CHUNK_SIZE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_to_grid_floors_at_tile_boundaries() {
        for (world, tile) in [
            (0, 0),
            (TILE_SIZE - 1, 0),
            (TILE_SIZE, 1),
            (-1, -1),
            (-TILE_SIZE, -1),
            (-TILE_SIZE - 1, -2),
        ] {
            assert_eq!(
                WorldCoords::new(world, -1).to_grid_coords(),
                GridCoords::new(tile, -1),
            );
        }
    }

    #[test]
    fn grid_to_chunk_floors_at_chunk_boundaries() {
        for (tile, chunk) in [
            (0, 0),
            (CHUNK_TILES - 1, 0),
            (CHUNK_TILES, 1),
            (-1, -1),
            (-CHUNK_TILES, -1),
            (-CHUNK_TILES - 1, -2),
        ] {
            assert_eq!(
                GridCoords::new(tile, -1).to_chunk_coords(),
                ChunkCoords::new(chunk, -1),
            );
        }
    }

    #[test]
    fn world_to_chunk_matches_conversion_through_grid() {
        for (world, chunk) in [
            (0, 0),
            (550, 2),
            (-260, -2),
            (CHUNK_SIZE - 1, 0),
            (CHUNK_SIZE, 1),
            (-1, -1),
            (-CHUNK_SIZE, -1),
            (-CHUNK_SIZE - 1, -2),
        ] {
            let position = WorldCoords::new(world, -1);
            assert_eq!(position.to_chunk_coords(), ChunkCoords::new(chunk, -1));
            assert_eq!(
                position.to_chunk_coords(),
                position.to_grid_coords().to_chunk_coords(),
            );
        }
    }

    #[test]
    fn origins_scale_indices_and_round_trip() {
        for (x, y) in [(0, 0), (2, 3), (-2, -3), (1, -2)] {
            let chunk = ChunkCoords::new(x, y);
            let tile = GridCoords::new(x, y);
            assert_eq!(
                chunk.world_origin(),
                WorldCoords::new(x * CHUNK_SIZE, y * CHUNK_SIZE),
            );
            assert_eq!(
                tile.world_origin(),
                WorldCoords::new(x * TILE_SIZE, y * TILE_SIZE),
            );
            assert_eq!(chunk.world_origin().to_chunk_coords(), chunk);
            assert_eq!(chunk.grid_origin().to_chunk_coords(), chunk);
            assert_eq!(chunk.grid_origin().world_origin(), chunk.world_origin());
            assert_eq!(tile.world_origin().to_grid_coords(), tile);
        }
    }
}
