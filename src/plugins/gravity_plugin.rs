#![allow(clippy::needless_pass_by_value)]
use bevy::{
    app::{Plugin, Update},
    ecs::{
        entity::Entity,
        hierarchy::Children,
        query::{With, Without},
        resource::Resource,
        system::{Commands, Query, ResMut, Single},
    },
    time::{Time, Timer, TimerMode},
    transform::components::Transform,
};

use crate::{
    board::{grid_matrix::GridMatrix, tetrimino::Tetrimino, tetrimino_square::TetriminoSquare},
    plugins::controls::{handle_input::Movement, handle_movement::handle_move},
};

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GravityTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, gravity);
    }
}

#[derive(Resource)]
struct GravityTimer(Timer);

fn gravity(
    mut timer: ResMut<GravityTimer>,
    time: ResMut<Time>,
    mut commands: Commands,
    tetrimino_query: Single<(Entity, &mut Transform), With<Tetrimino>>,
    children_of: Query<&Children>,
    squares: Query<(&mut TetriminoSquare, &mut Transform), Without<Tetrimino>>,
    grid_matrix: Query<&GridMatrix>,
) {
    timer.0.tick(time.delta());

    if timer.0.is_finished() {
        handle_move(
            &mut commands,
            tetrimino_query,
            children_of,
            squares,
            Movement::Down,
            grid_matrix,
        );
    }
}
