mod chunk;
mod entity_info;
#[allow(clippy::module_inception)]
mod world;

pub use chunk::{CHUNK_SIZE, CHUNK_TILES, Chunk, ChunkCoords, TILE_SIZE};
pub use entity_info::{EntityInfo, EntityType, Position};
pub use world::World;
