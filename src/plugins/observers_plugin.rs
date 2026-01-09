#![allow(clippy::needless_pass_by_value)]
use bevy::{
    app::App,
    ecs::{event::Event, observer::On},
    prelude::Plugin,
};

pub struct ObserversPlugin;

impl Plugin for ObserversPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observer);
    }
}
#[derive(Event)]
pub struct TetriminoPlaced;

fn observer(_event: On<TetriminoPlaced>) {
    println!("Tetrimino collides!");
}
