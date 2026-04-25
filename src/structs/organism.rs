use crate::Color;
use crate::enums::{status::Status, team::Team};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Organism {
    status: Status,
    team: Team,
}

impl Organism {
    pub fn new(status: Status, team: Team) -> Organism {
        Organism { status, team }
    }

    pub fn default() -> Organism {
        Organism {
            status: Status::default(),
            team: Team::default(),
        }
    }

    /// Implements the core game logic: determines the status of the organism for the next generation, based on its
    ///   current status and number of neighbors.
    /// Rules from Wikipedia:
    /// * Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    /// * Any live cell with two or three live neighbours lives on to the next generation.
    /// * Any live cell with more than three live neighbours dies, as if by overpopulation.
    /// * Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    pub fn set_status_for_neighbor_count(&mut self, neighbor_counts: HashMap<Team, usize>) {
        let sum: usize = neighbor_counts.values().sum();

        let max: usize;
        let highest_team: Team;

        if sum > 0 {
            max = *neighbor_counts.values().max().unwrap();
            highest_team = **neighbor_counts
                .keys()
                .filter_map(|k| match neighbor_counts.get(k) {
                    Some(i) => {
                        if *i == max {
                            Some(k)
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .collect::<Vec<&Team>>()
                .get(0)
                .unwrap();
        } else {
            highest_team = self.team;
        }

        match self.status {
            Status::ALIVE => {
                if sum < 2 || sum > 3 {
                    self.status = Status::DEAD;
                } else {
                    self.status = Status::ALIVE;
                    self.team = highest_team;
                }
            }
            Status::DEAD => {
                if sum == 3 {
                    self.status = Status::ALIVE;
                    self.team = highest_team;
                }
            }
        };
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn get_team(&self) -> Team {
        self.team
    }

    pub fn set_team(&mut self, team: Team) {
        self.team = team;
    }

    pub fn get_color(&self) -> Color {
        Team::team_color(self.team)
    }
}
