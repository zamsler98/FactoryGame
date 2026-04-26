use crate::screens::{Screen, ScreenCommand, ScreenId};
use macroquad::prelude::{get_frame_time, next_frame};

pub async fn run() {
    let mut screen = Screen::new(ScreenId::Home);

    loop {
        match screen.update(get_frame_time()) {
            ScreenCommand::None => {}
            ScreenCommand::Switch(screen_id) => {
                screen = Screen::new(screen_id);
            }
            ScreenCommand::Quit => break,
        }

        screen.draw();
        next_frame().await;
    }
}
