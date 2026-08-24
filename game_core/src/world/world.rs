use crate::{entity::Entity, world::chunk::ChunkCoords, Chunk};
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
