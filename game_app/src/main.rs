//! game_app: Macroquad application glue.
//! - captures platform input and fills `InputFrame`
//! - calls `game_logic::update_world`
//! - performs rendering using Macroquad APIs
//!
//! Only this crate depends on `macroquad`.
//!

use game_logic::{update_world, InputFrame};
use macroquad::prelude::*;

mod render_grid;

#[macroquad::main("FactoryGame - Macroquad")]
async fn main() {
    // Create and populate the game world (game_core)
    let mut world = game_core::World::new();
    world.spawn_player(200.0, 200.0);
    world.spawn_enemy(500.0, 200.0);
    world.spawn_enemy(500.0, 400.0);

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

        // --- Mobile-friendly touch controls (virtual joystick + action button) ---
        let screen_w = screen_width();
        let screen_h = screen_height();
        let joystick_base = vec2(screen_w * 0.2, screen_h * 0.75);
        let joystick_radius = 50.0;
        let action_center = vec2(screen_w * 0.85, screen_h * 0.75);
        let action_radius = 44.0;

        let mut touch_pointer: Option<Vec2> = None;
        let mut joystick_vec = vec2(0.0, 0.0);
        let mut action_pressed = false;

        for t in touches() {
            let tp = t.position;
            touch_pointer = Some(tp);

            // If the touch is on the left half, treat it as joystick input
            if tp.x < screen_w * 0.5 {
                let delta = tp - joystick_base;
                let dist = delta.length();
                let clamped = if dist > joystick_radius {
                    delta.normalize() * joystick_radius
                } else {
                    delta
                };
                joystick_vec = clamped / joystick_radius; // normalized [-1..1]
            }

            // If the touch is on the right half near the action button, mark action
            if (tp - action_center).length() <= action_radius * 1.5 {
                action_pressed = true;
            }
        }

        // If we have joystick input from touch, override keyboard movement
        if joystick_vec.length() > 0.001 {
            input.move_x = joystick_vec.x;
            input.move_y = joystick_vec.y;
        }

        // Prefer touch action if present
        if action_pressed {
            input.action = true;
        }

        // Pointer / touch: prefer first touch if present, otherwise mouse.
        if let Some(tp) = touch_pointer {
            input.pointer = Some((tp.x, tp.y));
        } else if is_mouse_button_down(MouseButton::Left) {
            input.pointer = Some(mouse_position());
        }

        // Update game state using platform-agnostic logic
        update_world(&mut world, &input, dt);

            // --- Rendering (platform-specific) ---
        // We'll render the grid (top-left aligned) and then other HUD on top.
        let grid_snapshot = {
            // For now create a tiny empty snapshot; eventually this will come from game_logic's grid.
            // We'll construct a snapshot sized 128x128 with no instances when none present.
            let mut snap = game_logic::placement::TileGridSnapshot { width: 128, height: 128, instances: Vec::new() };
            // TODO: integrate real grid snapshot from world when world contains one.
            snap
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

        // Draw mobile HUD: virtual joystick and action button
        // (only visible if touch is available or on mobile)
        let base_color = Color::new(1.0, 1.0, 1.0, 0.08);
        let knob_color = Color::new(1.0, 1.0, 1.0, 0.18);
        draw_circle(
            joystick_base.x,
            joystick_base.y,
            joystick_radius + 8.0,
            base_color,
        );
        let knob_pos = joystick_base + joystick_vec * joystick_radius;
        draw_circle(knob_pos.x, knob_pos.y, 28.0, knob_color);

        // Action button
        let action_color = if action_pressed {
            Color::new(1.0, 0.4, 0.2, 0.9)
        } else {
            Color::new(1.0, 1.0, 1.0, 0.12)
        };
        draw_circle(
            action_center.x,
            action_center.y,
            action_radius,
            action_color,
        );
        draw_text(
            "A",
            action_center.x - 6.0,
            action_center.y + 8.0,
            26.0,
            WHITE,
        );

        // HUD: draw simple pointer marker
        if let Some((px, py)) = input.pointer {
            draw_circle(px, py, 6.0, Color::new(1.0, 1.0, 0.0, 1.0));
        }

        // Simple text showing instructions
        draw_text(
            "WASD / Arrow keys to move, Space / Click to action",
            20.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
