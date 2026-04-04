use crate::consts::*;
use macroquad::window::{next_frame, screen_height, screen_width, set_fullscreen};

pub fn origin() -> Point {
    (
        (screen_width() / (SIZE as f32) / 2.) as i16,
        (screen_height() / (SIZE as f32) / 2.) as i16,
    )
}

pub async fn fullscreen() {
    set_fullscreen(true);

    loop {
        if screen_width() != 400. && screen_height() != 300. {
            break;
        }
        next_frame().await;
    }
}
