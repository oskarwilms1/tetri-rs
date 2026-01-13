use bevy::{
    ecs::{entity::Entity, event::Event, observer::On},
    prelude::*,
};

use crate::{
    board::{
        grid::Grid,
        grid_matrix::GridMatrix,
        tetrimino::{spawn_tetrimino, Tetrimino},
        tetrimino_square::TetriminoSquare,
    },
    plugins::assets_plugin::TetriminoAssets,
};
#[derive(Event)]
pub struct TetriminoPlaced;

pub fn handle_on_placed(
    _event: On<TetriminoPlaced>,
    mut commands: Commands,
    parent_query: Query<Entity, With<Grid>>,
    matrix_query: Query<(Entity, &Children, &mut GridMatrix), With<GridMatrix>>,
    child_query: Query<(Entity, &Transform, &Children), With<Tetrimino>>,
    squares_query: Query<
        (Entity, &MeshMaterial2d<ColorMaterial>, &Transform),
        With<TetriminoSquare>,
    >,
    tetrimino_assets: ResMut<TetriminoAssets>,
) {
    let (tetrimino, tetrimino_position, tetrimino_children) =
        child_query.single().expect("No tetrimino active");

    update_matrix(
        &mut commands,
        matrix_query,
        tetrimino_position.translation,
        tetrimino_children,
        squares_query,
    );
    handle_respawn(commands, tetrimino, parent_query, &tetrimino_assets);
}

fn handle_full_rows(
    commands: &mut Commands,
    matrix_entity: Entity,
    matrix: &mut GridMatrix,
    matrix_children: &Children,
) {
    let full_rows = matrix.check_full_rows();
    matrix.empty_rows(full_rows);
    for child in matrix_children.iter() {
        commands.entity(child).despawn();
    }

    matrix.spawn_cells(commands, matrix_entity, 0.0);
}

fn handle_respawn(
    mut commands: Commands,
    entity: Entity,
    parent_query: Query<Entity, With<Grid>>,
    tetrimino_assets: &ResMut<TetriminoAssets>,
) {
    let parent = parent_query.single().expect("Grid not found");
    commands.entity(entity).despawn();
    spawn_tetrimino(&mut commands, parent, &tetrimino_assets);
}
fn update_matrix(
    commands: &mut Commands,
    mut matrix_query: Query<(Entity, &Children, &mut GridMatrix), With<GridMatrix>>,
    tetrimino_position: Vec3,
    tetrimino_children: &Children,
    squares_query: Query<
        (Entity, &MeshMaterial2d<ColorMaterial>, &Transform),
        With<TetriminoSquare>,
    >,
) {
    let (matrix_entity, matrix_children, mut matrix) =
        matrix_query.single_mut().expect("GridMatrix not found");

    matrix.place_tetrimino(tetrimino_position, tetrimino_children, squares_query);
    let full_rows = matrix.check_full_rows();
    matrix.empty_rows(full_rows);
    for child in matrix_children.iter() {
        commands.entity(child).despawn();
    }

    matrix.spawn_cells(commands, matrix_entity, 0.0);
}
