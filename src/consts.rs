use crate::{Color, GOLD, LIME, RED, SKYBLUE};

pub const SCALE: i16 = 8;
pub const FONT_SIZE: i16 = 14;
pub const SPEED: f64 = 0.05;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Team {
    BLUE,
    RED,
    GOLD,
    GREEN,
}

impl Team {
    pub fn team_color(team: Team) -> Color {
        match team {
            Team::BLUE => SKYBLUE,
            Team::RED => RED,
            Team::GOLD => GOLD,
            Team::GREEN => LIME,
        }
    }
}
