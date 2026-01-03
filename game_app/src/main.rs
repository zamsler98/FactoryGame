//! game_app: Macroquad application glue.
//! - captures platform input and fills `InputFrame`
//! - calls `game_logic::update_world`
//! - performs rendering using Macroquad APIs
//!
//! Only this crate depends on `macroquad`.
//!

use game_core::EntityType;
use game_logic::{update_world, InputFrame};
use macroquad::prelude::*;

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

        // Keyboard / gamepad movement mapping (WASD / arrow keys)
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

        // Normalize diagonal movement
        let mag = (input.move_x * input.move_x + input.move_y * input.move_y).sqrt();
        if mag > 1.0 {
            input.move_x /= mag;
            input.move_y /= mag;
        }

        // Action (space / touch)
        input.action = is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left);

        // Pointer / touch: prefer first touch if present, otherwise mouse.
        if let Some(touch) = touches().first() {
            // touches() yields Touch objects; `position` is a tuple field
            input.pointer = Some((touch.position.x, touch.position.y));
        } else if is_mouse_button_down(MouseButton::Left) {
            input.pointer = Some(mouse_position());
        }

        // Update game state using platform-agnostic logic
        update_world(&mut world, &input, dt);

        // --- Rendering (platform-specific) ---
        clear_background(Color::from_rgba(20, 20, 20, 255));

        for e in &world.entities {
            let (r, g, b) = match e.ty {
                EntityType::Player => (50.0 / 255.0, 120.0 / 255.0, 220.0 / 255.0),
                EntityType::Enemy => (220.0 / 255.0, 60.0 / 255.0, 60.0 / 255.0),
            };
            draw_circle(
                e.transform.x,
                e.transform.y,
                e.radius,
                Color::new(r, g, b, 1.0),
            );
        }

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
