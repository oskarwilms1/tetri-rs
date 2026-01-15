use bevy::{ math::Vec3};

use crate::{
    board::grid_matrix::{ CellState, GridMatrix},
    config::grid::grid_config::{CELL_SIZE, ROW_AMOUNT}
};

pub fn check_tetrimino_collision(
    matrix: &GridMatrix,
    parent_position: Vec3,
    child_positions: &[Vec3],
    x_offset: f32,
    y_offset: f32,
) -> bool {
    for block in child_positions {
        let x_index: usize = ((parent_position.x + block.x).abs() / CELL_SIZE + x_offset) as usize;
        let y_index: usize = ((parent_position.y + block.y).abs() / CELL_SIZE + y_offset) as usize;
        if y_index == ROW_AMOUNT as usize{
            return true
        }
        if let Some(cell_state) = matrix.get_cell_state(x_index, y_index) && *cell_state==CellState::Full {
            return true
        }
    }
    false
}

pub fn check_lowest_collision(    
    matrix: &GridMatrix,
    parent_position: Vec3,
    child_positions: &[Vec3],
    
    ) -> f32{
    let mut y_offset :f32 = 0.;
    loop{
        if check_tetrimino_collision(matrix, parent_position, child_positions, 0., y_offset+1.){
            return y_offset * CELL_SIZE;
        }
        else{
            y_offset+=1.
        }
    }
}
