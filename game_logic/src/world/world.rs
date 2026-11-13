use crate::{
    Building, BuildingType, CHUNK_SIZE, GridCoords,
    world::{Chunk, ChunkCoords, entity_registry::EntityRegistry},
};
use game_core::{Entity, EntityAllocator, EntitySparseSet, Rectf, Vec2i};
use std::collections::HashMap;

#[derive(Default)]
pub struct World {
    chunks: HashMap<ChunkCoords, Chunk>,
    pub entity_registry: EntityRegistry,
}

impl World {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            entity_registry: EntityRegistry::default(),
        }
    }

    pub fn remove_miner(&mut self, position: GridCoords) {
        let entity_option = self
            .entity_registry
            .infos
            .find_first(|x| x.position == position);
        if let Some((entity, _)) = entity_option {
            self.entity_registry.despawn(entity);
        }
    }

    pub fn place_miner(&mut self, position: GridCoords) -> Option<Entity> {
        let size = BuildingType::Miner.size();
        log::debug!("Placing miner at {position:?}. Size of building: {size:?}");
        let tiles_to_check = Self::tiles_overlap_building(position, size);
        if self.check_tiles_occupied(&tiles_to_check) {
            return None;
        }

        let entity = self.entity_registry.spawn();
        let entity_info = Building {
            position,
            entity_type: BuildingType::Miner,
        };
        self.entity_registry.infos.insert(entity, entity_info);
        let chunk_coords = position.to_chunk_coords();
        let chunk = self
            .chunks
            .entry(chunk_coords)
            .or_insert_with(|| Chunk::new(chunk_coords));

        chunk.add_entity(entity);
        for tile in tiles_to_check {
            let chunk_coords = tile.to_chunk_coords();
            if let Some(tile_chunk) = self.chunks.get_mut(&chunk_coords) {
                tile_chunk.set_tile_entity(tile, entity);
            }
        }
        Some(entity)
    }

    fn is_space_occupied(&self, top_left: GridCoords, size: Vec2i) -> bool {
        let tiles_to_check = Self::tiles_overlap_building(top_left, size);
        self.check_tiles_occupied(&tiles_to_check)
    }

    fn check_tiles_occupied(&self, tiles: &[GridCoords]) -> bool {
        tiles.iter().any(|tile| {
            self.chunks
                .get(&tile.to_chunk_coords())
                .is_some_and(|c| c.is_tile_occupied(*tile))
        })
    }

    fn tiles_overlap_building(top_left: GridCoords, size: Vec2i) -> Vec<GridCoords> {
        let mut res: Vec<GridCoords> = Vec::new();
        for i in top_left.x..(top_left.x + size.x) {
            for j in top_left.y..(top_left.y + size.y) {
                let tile_position = GridCoords::new(i, j);
                res.push(tile_position);
            }
        }
        res
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

#[cfg(test)]
mod tests {
    use crate::CHUNK_TILES;

    use super::*;

    #[test]
    fn empty_space_not_occupied() {
        let world = World::new();
        let position = GridCoords::new(2, 2);
        let size = Vec2i::new(2, 2);
        let is_occupied = world.is_space_occupied(position, size);
        assert!(!is_occupied);
    }

    #[test]
    fn empty_space_between_chunks_not_occupied() {
        let world = World::new();
        let position = GridCoords::new(CHUNK_TILES - 1, CHUNK_TILES - 1);
        let size = Vec2i::new(2, 2);
        let is_occupied = world.is_space_occupied(position, size);
        assert!(!is_occupied);
    }

    #[test]
    fn place_miner_marks_space_occupied() {
        let mut world = World::new();
        let position = GridCoords::new(2, 2);
        let entity = world.place_miner(position);
        assert!(entity.is_some());
        let size = BuildingType::Miner.size();
        let is_occupied = world.is_space_occupied(position, size);
        assert!(is_occupied);
        for i in 0..size.x {
            for j in 0..size.y {
                let tile_to_check = GridCoords::new(position.x + i, position.y + j);
                let is_occupied = world.is_space_occupied(tile_to_check, Vec2i::new(1, 1));
                assert!(is_occupied)
            }
        }
    }

    #[test]
    fn place_miner_not_place_when_occupied() {
        let mut world = World::new();
        let position = GridCoords::new(2, 2);
        world.place_miner(position);
        let new_position = GridCoords::new(3, 3);
        let entity = world.place_miner(new_position);
        assert!(entity.is_none())
    }
}
