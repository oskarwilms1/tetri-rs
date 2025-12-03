use crate::{
    board::{tetrimino::tetrimino, tetrimino_square::TetriminoVariant},
    config::grid::grid_config::*,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Grid;

pub fn grid_background(
    cell_mesh: &Handle<Mesh>,
    cell_material: &Handle<ColorMaterial>,
) -> Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform)> {
    let mut result = Vec::with_capacity(ROW_AMOUNT as usize * COLUMN_AMOUNT as usize);

    for x in 0..COLUMN_AMOUNT as i32 {
        for y in 0..ROW_AMOUNT as i32 {
            result.push((
                Mesh2d(cell_mesh.clone()),
                MeshMaterial2d(cell_material.clone()),
                Transform::from_xyz(x as f32 * CELL_SIZE, -(y as f32) * CELL_SIZE, 0.)
                    .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS)),
            ));
        }
    }
    result
}

pub fn spawn_grid(
    commands: &mut Commands,
    cell_mesh: &Handle<Mesh>,
    background_cell_material: &Handle<ColorMaterial>,
    tetrimino_variant: TetriminoVariant,
    tetrinimo_i_material: &Handle<ColorMaterial>,
    x_offset: f32,
    y_offset: f32,
) {
    let parent = commands
        .spawn((
            Grid,
            Transform::from_xyz(x_offset, y_offset, 0.),
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .id();
    for child in grid_background(cell_mesh, background_cell_material) {
        commands.entity(parent).with_children(|parent| {
            parent.spawn(child);
        });
    }
    let tetrimino = commands
        .spawn(tetrimino(
            &tetrimino_variant,
            cell_mesh,
            tetrinimo_i_material,
            0.,
            0.,
        ))
        .id();
    commands.entity(parent).add_child(tetrimino);
}
