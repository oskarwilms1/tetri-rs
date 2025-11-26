use bevy::prelude::*;

mod startup;

use startup::startup_plugin::StartupPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(StartupPlugin)
        .run();
}
