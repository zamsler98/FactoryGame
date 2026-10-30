use crate::{
    CHUNK_SIZE, EntityInfo, EntityType, GridCoords,
    world::{Chunk, ChunkCoords},
};
use game_core::{EntityAllocator, EntitySparseSet, Rectf};
use std::collections::HashMap;

#[derive(Default)]
pub struct World {
    chunks: HashMap<ChunkCoords, Chunk>,
    pub entity_allocator: EntityAllocator,
    pub entity_infos: EntitySparseSet<EntityInfo>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            entity_allocator: EntityAllocator::default(),
            entity_infos: EntitySparseSet::default(),
        }
    }

    pub fn place_miner(&mut self, position: GridCoords) {
        let entity = self.entity_allocator.spawn();
        let entity_info = EntityInfo {
            position,
            entity_type: EntityType::Miner,
        };
        self.entity_infos.insert(entity, entity_info);
        let chunk_coords = position.to_chunk_coords();
        let chunk = self
            .chunks
            .entry(chunk_coords)
            .or_insert_with(|| Chunk::new(chunk_coords));

        chunk.entities.push(entity);
    }

    pub fn create_chunks_in_rect(&mut self, rect: Rectf) {
        let (top_left, bot_right) = World::get_top_left_and_bot_right(rect);
        for row in top_left.y..bot_right.y {
            for col in top_left.x..bot_right.x {
                let chunk_coords = ChunkCoords::new(col, row);
                self.chunks
                    .entry(chunk_coords)
                    .or_insert_with(|| Chunk::new(chunk_coords));
            }
        }
    }

    fn get_top_left_and_bot_right(rect: Rectf) -> (ChunkCoords, ChunkCoords) {
        let chunk_size = CHUNK_SIZE as f32;
        let top_left = (rect.position / chunk_size).floor();
        let bot_right = ((rect.position + rect.size) / chunk_size).ceil();
        (
            ChunkCoords::new(top_left.x, top_left.y),
            ChunkCoords::new(bot_right.x, bot_right.y),
        )
    }

    pub fn chunks_overlapping_rect(&self, rect: Rectf) -> Vec<&Chunk> {
        let (top_left, bot_right) = World::get_top_left_and_bot_right(rect);
        let mut chunks = Vec::new();
        for row in top_left.y..bot_right.y {
            for col in top_left.x..bot_right.x {
                let chunk_coords = ChunkCoords::new(col, row);
                let maybe_chunk = self.chunks.get(&chunk_coords);
                if let Some(chunk) = maybe_chunk {
                    chunks.push(chunk);
                }
            }
        }
        chunks
    }
}
