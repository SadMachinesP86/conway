mod consts;
mod enums;
mod screen;
mod structs;

use crate::consts::SPEED;
use crate::enums::{status::Status, team::Team};
use crate::screen::fullscreen;
use crate::structs::cursor::Cursor;
use crate::structs::instructions::Instructions;
use crate::structs::world::World;
use macroquad::prelude::*;

async fn pregame_loop(world: &mut World) -> bool {
    let mut cursor = Cursor::default();
    let mut instructions = Instructions::default();
    let mut resume = true;

    loop {
        world.draw();
        cursor.draw();
        instructions.draw();

        let mut toggled = false;

        if is_mouse_button_pressed(MouseButton::Left) {
            cursor.move_to(mouse_position());
            toggled = true;

            cursor.set_status(match world.get_organism_at_mouse_position() {
                Some(o) => !o.get_status(),
                None => Status::ALIVE,
            });
        } else if is_mouse_button_down(MouseButton::Left) {
            let prev_location = cursor.get_location();
            cursor.move_to(mouse_position());
            toggled = cursor.get_location() != prev_location;
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
        } else if is_key_pressed(KeyCode::Key1) {
            cursor.set_team(Team::BLUE);
        } else if is_key_pressed(KeyCode::Key2) {
            cursor.set_team(Team::RED);
        } else if is_key_pressed(KeyCode::Key3) {
            cursor.set_team(Team::GOLD);
        } else if is_key_pressed(KeyCode::Key4) {
            cursor.set_team(Team::GREEN);
        }

        if toggled {
            world.set_organism_at(
                cursor.get_location(),
                cursor.get_status(),
                cursor.get_team(),
            );
        }

        next_frame().await;
    }

    resume
}

async fn game_loop(initial_world: World) {
    let mut last_update = get_time();
    let mut paused = false;
    let mut frame_number: usize = 0;
    let mut generations = vec![initial_world];

    loop {
        match generations.get(frame_number) {
            Some(x) => x.draw(),
            None => {
                let next_generation = generations.last().unwrap().next_generation();
                next_generation.draw();
                generations.push(next_generation);
            }
        }

        if is_key_pressed(KeyCode::Escape) {
            // Wait one more frame to clear keycodes before exiting.
            next_frame().await;
            break;
        } else if is_key_pressed(KeyCode::Space) {
            paused = !paused;
        } else if is_key_pressed(KeyCode::Left) {
            frame_number -= 1;
        } else if is_key_pressed(KeyCode::Right) {
            frame_number += 1;
        }

        if !paused && get_time() - last_update > SPEED {
            last_update = get_time();
            frame_number += 1;
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
            game_loop(world).await;
        } else {
            break;
        }

        world = World::with_initial_population(initial_population);
    }
}
