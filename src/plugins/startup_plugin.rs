use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::board::grid::*;
use crate::board::tetrimino::*;
use crate::board::tetrimino_square::TetriminoVariant;

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
    let tetrimino_cell_color: Handle<ColorMaterial> =
        materials.add(ColorMaterial::from_color(Color::BLACK));
    let x_offset: f32 = -window.width() / 2.;
    let y_offset: f32 = window.height() / 2.;
    // Spawn camera
    commands.spawn(Camera2d);
    // Spawn empty grid with default assets
    spawn_grid(
        &mut commands,
        &cell_mesh,
        &cell_material,
        x_offset,
        y_offset,
    );
    commands.spawn(tetrimino(
        &TetriminoVariant::I,
        &cell_mesh,
        &tetrimino_cell_color,
        x_offset,
        y_offset,
    ));
}
