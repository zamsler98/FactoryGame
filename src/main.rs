use macroquad::prelude::*;

// This attribute is required by macroquad for main/entry.
#[macroquad::main("FactoryGame")]
async fn main() {
    let mut pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
    let radius = 40.0;

    loop {
        clear_background(LIGHTGRAY);

        // Touch input (also works as mouse input on desktop)
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            pos = vec2(x, y);
        }

        draw_circle(pos.x, pos.y, radius, BLUE);
        draw_text("Tap/click to move the ball!", 15.0, 40.0, 32.0, DARKGRAY);

        next_frame().await;
    }
}
