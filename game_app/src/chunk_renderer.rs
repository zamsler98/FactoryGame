use crate::entity_renderers::EntityRenderer;
use game_logic::Chunk;
use macroquad::{color::WHITE, shapes::draw_rectangle_lines, text::draw_text};

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
        let w = 32.0;
        let x = chunk.coords.x;
        let y = chunk.coords.y;
        let s = format!("({x},{y})");
        draw_text(&s, x as f32, y as f32 + 10.0, 16.0, WHITE);
        for i in 0..8 {
            for j in 0..8 {
                draw_rectangle_lines(
                    chunk.coords.x as f32 + (i as f32 * w),
                    chunk.coords.y as f32 + (j as f32 * w),
                    w,
                    w,
                    1.0,
                    WHITE,
                );
            }
        }
    }
}
