pub mod gameplay;
pub mod home;
pub mod settings;

use gameplay::GameplayScreen;
use home::HomeScreen;
use settings::SettingsScreen;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenId {
    Home,
    Settings,
    Gameplay,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScreenCommand {
    None,
    Switch(ScreenId),
    Quit,
}

pub enum Screen {
    Home(HomeScreen),
    Settings(SettingsScreen),
    Gameplay(Box<GameplayScreen>),
}

impl Screen {
    pub fn new(screen_id: ScreenId) -> Self {
        match screen_id {
            ScreenId::Home => Self::Home(HomeScreen::new()),
            ScreenId::Settings => Self::Settings(SettingsScreen::new()),
            ScreenId::Gameplay => Self::Gameplay(Box::new(GameplayScreen::new())),
        }
    }

    pub fn update(&mut self, dt: f32) -> ScreenCommand {
        match self {
            Self::Home(screen) => screen.update(),
            Self::Settings(screen) => screen.update(),
            Self::Gameplay(screen) => screen.update(dt),
        }
    }

    pub fn draw(&mut self) {
        match self {
            Self::Home(screen) => screen.draw(),
            Self::Settings(screen) => screen.draw(),
            Self::Gameplay(screen) => screen.draw(),
        }
    }
}
