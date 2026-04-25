mod consts;
mod screen;
mod structs;

use crate::consts::SPEED;
use crate::screen::fullscreen;
use crate::structs::cursor::Cursor;
use crate::structs::instructions::Instructions;
use crate::structs::organism::Status;
use crate::structs::world::World;
use macroquad::prelude::*;

async fn pregame_loop(world: &mut World) -> bool {
    let mut cursor = Cursor::default();
    let mut instructions = Instructions::default();
    let mut resume = true;
    let mut locked_status_for_mouse_move = Status::DEAD;
    let color = SKYBLUE;

    loop {
        world.draw();
        instructions.draw();

        let mut toggled = false;

        if is_mouse_button_pressed(MouseButton::Left) {
            cursor.move_to(mouse_position());
            toggled = true;

            locked_status_for_mouse_move = match world.get_organism_at_mouse_position() {
                Some(o) => !o.get_status(),
                None => Status::ALIVE,
            }
        } else if is_mouse_button_down(MouseButton::Left) {
            let prev_location = cursor.location.clone();
            cursor.move_to(mouse_position());
            toggled = cursor.location != prev_location;
        }

        if is_key_pressed(KeyCode::Enter) {
            break;
        } else if is_key_pressed(KeyCode::Escape) {
            resume = false;
            break;
        } else if is_key_pressed(KeyCode::H) {
            instructions.toggle_visibility();
        } else if is_key_pressed(KeyCode::C) {
            world.clear_population();
        }

        if toggled {
            world.set_organism_at(cursor.location, locked_status_for_mouse_move, color);
        }

        next_frame().await;
    }

    resume
}

async fn game_loop(world: &mut World) {
    let mut last_update = get_time();
    let mut paused = false;

    loop {
        world.draw();

        if is_key_pressed(KeyCode::Escape) {
            // Wait one more frame to clear keycodes before exiting.
            next_frame().await;
            break;
        } else if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        }

        if !paused && get_time() - last_update > SPEED {
            last_update = get_time();
            world.advance_generation();
        }

        next_frame().await;
    }
}

#[macroquad::main("Conway's Game of Life")]
async fn main() {
    fullscreen().await;
    let mut world = World::default();

    loop {
        let resume = pregame_loop(&mut world).await;
        let initial_population = world.clone_population();

        if resume {
            game_loop(&mut world).await;
        } else {
            break;
        }

        world = World::with_initial_population(initial_population);
    }
}
