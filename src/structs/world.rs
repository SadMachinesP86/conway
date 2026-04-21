use super::organism::{Organism, Status};
use crate::consts::*;
use crate::{BLACK, clear_background, mouse_position};

pub struct World {
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

    pub fn with_initial_population(initial_population: Vec<Organism>) -> World {
        World {
            population: initial_population,
        }
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

    pub fn clone_population(&self) -> Vec<Organism> {
        self.population.clone()
    }

    // Returns the organism at the provided location, either by retrieving it from the existing population, or creating
    //   a new (dead) one.
    pub fn organism_at(&mut self, point: Point) -> &mut Organism {
        if self.get_organism_at(point).is_none() {
            self.create_organism_at(point, Status::DEAD);
        }

        self.get_organism_at(point).unwrap()
    }

    // Looks up the organism at the provided location.
    pub fn get_organism_at(&mut self, point: Point) -> Option<&mut Organism> {
        self.population.iter_mut().find(|o| o.location == point)
    }

    pub fn get_organism_at_mouse_position(&mut self) -> Option<&mut Organism> {
        let mouse_position = mouse_position();

        self.population.iter_mut().find(|o| {
            o.location
                == (
                    (mouse_position.0 as i16) / SCALE,
                    (mouse_position.1 as i16) / SCALE,
                )
        })
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
        self.population.retain_mut(|o| o.status == Status::ALIVE);
    }

    pub fn flip_organism_at(&mut self, point: Point) {
        self.organism_at(point).flip_status();
        self.clear_dead();
    }

    pub fn set_organism_at(&mut self, point: Point, status: Status) {
        self.organism_at(point).set_status(status);
        self.clear_dead();
    }

    pub fn prepare_sample(&mut self) {
        self.create_organism_at((3, 2), Status::ALIVE);
        self.create_organism_at((4, 3), Status::ALIVE);
        self.create_organism_at((2, 4), Status::ALIVE);
        self.create_organism_at((3, 4), Status::ALIVE);
        self.create_organism_at((4, 4), Status::ALIVE);
    }

    pub fn clear_population(&mut self) {
        self.population = Vec::new();
    }
}
