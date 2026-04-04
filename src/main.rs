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

    resume
}

async fn game_loop(world: &mut World) {
    let mut last_update = get_time();

    loop {
        world.draw();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if get_time() - last_update > SPEED {
            last_update = get_time();
            world.advance_generation();
        }

        next_frame().await;
    }

    next_frame().await;
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
