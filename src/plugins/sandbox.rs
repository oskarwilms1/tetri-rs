use bevy::{
    app::{Plugin, Update},
    ecs::{
        event::Event,
        observer::On,
        resource::Resource,
        system::{Commands, Res, ResMut},
    },
    time::{Time, Timer, TimerMode},
};

pub struct SandboxPlugin;

#[derive(Event)]
pub struct SandboxEvent;

#[derive(Resource)]
pub struct SandboxTimer(pub Timer);

impl Plugin for SandboxPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(SandboxTimer(Timer::from_seconds(1., TimerMode::Repeating)));
        app.add_systems(Update, sandbox);
        app.add_observer(sandbox2);
    }
}

pub fn sandbox(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SandboxTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.trigger(SandboxEvent);
    }
}
pub fn sandbox2(_event: On<SandboxEvent>) {
    println!("Message read!");
}
