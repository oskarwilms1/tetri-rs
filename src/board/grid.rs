use crate::{
    board::grid_matrix::GridMatrix,
    config::grid::grid_config::{CELL_BORDER_THICKNESS, CELL_SIZE, COLUMN_AMOUNT, ROW_AMOUNT},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Grid;
#[derive(Component)]
pub struct GridBackground;

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
    x_offset: f32,
    y_offset: f32,
) -> Entity {
    let parent = commands
        .spawn((
            Grid,
            GridMatrix::new(COLUMN_AMOUNT as usize, ROW_AMOUNT as usize),
            Transform::from_xyz(x_offset, y_offset, 0.),
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .id();

    let background = commands
        .spawn((
            GridBackground,
            Transform::from_xyz(0., 0., 0.),
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .id();

    for child in grid_background(cell_mesh, background_cell_material) {
        commands.entity(background).with_children(|parent| {
            parent.spawn(child.clone());
        });
    }
    commands.entity(parent).add_child(background);
    parent
}
