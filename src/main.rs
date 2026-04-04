pub mod consts;
pub mod cursor;
pub mod organism;

use crate::consts::*;
use crate::cursor::Cursor;
use crate::organism::{Organism, Status};
use macroquad::prelude::*;

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
        clear_background(BLACK);

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

struct Instructions {
    instructions: Vec<String>,
    font_size: f32,
    visibility: bool,
}

impl Instructions {
    fn default() -> Instructions {
        Instructions {
            instructions: vec![
                "Instructions: Set the starting population, then run the simulation.".to_owned(),
                "Use arrow keys to move the cursor. Press [Space] to flip the cell at the cursor."
                    .to_owned(),
                "Press [H] to toggle instructions.".to_owned(),
                "Press [Enter] to run the game. Press [Escape] to quit.".to_owned(),
            ],
            font_size: FONT_SIZE as f32,
            visibility: true,
        }
    }

    fn draw(&self) {
        if !self.visibility {
            return;
        }

        let mut offset = 0.;

        for text in self.instructions.iter() {
            let text_size = measure_text(text, None, self.font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2. + offset,
                self.font_size,
                LIGHTGRAY,
            );

            offset += self.font_size;
        }
    }

    fn toggle_visibility(&mut self) {
        self.visibility = !self.visibility
    }

    fn hide(&mut self) {
        self.visibility = false
    }
}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    set_fullscreen(true);
    let mut world = World::default();

    // "Pre-game" - lets the user move the cursor around and assign starting organisms.
    let mut cursor = Cursor { location: (8, 8) };
    let mut instructions = Instructions::default();
    let mut resume = true;

    loop {
        world.draw();
        cursor.draw();
        instructions.draw();

        if is_key_pressed(KeyCode::Enter) {
            break;
        } else if is_key_pressed(KeyCode::Escape) {
            resume = false;
            break;
        } else if is_key_pressed(KeyCode::Space) {
            world.flip_organism_at(cursor.location);
        } else if is_key_pressed(KeyCode::Up) {
            cursor.up();
        } else if is_key_pressed(KeyCode::Down) {
            cursor.down();
        } else if is_key_pressed(KeyCode::Left) {
            cursor.left();
        } else if is_key_pressed(KeyCode::Right) {
            cursor.right();
        } else if is_key_pressed(KeyCode::H) {
            instructions.toggle_visibility();
        }

        next_frame().await;
    }

    if resume {
        // Live game
        instructions.hide();
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
}
