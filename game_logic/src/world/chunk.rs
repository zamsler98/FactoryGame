use game_core::Entity;

use crate::GridCoords;

use super::ChunkCoords;

/// Width and height of a chunk, in tiles.
pub const CHUNK_TILES: i32 = 8;
/// Width and height of a tile, in world units.
pub const TILE_SIZE: i32 = 32;
/// Width and height of a chunk, in world units.
pub const CHUNK_SIZE: i32 = CHUNK_TILES * TILE_SIZE;

#[derive(Debug)]
pub struct Chunk {
    pub entities: Vec<Entity>,
    pub coords: ChunkCoords,
    tiles: Box<[Option<Entity>; (CHUNK_TILES * CHUNK_TILES) as usize]>,
}

impl Chunk {
    pub fn new(coords: ChunkCoords) -> Self {
        Self {
            entities: Vec::new(),
            coords,
            tiles: Box::new(std::array::from_fn(|_| None)),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    pub fn is_tile_occupied(&self, tile: GridCoords) -> bool {
        let index = self.index_of_coords(tile);
        self.tiles[index].is_some()
    }

    pub fn set_tile_entity(&mut self, tile: GridCoords, entity: Entity) {
        let index = self.index_of_coords(tile);
        self.tiles[index] = Some(entity);
    }

    fn index_of_coords(&self, tile: GridCoords) -> usize {
        let local_coords = tile.local_in_chunk();
        (local_coords.x + local_coords.y * CHUNK_TILES) as usize
    }
}

#[cfg(test)]
mod tests {
    use game_core::EntityAllocator;

    use super::*;

    #[test]
    fn tile_start_not_occupied() {
        let chunk = Chunk::new(ChunkCoords::new(0, 0));
        let tile_coords = GridCoords::new(4, 5);
        let is_occupied = chunk.is_tile_occupied(tile_coords);
        assert!(!is_occupied)
    }

    #[test]
    fn set_tile_entity_marks_tile_occupied() {
        let mut chunk = Chunk::new(ChunkCoords::new(0, 0));
        let tile_coords = GridCoords::new(4, 5);
        let mut entity_allocator = EntityAllocator::default();
        let entity = entity_allocator.spawn();
        chunk.set_tile_entity(tile_coords, entity);
        let is_occupied = chunk.is_tile_occupied(tile_coords);
        assert!(is_occupied);
    }

    #[test]
    fn set_tile_handles_farther_tiles() {
        let mut chunk = Chunk::new(ChunkCoords::new(2, 2));
        let tile_coords = GridCoords::new(2 * CHUNK_TILES + 3, 2 * CHUNK_TILES + 1);
        let mut entity_allocator = EntityAllocator::default();
        let entity = entity_allocator.spawn();
        chunk.set_tile_entity(tile_coords, entity);
        let is_occupied = chunk.is_tile_occupied(tile_coords);
        assert!(is_occupied);
    }
}
