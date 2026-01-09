use bevy::prelude::*;

use crate::plugins::controls::handle_movement::handle_movement;
use crate::plugins::controls::handle_rotation::handle_rotation;
use crate::plugins::controls::tick_movement::{tick_down, DownTimer};

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DownTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(Update, handle_movement);
        app.add_systems(Update, handle_rotation);
        app.add_systems(Update, tick_down);
    }
}
