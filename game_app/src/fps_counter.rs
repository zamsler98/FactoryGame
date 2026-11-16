use macroquad::{color::WHITE, text::draw_text};

/// How often the displayed FPS value refreshes, in seconds.
const REFRESH_INTERVAL: f32 = 0.5;

/// FPS counter that averages over REFRESH_INTERVAL so the number is readable.
#[derive(Default)]
pub struct FpsCounter {
    elapsed: f32,
    frames: u32,
    shown: u32,
}

impl FpsCounter {
    /// Call once per frame with the frame's duration in seconds.
    pub fn update(&mut self, dt: f32) {
        self.elapsed += dt;
        self.frames += 1;
        if self.elapsed >= REFRESH_INTERVAL {
            self.shown = (self.frames as f32 / self.elapsed).round() as u32;
            self.elapsed = 0.0;
            self.frames = 0;
        }
    }

    /// Draws in the current camera space; call after set_default_camera().
    pub fn draw(&self) {
        draw_text(&format!("FPS: {}", self.shown), 10.0, 20.0, 20.0, WHITE);
    }
}
