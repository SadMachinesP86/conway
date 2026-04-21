use std::ops::Not;

#[derive(Clone, PartialEq, Copy)]
pub enum Status {
    ALIVE,
    DEAD,
}

impl Not for Status {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Status::ALIVE => Status::DEAD,
            Status::DEAD => Status::ALIVE,
        }
    }
}

#[derive(Clone)]
pub struct Organism {
    status: Status,
}

impl Organism {
    pub fn new(status: Status) -> Organism {
        Organism { status }
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

    pub fn flip_status(&mut self) {
        self.status = !self.status;
    }

    pub fn get_status(&self) -> Status {
        self.status
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }
}
