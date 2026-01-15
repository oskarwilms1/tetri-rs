use bevy::{
    ecs::{
        entity::Entity,
        event::Event,
        hierarchy::Children,
        observer::On,
        query::{With, Without},
        system::Query,
    },
    transform::components::Transform,
};

use crate::board::{
    grid_matrix::GridMatrix,
    tetrimino::Tetrimino,
    tetrimino_shadow::{update_shadow, ShadowSquare, TetriminoShadow},
    tetrimino_square::TetriminoSquare,
};

#[derive(Event)]
pub struct UpdateShadow;

pub fn handle_update_shadow(
    _event: On<UpdateShadow>,
    shadow_query: Query<(Entity, &mut Transform), With<TetriminoShadow>>,
    tetrimino_query: Query<(Entity, &mut Transform), (With<Tetrimino>, Without<TetriminoShadow>)>,
    square_query: Query<
        &mut Transform,
        (
            With<TetriminoSquare>,
            Without<Tetrimino>,
            Without<TetriminoShadow>,
        ),
    >,
    shadow_squares_query: Query<
        &mut Transform,
        (
            With<ShadowSquare>,
            Without<TetriminoSquare>,
            Without<TetriminoShadow>,
            Without<Tetrimino>,
        ),
    >,
    children_query: Query<&Children>,
    matrix_query: Query<&GridMatrix>,
) {
    update_shadow(
        shadow_query,
        tetrimino_query,
        square_query,
        shadow_squares_query,
        children_query,
        matrix_query,
    );
}
