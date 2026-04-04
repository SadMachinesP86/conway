use macroquad::prelude::*;

const SIZE: i16 = 16;

type Point = (i16, i16);

#[derive(Clone, PartialEq)]
enum Status {
    ALIVE,
    DEAD,
}

#[derive(Clone)]
struct Organism {
    location: Point,
    status: Status,
}

impl Organism {
    pub fn draw(&self) {
        draw_rectangle(
            (self.location.0 * SIZE) as f32,
            (self.location.1 * SIZE) as f32,
            SIZE as f32,
            SIZE as f32,
            BLACK,
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

struct World {
    population: Vec<Organism>,
}

impl World {
    pub fn default() -> World {
        let mut world = World {
            population: Vec::new(),
        };

        world.prepare_sample();
        world
    }

    pub fn draw(&self) {
        clear_background(WHITE);

        for organism in self.get_population() {
            organism.draw();
        }
    }

    pub fn get_population(&self) -> &Vec<Organism> {
        &self.population
    }

    // Returns the organism at the provided location, either by retrieving it from the existing population, or creating
    //   a new (dead) one.
    pub fn organism_at(&mut self, point: Point) -> &mut Organism {
        if self.get_organism_at(point).is_none() {
            self.create_organism_at(point, Status::DEAD);
        }

        self.population
            .iter_mut()
            .find(|o| o.location == point)
            .unwrap()
    }

    // Looks up the organism at the provided location.
    pub fn get_organism_at(&mut self, point: Point) -> Option<&mut Organism> {
        self.population.iter_mut().find(|o| o.location == point)
    }

    // Creates a new organism at the provided location.
    pub fn create_organism_at(&mut self, point: Point, status: Status) -> &mut Organism {
        self.population.push(Organism {
            location: (point),
            status,
        });
        self.population.last_mut().unwrap()
    }

    pub fn advance_generation(&mut self) {
        let previous_population = self.population.clone();

        // Infill dead organisms at neighboring points for all current organisms.
        for previous_organism in previous_population.iter() {
            for point in previous_organism.neighboring_points() {
                let _ = self.organism_at(point);
            }
        }

        // Count each organism's live neighbors, and update their status accordingly.
        for organism in self.population.iter_mut() {
            let live_neighbors = previous_population
                .iter()
                .filter(|o| o.status == Status::ALIVE && o.is_neighbor_of(organism))
                .count();

            organism.set_status_for_neighbor_count(live_neighbors);
        }

        self.clear_dead();
    }

    pub fn clear_dead(&mut self) {
        self.population.retain_mut(|o| match o.status {
            Status::ALIVE => true,
            Status::DEAD => false,
        });
    }

    pub fn flip_organism_at(&mut self, point: Point) {
        self.organism_at(point).flip_status();
        self.clear_dead();
    }

    pub fn prepare_sample(&mut self) {
        self.create_organism_at((3, 2), Status::ALIVE);
        self.create_organism_at((4, 3), Status::ALIVE);
        self.create_organism_at((2, 4), Status::ALIVE);
        self.create_organism_at((3, 4), Status::ALIVE);
        self.create_organism_at((4, 4), Status::ALIVE);
    }
}

struct Cursor {
    location: Point,
}

impl Cursor {
    fn draw(&self) {
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

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut world = World::default();

    // "Pre-game" - lets the user move the cursor around and assign starting organisms.
    let mut cursor = Cursor { location: (8, 8) };

    loop {
        world.draw();
        cursor.draw();

        if is_key_pressed(KeyCode::Enter) {
            break;
        } else if is_key_pressed(KeyCode::Space) {
            world.flip_organism_at(cursor.location);
        } else if is_key_pressed(KeyCode::Up) {
            cursor.up()
        } else if is_key_pressed(KeyCode::Down) {
            cursor.down()
        } else if is_key_pressed(KeyCode::Left) {
            cursor.left()
        } else if is_key_pressed(KeyCode::Right) {
            cursor.right()
        }

        next_frame().await;
    }

    // Live game
    let speed = 0.3;
    let mut last_update = get_time();

    loop {
        world.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if get_time() - last_update > speed {
            last_update = get_time();
            world.advance_generation();
        }

        next_frame().await;
    }
}
