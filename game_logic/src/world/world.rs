use crate::{
    CHUNK_SIZE, Position,
    world::{Chunk, ChunkCoords},
};
use game_core::{EntityAllocator, Rectf};
use std::collections::HashMap;

#[derive(Default)]
pub struct World {
    chunks: HashMap<ChunkCoords, Chunk>,
    pub entities: EntityAllocator,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            entities: EntityAllocator::default(),
        }
    }

    pub fn place_miner(_position: Position) {}

    pub fn chunks_overlapping_rect_or_create(&mut self, rect: Rectf) -> Vec<&Chunk> {
        let chunk_size = CHUNK_SIZE as f32;
        let top_left = (rect.position / chunk_size).floor();
        let bot_right = ((rect.position + rect.size) / chunk_size).ceil();

        for row in top_left.y..bot_right.y {
            for col in top_left.x..bot_right.x {
                let chunk_coords = ChunkCoords::new(col, row);
                self.chunks
                    .entry(chunk_coords)
                    .or_insert_with(|| Chunk::new(chunk_coords));
            }
        }

        let mut chunks = Vec::new();
        for row in top_left.y..bot_right.y {
            for col in top_left.x..bot_right.x {
                let chunk_coords = ChunkCoords::new(col, row);
                let chunk = self
                    .chunks
                    .get(&chunk_coords)
                    .expect("chunk was ensured in the first pass");
                chunks.push(chunk);
            }
        }
        chunks
    }
}
