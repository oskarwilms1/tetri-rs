#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
use bevy::{prelude::*, window::WindowResolution};
mod board;
mod config;
mod game;
mod plugins;
mod scoreboard;

use crate::{
    game::game_state::GameState,
    plugins::{
        assets_plugin::AssetsPlugin, controls::controls_plugin::ControlsPlugin,
        gravity_plugin::GravityPlugin, observers_plugin::ObserversPlugin,
        startup_plugin::StartupPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tetri-rs".into(),
                name: Some("Tetri-rs".into()),
                resolution: WindowResolution::new(450, 900).with_scale_factor_override(1.0),
                resizable: false,
                prevent_default_event_handling: false,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(GameState::Playing)
        .add_plugins(AssetsPlugin)
        .add_plugins(ObserversPlugin)
        .add_plugins(StartupPlugin)
        .add_plugins(ControlsPlugin)
        .add_plugins(GravityPlugin)
        .run();
}
