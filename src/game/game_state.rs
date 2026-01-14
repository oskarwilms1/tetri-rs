use bevy::{
    asset::Handle,
    camera::visibility::Visibility,
    color::Color,
    ecs::{bundle::Bundle, component::Component, query::With, resource::Resource, system::Query},
    text::{Font, Justify, TextFont, TextLayout},
    ui::{widget::Text, AlignItems, BackgroundColor, JustifyContent, Node, Val},
    utils::default,
};

use crate::plugins::startup_plugin::GameEntity;
#[derive(Resource, PartialEq, Eq)]
pub enum GameState {
    Playing,
    GameOver,
}
#[derive(Component)]
pub struct GameOverUI;

pub fn gameover_ui() -> impl Bundle {
    (
        GameOverUI,
        GameEntity,
        Visibility::Hidden,
        BackgroundColor(Color::linear_rgba(0., 0., 0., 0.5)),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
    )
}

pub fn gameover_text(font: Handle<Font>) -> impl Bundle {
    (
        Text::new("Game Over!\nPress R to restart"),
        TextFont {
            font,
            font_size: 60.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
    )
}

pub fn show_gameover(mut query: Query<&mut Visibility, With<GameOverUI>>) {
    let mut visibility = query.single_mut().unwrap();
    *visibility = Visibility::Visible;
}
