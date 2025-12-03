use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::board::grid::*;
use crate::board::tetrimino_square::TetriminoVariant;
use crate::plugins::assets_plugin::GameAssets;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_setup);
    }
}

pub fn game_setup(
    mut commands: Commands,
    tetrimino_assets: ResMut<GameAssets>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Prepare default assets
    let window: &Window = window_query.single().unwrap();
    let cell_mesh: &Handle<Mesh> = &tetrimino_assets.cell_mesh;

    let background_material: &Handle<ColorMaterial> = &tetrimino_assets.background_material;
    let tetrimino_cell_color: &Handle<ColorMaterial> = &tetrimino_assets.tetrimino_i_material;
    let x_offset: f32 = -window.width() / 2.;
    let y_offset: f32 = window.height() / 2.;
    // Spawn camera
    commands.spawn(Camera2d);
    // Spawn empty grid with default assets
    spawn_grid(
        &mut commands,
        cell_mesh,
        background_material,
        TetriminoVariant::I,
        tetrimino_cell_color,
        x_offset,
        y_offset,
    );
}
