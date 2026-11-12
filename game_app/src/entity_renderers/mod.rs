use game_logic::Building;

pub mod entity_renderer;
pub mod miner_renderer;

pub use entity_renderer::EntityRenderer;

pub trait RenderEntity {
    fn render(&self, entity: &Building);
}
