use crate::math_interop::{ToCore, ToMacroquad};
use game_core::Vec2f;
use macroquad::{
    camera::{Camera2D, set_camera},
    math::{Rect, Vec2, vec2},
    window::{screen_height, screen_width},
};

const VIEW_HEIGHT: f32 = 600.0;

#[derive(Default)]
pub struct MainCamera {
    pub position: Vec2f,
}

impl MainCamera {
    /// The world-space region currently visible, in world units (not pixels).
    #[expect(dead_code, reason = "for the chunk culling that is not wired up yet")]
    pub fn bounds(&self) -> Rect {
        let camera = self.camera();
        let top_left = camera.screen_to_world(Vec2::ZERO).to_core();
        let bot_right = camera
            .screen_to_world(vec2(screen_width(), screen_height()))
            .to_core();
        let size = bot_right - top_left;

        Rect::new(top_left.x, top_left.y, size.x, size.y)
    }

    pub fn pan(&mut self, delta: Vec2f) {
        self.position += delta;
    }

    pub fn use_camera(&self) {
        set_camera(&self.camera());
    }

    fn camera(&self) -> Camera2D {
        let view_w = VIEW_HEIGHT * screen_width() / screen_height();
        let half_view = Vec2f::new(view_w, VIEW_HEIGHT) / 2.0;

        // Y-down camera: macroquad's `matrix()` negates `zoom.y` when rendering to the
        // screen, so a positive `zoom.y` here is what makes +Y point down. Using
        // `Camera2D::from_display_rect` instead would double-negate and flip text.
        Camera2D {
            target: (self.position + half_view).to_macroquad(),
            zoom: vec2(2.0 / view_w, 2.0 / VIEW_HEIGHT),
            ..Default::default()
        }
    }
}
