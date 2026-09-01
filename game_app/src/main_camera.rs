use macroquad::{
    camera::{Camera2D, set_camera},
    math::{Rect, Vec2, vec2},
    window::{screen_height, screen_width},
};

const VIEW_HEIGHT: f32 = 600.0;

#[derive(Default)]
pub struct MainCamera {
    pub position: Vec2,
}

impl MainCamera {
    /// The world-space region currently visible, in world units (not pixels).
    pub fn bounds(&self) -> Rect {
        let camera = self.camera();
        let top_left = camera.screen_to_world(Vec2::ZERO);
        let bot_right = camera.screen_to_world(vec2(screen_width(), screen_height()));

        Rect::new(
            top_left.x,
            top_left.y,
            bot_right.x - top_left.x,
            bot_right.y - top_left.y,
        )
    }

    pub fn pan(&mut self, delta: Vec2) {
        self.position += delta;
    }

    pub fn use_camera(&self) {
        set_camera(&self.camera());
    }

    fn camera(&self) -> Camera2D {
        let view_w = VIEW_HEIGHT * screen_width() / screen_height();

        // Y-down camera: macroquad's `matrix()` negates `zoom.y` when rendering to the
        // screen, so a positive `zoom.y` here is what makes +Y point down. Using
        // `Camera2D::from_display_rect` instead would double-negate and flip text.
        Camera2D {
            target: vec2(
                self.position.x + view_w / 2.0,
                self.position.y + VIEW_HEIGHT / 2.0,
            ),
            zoom: vec2(2.0 / view_w, 2.0 / VIEW_HEIGHT),
            ..Default::default()
        }
    }
}
