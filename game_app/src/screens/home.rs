use crate::screens::{ScreenCommand, ScreenId};
use crate::ui::{centered_button_stack, hit_test, wrap_selection, UiRect};
use macroquad::prelude::*;

const BUTTON_WIDTH: f32 = 280.0;
const BUTTON_HEIGHT: f32 = 64.0;
const BUTTON_SPACING: f32 = 18.0;

#[cfg(target_arch = "wasm32")]
const MENU_ITEMS: [HomeAction; 2] = [HomeAction::Play, HomeAction::Settings];

#[cfg(not(target_arch = "wasm32"))]
const MENU_ITEMS: [HomeAction; 3] = [HomeAction::Play, HomeAction::Settings, HomeAction::Quit];

#[derive(Clone, Copy)]
enum HomeAction {
    Play,
    Settings,
    Quit,
}

impl HomeAction {
    fn label(self) -> &'static str {
        match self {
            Self::Play => "Play",
            Self::Settings => "Settings",
            Self::Quit => "Quit",
        }
    }

    fn command(self) -> ScreenCommand {
        match self {
            Self::Play => ScreenCommand::Switch(ScreenId::Gameplay),
            Self::Settings => ScreenCommand::Switch(ScreenId::Settings),
            Self::Quit => ScreenCommand::Quit,
        }
    }
}

pub struct HomeScreen {
    selected_index: usize,
    hovered_index: Option<usize>,
}

impl HomeScreen {
    pub fn new() -> Self {
        Self {
            selected_index: 0,
            hovered_index: None,
        }
    }

    pub fn update(&mut self) -> ScreenCommand {
        let button_rects = self.button_rects();

        self.hovered_index = current_pointer().and_then(|point| hit_test(&button_rects, point));
        if let Some(index) = self.hovered_index {
            self.selected_index = index;
        }

        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) {
            self.selected_index = wrap_selection(self.selected_index, MENU_ITEMS.len(), -1);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.selected_index = wrap_selection(self.selected_index, MENU_ITEMS.len(), 1);
        }

        if let Some(index) = activated_button(&button_rects) {
            self.selected_index = index;
            return MENU_ITEMS[index].command();
        }

        if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            return MENU_ITEMS[self.selected_index].command();
        }

        ScreenCommand::None
    }

    pub fn draw(&self) {
        clear_background(Color::from_rgba(18, 22, 30, 255));

        let title = "FactoryGame";
        let title_size = 52u16;
        let title_dims = measure_text(title, None, title_size, 1.0);
        draw_text(
            title,
            (screen_width() - title_dims.width) / 2.0,
            screen_height() * 0.24,
            title_size as f32,
            WHITE,
        );

        let subtitle = "A new home screen for the factory.";
        let subtitle_size = 26u16;
        let subtitle_dims = measure_text(subtitle, None, subtitle_size, 1.0);
        draw_text(
            subtitle,
            (screen_width() - subtitle_dims.width) / 2.0,
            screen_height() * 0.32,
            subtitle_size as f32,
            Color::from_rgba(186, 196, 210, 255),
        );

        for (index, (action, rect)) in MENU_ITEMS.iter().zip(self.button_rects()).enumerate() {
            draw_menu_button(
                rect,
                action.label(),
                self.selected_index == index,
                self.hovered_index == Some(index),
            );
        }

        let hint = "Use W/S or arrow keys, Enter, or click.";
        let hint_size = 24u16;
        let hint_dims = measure_text(hint, None, hint_size, 1.0);
        draw_text(
            hint,
            (screen_width() - hint_dims.width) / 2.0,
            screen_height() * 0.84,
            hint_size as f32,
            Color::from_rgba(145, 155, 170, 255),
        );
    }

    fn button_rects(&self) -> Vec<UiRect> {
        centered_button_stack(
            screen_width(),
            screen_height() * 0.57,
            MENU_ITEMS.len(),
            BUTTON_WIDTH,
            BUTTON_HEIGHT,
            BUTTON_SPACING,
        )
    }
}

fn current_pointer() -> Option<(f32, f32)> {
    touches()
        .first()
        .map(|touch| (touch.position.x, touch.position.y))
        .or_else(|| Some(mouse_position()))
}

fn activated_button(rects: &[UiRect]) -> Option<usize> {
    let active_touches = touches();

    if let Some(touch) = active_touches
        .iter()
        .find(|touch| touch.phase == TouchPhase::Ended)
    {
        return hit_test(rects, (touch.position.x, touch.position.y));
    }

    if active_touches.is_empty() && is_mouse_button_released(MouseButton::Left) {
        return hit_test(rects, mouse_position());
    }

    None
}

fn draw_menu_button(rect: UiRect, label: &str, selected: bool, hovered: bool) {
    let fill = if selected {
        Color::from_rgba(77, 128, 227, 255)
    } else if hovered {
        Color::from_rgba(50, 74, 120, 255)
    } else {
        Color::from_rgba(33, 43, 58, 255)
    };
    let outline = if selected {
        Color::from_rgba(241, 207, 95, 255)
    } else {
        Color::from_rgba(92, 108, 130, 255)
    };

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, fill);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 3.0, outline);

    let text_size = 32u16;
    let dims = measure_text(label, None, text_size, 1.0);
    draw_text(
        label,
        rect.x + (rect.w - dims.width) / 2.0,
        rect.y + rect.h * 0.63,
        text_size as f32,
        WHITE,
    );
}
