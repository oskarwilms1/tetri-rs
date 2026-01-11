#![allow(clippy::needless_pass_by_value)]
use crate::{
    board::{grid_matrix::GridMatrix, tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    plugins::controls::{
        boundary_checks::corrected_translation_rotation, collision::check_tetrimino_collision,
    },
};
use bevy::prelude::*;

pub fn handle_rotate(
    grid_matrix: Query<&GridMatrix>,
    query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    mut squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
) {
    let (entity, mut transform) = query.into_inner();
    let children = children_of.get(entity).unwrap();
    let matrix = grid_matrix.single().unwrap();

    let child_positions: Vec<Vec3> = children
        .iter()
        .filter_map(|child| squares.get(child).ok())
        .map(|gt| gt.1.translation)
        .collect();

    let mut movement_vectors = Vec::new();

    for child in children.iter() {
        if let Ok((square, _transform)) = squares.get_mut(child) {
            movement_vectors.push(square.get_next_rotation().translation);
        }
    }
    if check_tetrimino_collision(matrix, transform.translation, &movement_vectors, 0., 0.) {
        return;
    }
    movement_vectors.clear();
    for child in children.iter() {
        if let Ok((mut square, mut transform)) = squares.get_mut(child) {
            let position = square.get_rotation();
            let next_position = square.get_next_rotation();
            let movement_vector = next_position.translation - position.translation;
            square.rotate();
            *transform = next_position;
            movement_vectors.push(movement_vector);
        }
    }
    transform.translation =
        corrected_translation_rotation(transform.translation, &child_positions, &movement_vectors);
}
