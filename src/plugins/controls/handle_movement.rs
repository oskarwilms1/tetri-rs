#![allow(clippy::needless_pass_by_value)]
use crate::{
    board::{grid_matrix::GridMatrix, tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    config::grid::grid_config::CELL_SIZE,
    plugins::{
        controls::{
            boundary_checks::corrected_translation,
            collision::{check_lowest_collision, check_tetrimino_collision},
            handle_input::Movement,
        },
        observers::handle_on_placed::TetriminoPlaced,
    },
};
use bevy::prelude::*;

pub fn handle_move(
    commands: &mut Commands,
    mut tetrimino_query: Query<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
    movement: Movement,
    grid_matrix: Query<&GridMatrix>,
) {
    let (entity, mut transform) = tetrimino_query.single_mut().unwrap();
    let children = children_of.get(entity).unwrap();
    let matrix = grid_matrix.single().unwrap();

    let child_positions: Vec<Vec3> = children
        .iter()
        .filter_map(|child| squares.get(child).ok())
        .map(|gt| gt.1.translation)
        .collect();

    let movement_vec = match movement {
        Movement::Right => {
            if check_tetrimino_collision(matrix, transform.translation, &child_positions, 1., 0.) {
                return;
            }
            Vec3::new(CELL_SIZE, 0., 0.)
        }
        Movement::Left => {
            if check_tetrimino_collision(matrix, transform.translation, &child_positions, -1., 0.) {
                return;
            }
            Vec3::new(-CELL_SIZE, 0., 0.)
        }

        Movement::Down => {
            if check_tetrimino_collision(matrix, transform.translation, &child_positions, 0., 1.) {
                commands.trigger(TetriminoPlaced);
                return;
            }

            Vec3::new(0., -CELL_SIZE, 0.)
        }
        Movement::Space => {
            let new_y_value =
                check_lowest_collision(matrix, transform.translation, &child_positions);
            transform.translation.y -= new_y_value;
            commands.trigger(TetriminoPlaced);
            return;
        }
    };

    let new_position =
        corrected_translation(transform.translation, &child_positions, &movement_vec);

    transform.translation = new_position;
}
