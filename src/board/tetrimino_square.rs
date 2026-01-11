use bevy::prelude::*;

use crate::config::{
    grid::grid_config::{CELL_BORDER_THICKNESS, CELL_SIZE},
    tetrimino::static_cells::{
        tetrimino_i_cells::TETRIMINO_I_CELLS, tetrimino_j_cells::TETRIMINO_J_CELLS,
        tetrimino_l_cells::TETRIMINO_L_CELLS, tetrimino_o_cells::TETRIMINO_O_CELLS,
        tetrimino_s_cells::TETRIMINO_S_CELLS, tetrimino_t_cells::TETRIMINO_T_CELLS,
        tetrimino_z_cells::TETRIMINO_Z_CELLS,
    },
};
#[repr(usize)]
#[derive(Copy, Clone, Debug)]
pub enum TetriminoVariant {
    I = 0,
    O = 1,
    T = 2,
    S = 3,
    Z = 4,
    J = 5,
    L = 6,
}
pub fn tetrimino_cell_data(variant: &TetriminoVariant) -> [[Vec3; 4]; 4] {
    match variant {
        TetriminoVariant::I => TETRIMINO_I_CELLS,
        TetriminoVariant::O => TETRIMINO_O_CELLS,
        TetriminoVariant::T => TETRIMINO_T_CELLS,
        TetriminoVariant::S => TETRIMINO_S_CELLS,
        TetriminoVariant::Z => TETRIMINO_Z_CELLS,
        TetriminoVariant::J => TETRIMINO_J_CELLS,
        TetriminoVariant::L => TETRIMINO_L_CELLS,
    }
}

#[derive(Component)]
pub struct TetriminoSquare {
    pub child_id: usize,
    rotation: usize,
    next_rotation: usize,
    cells: [[Vec3; 4]; 4],
}
impl TetriminoSquare {
    pub fn new(child_id: usize, cells: [[Vec3; 4]; 4]) -> Self {
        Self {
            child_id,
            rotation: 0,
            next_rotation: 1,
            cells,
        }
    }
    pub fn rotate(&mut self) {
        if self.rotation < 3 {
            self.rotation += 1;
        } else {
            self.rotation = 0;
        }
        if self.next_rotation < 3 {
            self.next_rotation += 1;
        } else {
            self.next_rotation = 0;
        }
    }
    pub fn get_rotation(&self) -> Transform {
        Transform::from_translation(self.cells[self.rotation][self.child_id] * CELL_SIZE)
            .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS))
    }
    pub fn get_next_rotation(&self) -> Transform {
        Transform::from_translation(self.cells[self.next_rotation][self.child_id] * CELL_SIZE)
            .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS))
    }
}

#[derive(Bundle)]
pub struct TetriminoSquareBundle {
    pub tetrimino_square: TetriminoSquare,
    pub cell_mesh: Mesh2d,
    cell_color: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
}

impl TetriminoSquareBundle {
    pub fn new(
        variant: &TetriminoVariant,
        cell_mesh: &Handle<Mesh>,
        cell_color: &Handle<ColorMaterial>,
        rotation: usize,
        child_id: usize,
    ) -> Self {
        let cells = tetrimino_cell_data(variant);
        let tetrimino_square = TetriminoSquare::new(child_id, cells);
        Self {
            tetrimino_square,
            cell_mesh: Mesh2d(cell_mesh.clone()),
            cell_color: MeshMaterial2d(cell_color.clone()),
            transform: Transform::from_translation(cells[rotation][child_id] * CELL_SIZE)
                .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS)),
        }
    }
}

pub fn spawn_tetrinimo_children(
    variant: &TetriminoVariant,
    cell_mesh: &Handle<Mesh>,
    cell_color: &Handle<ColorMaterial>,
) -> impl Bundle {
    children![
        TetriminoSquareBundle::new(variant, cell_mesh, cell_color, 0, 0),
        TetriminoSquareBundle::new(variant, cell_mesh, cell_color, 0, 1),
        TetriminoSquareBundle::new(variant, cell_mesh, cell_color, 0, 2),
        TetriminoSquareBundle::new(variant, cell_mesh, cell_color, 0, 3),
    ]
}
