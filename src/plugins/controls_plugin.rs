use crate::{
    board::{tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    config::grid::grid_config::CELL_SIZE,
};
use bevy::prelude::*;
pub struct ControlsPlugin;

impl Plugin for ControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tetrimino_movement);
        app.add_systems(Update, rotate_tetrimino_squares);
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

pub fn rotate_tetrimino_squares(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut TetriminoSquare, &mut Transform)>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        for (mut square, mut transform) in &mut query {
            square.rotate();
            *transform = square.get_rotation();
        }
    }
}
