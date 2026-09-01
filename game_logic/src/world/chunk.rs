use game_core::{Entity, Vec2i};

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

/// A chunk index. Wraps a `Vec2i` rather than aliasing it so chunk indices
/// cannot be confused with the world-unit `Position` they scale from.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct ChunkCoords(pub Vec2i);

impl ChunkCoords {
    pub const fn new(x: i32, y: i32) -> Self {
        Self(Vec2i::new(x, y))
    }

    /// The chunk containing `position`. Euclidean division so that negative
    /// coordinates floor toward the chunk they sit in rather than toward zero.
    pub const fn from_position(position: Position) -> Self {
        Self(position.div_euclid(CHUNK_SIZE))
    }

    /// The world position of this chunk's top-left corner. The inverse of
    /// `from_position`, rounded down to the chunk it names.
    pub fn origin(self) -> Position {
        self.0 * CHUNK_SIZE
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

    /// `origin` names the corner of the chunk, so feeding it back through
    /// `from_position` must land on the same chunk.
    #[test]
    fn origin_round_trips_through_from_position() {
        for coords in [
            ChunkCoords::new(0, 0),
            ChunkCoords::new(2, 3),
            ChunkCoords::new(-2, -3),
        ] {
            assert_eq!(ChunkCoords::from_position(coords.origin()), coords);
        }
    }

    #[test]
    fn origin_scales_indices_to_world_units() {
        assert_eq!(ChunkCoords::new(0, 0).origin(), Position { x: 0, y: 0 });
        assert_eq!(
            ChunkCoords::new(1, -2).origin(),
            Position {
                x: CHUNK_SIZE,
                y: -2 * CHUNK_SIZE,
            }
        );
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
