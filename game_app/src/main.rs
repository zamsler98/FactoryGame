//! game_app: Macroquad application glue.
//! - captures platform input
//! - performs rendering using Macroquad APIs
//!
//! Only this crate depends on `macroquad`.
//!

use game_core::Vec2f;
use game_logic::{CHUNK_TILES, Chunk, ChunkCoords, TILE_SIZE, World};
use macroquad::prelude::*;

mod chunk_renderer;
// Unwired until chunks can resolve their entity handles into drawable EntityInfo.
// mod entity_renderers;
mod macroquad_logger;
mod main_camera;
mod math_interop;

use chunk_renderer::ChunkRenderer;
use main_camera::MainCamera;

#[macroquad::main("FactoryGame - Macroquad")]
async fn main() {
    macroquad_logger::init_logging();
    let chunk_renderer = ChunkRenderer::load().await;

    let mut prev_mouse_position: Option<Vec2f> = None;
    let mut main_camera = MainCamera::default();
    let mut world = World::new();

    loop {
        clear_background(BLACK);

        main_camera.use_camera();
        let chunks = world.chunks_overlapping_rect_or_create(main_camera.bounds());
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
            let mouse = Vec2f::new(x, y);
            // Drag the world under the cursor: the camera moves opposite the mouse.
            if let Some(prev) = prev_mouse_position {
                main_camera.pan(prev - mouse);
            }
            prev_mouse_position = Some(mouse);
        } else {
            prev_mouse_position = None;
        }

        let bounds = main_camera.bounds();

        let x = bounds.position.x;
        let y = bounds.position.y;
        let x_chunk = x / (CHUNK_TILES * TILE_SIZE) as f32;
        let y_chunk = y / (CHUNK_TILES * TILE_SIZE) as f32;
        log::debug!("Camera bounds ({x},{y}). Chunk coords: ({x_chunk}, {y_chunk})");

        next_frame().await;
    }
}
