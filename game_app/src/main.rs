//! game_app: Macroquad application glue.
//! - captures platform input
//! - performs rendering using Macroquad APIs
//!
//! Only this crate depends on `macroquad`.
//!

use macroquad::prelude::*;

mod chunk_renderer;
mod entity_renderers;

use chunk_renderer::ChunkRenderer;

#[macroquad::main("FactoryGame - Macroquad")]
async fn main() {
    let chunk_renderer = ChunkRenderer::load().await;

    let mut chunk = game_logic::Chunk::new();
    chunk.add_entity(game_logic::EntityInfo {
        entity_type: game_logic::EntityType::Miner,
        position: game_logic::Position { x: 10, y: 10 },
    });
    chunk.add_entity(game_logic::EntityInfo {
        entity_type: game_logic::EntityType::Miner,
        position: game_logic::Position { x: 50, y: 50 },
    });

    loop {
        clear_background(BLACK);
        chunk_renderer.render(&chunk);
        next_frame().await;
    }
}
