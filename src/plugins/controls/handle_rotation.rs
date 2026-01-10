#![allow(clippy::needless_pass_by_value)]
use crate::{
    board::{tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    plugins::controls::boundary_checks::corrected_translation_rotation,
};
use bevy::prelude::*;

pub fn handle_rotate(
    query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    mut squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
) {
    let (entity, mut transform) = query.into_inner();
    let children = children_of.get(entity).unwrap();

    let child_positions: Vec<Vec3> = children
        .iter()
        .filter_map(|child| squares.get(child).ok())
        .map(|gt| gt.1.translation)
        .collect();

    let mut movement_vectors = Vec::new();

    for child in children.iter() {
        if let Ok((mut square, mut tf)) = squares.get_mut(child) {
            let square_position = square.get_rotation();
            square.rotate();
            let movement_vector = square.get_rotation().translation - square_position.translation;
            *tf = square.get_rotation();
            movement_vectors.push(movement_vector);
        }
    }
    transform.translation =
        corrected_translation_rotation(transform.translation, &child_positions, &movement_vectors);
}
