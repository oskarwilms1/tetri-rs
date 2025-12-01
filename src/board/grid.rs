use crate::config::grid::grid_config::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Grid;

pub fn grid_background(
    cell_mesh: &Handle<Mesh>,
    cell_material: &Handle<ColorMaterial>,
) -> Vec<(Mesh2d, MeshMaterial2d<ColorMaterial>, Transform)> {
    let mut result = Vec::with_capacity(ROW_AMOUNT * COLUMN_AMOUNT);

    for x in 0..COLUMN_AMOUNT {
        for y in 0..ROW_AMOUNT {
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
    cell_material: &Handle<ColorMaterial>,
    x_offset: f32,
    y_offset: f32,
) {
    let parent = commands
        .spawn((
            Grid,
            Transform::from_xyz(x_offset, y_offset, 0.),
            InheritedVisibility::default(),
        ))
        .id();
    for child in grid_background(cell_mesh, cell_material) {
        commands.entity(parent).with_children(|parent| {
            parent.spawn(child);
        });
    }
}
