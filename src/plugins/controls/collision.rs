use bevy::math::Vec3;

use crate::config::grid::grid_config::{CELL_SIZE, ROW_AMOUNT};

pub fn check_collision(tetrimino_translation: Vec3, child_positions: &[Vec3]) -> bool {
    for position in child_positions {
        let adjusted_position = tetrimino_translation + position;
        if adjusted_position.y == -CELL_SIZE * (ROW_AMOUNT - 1.0) {
            return true;
        }
    }
    false
}
