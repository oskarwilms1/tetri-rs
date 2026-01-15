use bevy::ecs::{
    entity::Entity,
    event::Event,
    observer::On,
    query::With,
    system::{Commands, Query, Res, ResMut},
};

use crate::{
    game::game_state::GameState,
    plugins::{
        assets_plugin::{BackgroundAssets, ShadowAssets, TetriminoAssets, UiFont},
        startup_plugin::{restart, GameEntity},
    },
};

#[derive(Event)]
pub struct Restart;

pub fn handle_on_restart(
    _event: On<Restart>,
    commands: Commands,
    game_state: ResMut<GameState>,
    query: Query<Entity, With<GameEntity>>,
    background_assets: Res<BackgroundAssets>,
    tetrimino_assets: ResMut<TetriminoAssets>,
    shadow_assets: Res<ShadowAssets>,
    ui_font: Res<UiFont>,
) {
    restart(
        commands,
        game_state,
        query,
        background_assets,
        tetrimino_assets,
        shadow_assets,
        ui_font,
    );
}
