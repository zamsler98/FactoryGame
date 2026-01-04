use game_core::{Rotation, Size2, TilePos};
use game_logic::grid_snapshot;
use macroquad::prelude::*;

pub const TILE_PX: f32 = 32.0;

pub fn draw_grid(snapshot: &game_logic::placement::TileGridSnapshot, hover: Option<TilePos>) {
    let width = snapshot.width as i32;
    let height = snapshot.height as i32;
    let screen_w = screen_width();
    let screen_h = screen_height();

    // draw background for grid area (top-left aligned)
    clear_background(Color::from_rgba(20, 20, 20, 255));

    // draw tile lines
    let line_color = Color::new(0.7, 0.7, 0.7, 0.18);
    for x in 0..=width {
        let sx = x as f32 * TILE_PX;
        draw_line(sx, 0.0, sx, (height as f32) * TILE_PX, 1.0, line_color);
    }
    for y in 0..=height {
        let sy = y as f32 * TILE_PX;
        draw_line(0.0, sy, (width as f32) * TILE_PX, sy, 1.0, line_color);
    }

    // darker major grid lines every 8 tiles
    let major_color = Color::new(0.6, 0.6, 0.6, 0.25);
    for x in (0..=width).step_by(8) {
        let sx = x as f32 * TILE_PX;
        draw_line(sx, 0.0, sx, (height as f32) * TILE_PX, 2.0, major_color);
    }
    for y in (0..=height).step_by(8) {
        let sy = y as f32 * TILE_PX;
        draw_line(0.0, sy, (width as f32) * TILE_PX, sy, 2.0, major_color);
    }

    // draw existing instances as filled rects
    for inst in &snapshot.instances {
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
        if h.x >= 0
            && h.y >= 0
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
