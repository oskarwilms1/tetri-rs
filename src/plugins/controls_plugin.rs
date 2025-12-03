use crate::{
    board::{tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    config::grid::grid_config::{CELL_SIZE, COLUMN_AMOUNT},
};
use bevy::prelude::*;
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_movement);
        app.add_systems(Update, handle_rotation);
    }
}

pub fn handle_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    tetrimino_query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    squares: Query<&Transform, (With<TetriminoSquare>, Without<Tetrimino>)>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyA) && !keyboard_input.just_pressed(KeyCode::KeyD) {
        return;
    }
    let (tetrimino_entity, mut tetrimino_transform) = tetrimino_query.into_inner();

    let children = children_of.get(tetrimino_entity).unwrap();

    let child_positions: Vec<Vec3> = children
        .iter()
        .filter_map(|child| squares.get(child).ok())
        .map(|gt| gt.translation)
        .collect();

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        let movement = Vec3::new(-CELL_SIZE, 0., 0.);
        tetrimino_transform.translation =
            corrected_translation(tetrimino_transform.translation, &child_positions, &movement);
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        let movement = Vec3::new(CELL_SIZE, 0., 0.);
        tetrimino_transform.translation =
            corrected_translation(tetrimino_transform.translation, &child_positions, &movement);
    }
}

pub fn handle_rotation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    tetrimino_query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    mut squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyW) {
        return;
    }

    let (tetrimino_entity, mut tetrimino_transform) = tetrimino_query.into_inner();
    let children = children_of.get(tetrimino_entity).unwrap();

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
    println!("Before: {}", tetrimino_transform.translation);
    tetrimino_transform.translation = corrected_translation_rotation(
        tetrimino_transform.translation,
        &child_positions,
        &movement_vectors,
    );
    println!("After: {}", tetrimino_transform.translation);
}

fn corrected_translation(
    tetrimino_position: Vec3,
    children_positions: &[Vec3],
    movement_vector: &Vec3,
) -> Vec3 {
    let mut new_position: Vec3 = tetrimino_position + movement_vector;
    for child_position in children_positions.iter() {
        let adjusted_position = child_position + new_position;
        new_position.x += left_border_correction(adjusted_position.x);
        new_position.x += right_border_correction(adjusted_position.x);
    }

    new_position
}

fn corrected_translation_rotation(
    tetrimino_position: Vec3,
    children_positions: &[Vec3],
    movement_vectors: &[Vec3],
) -> Vec3 {
    let mut new_position: Vec3 = tetrimino_position;

    for (i, &movement_vector) in movement_vectors.iter().enumerate() {
        let child_position = children_positions[i];
        let adjusted_position = new_position + child_position + movement_vector;

        new_position.x += left_border_correction(adjusted_position.x);
        new_position.x += right_border_correction(adjusted_position.x);
    }

    new_position
}

fn left_border_correction(x: f32) -> f32 {
    if x < -CELL_SIZE {
        return 2. * CELL_SIZE;
    }
    if x < 0. {
        return CELL_SIZE;
    }
    0.
}
fn right_border_correction(x: f32) -> f32 {
    if x > CELL_SIZE * COLUMN_AMOUNT {
        return -2. * CELL_SIZE;
    }
    if x > CELL_SIZE * (COLUMN_AMOUNT - 1.) {
        return -CELL_SIZE;
    }
    0.
}
