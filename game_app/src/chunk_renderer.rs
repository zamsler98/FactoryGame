use crate::entity_renderers::EntityRenderer;
use game_logic::Chunk;

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
        for entity in &chunk.entities {
            // self.entity_renderer.render(entity);
        }
    }
}
