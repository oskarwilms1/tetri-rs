use bevy::prelude::*;

mod bundles;
mod startup;
use bundles::tetrimino::*;
use startup::startup_plugin::StartupPlugin;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(StartupPlugin)
        .add_systems(Update, move_tetrimino)
        .run();
}
