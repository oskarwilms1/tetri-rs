use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::InspectorOptions;
mod board;
mod config;
mod plugins;

use crate::plugins::{
    assets_plugin::AssetsPlugin, controls::controls_plugin::ControlsPlugin,
    gravity_plugin::GravityPlugin, observers_plugin::ObserversPlugin,
    startup_plugin::StartupPlugin,
};
#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
struct Configuration {
    name: String,
    #[inspector(min = 0.0, max = 1.0)]
    option: f32,
}
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
        .add_plugins(EguiPlugin::default())
        .init_resource::<Configuration>()
        .register_type::<Configuration>()
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(AssetsPlugin)
        .add_plugins(ObserversPlugin)
        .add_plugins(StartupPlugin)
        .add_plugins(ControlsPlugin)
        .add_plugins(GravityPlugin)
        .run();
}
