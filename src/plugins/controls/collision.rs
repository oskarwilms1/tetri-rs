use bevy::{ecs::system::Commands, math::Vec3};

use crate::{
    board::grid_matrix::{ CellState, GridMatrix},
    config::grid::grid_config::{CELL_SIZE, ROW_AMOUNT},
    plugins::observers_plugin::TetriminoPlaced,
};

pub fn check_collision(
    commands: &mut Commands,
    tetrimino_translation: Vec3,
    child_positions: &[Vec3],
) -> bool {
    for position in child_positions {
        let adjusted_position = tetrimino_translation + position;
        if adjusted_position.y == -CELL_SIZE * (ROW_AMOUNT - 1.0) {
            commands.trigger(TetriminoPlaced);
            return true;
        }
    }
    false
}

pub fn check_tetrimino_collision(
    matrix: &GridMatrix,
    parent_position: Vec3,
    child_positions: &[Vec3],
    x_offset: f32,
    y_offset: f32,
) -> bool {
    for block in child_positions {
        //These indexes are for checking the positions of blocks that may collide 
        let x_index: usize = ((parent_position.x + block.x).abs() / CELL_SIZE + x_offset) as usize;
        let y_index: usize = ((parent_position.y + block.y).abs() / CELL_SIZE + y_offset) as usize;
        if let Some(cell_state) = matrix.get_cell_state(x_index, y_index) && *cell_state==CellState::Full {
            return true
        }
    }
    false
}
