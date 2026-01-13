use bevy::{
    asset::Handle,
    color::Color,
    ecs::{bundle::Bundle, component::Component},
    text::{Font, Justify, TextFont, TextLayout},
    ui::{widget::Text, AlignItems, BackgroundColor, JustifyContent, Node, Val},
    utils::default,
};

pub enum GameState {
    Playing,
    GameOver,
}
#[derive(Component)]
pub struct GameOverUI;

pub fn gameover_ui() -> impl Bundle {
    (
        GameOverUI,
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
        Text::new("Game Over!"),
        TextFont {
            font,
            font_size: 90.0,
            ..default()
        },
        TextLayout::new_with_justify(Justify::Center),
    )
}
