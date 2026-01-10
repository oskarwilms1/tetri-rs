#![allow(clippy::needless_pass_by_value)]
use bevy::{
    app::App,
    ecs::{entity::Entity, event::EntityEvent, observer::On},
    prelude::*,
};

use crate::{
    board::{
        grid::Grid,
        tetrimino::{spawn_tetrimino, Tetrimino},
    },
    plugins::assets_plugin::TetriminoAssets,
};

pub struct ObserversPlugin;

impl Plugin for ObserversPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observer);
    }
}

#[derive(EntityEvent)]
pub struct TetriminoPlaced(pub Entity);

fn observer(
    event: On<TetriminoPlaced>,
    mut commands: Commands,
    parent_query: Query<Entity, With<Grid>>,
    tetrimino_assets: ResMut<TetriminoAssets>,
) {
    let entity = event.0;
    let parent = parent_query.single().expect("Didn't find the Grid");

    commands.entity(entity).remove::<Tetrimino>();

    spawn_tetrimino(&mut commands, parent, &tetrimino_assets);
}
