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

impl Status {
    pub fn default() -> Status {
        Status::DEAD
    }
}
