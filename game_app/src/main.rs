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

    // Camera & zoom state for panning/zooming
    let mut camera = Camera2D {
        target: vec2(0.0, 0.0),
        zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0),
        ..Default::default()
    };
    let mut zoom: f32 = 1.0;

    // Touch tap detection state (for mobile taps -> action)
    let mut prev_touches: HashMap<u64, Vec2> = HashMap::new();
    let mut touch_start: HashMap<u64, Vec2> = HashMap::new();
    const TAP_MAX_MOVEMENT: f32 = 10.0;

    // Previous mouse pos for mouse-drag panning
    let mut prev_mouse: Option<Vec2> = None;

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

        // -----------------------------
        // Panning: single-finger drag and mouse drag
        // -----------------------------
        if touches_now.len() == 1 {
            let t = &touches_now[0];
            let pos = t.position;
            if let Some(last) = prev_touches.get(&t.id) {
                let delta = pos - *last;
                // note: screen Y is flipped for world coords
                camera.target -= vec2(delta.x, -delta.y) / zoom;
            }
        }

        if touches_now.len() == 2 {
            // optional: could implement pinch-to-zoom here later
        }

        // Mouse-drag panning (desktop)
        if touches_now.is_empty() {
            let mouse_pos = vec2(mouse_position().0, mouse_position().1);
            if is_mouse_button_down(MouseButton::Left) {
                if let Some(prev) = prev_mouse {
                    let delta = mouse_pos - prev;
                    camera.target -= vec2(delta.x, -delta.y) / zoom;
                }
                prev_mouse = Some(mouse_pos);
            } else {
                prev_mouse = None;
            }
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
        // We'll render a large grid snapshot (1000x1000 for this feature) and then other HUD on top.
        let grid_snapshot = game_logic::placement::TileGridSnapshot {
            width: 1000,
            height: 1000,
            instances: Vec::new(),
        };

        // Compute visible tile range by converting screen corners to world coords
        fn screen_to_world_vec2(screen_pos: Vec2, camera: &Camera2D, zoom: f32) -> Vec2 {
            let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
            let relative = screen_pos - screen_center;
            let world_x = camera.target.x + relative.x / zoom;
            let world_y = camera.target.y - relative.y / zoom;
            vec2(world_x, world_y)
        }

        // Determine hovered tile from pointer by converting to world coords
        let hover_tile = if let Some((sx, sy)) = input.pointer {
            let screen = vec2(sx, sy);
            let world = screen_to_world_vec2(screen, &camera, zoom);
            let tx = (world.x / crate::render_grid::TILE_PX).floor() as i32;
            let ty = (world.y / crate::render_grid::TILE_PX).floor() as i32;
            Some(game_core::TilePos { x: tx, y: ty })
        } else {
            None
        };

        // compute visible bounds in tile coordinates
        let top_left_world = screen_to_world_vec2(vec2(0.0, 0.0), &camera, zoom);
        let bottom_right_world =
            screen_to_world_vec2(vec2(screen_width(), screen_height()), &camera, zoom);
        let min_x = (top_left_world.x / crate::render_grid::TILE_PX).floor() as i32;
        let min_y = (top_left_world.y / crate::render_grid::TILE_PX).floor() as i32;
        let max_x = (bottom_right_world.x / crate::render_grid::TILE_PX).floor() as i32;
        let max_y = (bottom_right_world.y / crate::render_grid::TILE_PX).floor() as i32;

        // Apply camera and draw world-space grid
        set_camera(&camera);
        crate::render_grid::draw_grid(&grid_snapshot, hover_tile, min_x, max_x, min_y, max_y);
        set_default_camera();

        // Update camera zoom into camera struct (so world drawing respects zoom if changed later)
        camera.zoom = vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0);

        // HUD: draw simple pointer marker (screen-space)
        if let Some((px, py)) = input.pointer {
            draw_circle(px, py, 6.0, Color::new(1.0, 1.0, 0.0, 1.0));
        }

        // Simple text showing instructions (no mobile joystick)
        draw_text(
            "Click or tap to select tiles | 1-finger: pan",
            20.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await
    }
}
