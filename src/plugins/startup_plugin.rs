use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::board::grid::*;
use crate::board::tetrimino::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, game_setup);
    }
}

pub fn game_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // Prepare default assets
    let window: &Window = window_query.single().unwrap();
    let cell_mesh: Handle<Mesh> = meshes.add(Rectangle::default());
    let cell_material: Handle<ColorMaterial> = materials.add(ColorMaterial::default());
    let window_x_offset: f32 = -window.width() / 2.;
    let window_y_offset: f32 = window.height() / 2.;
    // Spawn camera
    commands.spawn(Camera2d);
    // Spawn empty grid with default assets
    commands.spawn_batch(empty_grid(
        &cell_mesh,
        &cell_material,
        window_x_offset,
        window_y_offset,
    ));
    commands.spawn(tetrimino(
        TetriminoVariant::I,
        cell_mesh,
        materials,
        window_x_offset,
        window_y_offset,
    ));
}
