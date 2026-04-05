use crate::consts::*;
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
        let mut x_offset = SCALE / 4;
        let mut y_offset = SCALE / 4;

        if self.location.0 < 0 {
            x_offset = x_offset * -1
        }

        if self.location.1 < 0 {
            y_offset = y_offset * -1
        }

        draw_rectangle(
            ((self.location.0 * SCALE) + x_offset) as f32,
            ((self.location.1 * SCALE) + y_offset) as f32,
            (SCALE / 2) as f32,
            (SCALE / 2) as f32,
            GOLD,
        );
    }

    pub fn left(&mut self, scale: i16) {
        self.location.0 -= scale
    }

    pub fn right(&mut self, scale: i16) {
        self.location.0 += scale
    }

    pub fn up(&mut self, scale: i16) {
        self.location.1 -= scale
    }

    pub fn down(&mut self, scale: i16) {
        self.location.1 += scale
    }
}
