#![allow(clippy::needless_pass_by_value)]
use bevy::{
    ecs::{
        entity::Entity,
        hierarchy::Children,
        query::{With, Without},
        system::{Commands, Query, Res, Single},
    },
    input::{keyboard::KeyCode, ButtonInput},
    transform::components::Transform,
};

use crate::{
    board::{grid_matrix::GridMatrix, tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    game::game_state::GameState,
    plugins::{
        controls::{handle_movement::handle_move, handle_rotation::handle_rotate},
        observers::handle_on_restart::Restart,
    },
};

pub enum Movement {
    Left,
    Right,
    Down,
}

pub fn handle_input(
    game_state: Res<GameState>,
    mut commands: Commands,
    tetrimino_query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
    grid_matrix: Query<&GridMatrix>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if *game_state == GameState::GameOver {
        if input.just_pressed(KeyCode::KeyR) {
            commands.trigger(Restart);
        }
        return;
    }
    let mut movement: Option<Movement> = None;

    if let Some(key) = input.get_just_pressed().next() {
        match key {
            KeyCode::KeyW => {
                handle_rotate(grid_matrix, tetrimino_query, children_of, squares);
                return;
            }
            KeyCode::KeyA => movement = Some(Movement::Left),
            KeyCode::KeyD => movement = Some(Movement::Right),
            KeyCode::KeyS => movement = Some(Movement::Down),
            _ => {}
        }
    }

    if let Some(movement) = movement {
        handle_move(
            &mut commands,
            tetrimino_query,
            children_of,
            squares,
            movement,
            grid_matrix,
        );
    }
}
