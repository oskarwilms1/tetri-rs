use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::bundles::tetrimino::*;
pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window: &Window = window_query.single().unwrap();

    commands.spawn(camera(window));
    commands.spawn(tetrimino(&asset_server, window));
}
fn camera(window: &Window) -> (Camera2d, Transform) {
    (
        Camera2d,
        Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
    )
}
