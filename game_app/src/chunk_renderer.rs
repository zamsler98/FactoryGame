// use crate::entity_renderers::EntityRenderer;
use game_logic::{CHUNK_SIZE, CHUNK_TILES, Chunk, TILE_SIZE};
use macroquad::{color::WHITE, shapes::draw_rectangle_lines, text::draw_text};

/// `TILE_SIZE` as a float, since every drawing call takes one.
const TILE_SIZE_F: f32 = TILE_SIZE as f32;
/// `CHUNK_SIZE` as a float, since every drawing call takes one.
const CHUNK_SIZE_F: f32 = CHUNK_SIZE as f32;
/// Line thickness of the tile grid.
const TILE_LINE_THICKNESS: f32 = 1.0;
/// Line thickness of the chunk border, drawn bolder than the tile grid.
const CHUNK_LINE_THICKNESS: f32 = 2.0;

/// Renders every entity in a chunk.
pub struct ChunkRenderer {
    // entity_renderer: EntityRenderer,
}

impl ChunkRenderer {
    pub async fn load() -> Self {
        Self {
            // entity_renderer: EntityRenderer::load().await,
        }
    }

    pub fn render(&self, chunk: &Chunk) {
        // Chunk coords are chunk indices, so scale them up to the world origin.
        let origin = chunk.coords.origin().as_vec2f();
        let origin_x = origin.x;
        let origin_y = origin.y;

        for i in 0..CHUNK_TILES {
            for j in 0..CHUNK_TILES {
                draw_rectangle_lines(
                    origin_x + (i as f32 * TILE_SIZE_F),
                    origin_y + (j as f32 * TILE_SIZE_F),
                    TILE_SIZE_F,
                    TILE_SIZE_F,
                    TILE_LINE_THICKNESS,
                    WHITE,
                );
            }
        }

        // Bolder border last so it draws over the tile grid it shares edges with.
        draw_rectangle_lines(
            origin_x,
            origin_y,
            CHUNK_SIZE_F,
            CHUNK_SIZE_F,
            CHUNK_LINE_THICKNESS,
            WHITE,
        );

        let s = format!("({},{})", chunk.coords.0.x, chunk.coords.0.y);
        draw_text(&s, origin_x, origin_y + 10.0, 16.0, WHITE);
    }
}
