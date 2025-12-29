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

    // Track previous touch positions
    let mut last_touches: HashMap<u64, Vec2> = HashMap::new();
    let mut last_pinch_distance: Option<f32> = None;

    loop {
        let touches = touches();

        // -----------------------------
        // Touch controls (mobile)
        // -----------------------------
        if touches.len() == 1 {
            let touch = &touches[0];
            let pos = touch.position;

            if let Some(last_pos) = last_touches.get(&touch.id) {
                let delta = pos - *last_pos;
                camera.target -= vec2(delta.x, -delta.y) / zoom;
            }

            last_pinch_distance = None;
        }

        if touches.len() == 2 {
            let p1 = touches[0].position;
            let p2 = touches[1].position;

            let current_distance = p1.distance(p2);

            if let Some(last_distance) = last_pinch_distance {
                let zoom_delta = current_distance / last_distance;
                zoom *= zoom_delta;
                zoom = zoom.clamp(0.2, 5.0);

                camera.zoom = vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0);
            }

            last_pinch_distance = Some(current_distance);
        }

        // Update last touch positions
        last_touches.clear();
        for touch in &touches {
            last_touches.insert(touch.id, touch.position);
        }

        // -----------------------------
        // Mouse fallback (desktop)
        // -----------------------------
        if touches.is_empty() {
            let mouse_pos = mouse_position();

            if is_mouse_button_down(MouseButton::Left) {
                let delta = vec2(mouse_pos.0, mouse_pos.1)
                    - vec2(screen_width() / 2.0, screen_height() / 2.0);
                camera.target -= delta / zoom * 0.01;
            }

            let (_mx, scroll) = mouse_wheel();
            if scroll != 0.0 {
                zoom *= 1.1_f32.powf(scroll);
                zoom = zoom.clamp(0.2, 5.0);

                camera.zoom = vec2(zoom / screen_width() * 2.0, -zoom / screen_height() * 2.0);
            }
        }

        // -----------------------------
        // Draw
        // -----------------------------
        set_camera(&camera);
        clear_background(DARKGRAY);

        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                draw_rectangle(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    TILE_SIZE - 1.0,
                    TILE_SIZE - 1.0,
                    GRAY,
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
