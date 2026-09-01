use game_core::Entity;

/// Width and height of a chunk, in tiles.
pub const CHUNK_TILES: u32 = 8;
/// Width and height of a tile, in world units.
pub const TILE_SIZE: f32 = 32.0;
/// Width and height of a chunk, in world units.
pub const CHUNK_SIZE: f32 = CHUNK_TILES as f32 * TILE_SIZE;

#[derive(Debug, Default)]
pub struct Chunk {
    pub entities: Vec<Entity>,
    pub coords: ChunkCoords,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkCoords {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
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
