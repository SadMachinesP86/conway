pub mod consts;
pub mod cursor;
pub mod organism;
pub mod world;

use crate::consts::*;
use crate::cursor::Cursor;
use crate::world::World;
use macroquad::prelude::*;

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
