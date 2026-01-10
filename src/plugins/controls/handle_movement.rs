#![allow(clippy::needless_pass_by_value)]
use crate::{
    board::{tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    config::grid::grid_config::CELL_SIZE,
    plugins::controls::{
        boundary_checks::corrected_translation, collision::check_collision, handle_input::Movement,
    },
};
use bevy::prelude::*;

pub fn handle_move(
    commands: &mut Commands,
    tetrimino_query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
    movement: Movement,
) {
    let movement_vec = match movement {
        Movement::Right => Vec3::new(CELL_SIZE, 0., 0.),
        Movement::Left => Vec3::new(-CELL_SIZE, 0., 0.),
        Movement::Down => Vec3::new(0., -CELL_SIZE, 0.),
    };
    let (entity, mut transform) = tetrimino_query.into_inner();
    let children = children_of.get(entity).unwrap();

    let child_positions: Vec<Vec3> = children
        .iter()
        .filter_map(|child| squares.get(child).ok())
        .map(|gt| gt.1.translation)
        .collect();

    let new_position =
        corrected_translation(transform.translation, &child_positions, &movement_vec);

    transform.translation = new_position;
    check_collision(commands, entity, new_position, &child_positions);
}
