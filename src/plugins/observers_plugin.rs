#![allow(clippy::needless_pass_by_value)]

use bevy::{app::App, prelude::*};

use crate::plugins::observers::{
    handle_on_placed::handle_on_placed, handle_on_restart::handle_on_restart,
};

pub struct ObserversPlugin;

impl Plugin for ObserversPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_on_placed);
        app.add_observer(handle_on_restart);
    }
}
