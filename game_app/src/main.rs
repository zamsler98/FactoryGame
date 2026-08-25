//! game_app: Macroquad application glue.
//! - captures platform input
//! - performs rendering using Macroquad APIs
//!
//! Only this crate depends on `macroquad`.
//!

use macroquad::prelude::*;

mod chunk_renderer;
mod entity_renderers;
mod macroquad_logger;

use chunk_renderer::ChunkRenderer;

#[macroquad::main("FactoryGame - Macroquad")]
async fn main() {
    macroquad_logger::init_logging();
    let chunk_renderer = ChunkRenderer::load().await;

    let mut chunk = game_logic::Chunk::new();
    // chunk.add_entity(game_logic::EntityInfo {
    //     entity_type: game_logic::EntityType::Miner,
    //     position: game_logic::Position { x: 10, y: 10 },
    // });
    // chunk.add_entity(game_logic::EntityInfo {
    //     entity_type: game_logic::EntityType::Miner,
    //     position: game_logic::Position { x: 50, y: 50 },
    // });

    loop {
        clear_background(BLACK);
        chunk_renderer.render(&chunk);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            log::debug!("Mouse clicked at ({x}, {y})");
        }
        next_frame().await;
    }
}
