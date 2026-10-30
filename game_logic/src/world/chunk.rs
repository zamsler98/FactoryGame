use game_core::Entity;

use super::ChunkCoords;

/// Width and height of a chunk, in tiles.
pub const CHUNK_TILES: i32 = 8;
/// Width and height of a tile, in world units.
pub const TILE_SIZE: i32 = 32;
/// Width and height of a chunk, in world units.
pub const CHUNK_SIZE: i32 = CHUNK_TILES * TILE_SIZE;

#[derive(Debug, Default)]
pub struct Chunk {
    pub entities: Vec<Entity>,
    pub coords: ChunkCoords,
}

impl Chunk {
    pub fn new(coords: ChunkCoords) -> Self {
        Self {
            entities: Vec::new(),
            coords,
        }
    }
    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}
