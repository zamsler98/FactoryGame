//! game_app: Macroquad application glue.
//! - captures platform input
//! - performs rendering using Macroquad APIs
//!
//! Only this crate depends on `macroquad`.
//!

use game_logic::{Chunk, ChunkCoords};
use macroquad::prelude::*;

mod chunk_renderer;
// Unwired until chunks can resolve their entity handles into drawable EntityInfo.
// mod entity_renderers;
mod macroquad_logger;
mod main_camera;

use chunk_renderer::ChunkRenderer;
use main_camera::MainCamera;

#[macroquad::main("FactoryGame - Macroquad")]
async fn main() {
    macroquad_logger::init_logging();
    let chunk_renderer = ChunkRenderer::load().await;

    let chunks = [
        Chunk::new(ChunkCoords::new(0, 0)),
        Chunk::new(ChunkCoords::new(1, 0)),
        Chunk::new(ChunkCoords::new(0, 1)),
        Chunk::new(ChunkCoords::new(1, 1)),
    ];

    let mut prev_mouse_position_x: Option<f32> = None;
    let mut prev_mouse_position_y: Option<f32> = None;
    let mut main_camera = MainCamera::default();

    loop {
        clear_background(BLACK);

        main_camera.use_camera();
        for chunk in &chunks {
            chunk_renderer.render(chunk);
        }

        // let w = 15.0;
        // for i in 0..8 {
        //     for j in 0..8 {
        //         draw_rectangle_lines(i as f32 * w, j as f32 * w, w, w, 1.0, WHITE);
        //     }
        // }
        //

        // if is_mouse_button_pressed(MouseButton::Left) {
        //     let (x, y) = mouse_position();
        //
        //     let x = x - (4.0 * w);
        //     let y = y - (4.0 * w);
        //
        //     let grid_x = (x.div_euclid(w)) as i32;
        //     let grid_y = (y.div_euclid(w)) as i32;
        //
        //     let chunk_x = grid_x.div_euclid(2);
        //     let chunk_y = grid_y.div_euclid(2);
        //     // log::debug!("Mouse clicked at ({x}, {y})");
        //     // log::debug!("Grid location ({grid_x}, {grid_y})");
        //     // log::debug!("Chunk coord ({chunk_x}, {chunk_y})");
        // }

        if is_mouse_button_down(MouseButton::Left) {
            let (x, y) = mouse_position();
            if let Some(prev_x) = prev_mouse_position_x
                && let Some(prev_y) = prev_mouse_position_y
            {
                main_camera.pan(vec2(prev_x - x, prev_y - y));
            }
            prev_mouse_position_x = Some(x);
            prev_mouse_position_y = Some(y);
        } else {
            prev_mouse_position_x = None;
            prev_mouse_position_y = None;
        }

        // draw_rectangle_lines(100.0, 100.0, 32.0, 32.0, 2.0, WHITE);
        next_frame().await;
    }
}
