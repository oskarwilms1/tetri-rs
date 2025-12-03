use bevy::prelude::*;

mod board;
mod config;
mod plugins;

use crate::plugins::{
    assets_plugin::AssetsPlugin, controls_plugin::ControlsPlugin, startup_plugin::StartupPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetri-rs".into(),
                name: Some("Tetri-rs".into()),
                resolution: (300, 600).into(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        //.insert_resource(ClearColor(Color::srgb(0., 0., 0.)))
        .add_plugins(AssetsPlugin)
        .add_plugins(StartupPlugin)
        .add_plugins(ControlsPlugin)
        .run();
}
