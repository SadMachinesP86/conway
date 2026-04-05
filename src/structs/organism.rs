use crate::consts::*;
use crate::{SKYBLUE, draw_rectangle};

#[derive(Clone, PartialEq)]
pub enum Status {
    ALIVE,
    DEAD,
}

#[derive(Clone)]
pub struct Organism {
    pub location: Point,
    pub status: Status,
}

impl Organism {
    pub fn draw(&self) {
        draw_rectangle(
            (self.location.0 * SCALE) as f32,
            (self.location.1 * SCALE) as f32,
            SCALE as f32,
            SCALE as f32,
            SKYBLUE,
        );
    }

    /// Implements the core game logic: determines the status of the organism for the next generation, based on its
    ///   current status and number of neighbors.
    /// Rules from Wikipedia:
    /// * Any live cell with fewer than two live neighbours dies, as if by underpopulation.
    /// * Any live cell with two or three live neighbours lives on to the next generation.
    /// * Any live cell with more than three live neighbours dies, as if by overpopulation.
    /// * Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
    pub fn set_status_for_neighbor_count(&mut self, neighbor_count: usize) {
        self.status = match self.status {
            Status::ALIVE => {
                if neighbor_count < 2 || neighbor_count > 3 {
                    Status::DEAD
                } else {
                    Status::ALIVE
                }
            }
            Status::DEAD => {
                if neighbor_count == 3 {
                    Status::ALIVE
                } else {
                    Status::DEAD
                }
            }
        };
    }

    pub fn neighboring_points(&self) -> Vec<Point> {
        vec![
            (self.location.0 - 1, self.location.1 - 1),
            (self.location.0 - 1, self.location.1),
            (self.location.0 - 1, self.location.1 + 1),
            (self.location.0, self.location.1 - 1),
            (self.location.0, self.location.1 + 1),
            (self.location.0 + 1, self.location.1 - 1),
            (self.location.0 + 1, self.location.1),
            (self.location.0 + 1, self.location.1 + 1),
        ]
    }

    pub fn is_neighbor_of(&self, other: &Organism) -> bool {
        ((self.location.0 - other.location.0).abs() <= 1
            && (self.location.1 - other.location.1).abs() <= 1)
            && !(self.location.0 == other.location.0 && self.location.1 == other.location.1)
    }

    pub fn flip_status(&mut self) {
        self.status = match self.status {
            Status::ALIVE => Status::DEAD,
            Status::DEAD => Status::ALIVE,
        };
    }
}
