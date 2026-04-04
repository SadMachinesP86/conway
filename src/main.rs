mod screen;
mod structs;

use crate::screen::fullscreen;
use crate::structs::cursor::Cursor;
use crate::structs::instructions::Instructions;
use crate::structs::world::World;
use macroquad::prelude::*;

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    fullscreen().await;
    let mut world = World::default();

    // "Pre-game" - lets the user move the cursor around and assign starting organisms.
    let mut cursor = Cursor::default();
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
