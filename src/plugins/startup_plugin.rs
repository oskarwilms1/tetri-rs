#![allow(clippy::needless_pass_by_value)]
use crate::board::tetrimino::spawn_tetrimino;
use crate::board::tetrimino_shadow::tetrimino_shadow;
use crate::config::grid::grid_config::{CELL_SIZE, COLUMN_AMOUNT, ROW_AMOUNT};
use crate::game::game_state::{gameover_text, gameover_ui, GameState};
use crate::plugins::assets_plugin::{
    AssetLoading, BackgroundAssets, ShadowAssets, TetriminoAssets, UiFont,
};
use crate::plugins::observers::shadow_update::UpdateShadow;
use crate::scoreboard::scoreboard::ScoreboardBundle;
use bevy::prelude::*;

use crate::board::grid::spawn_grid;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_setup.after(AssetLoading));
    }
}
#[derive(Component)]
pub struct GameEntity;
pub fn game_setup(
    mut commands: Commands,
    background_assets: Res<BackgroundAssets>,
    tetrimino_assets: ResMut<TetriminoAssets>,
    shadow_assets: Res<ShadowAssets>,
    ui_font: Res<UiFont>,
) {
    // Spawn centered 2D camera
    let font = ui_font.font.clone();
    commands.spawn((Camera2d, Transform::default(), GameEntity));
    let cell_mesh = &background_assets.cell_mesh;
    let background_material = &background_assets.background_material;

    // Center the grid in world space
    let grid_width = COLUMN_AMOUNT * CELL_SIZE;
    let grid_height = -ROW_AMOUNT * CELL_SIZE;

    let x_offset = -grid_width / 2.0 + CELL_SIZE / 2.0;
    let y_offset = -grid_height / 2.0;

    commands.spawn((
        ScoreboardBundle::new(font.clone(), -grid_height / 1.5),
        GameEntity,
    ));
    let grid = spawn_grid(
        &mut commands,
        cell_mesh,
        background_material,
        x_offset,
        y_offset,
    );

    spawn_tetrimino(&mut commands, grid, &tetrimino_assets);
    let shadow = commands
        .spawn(tetrimino_shadow(
            shadow_assets.mesh.clone(),
            shadow_assets.color.clone(),
        ))
        .id();
    commands.entity(grid).add_child(shadow);
    commands.spawn(gameover_ui()).with_children(|parent| {
        parent.spawn(gameover_text(font));
    });
    commands.trigger(UpdateShadow);
}

pub fn restart(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    query: Query<Entity, With<GameEntity>>,
    background_assets: Res<BackgroundAssets>,
    tetrimino_assets: ResMut<TetriminoAssets>,
    shadow_assets: Res<ShadowAssets>,
    ui_font: Res<UiFont>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    game_setup(
        commands,
        background_assets,
        tetrimino_assets,
        shadow_assets,
        ui_font,
    );
    *game_state = GameState::Playing;
}
