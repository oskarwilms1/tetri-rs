use crate::{board::tetrimino::Tetrimino, config::grid::grid_config::CELL_SIZE};
use bevy::prelude::*;
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tetrimino_movement);
    }
}

pub fn tetrimino_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut tetrimino: Single<&mut Transform, With<Tetrimino>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        tetrimino.translation.x -= CELL_SIZE;
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        tetrimino.translation.x += CELL_SIZE;
    }
}
