//! game_logic: processes inputs, game rules, and building placement.
//! Depends on `game_core` only. It exposes an `InputFrame`.

mod world;

pub use game_core::{Vec2, Vec2f, Vec2i};
pub use world::{
    CHUNK_SIZE, CHUNK_TILES, Chunk, ChunkCoords, EntityInfo, EntityType, Position, TILE_SIZE, World,
};

/// `InputFrame` is the platform-agnostic input snapshot.
/// The platform layer (`game_app`) fills this each frame and passes to logic.
#[derive(Clone, Debug, Default)]
pub struct InputFrame {
    /// Whether the primary action (e.g., tap/click) was pressed this frame
    pub action: bool,
    /// Optional pointer/touch position in world / screen coords
    pub pointer: Option<Vec2f>,
}
