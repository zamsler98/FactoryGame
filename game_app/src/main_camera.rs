use macroquad::{
    camera::{Camera2D, set_camera},
    math::{Vec2, vec2},
    window::{screen_height, screen_width},
};

const VIEW_HEIGHT: f32 = 600.0;

#[derive(Default)]
pub struct MainCamera {
    pub position: Vec2,
}

impl MainCamera {
    pub fn pan(&mut self, delta: Vec2) {
        self.position += delta;
    }

    pub fn use_camera(&self) {
        let view_w = VIEW_HEIGHT * screen_width() / screen_height();

        // Y-down camera: macroquad's `matrix()` negates `zoom.y` when rendering to the
        // screen, so a positive `zoom.y` here is what makes +Y point down. Using
        // `Camera2D::from_display_rect` instead would double-negate and flip text.
        set_camera(&Camera2D {
            target: vec2(
                self.position.x + view_w / 2.0,
                self.position.y + VIEW_HEIGHT / 2.0,
            ),
            zoom: vec2(2.0 / view_w, 2.0 / VIEW_HEIGHT),
            ..Default::default()
        });
    }
}
