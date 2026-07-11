use game_core::{BuildingKind, ItemKind, Rotation, Size2, TilePos};
use game_logic::view::{ItemView, MachineView};

use macroquad::prelude::*;

pub const TILE_PX: f32 = 32.0;

pub fn building_color(kind: Option<BuildingKind>) -> Color {
    match kind {
        Some(BuildingKind::Conveyor) => Color::new(1.0, 1.0, 1.0, 0.95),
        Some(BuildingKind::Miner) => Color::new(0.9, 0.6, 0.3, 0.9),
        Some(BuildingKind::Smelter) => Color::new(0.3, 0.8, 0.4, 0.9),
        None => Color::new(0.7, 0.7, 0.7, 0.9),
    }
}

pub fn item_color(kind: ItemKind) -> Color {
    match kind {
        ItemKind::IronOre => Color::new(0.45, 0.35, 0.30, 1.0),
        ItemKind::IronIngot => Color::new(0.78, 0.82, 0.90, 1.0),
    }
}

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
        let rs = match inst.rotation {
            Rotation::R0 | Rotation::R180 => inst.size,
            Rotation::R90 | Rotation::R270 => Size2 {
                w: inst.size.h,
                h: inst.size.w,
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
        let kind = BuildingKind::from_spec_id(inst.spec_id);
        draw_rectangle(x, y, w, h, building_color(kind));

        // Direction indicator: every current building pushes items out of its
        // facing side, so all of them get an arrow.
        if kind.is_some() {
            // center of footprint
            let cx = x + w * 0.5;
            let cy = y + h * 0.5;
            let len = TILE_PX * 0.28; // arrow length
            let (dx, dy) = inst.rotation.dir();
            let (dx, dy) = (dx as f32, dy as f32);
            let tip_x = cx + dx * len;
            let tip_y = cy + dy * len;
            // base width for triangle
            let bw = TILE_PX * 0.14;
            // perpendicular vector
            let (px_off, py_off) = (-dy * bw, dx * bw);
            let p1 = Vec2::new(tip_x, tip_y);
            let p2 = Vec2::new(cx + px_off, cy + py_off);
            let p3 = Vec2::new(cx - px_off, cy - py_off);
            let arrow_color = Color::new(0.1, 0.1, 0.1, 0.95);
            draw_triangle(p1, p2, p3, arrow_color);
        }
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

/// Draw items sitting on belts / waiting in miners. Positions are in
/// fractional tile units (see `game_logic::view::ItemView`).
pub fn draw_items(items: &[ItemView]) {
    for item in items {
        let x = item.x * TILE_PX;
        let y = item.y * TILE_PX;
        let r = TILE_PX * 0.2;
        draw_circle(x, y, r, item_color(item.kind));
        draw_circle_lines(x, y, r, 1.5, Color::new(0.0, 0.0, 0.0, 0.6));
    }
}

/// Draw machine status overlays: a working progress bar, and buffer counts
/// for smelters.
pub fn draw_machine_overlays(machines: &[MachineView]) {
    for m in machines {
        let x = m.origin.x as f32 * TILE_PX;
        let y = m.origin.y as f32 * TILE_PX;
        if let Some(p) = m.progress {
            let bar_h = TILE_PX * 0.12;
            let bar_w = TILE_PX - 4.0;
            let by = y + TILE_PX - bar_h - 2.0;
            draw_rectangle(x + 2.0, by, bar_w, bar_h, Color::new(0.0, 0.0, 0.0, 0.5));
            draw_rectangle(
                x + 2.0,
                by,
                bar_w * p.clamp(0.0, 1.0),
                bar_h,
                Color::new(1.0, 0.85, 0.2, 0.9),
            );
        }
        if m.kind == BuildingKind::Smelter {
            let label = format!("{}/{}", m.input_count, m.output_count);
            draw_text(&label, x + 3.0, y + 12.0, 14.0, BLACK);
        }
    }
}
