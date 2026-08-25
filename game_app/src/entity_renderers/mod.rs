use game_logic::EntityInfo;

pub mod entity_renderer;
pub mod miner_renderer;

pub use entity_renderer::EntityRenderer;

pub trait RenderEntity {
    fn render(&self, entity: &EntityInfo);
}
