use macroquad::prelude::*;

const SIZE: i16 = 16;

type Point = (i16, i16);

#[derive(Clone, Copy)]
enum Status {
    ALIVE,
    DEAD,
}

#[derive(Clone, Copy)]
struct Organism {
    location: Point,
    status: Status,
}

struct World {
    population: Vec<Organism>,
}

impl World {
    pub fn get_population(&self) -> &Vec<Organism> {
        &self.population
    }

    // Returns the organism at the provided location, either by retrieving it from the existing population, or creating
    //   a new (dead) one.
    pub fn organism_at(&mut self, x: i16, y: i16) -> &Organism {
        if self.get_organism_at(x, y).is_none() {
            self.create_organism_at(x, y, Status::DEAD);
        }

        self.population
            .iter()
            .find(|o| o.location == (x, y))
            .unwrap()
    }

    // Looks up the organism at the provided location.
    pub fn get_organism_at(&mut self, x: i16, y: i16) -> Option<&Organism> {
        self.population.iter().find(|o| o.location == (x, y))
    }

    // Creates a new organism at the provided location.
    pub fn create_organism_at(&mut self, x: i16, y: i16, status: Status) -> &Organism {
        self.population.push(Organism {
            location: (x, y),
            status,
        });
        self.population.last().unwrap()
    }

    pub fn clear_dead(&mut self) {
        self.population.retain_mut(|o| match o.status {
            Status::ALIVE => true,
            Status::DEAD => false,
        });
    }

    pub fn infill_neighbors(&mut self) {
        let points_of_existing_organisms = self.population.clone().into_iter().map(|o| o.location);

        for point in points_of_existing_organisms {
            let _ = self.organism_at(point.0 - 1, point.1 - 1);
        }

        ()
    }
}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    let mut world = World {
        population: Vec::new(),
    };

    world.create_organism_at(3, 2, Status::ALIVE);
    world.create_organism_at(4, 3, Status::ALIVE);
    world.create_organism_at(2, 4, Status::ALIVE);
    world.create_organism_at(3, 4, Status::ALIVE);
    world.create_organism_at(4, 4, Status::ALIVE);

    // "Pre-game" - lets the user move the cursor around and assign starting organisms.
    loop {
        clear_background(WHITE);

        for organism in world.get_population() {
            draw_rectangle(
                (organism.location.0 * SIZE) as f32,
                (organism.location.1 * SIZE) as f32,
                SIZE as f32,
                SIZE as f32,
                BLACK,
            );
        }

        next_frame().await;
    }
}
