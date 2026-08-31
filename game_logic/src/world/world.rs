use crate::{
    Position,
    world::{Chunk, ChunkCoords},
};
use game_core::EntityAllocator;
use std::collections::HashMap;

#[derive(Default)]
pub struct World {
    pub chunks: HashMap<ChunkCoords, Chunk>,
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

    // fn get_chunk_coords_from_position(position: Position) -> ChunkCoords {
    //
    // }
}
