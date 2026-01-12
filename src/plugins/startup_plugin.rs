#![allow(clippy::needless_pass_by_value)]
use crate::board::tetrimino::spawn_tetrimino;
use crate::config::grid::grid_config::{CELL_SIZE, COLUMN_AMOUNT, ROW_AMOUNT};
use crate::plugins::assets_plugin::{AssetLoading, BackgroundAssets, TetriminoAssets};
use bevy::prelude::*;


use crate::board::grid::spawn_grid;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_setup.after(AssetLoading));
    }
}

pub fn game_setup(
    mut commands: Commands,
    background_assets: Res<BackgroundAssets>,
    tetrimino_assets: ResMut<TetriminoAssets>,
) {
    // Spawn centered 2D camera
    commands.spawn((
        Camera2d,
        Transform::default(),
    ));

    let cell_mesh = &background_assets.cell_mesh;
    let background_material = &background_assets.background_material;

    // Center the grid in world space
    let grid_width = COLUMN_AMOUNT as f32 * CELL_SIZE;
    let grid_height = -ROW_AMOUNT as f32 * CELL_SIZE;

    let x_offset = -grid_width / 2.0 + CELL_SIZE/2.0;
    let y_offset = -grid_height / 2.0;

    let grid = spawn_grid(
        &mut commands,
        cell_mesh,
        background_material,
        x_offset,
        y_offset,
    );

    spawn_tetrimino(&mut commands, grid, &tetrimino_assets);
}
