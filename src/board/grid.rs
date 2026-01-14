use crate::{
    board::grid_matrix::GridMatrix,
    config::grid::grid_config::{COLUMN_AMOUNT, ROW_AMOUNT},
    plugins::startup_plugin::GameEntity,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Grid;

pub fn spawn_grid(
    commands: &mut Commands,
    cell_mesh: &Handle<Mesh>,
    background_cell_material: &Handle<ColorMaterial>,
    x_offset: f32,
    y_offset: f32,
) -> Entity {
    let grid_entity = commands
        .spawn((
            Grid,
            GameEntity,
            Transform::from_xyz(x_offset, y_offset, 0.),
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .id();

    let grid_matrix_data = GridMatrix::new(
        COLUMN_AMOUNT as usize,
        ROW_AMOUNT as usize,
        cell_mesh.clone(),
        background_cell_material.clone(),
    );

    let grid_matrix_entity = commands
        .spawn((
            grid_matrix_data.clone(),
            Transform::default(),
            Visibility::default(),
            InheritedVisibility::default(),
        ))
        .id();

    commands.entity(grid_entity).add_child(grid_matrix_entity);

    grid_matrix_data.spawn_cells(commands, grid_matrix_entity, 0.0);

    grid_entity
}
