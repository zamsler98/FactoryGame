use game_core::Entity;

use crate::Position;

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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkCoords {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoords {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    /// The chunk containing `position`. Euclidean division so that negative
    /// coordinates floor toward the chunk they sit in rather than toward zero.
    pub fn from_position(position: Position) -> Self {
        Self {
            x: position.x.div_euclid(CHUNK_SIZE),
            y: position.y.div_euclid(CHUNK_SIZE),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_position_0_0() {
        let position = Position { x: 0, y: 0 };
        let chunk_coords = ChunkCoords::from_position(position);
        assert_eq!(chunk_coords, ChunkCoords::new(0, 0));
    }

    #[test]
    fn from_position_2_2() {
        let position = Position { x: 550, y: 550 };
        let chunk_coords = ChunkCoords::from_position(position);
        assert_eq!(chunk_coords, ChunkCoords::new(2, 2));
    }

    #[test]
    fn from_position_neg2_neg2() {
        let position = Position { x: -260, y: -260 };
        let chunk_coords = ChunkCoords::from_position(position);
        assert_eq!(chunk_coords, ChunkCoords::new(-2, -2));
    }

    /// The chunk edges: the last unit of a chunk and the first of the next.
    #[test]
    fn from_position_chunk_boundaries() {
        let cases = [
            (CHUNK_SIZE - 1, 0),
            (CHUNK_SIZE, 1),
            (-1, -1),
            (-CHUNK_SIZE, -1),
            (-CHUNK_SIZE - 1, -2),
        ];

        for (world, chunk) in cases {
            let position = Position { x: world, y: world };
            assert_eq!(
                ChunkCoords::from_position(position),
                ChunkCoords::new(chunk, chunk),
                "world coordinate {world} should land in chunk {chunk}"
            );
        }
    }
}
