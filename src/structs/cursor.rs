use crate::consts::*;
use crate::screen::origin;
use crate::structs::point::Point;

pub struct Cursor {
    pub location: Point,
}

impl Cursor {
    pub fn default() -> Cursor {
        Cursor { location: origin() }
    }

    pub fn move_to(&mut self, mouse_position: (f32, f32)) {
        self.location.0 = (mouse_position.0 as i16) / SCALE;
        self.location.1 = (mouse_position.1 as i16) / SCALE;
    }
}
