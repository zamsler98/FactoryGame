use crate::entity_renderers::EntityRenderer;
use game_logic::Chunk;
use macroquad::{color::WHITE, shapes::draw_rectangle_lines, text::draw_text};

/// Width and height of a chunk, in tiles.
const CHUNK_TILES: u32 = 8;
/// Width and height of a tile, in world units.
const TILE_SIZE: f32 = 32.0;
/// Width and height of a chunk, in world units.
const CHUNK_SIZE: f32 = CHUNK_TILES as f32 * TILE_SIZE;
/// Line thickness of the tile grid.
const TILE_LINE_THICKNESS: f32 = 1.0;
/// Line thickness of the chunk border, drawn bolder than the tile grid.
const CHUNK_LINE_THICKNESS: f32 = 2.0;

/// Renders every entity in a chunk.
pub struct ChunkRenderer {
    entity_renderer: EntityRenderer,
}

impl ChunkRenderer {
    pub async fn load() -> Self {
        Self {
            entity_renderer: EntityRenderer::load().await,
        }
    }

    pub fn render(&self, chunk: &Chunk) {
        let x = chunk.coords.x;
        let y = chunk.coords.y;

        // Chunk coords are chunk indices, so scale them up to the world origin.
        let origin_x = x as f32 * CHUNK_SIZE;
        let origin_y = y as f32 * CHUNK_SIZE;

        for i in 0..CHUNK_TILES {
            for j in 0..CHUNK_TILES {
                draw_rectangle_lines(
                    origin_x + (i as f32 * TILE_SIZE),
                    origin_y + (j as f32 * TILE_SIZE),
                    TILE_SIZE,
                    TILE_SIZE,
                    TILE_LINE_THICKNESS,
                    WHITE,
                );
            }
        }

        // Bolder border last so it draws over the tile grid it shares edges with.
        draw_rectangle_lines(
            origin_x,
            origin_y,
            CHUNK_SIZE,
            CHUNK_SIZE,
            CHUNK_LINE_THICKNESS,
            WHITE,
        );

        let s = format!("({x},{y})");
        draw_text(&s, origin_x, origin_y + 10.0, 16.0, WHITE);
    }
}
