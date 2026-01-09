#![allow(clippy::needless_pass_by_value)]
use crate::{
    board::{tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    config::grid::grid_config::CELL_SIZE,
    plugins::{
        controls::{boundary_checks::corrected_translation, collision::check_collision},
        observers_plugin::TetriminoPlaced,
    },
};
use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum Movement {
    Right,
    Left,
    Down,
}
pub fn handle_movement(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    tetrimino_query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    squares: Query<&Transform, (With<TetriminoSquare>, Without<Tetrimino>)>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyA)
        && !keyboard_input.just_pressed(KeyCode::KeyD)
        && !keyboard_input.just_pressed(KeyCode::KeyS)
    {
        return;
    }
    let (tetrimino_entity, mut tetrimino_transform) = tetrimino_query.into_inner();
    let children = children_of.get(tetrimino_entity).unwrap();

    let child_positions: Vec<Vec3> = children
        .iter()
        .filter_map(|child| squares.get(child).ok())
        .map(|gt| gt.translation)
        .collect();

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        movement(
            &mut commands,
            &mut tetrimino_transform,
            &child_positions,
            Movement::Left,
        );
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        movement(
            &mut commands,
            &mut tetrimino_transform,
            &child_positions,
            Movement::Right,
        );
    }
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        movement(
            &mut commands,
            &mut tetrimino_transform,
            &child_positions,
            Movement::Down,
        );
    }
}
pub fn movement(
    commands: &mut Commands,
    tetrimino_transform: &mut Mut<'_, Transform>,
    child_positions: &[Vec3],
    movement: Movement,
) {
    let movement_vec = match movement {
        Movement::Right => Vec3::new(CELL_SIZE, 0., 0.),
        Movement::Left => Vec3::new(-CELL_SIZE, 0., 0.),
        Movement::Down => Vec3::new(0., -CELL_SIZE, 0.),
    };
    let new_position = corrected_translation(
        tetrimino_transform.translation,
        child_positions,
        &movement_vec,
    );
    tetrimino_transform.translation = new_position;
    if check_collision(new_position, child_positions) {
        commands.trigger(TetriminoPlaced);
    }
}
