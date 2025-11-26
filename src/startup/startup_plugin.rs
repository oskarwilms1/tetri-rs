use bevy::prelude::*;

use super::assets::background::Background;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let game_background = Background::new(&mut meshes, &mut materials, 800., 1000.);

    commands.spawn(game_background);
}
