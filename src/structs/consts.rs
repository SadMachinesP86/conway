use macroquad::window::{screen_height, screen_width};

pub const SIZE: i16 = 16;
pub const FONT_SIZE: i16 = 14;

pub type Point = (i16, i16);

pub fn origin() -> Point {
    (
        (screen_width() / (SIZE as f32) / 2.) as i16,
        (screen_height() / (SIZE as f32) / 2.) as i16,
    )
}
