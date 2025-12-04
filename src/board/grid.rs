use crate::{
    board::{tetrimino::tetrimino, tetrimino_square::TetriminoVariant},
    config::grid::grid_config::{CELL_BORDER_THICKNESS, CELL_SIZE, COLUMN_AMOUNT, ROW_AMOUNT},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Grid;

pub fn grid_background(
    cell_mesh: &Handle<Mesh>,
    cell_material: &Handle<ColorMaterial>,
) -> Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform)> {
    let mut result = Vec::new();

    let mut x: f32 = 0.0;
    while x < COLUMN_AMOUNT {
        let mut y: f32 = 0.0;
        while y < ROW_AMOUNT {
            result.push((
                Mesh2d(cell_mesh.clone()),
                MeshMaterial2d(cell_material.clone()),
                Transform::from_xyz(x * CELL_SIZE, -y * CELL_SIZE, 0.)
                    .with_scale(Vec3::splat(CELL_SIZE - CELL_BORDER_THICKNESS)),
            ));
            y += 1.0;
        }
        x += 1.0;
    }
    result
}

pub fn spawn_grid(
    commands: &mut Commands,
    cell_mesh: &Handle<Mesh>,
    background_cell_material: &Handle<ColorMaterial>,
    tetrimino_variant: &TetriminoVariant,
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
            tetrimino_variant,
            cell_mesh,
            tetrinimo_i_material,
            3. * CELL_SIZE,
            0.,
        ))
        .id();
    commands.entity(parent).add_child(tetrimino);
}
