mod consts;
mod screen;
mod structs;

use crate::consts::SPEED;
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

        let movement_scale = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift)
        {
            5
        } else {
            1
        };

        let mut did_move = false;

        if is_key_pressed(KeyCode::Enter) {
            break;
        } else if is_key_pressed(KeyCode::Escape) {
            resume = false;
            break;
        } else if is_key_pressed(KeyCode::Space) {
            world.flip_organism_at(cursor.location);
        } else if is_key_pressed(KeyCode::Up) {
            cursor.up(movement_scale);
            did_move = true;
        } else if is_key_pressed(KeyCode::Down) {
            cursor.down(movement_scale);
            did_move = true;
        } else if is_key_pressed(KeyCode::Left) {
            cursor.left(movement_scale);
            did_move = true;
        } else if is_key_pressed(KeyCode::Right) {
            cursor.right(movement_scale);
            did_move = true;
        } else if is_key_pressed(KeyCode::H) {
            instructions.toggle_visibility();
        } else if is_key_pressed(KeyCode::C) {
            world.clear_population();
        }

        if did_move && is_key_down(KeyCode::Space) {
            world.flip_organism_at(cursor.location);
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
