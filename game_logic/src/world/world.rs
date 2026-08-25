use crate::world::{Chunk, ChunkCoords};
use game_core::Entity;
use std::collections::HashMap;

pub struct World {
    pub chunks: HashMap<ChunkCoords, Chunk>,
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            entities: Vec::new(),
        }
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}
