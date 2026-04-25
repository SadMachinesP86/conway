use super::organism::{Organism, Status};
use crate::consts::*;
use crate::structs::point::Point;
use crate::{BLACK, clear_background, draw_rectangle, mouse_position};
use std::collections::HashMap;

pub struct World {
    population: HashMap<Point, Organism>,
}

impl World {
    pub fn default() -> World {
        let mut world = World {
            population: HashMap::new(),
        };

        world.prepare_sample();
        world
    }

    pub fn with_initial_population(initial_population: HashMap<Point, Organism>) -> World {
        World {
            population: initial_population,
        }
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        for organism in self.get_population() {
            draw_rectangle(
                (organism.0.0 * SCALE) as f32,
                (organism.0.1 * SCALE) as f32,
                SCALE as f32,
                SCALE as f32,
                organism.1.get_color(),
            );
        }
    }

    pub fn get_population(&self) -> &HashMap<Point, Organism> {
        &self.population
    }

    pub fn clone_population(&self) -> HashMap<Point, Organism> {
        self.population.clone()
    }

    // Returns the organism at the provided location, either by retrieving it from the existing population, or creating
    //   a new (default) one.
    pub fn organism_at(&mut self, point: Point, team: Option<Team>) -> &mut Organism {
        if self.get_organism_at(point).is_none() {
            match team {
                Some(team) => self.create_organism_at(point, Status::DEAD, team),
                None => self.create_default_organism_at(point),
            }
        }

        self.get_organism_at(point).unwrap()
    }

    // Looks up the organism at the provided location.
    pub fn get_organism_at(&mut self, point: Point) -> Option<&mut Organism> {
        self.population.get_mut(&point)
    }

    pub fn get_organism_at_mouse_position(&mut self) -> Option<&mut Organism> {
        let mouse_position = mouse_position();

        self.get_organism_at(Point(
            (mouse_position.0 as i16) / SCALE,
            (mouse_position.1 as i16) / SCALE,
        ))
    }

    // Creates a new organism at the provided location.
    pub fn create_organism_at(&mut self, point: Point, status: Status, team: Team) {
        self.population.insert(point, Organism::new(status, team));
    }

    // Creates a new default organism at the provided location.
    pub fn create_default_organism_at(&mut self, point: Point) {
        self.population.insert(point, Organism::default());
    }

    pub fn advance_generation(&mut self) {
        let previous_population = self.population.clone();

        // Infill dead organisms at neighboring points for all current organisms.
        for previous_organism in previous_population.iter() {
            for point in previous_organism.0.neighboring_points() {
                self.organism_at(point, None);
            }
        }

        // Count each organism's live neighbors, and update their status accordingly.
        for organism in self.population.iter_mut() {
            let live_neighbors: Vec<&Organism> = organism
                .0
                .neighboring_points()
                .iter()
                .filter_map(|o| match previous_population.get(o) {
                    Some(o) => {
                        if o.get_status() == Status::ALIVE {
                            Some(o)
                        } else {
                            None
                        }
                    }
                    None => None,
                })
                .collect();

            let mut neighbor_counts: HashMap<Team, usize> = HashMap::new();

            for organism in live_neighbors {
                neighbor_counts.insert(
                    organism.get_team(),
                    match neighbor_counts.get(&organism.get_team()) {
                        Some(x) => x + 1,
                        None => 1,
                    },
                );
            }

            organism.1.set_status_for_neighbor_count(neighbor_counts);
        }

        self.clear_dead();
    }

    pub fn clear_dead(&mut self) {
        self.population
            .retain(|_p, o| o.get_status() == Status::ALIVE);
    }

    pub fn set_organism_at(&mut self, point: Point, status: Status, team: Team) {
        let organism = self.organism_at(point, Some(team));
        organism.set_status(status);
        organism.set_team(team);
        self.clear_dead();
    }

    pub fn prepare_sample(&mut self) {
        self.create_organism_at(Point(3, 2), Status::ALIVE, Team::BLUE);
        self.create_organism_at(Point(4, 3), Status::ALIVE, Team::BLUE);
        self.create_organism_at(Point(2, 4), Status::ALIVE, Team::BLUE);
        self.create_organism_at(Point(3, 4), Status::ALIVE, Team::BLUE);
        self.create_organism_at(Point(4, 4), Status::ALIVE, Team::BLUE);
    }

    pub fn clear_population(&mut self) {
        self.population = HashMap::new();
    }
}
