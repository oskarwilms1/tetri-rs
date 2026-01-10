#![allow(clippy::needless_pass_by_value)]
use bevy::{
    app::App,
    ecs::{entity::Entity, event::EntityEvent, observer::On},
    prelude::Plugin,
};

pub struct ObserversPlugin;

impl Plugin for ObserversPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(observer);
    }
}
#[derive(EntityEvent)]
pub struct TetriminoPlaced(Entity);

fn observer(_event: On<TetriminoPlaced>) {}
