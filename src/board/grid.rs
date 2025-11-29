use crate::config::grid::grid_config::*;
use bevy::prelude::*;

pub fn empty_grid(
    cell_mesh: &Handle<Mesh>,
    cell_material: &Handle<ColorMaterial>,
    x_offset: f32,
    y_offset: f32,
) -> Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform)> {
    let mut result = Vec::with_capacity(ROW_AMOUNT * COLUMN_AMOUNT);

    for x in 0..COLUMN_AMOUNT {
        for y in 0..ROW_AMOUNT {
            result.push((
                Mesh2d(cell_mesh.clone()),
                MeshMaterial2d(cell_material.clone()),
                Transform::from_xyz(
                    x_offset + x as f32 * CELL_SIZE,
                    y_offset - y as f32 * CELL_SIZE,
                    0.,
                )
                .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS)),
            ));
        }
    }
    result
}
