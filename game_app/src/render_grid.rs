use game_core::{Rotation, Size2, TilePos};

use macroquad::prelude::*;

pub const TILE_PX: f32 = 32.0;

// Draw only the visible portion of the grid. The min/max tile bounds are inclusive.
pub fn draw_grid(
    snapshot: &game_logic::placement::TileGridSnapshot,
    hover: Option<TilePos>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) {
    let width = snapshot.width as i32;
    let height = snapshot.height as i32;

    // draw background for grid area (world-space)
    clear_background(Color::from_rgba(20, 20, 20, 255));

    // clamp bounds to snapshot
    let min_x = min_x.max(0).min(width);
    let min_y = min_y.max(0).min(height);
    let max_x = max_x.max(0).min(width);
    let max_y = max_y.max(0).min(height);

    // draw tile lines (vertical)
    let line_color = Color::new(0.7, 0.7, 0.7, 0.18);
    for x in min_x..=max_x {
        let sx = x as f32 * TILE_PX;
        draw_line(
            sx,
            (min_y as f32) * TILE_PX,
            sx,
            (max_y as f32) * TILE_PX,
            1.0,
            line_color,
        );
    }
    // horizontal
    for y in min_y..=max_y {
        let sy = y as f32 * TILE_PX;
        draw_line(
            (min_x as f32) * TILE_PX,
            sy,
            (max_x as f32) * TILE_PX,
            sy,
            1.0,
            line_color,
        );
    }

    // darker major grid lines every 8 tiles (aligned to full grid)
    let major_color = Color::new(0.6, 0.6, 0.6, 0.25);
    let start_x_major = (min_x / 8) * 8;
    let start_y_major = (min_y / 8) * 8;
    for x in (start_x_major..=max_x).step_by(8) {
        let sx = x as f32 * TILE_PX;
        draw_line(
            sx,
            (min_y as f32) * TILE_PX,
            sx,
            (max_y as f32) * TILE_PX,
            2.0,
            major_color,
        );
    }
    for y in (start_y_major..=max_y).step_by(8) {
        let sy = y as f32 * TILE_PX;
        draw_line(
            (min_x as f32) * TILE_PX,
            sy,
            (max_x as f32) * TILE_PX,
            sy,
            2.0,
            major_color,
        );
    }

    // draw existing instances as filled rects (only those in range)
    for inst in &snapshot.instances {
        if inst.origin.x < min_x || inst.origin.y < min_y {
            continue;
        }
        // determine footprint size based on spec id (same mapping as core)
        let size = match inst.spec_id {
            1 => Size2 { w: 1, h: 1 },
            2 => Size2 { w: 2, h: 2 },
            3 => Size2 { w: 3, h: 3 },
            _ => Size2 { w: 1, h: 1 },
        };
        let rs = match inst.rotation {
            Rotation::R0 | Rotation::R180 => size,
            Rotation::R90 | Rotation::R270 => Size2 {
                w: size.h,
                h: size.w,
            },
        };
        let x = inst.origin.x as f32 * TILE_PX;
        let y = inst.origin.y as f32 * TILE_PX;
        let w = rs.w as f32 * TILE_PX;
        let h = rs.h as f32 * TILE_PX;
        // quick cull: skip if completely outside
        if x + w < (min_x as f32) * TILE_PX || x > (max_x as f32) * TILE_PX {
            continue;
        }
        if y + h < (min_y as f32) * TILE_PX || y > (max_y as f32) * TILE_PX {
            continue;
        }
        let color = match inst.spec_id {
            1 => Color::new(0.8, 0.8, 0.8, 0.9),
            2 => Color::new(0.9, 0.6, 0.3, 0.9),
            3 => Color::new(0.3, 0.8, 0.4, 0.9),
            _ => Color::new(0.7, 0.7, 0.7, 0.9),
        };
        draw_rectangle(x, y, w, h, color);
    }

    // hover highlight
    if let Some(h) = hover {
        if h.x >= min_x
            && h.y >= min_y
            && (h.x as usize) < snapshot.width
            && (h.y as usize) < snapshot.height
        {
            let rx = h.x as f32 * TILE_PX;
            let ry = h.y as f32 * TILE_PX;
            draw_rectangle_lines(
                rx,
                ry,
                TILE_PX,
                TILE_PX,
                3.0,
                Color::new(1.0, 1.0, 0.0, 0.9),
            );
            draw_rectangle(rx, ry, TILE_PX, TILE_PX, Color::new(1.0, 1.0, 0.0, 0.06));
        }
    }
}
