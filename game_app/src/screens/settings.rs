use crate::screens::{ScreenCommand, ScreenId};
use crate::ui::{centered_button_stack, hit_test, UiRect};
use macroquad::prelude::*;

const BUTTON_WIDTH: f32 = 240.0;
const BUTTON_HEIGHT: f32 = 58.0;

pub struct SettingsScreen {
    hovered_back: bool,
}

impl SettingsScreen {
    pub fn new() -> Self {
        Self {
            hovered_back: false,
        }
    }

    pub fn update(&mut self) -> ScreenCommand {
        let back_rect = self.back_button_rect();
        let back_hovered = hit_test(&[back_rect], mouse_position()).is_some();
        self.hovered_back = back_hovered;

        if is_key_pressed(KeyCode::Escape)
            || is_key_pressed(KeyCode::Enter)
            || is_key_pressed(KeyCode::Space)
            || (is_mouse_button_released(MouseButton::Left) && back_hovered)
        {
            return ScreenCommand::Switch(ScreenId::Home);
        }

        ScreenCommand::None
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(14, 18, 24, 255));

        let title = "Settings";
        let title_size = 48u16;
        let title_dims = measure_text(title, None, title_size, 1.0);
        draw_text(
            title,
            (screen_width() - title_dims.width) / 2.0,
            screen_height() * 0.24,
            title_size as f32,
            WHITE,
        );

        let body = "Settings will live here.";
        let body_size = 28u16;
        let body_dims = measure_text(body, None, body_size, 1.0);
        draw_text(
            body,
            (screen_width() - body_dims.width) / 2.0,
            screen_height() * 0.42,
            body_size as f32,
            Color::from_rgba(180, 190, 205, 255),
        );

        let hint = "Press Esc, Enter, Space, or click Back.";
        let hint_size = 22u16;
        let hint_dims = measure_text(hint, None, hint_size, 1.0);
        draw_text(
            hint,
            (screen_width() - hint_dims.width) / 2.0,
            screen_height() * 0.52,
            hint_size as f32,
            Color::from_rgba(136, 149, 168, 255),
        );

        let back_rect = self.back_button_rect();
        draw_rectangle(
            back_rect.x,
            back_rect.y,
            back_rect.w,
            back_rect.h,
            if self.hovered_back {
                Color::from_rgba(58, 84, 136, 255)
            } else {
                Color::from_rgba(32, 42, 58, 255)
            },
        );
        draw_rectangle_lines(
            back_rect.x,
            back_rect.y,
            back_rect.w,
            back_rect.h,
            3.0,
            Color::from_rgba(241, 207, 95, 255),
        );

        let back_label = "Back";
        let back_size = 30u16;
        let back_dims = measure_text(back_label, None, back_size, 1.0);
        draw_text(
            back_label,
            back_rect.x + (back_rect.w - back_dims.width) / 2.0,
            back_rect.y + back_rect.h * 0.63,
            back_size as f32,
            WHITE,
        );
    }

    fn back_button_rect(&self) -> UiRect {
        centered_button_stack(
            screen_width(),
            screen_height() * 0.72,
            1,
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            0.0,
        )[0]
    }
}
