#![allow(clippy::needless_pass_by_value)]
use crate::board::tetrimino::Tetrimino;
use crate::board::tetrimino_square::TetriminoSquare;
use crate::plugins::controls::handle_movement::{movement, Movement};
use bevy::prelude::*;
#[derive(Resource)]
pub struct DownTimer(pub Timer);

pub fn tick_down(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<DownTimer>,
    tetrimino_query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    squares: Query<&Transform, (With<TetriminoSquare>, Without<Tetrimino>)>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let (tetrimino_entity, mut tetrimino_transform) = tetrimino_query.into_inner();

        let children = children_of.get(tetrimino_entity).unwrap();

        let child_positions: Vec<Vec3> = children
            .iter()
            .filter_map(|child| squares.get(child).ok())
            .map(|gt| gt.translation)
            .collect();
        movement(
            &mut commands,
            &mut tetrimino_transform,
            &child_positions,
            Movement::Down,
        );
    }
}
