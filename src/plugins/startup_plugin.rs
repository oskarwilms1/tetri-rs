#![allow(clippy::needless_pass_by_value)]
use crate::board::tetrimino::spawn_tetrimino;
use crate::plugins::assets_plugin::{AssetLoading, BackgroundAssets, TetriminoAssets};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::board::grid::spawn_grid;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_setup.after(AssetLoading));
    }
}

pub fn game_setup(
    mut commands: Commands,
    background_assets: ResMut<BackgroundAssets>,
    tetrimino_assets: ResMut<TetriminoAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Prepare default assets
    let window: &Window = window_query.single().unwrap();
    let cell_mesh: &Handle<Mesh> = &background_assets.cell_mesh;

    let background_material: &Handle<ColorMaterial> = &background_assets.background_material;

    let x_offset: f32 = -window.width() / 2.;
    let y_offset: f32 = window.height() / 2.;
    // Spawn camera
    commands.spawn(Camera2d);
    // Spawn empty grid with default assets
    let grid: Entity = spawn_grid(
        &mut commands,
        cell_mesh,
        background_material,
        x_offset,
        y_offset,
    );

    spawn_tetrimino(&mut commands, grid, &tetrimino_assets);
}
