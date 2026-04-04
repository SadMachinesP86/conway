use super::consts::*;
use crate::screen::origin;
use crate::{GOLD, draw_rectangle};

pub struct Cursor {
    pub location: Point,
}

impl Cursor {
    pub fn default() -> Cursor {
        Cursor { location: origin() }
    }

    pub fn draw(&self) {
        let mut x_offset = SIZE / 4;
        let mut y_offset = SIZE / 4;

        if self.location.0 < 0 {
            x_offset = x_offset * -1
        }

        if self.location.1 < 0 {
            y_offset = y_offset * -1
        }

        draw_rectangle(
            ((self.location.0 * SIZE) + x_offset) as f32,
            ((self.location.1 * SIZE) + y_offset) as f32,
            (SIZE / 2) as f32,
            (SIZE / 2) as f32,
            GOLD,
        );
    }

    pub fn left(&mut self) {
        self.location.0 -= 1
    }

    pub fn right(&mut self) {
        self.location.0 += 1
    }

    pub fn up(&mut self) {
        self.location.1 -= 1
    }

    pub fn down(&mut self) {
        self.location.1 += 1
    }
}
