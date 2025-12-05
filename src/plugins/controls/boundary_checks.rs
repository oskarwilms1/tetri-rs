use crate::config::grid::grid_config::{CELL_SIZE, COLUMN_AMOUNT, ROW_AMOUNT};
use bevy::prelude::Vec3;

pub fn corrected_translation(
    tetrimino_position: Vec3,
    children_positions: &[Vec3],
    movement_vector: &Vec3,
) -> Vec3 {
    let mut new_position: Vec3 = tetrimino_position + movement_vector;
    for child_position in children_positions {
        let adjusted_position = child_position + new_position;
        new_position.x += left_border_correction(adjusted_position.x);
        new_position.x += right_border_correction(adjusted_position.x);
        new_position.y += down_border_correction(adjusted_position.y);
    }

    new_position
}
pub fn corrected_translation_rotation(
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
        new_position.y += down_border_correction(adjusted_position.y);
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
fn down_border_correction(y: f32) -> f32 {
    if y < -CELL_SIZE * ROW_AMOUNT {
        return 2. * CELL_SIZE;
    }
    if y < -CELL_SIZE * (ROW_AMOUNT - 1.) {
        return CELL_SIZE;
    }

    0.
}
