use crate::consts::SCALE;
use crate::enums::{status::Status, team::Team};
use crate::screen::origin;
use crate::structs::point::Point;
use crate::{Color, draw_rectangle};

pub struct Cursor {
    location: Point,
    status: Status,
    team: Team,
}

impl Cursor {
    pub fn default() -> Cursor {
        Cursor {
            location: origin(),
            status: Status::default(),
            team: Team::default(),
        }
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
            self.get_color(),
        );
    }

    pub fn move_to(&mut self, mouse_position: (f32, f32)) {
        self.location.0 = (mouse_position.0 as i16) / SCALE;
        self.location.1 = (mouse_position.1 as i16) / SCALE;
    }

    pub fn get_location(&self) -> Point {
        self.location
    }

    pub fn set_team(&mut self, team: Team) {
        self.team = team;
    }

    pub fn get_team(&self) -> Team {
        self.team
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn get_color(&self) -> Color {
        Team::team_color(self.team)
    }
}
