use macroquad::prelude::*;
use std::collections::HashMap;

const TILE_SIZE: f32 = 32.0;
const GRID_WIDTH: i32 = 100;
const GRID_HEIGHT: i32 = 100;

#[macroquad::main("FactoryGame")]
async fn main() {
    let mut camera = Camera2D {
        target: vec2(0.0, 0.0),
        zoom: vec2(1.0 / screen_width() * 2.0, -1.0 / screen_height() * 2.0),
        ..Default::default()
    };

    let mut zoom: f32 = 1.0;

    // Track previous touch positions (from the previous frame)
    let mut prev_touches: HashMap<u64, Vec2> = HashMap::new();
    // Track touch start positions to detect taps
    let mut touch_start: HashMap<u64, Vec2> = HashMap::new();
    // Currently selected tile (single selection)
    let mut selected_tile: Option<(i32, i32)> = None;

    // Threshold in pixels to consider a touch a tap vs a drag
    const TAP_MAX_MOVEMENT: f32 = 10.0;

    loop {
        let touches = touches();

        // -----------------------------
        // Touch controls (mobile)
        // -----------------------------
        if touches.len() == 1 {
            let touch = &touches[0];
            let pos = touch.position;

            // Record touch start if this is the first frame for this touch id
            touch_start.entry(touch.id).or_insert(pos);

            if let Some(last_pos) = prev_touches.get(&touch.id) {
                let delta = pos - *last_pos;
                camera.target -= vec2(delta.x, -delta.y) / zoom;
            }

            // When there's one touch, we won't consider pinch scaling
            // Keep last pinch distance state out of this simplified example
        }

        if touches.len() == 2 {
            let p1 = touches[0].position;
            let p2 = touches[1].position;

            let current_distance = p1.distance(p2);

            // A simple pinch implementation based on distance between two touches
            // We don't persist pinch distance across frames here; instead derive zoom from positions
            // To make it smoother you'd store last pinch distance similar to prev_touches
            // For now, compute a relative zoom factor from the previous frame's positions if available
            if let (Some(prev_p1), Some(prev_p2)) = (
                prev_touches.get(&touches[0].id),
                prev_touches.get(&touches[1].id),
            ) {
                let last_distance = prev_p1.distance(*prev_p2);
                if last_distance > 0.0 {
                    let zoom_delta = current_distance / last_distance;
                    zoom *= zoom_delta;
                    zoom = zoom.clamp(0.2, 5.0);

                    camera.zoom = vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0);
                }
            }

            // Record starts for both touches if not present
            for touch in &touches {
                touch_start.entry(touch.id).or_insert(touch.position);
            }
        }

        // -----------------------------
        // Mouse fallback (desktop)
        // -----------------------------
        if touches.is_empty() {
            let mouse_pos = mouse_position();

            // Pan while holding left mouse button
            if is_mouse_button_down(MouseButton::Left) {
                let delta = vec2(mouse_pos.0, mouse_pos.1)
                    - vec2(screen_width() / 2.0, screen_height() / 2.0);
                camera.target -= delta / zoom * 0.01;
            }

            // Mouse click selects a tile (pressed, not held)
            if is_mouse_button_pressed(MouseButton::Left) {
                let screen = vec2(mouse_pos.0, mouse_pos.1);
                let world = screen_to_world(screen, &camera, zoom);
                if let Some((tx, ty)) = world_to_tile(world) {
                    selected_tile = Some((tx, ty));
                }
            }

            let (_mx, scroll) = mouse_wheel();
            if scroll != 0.0 {
                zoom *= 1.1_f32.powf(scroll);
                zoom = zoom.clamp(0.2, 5.0);

                camera.zoom = vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0);
            }
        }

        // Detect ended touches and treat small-movement ends as taps
        {
            // build a set of current touch ids
            let mut current_ids: HashMap<u64, ()> = HashMap::new();
            for t in &touches {
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
                        let world = screen_to_world(end_pos, &camera, zoom);
                        if let Some((tx, ty)) = world_to_tile(world) {
                            selected_tile = Some((tx, ty));
                        }
                    }
                }

                // cleanup
                touch_start.remove(&id);
                prev_touches.remove(&id);
            }
        }

        // Update prev_touches to current touches for the next frame
        prev_touches.clear();
        for touch in &touches {
            prev_touches.insert(touch.id, touch.position);
        }

        // -----------------------------
        // Draw
        // -----------------------------
        set_camera(&camera);
        clear_background(DARKGRAY);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                let color = if selected_tile == Some((x, y)) {
                    YELLOW
                } else {
                    GRAY
                };

                draw_rectangle(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    TILE_SIZE - 1.0,
                    TILE_SIZE - 1.0,
                    color,
                );
            }
        }

        set_default_camera();
        draw_text(
            "1 finger drag: pan | 2 finger pinch: zoom",
            20.0,
            30.0,
            28.0,
            WHITE,
        );

        next_frame().await;
    }
}

fn screen_to_world(screen_pos: Vec2, camera: &Camera2D, zoom: f32) -> Vec2 {
    let screen_center = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let relative = screen_pos - screen_center;

    let world_x = camera.target.x + relative.x / zoom;
    let world_y = camera.target.y - relative.y / zoom;

    vec2(world_x, world_y)
}

fn world_to_tile(world_pos: Vec2) -> Option<(i32, i32)> {
    if world_pos.x < 0.0 || world_pos.y < 0.0 {
        return None;
    }

    let tx = (world_pos.x / TILE_SIZE).floor() as i32;
    let ty = (world_pos.y / TILE_SIZE).floor() as i32;

    if tx < 0 || ty < 0 || tx >= GRID_WIDTH || ty >= GRID_HEIGHT {
        return None;
    }

    Some((tx, ty))
}
