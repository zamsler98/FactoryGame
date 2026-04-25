//! game_app: Macroquad application glue.
//! - captures platform input and fills `InputFrame`
//! - calls `game_logic::update_world`
//! - performs rendering using Macroquad APIs
//!
//! Only this crate depends on `macroquad`.

mod app;
mod render_grid;
mod screens;
mod ui;

#[macroquad::main("FactoryGame - Macroquad")]
async fn main() {
    app::run().await;
}
