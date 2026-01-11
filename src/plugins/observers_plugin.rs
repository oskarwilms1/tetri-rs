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
    mut matrix_query: Query<&mut GridMatrix>,
    child_query: Query<(Entity, &Transform, &Children), With<Tetrimino>>,
    squares_query: Query<(Entity, &Transform), With<TetriminoSquare>>,

    tetrimino_assets: ResMut<TetriminoAssets>,
) {
    let (tetrimino, tetrimino_position, children) =
        child_query.single().expect("No tetrimino active");

    let matrix = &mut matrix_query.single_mut().expect("Matrix not found");
    let parent = parent_query.single().expect("Didn't find the Grid");
    commands.entity(tetrimino).remove::<Tetrimino>();
    matrix.place_tetrimino(tetrimino_position.translation, children, squares_query);
    spawn_tetrimino(&mut commands, parent, &tetrimino_assets);
    info!("{:?}", matrix_query.single().unwrap());
}
