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
    let zoom: f32 = 1.0;

    // Touch tap detection state (for mobile taps -> action)
    let mut prev_touches: HashMap<u64, Vec2> = HashMap::new();
    let mut touch_start: HashMap<u64, Vec2> = HashMap::new();
    const TAP_MAX_MOVEMENT: f32 = 10.0;

    // Previous mouse pos for mouse-drag panning
    let mut prev_mouse: Option<Vec2> = None;

    // Selected building and rotation state (persist across frames)
    let mut selected_spec_id: u32 = 1; // 1=conveyor,2=miner,3=smelter
    let mut selected_rotation: game_core::Rotation = game_core::Rotation::R0;

    let mut last_rotate_time: f64 = 0.0;
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

        // Mobile touch: collect touches for pointer and tap detection (no joystick)
        let mut touch_pointer: Option<Vec2> = None;
        let touches_now = touches();

        // Action (space / mouse click). Ignore mouse events when touch input is present
        input.action = is_key_pressed(KeyCode::Space)
            || (touches_now.is_empty() && is_mouse_button_pressed(MouseButton::Left));

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

        // Determine hovered tile from pointer by converting to world coords
        fn screen_to_world_vec2(screen_pos: Vec2, camera: &Camera2D, zoom: f32) -> Vec2 {
            let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
            let relative = screen_pos - screen_center;
            let world_x = camera.target.x + relative.x / zoom;
            let world_y = camera.target.y - relative.y / zoom;
            vec2(world_x, world_y)
        }

        let hover_tile = if let Some((sx, sy)) = input.pointer {
            let screen = vec2(sx, sy);
            let world = screen_to_world_vec2(screen, &camera, zoom);
            let tx = (world.x / crate::render_grid::TILE_PX).floor() as i32;
            let ty = (world.y / crate::render_grid::TILE_PX).floor() as i32;
            Some(game_core::TilePos { x: tx, y: ty })
        } else {
            None
        };

        // Update game state using platform-agnostic logic
        update_world(&mut world, &input, dt);

        // Handle HUD input and placement (screen-space)
        // Toolbar geometry
        let hud_margin = 16.0;
        let btn_size = 56.0;
        let spacing = 12.0;
        let screen_h = screen_height();
        let base_y = screen_h - hud_margin - btn_size;
        let mut hud_consumed = false;

        let btn1_x = hud_margin;
        let btn2_x = btn1_x + btn_size + spacing;
        let btn3_x = btn2_x + btn_size + spacing;
        let rotate_x = btn3_x + btn_size + spacing;

        // Handle desktop mouse clicks as edge-detected events so HUD clicks don't double-fire.
        // Ignore mouse clicks if touch input is present to avoid synthetic mouse events on touch devices.
        let mouse_clicked = touches_now.is_empty() && is_mouse_button_released(MouseButton::Left);
        if mouse_clicked {
            let (px, py) = mouse_position();
            // check HUD buttons first (point-in-rect)
            let in_btn1 =
                px >= btn1_x && px <= btn1_x + btn_size && py >= base_y && py <= base_y + btn_size;
            let in_btn2 =
                px >= btn2_x && px <= btn2_x + btn_size && py >= base_y && py <= base_y + btn_size;
            let in_btn3 =
                px >= btn3_x && px <= btn3_x + btn_size && py >= base_y && py <= base_y + btn_size;
            let in_rotate = px >= rotate_x
                && px <= rotate_x + btn_size
                && py >= base_y
                && py <= base_y + btn_size;

            if in_btn1 {
                selected_spec_id = 1;
                hud_consumed = true;
            } else if in_btn2 {
                selected_spec_id = 2;
                hud_consumed = true;
            } else if in_btn3 {
                selected_spec_id = 3;
                hud_consumed = true;
            } else if in_rotate {
                // cycle rotation (single step) with debounce to avoid rapid skips
                const ROTATE_DEBOUNCE: f64 = 0.15;
                let current_time = get_time();
                if (current_time - last_rotate_time) > ROTATE_DEBOUNCE {
                    selected_rotation = match selected_rotation {
                        game_core::Rotation::R0 => game_core::Rotation::R90,
                        game_core::Rotation::R90 => game_core::Rotation::R180,
                        game_core::Rotation::R180 => game_core::Rotation::R270,
                        game_core::Rotation::R270 => game_core::Rotation::R0,
                    };
                    last_rotate_time = current_time;
                }
                hud_consumed = true;
            } else {
                // Not HUD: treat as a placement click (set input pointer/action so placement code runs below)
                input.action = true;
                input.pointer = Some((px, py));
            }
        }

        // Also handle touch taps or other input.action sources (keyboard, gamepad). If input.action is set
        // (e.g., from touch tap detection earlier), handle HUD/placement similarly. This preserves touch behavior.
        if input.action && !hud_consumed {
            if let Some((px, py)) = input.pointer {
                // check HUD buttons first (point-in-rect)
                let in_btn1 = px >= btn1_x
                    && px <= btn1_x + btn_size
                    && py >= base_y
                    && py <= base_y + btn_size;
                let in_btn2 = px >= btn2_x
                    && px <= btn2_x + btn_size
                    && py >= base_y
                    && py <= base_y + btn_size;
                let in_btn3 = px >= btn3_x
                    && px <= btn3_x + btn_size
                    && py >= base_y
                    && py <= base_y + btn_size;
                let in_rotate = px >= rotate_x
                    && px <= rotate_x + btn_size
                    && py >= base_y
                    && py <= base_y + btn_size;

                if in_btn1 {
                    selected_spec_id = 1;
                    hud_consumed = true;
                } else if in_btn2 {
                    selected_spec_id = 2;
                    hud_consumed = true;
                } else if in_btn3 {
                    selected_spec_id = 3;
                    hud_consumed = true;
                } else if in_rotate {
                    // cycle rotation (single step) with debounce to avoid rapid skips
                    const ROTATE_DEBOUNCE: f64 = 0.15;
                    let current_time = get_time();
                    if (current_time - last_rotate_time) > ROTATE_DEBOUNCE {
                        selected_rotation = match selected_rotation {
                            game_core::Rotation::R0 => game_core::Rotation::R90,
                            game_core::Rotation::R90 => game_core::Rotation::R180,
                            game_core::Rotation::R180 => game_core::Rotation::R270,
                            game_core::Rotation::R270 => game_core::Rotation::R0,
                        };
                        last_rotate_time = current_time;
                    }
                    hud_consumed = true;
                }

                // If not consumed by HUD, attempt placement on grid tile
                if !hud_consumed {
                    if let Some(tile) = hover_tile {
                        let spec = game_core::BuildingSpec {
                            spec_id: selected_spec_id,
                            size: game_core::Size2 { w: 1, h: 1 },
                        };
                        let _ = game_logic::placement::try_place_building(
                            &mut world.tile_grid,
                            &spec,
                            tile,
                            selected_rotation,
                        );
                    }
                }
            }
        }

        // --- Rendering (platform-specific) ---
        // Use real grid snapshot from the world's tile grid
        let grid_snapshot = game_logic::placement::grid_snapshot(&world.tile_grid);

        // compute visible bounds in tile coordinates (handle inverted Y from camera)
        let top_left_world = screen_to_world_vec2(vec2(0.0, 0.0), &camera, zoom);
        let bottom_right_world =
            screen_to_world_vec2(vec2(screen_width(), screen_height()), &camera, zoom);
        let world_min_x = top_left_world.x.min(bottom_right_world.x);
        let world_max_x = top_left_world.x.max(bottom_right_world.x);
        let world_min_y = top_left_world.y.min(bottom_right_world.y);
        let world_max_y = top_left_world.y.max(bottom_right_world.y);
        let min_x = (world_min_x / crate::render_grid::TILE_PX).floor() as i32;
        let min_y = (world_min_y / crate::render_grid::TILE_PX).floor() as i32;
        let max_x = (world_max_x / crate::render_grid::TILE_PX).floor() as i32;
        let max_y = (world_max_y / crate::render_grid::TILE_PX).floor() as i32;

        // Apply camera and draw world-space grid
        set_camera(&camera);
        crate::render_grid::draw_grid(&grid_snapshot, hover_tile, min_x, max_x, min_y, max_y);
        set_default_camera();

        // Update camera zoom into camera struct (so world drawing respects zoom if changed later)
        camera.zoom = vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0);

        // --- HUD draw (screen-space)
        // Draw buttons
        let conveyor_color = Color::new(1.0, 1.0, 1.0, 0.95);
        draw_rectangle(btn1_x, base_y, btn_size, btn_size, conveyor_color);
        let miner_color = Color::new(0.9, 0.6, 0.3, 0.95);
        draw_rectangle(btn2_x, base_y, btn_size, btn_size, miner_color);
        let smelter_color = Color::new(0.3, 0.8, 0.4, 0.95);
        draw_rectangle(btn3_x, base_y, btn_size, btn_size, smelter_color);
        let rotate_color = Color::new(0.2, 0.2, 0.2, 0.95);
        draw_rectangle(rotate_x, base_y, btn_size, btn_size, rotate_color);

        // highlight selected building
        let (sel_x, sel_y) = match selected_spec_id {
            1 => (btn1_x, base_y),
            2 => (btn2_x, base_y),
            3 => (btn3_x, base_y),
            _ => (btn1_x, base_y),
        };
        draw_rectangle_lines(
            sel_x,
            sel_y,
            btn_size,
            btn_size,
            4.0,
            Color::new(1.0, 1.0, 0.0, 0.95),
        );

        // draw rotation label on rotate button
        let rot_label = match selected_rotation {
            game_core::Rotation::R0 => "0°",
            game_core::Rotation::R90 => "90°",
            game_core::Rotation::R180 => "180°",
            game_core::Rotation::R270 => "270°",
        };
        draw_text(
            rot_label,
            rotate_x + 8.0,
            base_y + btn_size / 2.0 + 6.0,
            20.0,
            WHITE,
        );

        // Draw selected building name above the toolbar
        let sel_name = match selected_spec_id {
            1 => "Conveyor",
            2 => "Miner",
            3 => "Smelter",
            _ => "Unknown",
        };
        draw_text(sel_name, hud_margin, base_y - 8.0, 20.0, WHITE);

        // HUD: draw simple pointer marker (screen-space)
        if let Some((px, py)) = input.pointer {
            draw_circle(px, py, 6.0, Color::new(1.0, 1.0, 0.0, 1.0));
        }

        // Simple text showing instructions (no mobile joystick)
        draw_text(
            "Tap button to select building, tap grid to place | 1-finger: pan",
            20.0,
            20.0,
            20.0,
            WHITE,
        );

        next_frame().await;
    }
}
