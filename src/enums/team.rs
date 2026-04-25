use crate::{Color, GOLD, LIME, RED, SKYBLUE};

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum Team {
    BLUE,
    RED,
    GOLD,
    GREEN,
}

impl Team {
    pub fn default() -> Team {
        Team::BLUE
    }

    pub fn team_color(team: Team) -> Color {
        match team {
            Team::BLUE => SKYBLUE,
            Team::RED => RED,
            Team::GOLD => GOLD,
            Team::GREEN => LIME,
        }
    }
}
