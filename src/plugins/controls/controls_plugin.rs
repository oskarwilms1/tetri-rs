use bevy::prelude::*;

use crate::plugins::controls::handle_input::handle_input;

pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input);
    }
}
