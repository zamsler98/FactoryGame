use macroquad::{color::WHITE, text::draw_text};

/// How often the displayed rate refreshes, in seconds.
const REFRESH_INTERVAL: f32 = 0.5;

/// Rate counter that averages over REFRESH_INTERVAL so the number is readable.
pub struct RateCounter {
    label: &'static str,
    y: f32,
    elapsed: f32,
    events: u32,
    shown: u32,
}

impl RateCounter {
    pub fn new(label: &'static str, y: f32) -> Self {
        Self {
            label,
            y,
            elapsed: 0.0,
            events: 0,
            shown: 0,
        }
    }

    /// Call once per frame with the frame's duration in seconds and the
    /// number of events that occurred during it.
    pub fn update(&mut self, dt: f32, events: u32) {
        self.elapsed += dt;
        self.events += events;
        if self.elapsed >= REFRESH_INTERVAL {
            self.shown = (self.events as f32 / self.elapsed).round() as u32;
            self.elapsed = 0.0;
            self.events = 0;
        }
    }

    /// Draws in the current camera space; call after set_default_camera().
    pub fn draw(&self) {
        draw_text(
            &format!("{}: {}", self.label, self.shown),
            10.0,
            self.y,
            20.0,
            WHITE,
        );
    }
}
