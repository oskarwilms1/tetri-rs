#![allow(clippy::needless_pass_by_value)]

use bevy::{
    app::App,
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

pub struct ObserversPlugin;

impl Plugin for ObserversPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observer);
    }
}

#[derive(Event)]
pub struct TetriminoPlaced;

fn observer(
    _event: On<TetriminoPlaced>,
    mut commands: Commands,
    parent_query: Query<Entity, With<Grid>>,
    mut matrix_query: Query<(Entity, &Children, &mut GridMatrix), With<GridMatrix>>,
    child_query: Query<(Entity, &Transform, &Children), With<Tetrimino>>,
    squares_query: Query<
        (Entity, &MeshMaterial2d<ColorMaterial>, &Transform),
        With<TetriminoSquare>,
    >,
    tetrimino_assets: ResMut<TetriminoAssets>,
) {
    let (tetrimino, tetrimino_position, tetrimino_children) =
        child_query.single().expect("No tetrimino active");

    let parent = parent_query.single().expect("Grid not found");

    let (matrix_entity, matrix_children, mut grid_matrix) =
        matrix_query.single_mut().expect("GridMatrix not found");

    grid_matrix.place_tetrimino(
        tetrimino_position.translation,
        tetrimino_children,
        squares_query,
    );

    commands.entity(tetrimino).despawn();

    handle_full_rows(
        &mut commands,
        matrix_entity,
        &mut grid_matrix,
        matrix_children,
    );

    spawn_tetrimino(&mut commands, parent, &tetrimino_assets);
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
