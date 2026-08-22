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

    let mut chunk = game_core::Chunk::new();
    chunk.add_entity(game_core::Entity {
        entity_type: game_core::EntityType::Miner,
        position: game_core::Position { x: 10, y: 10 },
    });
    chunk.add_entity(game_core::Entity {
        entity_type: game_core::EntityType::Miner,
        position: game_core::Position { x: 50, y: 50 },
    });

    loop {
        clear_background(BLACK);
        chunk_renderer.render(&chunk);
        next_frame().await;
    }
}
