use crate::config::{
    grid::grid_config::*, tetrimino::static_cells::tetrimino_i_cells::TETRINIMO_I_CELLS,
};
use bevy::{color::Color, prelude::*};
use std::collections::HashMap;

#[derive(Component)]
pub enum TetriminoVariant {
    I,
    //O,
    //T,
    //S,
    //Z,
    //J,
    //L,
}
#[derive(Component)]
pub struct Tetrimino;

pub struct TetriminoData {
    cells: HashMap<i32, [Vec3; 4]>,
    color: ColorMaterial,
}

pub fn tetrimino_data(variant: TetriminoVariant) -> TetriminoData {
    match variant {
        TetriminoVariant::I => TetriminoData {
            cells: TETRINIMO_I_CELLS.clone(),
            color: ColorMaterial::from_color(Color::BLACK),
        },
    }
}

pub fn tetrimino(
    variant: TetriminoVariant,
    cell_mesh: Handle<Mesh>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    x: f32,
    y: f32,
) -> impl Bundle {
    let tetrimino_data = tetrimino_data(variant);
    let cell_color = materials.add(tetrimino_data.color);
    let rotation_state: i32 = 0_i32;
    let cell_positions: [Vec3; 4] = match tetrimino_data.cells.get(&rotation_state) {
        Some(position) => *position,
        None => panic!("Cell variant is not valid!"),
    };
    (
        (
            Tetrimino,
            Transform::from_xyz(x, y, 1.),
            Visibility::Inherited,
            InheritedVisibility::default(),
        ),
        children![
            (
                Mesh2d(cell_mesh.clone()),
                MeshMaterial2d(cell_color.clone()),
                Transform::from_translation(cell_positions[0] * CELL_SIZE)
                    .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS))
            ),
            (
                Mesh2d(cell_mesh.clone()),
                MeshMaterial2d(cell_color.clone()),
                Transform::from_translation(cell_positions[1] * CELL_SIZE)
                    .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS))
            ),
            (
                Mesh2d(cell_mesh.clone()),
                MeshMaterial2d(cell_color.clone()),
                Transform::from_translation(cell_positions[2] * CELL_SIZE)
                    .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS))
            ),
            (
                Mesh2d(cell_mesh.clone()),
                MeshMaterial2d(cell_color.clone()),
                Transform::from_translation(cell_positions[3] * CELL_SIZE)
                    .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS))
            )
        ],
    )
}
