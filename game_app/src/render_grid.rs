use game_core::{BuildingKind, ItemKind, ResourceKind, Rotation, Size2, TilePos};
use game_logic::placement::ResourceView;
use game_logic::view::{ItemView, MachineView};

use macroquad::prelude::*;

pub const TILE_PX: f32 = 32.0;

pub fn building_color(kind: Option<BuildingKind>) -> Color {
    match kind {
        Some(BuildingKind::Belt) => Color::new(0.80, 0.80, 0.85, 0.95),
        Some(BuildingKind::Miner) => Color::new(0.90, 0.55, 0.25, 0.95),
        Some(BuildingKind::Furnace) => Color::new(0.85, 0.35, 0.25, 0.95),
        Some(BuildingKind::Inserter) => Color::new(0.30, 0.75, 0.90, 0.95),
        Some(BuildingKind::Assembler) => Color::new(0.35, 0.70, 0.55, 0.95),
        Some(BuildingKind::Chest) => Color::new(0.60, 0.45, 0.25, 0.95),
        None => Color::new(0.7, 0.7, 0.7, 0.9),
    }
}

pub fn item_color(kind: ItemKind) -> Color {
    match kind {
        ItemKind::IronOre => Color::new(0.55, 0.55, 0.62, 1.0),
        ItemKind::CopperOre => Color::new(0.72, 0.45, 0.30, 1.0),
        ItemKind::Coal => Color::new(0.12, 0.12, 0.14, 1.0),
        ItemKind::Stone => Color::new(0.62, 0.58, 0.48, 1.0),
        ItemKind::IronPlate => Color::new(0.78, 0.82, 0.90, 1.0),
        ItemKind::CopperPlate => Color::new(0.85, 0.52, 0.32, 1.0),
        ItemKind::StoneBrick => Color::new(0.55, 0.45, 0.40, 1.0),
        ItemKind::IronGearWheel => Color::new(0.60, 0.65, 0.72, 1.0),
        ItemKind::CopperCable => Color::new(0.90, 0.60, 0.25, 1.0),
        ItemKind::ElectronicCircuit => Color::new(0.30, 0.70, 0.35, 1.0),
        ItemKind::TransportBelt => Color::new(0.80, 0.80, 0.85, 1.0),
        ItemKind::BurnerMiningDrill => Color::new(0.90, 0.55, 0.25, 1.0),
        ItemKind::StoneFurnace => Color::new(0.85, 0.35, 0.25, 1.0),
        ItemKind::Inserter => Color::new(0.30, 0.75, 0.90, 1.0),
        ItemKind::AssemblingMachine => Color::new(0.35, 0.70, 0.55, 1.0),
        ItemKind::WoodenChest => Color::new(0.60, 0.45, 0.25, 1.0),
    }
}

pub fn resource_color(kind: ResourceKind) -> Color {
    match kind {
        ResourceKind::IronOre => Color::new(0.45, 0.55, 0.70, 0.5),
        ResourceKind::CopperOre => Color::new(0.70, 0.45, 0.28, 0.5),
        ResourceKind::Coal => Color::new(0.20, 0.20, 0.24, 0.7),
        ResourceKind::Stone => Color::new(0.55, 0.50, 0.40, 0.5),
    }
}

/// Draw ore patches under the buildings.
pub fn draw_resources(resources: &[ResourceView], min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
    for r in resources {
        if r.pos.x < min_x || r.pos.x > max_x || r.pos.y < min_y || r.pos.y > max_y {
            continue;
        }
        let x = r.pos.x as f32 * TILE_PX;
        let y = r.pos.y as f32 * TILE_PX;
        draw_rectangle(x, y, TILE_PX, TILE_PX, resource_color(r.kind));
    }
}

// Draw only the visible portion of the grid. The min/max tile bounds are inclusive.
pub fn draw_grid(
    snapshot: &game_logic::placement::TileGridSnapshot,
    hover: Option<TilePos>,
    hover_ok: bool,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) {
    let width = snapshot.width as i32;
    let height = snapshot.height as i32;

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
        if x + w < (min_x as f32) * TILE_PX || x > (max_x as f32) * TILE_PX {
            continue;
        }
        if y + h < (min_y as f32) * TILE_PX || y > (max_y as f32) * TILE_PX {
            continue;
        }
        let kind = BuildingKind::from_spec_id(inst.spec_id);
        // Inserters draw smaller so the belt beneath them stays visible.
        if kind == Some(BuildingKind::Inserter) {
            draw_rectangle(
                x + w * 0.3,
                y + h * 0.3,
                w * 0.4,
                h * 0.4,
                building_color(kind),
            );
        } else {
            draw_rectangle(x, y, w, h, building_color(kind));
        }

        // Direction arrow: every building has a facing.
        if kind.is_some() {
            let cx = x + w * 0.5;
            let cy = y + h * 0.5;
            let len = TILE_PX * 0.30;
            let (dx, dy) = inst.rotation.dir();
            let (dx, dy) = (dx as f32, dy as f32);
            let tip_x = cx + dx * len;
            let tip_y = cy + dy * len;
            let bw = TILE_PX * 0.14;
            let (px_off, py_off) = (-dy * bw, dx * bw);
            let p1 = Vec2::new(tip_x, tip_y);
            let p2 = Vec2::new(cx + px_off, cy + py_off);
            let p3 = Vec2::new(cx - px_off, cy - py_off);
            let arrow_color = Color::new(0.08, 0.08, 0.08, 0.95);
            draw_triangle(p1, p2, p3, arrow_color);
        }
    }

    // hover highlight (green when placeable, red when not)
    if let Some(h) = hover {
        if h.x >= min_x
            && h.y >= min_y
            && (h.x as usize) < snapshot.width
            && (h.y as usize) < snapshot.height
        {
            let rx = h.x as f32 * TILE_PX;
            let ry = h.y as f32 * TILE_PX;
            let c = if hover_ok {
                Color::new(0.3, 1.0, 0.3, 0.9)
            } else {
                Color::new(1.0, 0.3, 0.3, 0.9)
            };
            draw_rectangle_lines(rx, ry, TILE_PX, TILE_PX, 3.0, c);
            draw_rectangle(rx, ry, TILE_PX, TILE_PX, Color::new(c.r, c.g, c.b, 0.08));
        }
    }
}

/// Draw items sitting on belts, held by inserters, or waiting in miners.
pub fn draw_items(items: &[ItemView]) {
    for item in items {
        let x = item.x * TILE_PX;
        let y = item.y * TILE_PX;
        let r = TILE_PX * 0.2;
        draw_circle(x, y, r, item_color(item.kind));
        draw_circle_lines(x, y, r, 1.5, Color::new(0.0, 0.0, 0.0, 0.6));
    }
}

/// Draw machine status overlays: work progress bar and buffer counts.
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
        if let Some(label) = &m.label {
            draw_text(label, x + 3.0, y + 12.0, 14.0, BLACK);
        }
    }
}
