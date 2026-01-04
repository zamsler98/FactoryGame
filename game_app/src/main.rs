//! game_app: Macroquad application glue.
//! - captures platform input and fills `InputFrame`
//! - calls `game_logic::update_world`
//! - performs rendering using Macroquad APIs
//!
//! Only this crate depends on `macroquad`.
//!

use game_logic::{update_world, InputFrame};
use macroquad::prelude::*;
use std::collections::HashMap;

mod render_grid;

#[macroquad::main("FactoryGame - Macroquad")]
async fn main() {
    // Create and populate the game world (game_core)
    let mut world = game_core::World::new();
    world.spawn_player(200.0, 200.0);
    world.spawn_enemy(500.0, 200.0);
    world.spawn_enemy(500.0, 400.0);

    // Touch tap detection state (for mobile taps -> action)
    let mut prev_touches: HashMap<u64, Vec2> = HashMap::new();
    let mut touch_start: HashMap<u64, Vec2> = HashMap::new();
    const TAP_MAX_MOVEMENT: f32 = 10.0;

    loop {
        let dt = get_frame_time();

        // Build a platform-agnostic InputFrame from Macroquad input APIs.
        let mut input = InputFrame::default();

        // --- Desktop keyboard/gamepad movement mapping (WASD / arrow keys) ---
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            input.move_x -= 1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            input.move_x += 1.0;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            input.move_y -= 1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            input.move_y += 1.0;
        }

        // Normalize diagonal movement for keyboard
        let mag = (input.move_x * input.move_x + input.move_y * input.move_y).sqrt();
        if mag > 1.0 {
            input.move_x /= mag;
            input.move_y /= mag;
        }

        // Action (space / mouse click)
        input.action = is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left);

        // Mobile touch: collect touches for pointer and tap detection (no joystick)
        let mut touch_pointer: Option<Vec2> = None;
        let touches_now = touches();

        // Record current touches, set pointer to first touch
        for t in &touches_now {
            let tp = t.position;
            touch_pointer = Some(tp);
            touch_start.entry(t.id).or_insert(tp);
        }

        // Detect ended touches as taps (small movement)
        {
            // build a set of current touch ids
            let mut current_ids: HashMap<u64, ()> = HashMap::new();
            for t in &touches_now {
                current_ids.insert(t.id, ());
            }

            // Collect ended ids from touch_start where id is not in current_ids
            let ended_ids: Vec<u64> = touch_start
                .keys()
                .filter(|id| !current_ids.contains_key(id))
                .copied()
                .collect();

            for id in ended_ids {
                if let Some(start_pos) = touch_start.get(&id) {
                    // end_pos is the last known position from prev_touches if available
                    let end_pos = prev_touches.get(&id).cloned().unwrap_or(*start_pos);

                    if start_pos.distance(end_pos) < TAP_MAX_MOVEMENT {
                        // treat as tap
                        input.action = true;
                        // Also set pointer to tap end for selection
                        input.pointer = Some((end_pos.x, end_pos.y));
                    }
                }

                // cleanup
                touch_start.remove(&id);
                prev_touches.remove(&id);
            }
        }

        // Update prev_touches to current touches for the next frame
        prev_touches.clear();
        for t in &touches_now {
            prev_touches.insert(t.id, t.position);
        }

        // Pointer / touch: prefer first touch if present, otherwise mouse.
        if input.pointer.is_none() {
            if let Some(tp) = touch_pointer {
                input.pointer = Some((tp.x, tp.y));
            } else if is_mouse_button_down(MouseButton::Left) {
                input.pointer = Some(mouse_position());
            }
        }

        // Update game state using platform-agnostic logic
        update_world(&mut world, &input, dt);

        // --- Rendering (platform-specific) ---
        // We'll render the grid (top-left aligned) and then other HUD on top.
        let grid_snapshot = game_logic::placement::TileGridSnapshot {
            width: 128,
            height: 128,
            instances: Vec::new(),
        };

        // Determine hovered tile from pointer
        let hover_tile = if let Some((sx, sy)) = input.pointer {
            let tx = (sx / crate::render_grid::TILE_PX).floor() as i32;
            let ty = (sy / crate::render_grid::TILE_PX).floor() as i32;
            Some(game_core::TilePos { x: tx, y: ty })
        } else {
            None
        };

        crate::render_grid::draw_grid(&grid_snapshot, hover_tile);

        // HUD: draw simple pointer marker
        if let Some((px, py)) = input.pointer {
            draw_circle(px, py, 6.0, Color::new(1.0, 1.0, 0.0, 1.0));
        }

        // Simple text showing instructions (no mobile joystick)
        draw_text("Click or tap to select tiles", 20.0, 20.0, 20.0, WHITE);

        next_frame().await
    }
}
