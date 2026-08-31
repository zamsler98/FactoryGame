//! game_app: Macroquad application glue.
//! - captures platform input
//! - performs rendering using Macroquad APIs
//!
//! Only this crate depends on `macroquad`.
//!

use game_logic::{Chunk, ChunkCoords};
use macroquad::prelude::*;

mod chunk_renderer;
mod entity_renderers;
mod macroquad_logger;

use chunk_renderer::ChunkRenderer;

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
    let mut camera_position_x = 0.0;
    let mut camera_position_y = 0.0;

    loop {
        clear_background(BLACK);
        const VIEW_H: f32 = 600.0;
        let view_w = VIEW_H * screen_width() / screen_height();

        // Y-down camera: macroquad's `matrix()` negates `zoom.y` when rendering to the
        // screen, so a positive `zoom.y` here is what makes +Y point down. Using
        // `Camera2D::from_display_rect` instead would double-negate and flip text.
        set_camera(&Camera2D {
            target: vec2(
                camera_position_x + view_w / 2.0,
                camera_position_y + VIEW_H / 2.0,
            ),
            zoom: vec2(2.0 / view_w, 2.0 / VIEW_H),
            ..Default::default()
        });
        for chunk in &chunks {
            chunk_renderer.render(chunk);
        }

        let bot_right_x = camera_position_x + view_w;
        let bot_right_y = camera_position_y + VIEW_H;
        // log::debug!(
        //     "Current camera position: ({camera_position_x},{camera_position_y}),({bot_right_x}, {bot_right_y})"
        // );

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
                let diff_x = prev_x - x;
                let diff_y = prev_y - y;

                camera_position_x += diff_x;
                camera_position_y += diff_y;
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
