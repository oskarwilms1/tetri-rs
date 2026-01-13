use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component, query::With, system::Query},
    sprite::Text2d,
    text::{Font, Justify, TextFont, TextLayout},
    transform::components::Transform,
    utils::default,
};

#[derive(Component)]
pub struct Scoreboard;
#[derive(Component)]
pub struct Score(pub u64);
#[derive(Bundle)]
pub struct ScoreboardBundle {
    entity: Scoreboard,
    score: Score,
    pub text: Text2d,
    text_font: TextFont,
    text_layout: TextLayout,
    position: Transform,
}
impl ScoreboardBundle {
    pub fn new(font: Handle<Font>, window_height: f32) -> Self {
        Self {
            entity: Scoreboard,
            score: Score(0),
            text: Text2d::new("0"),
            text_font: TextFont {
                font,
                font_size: 67.0,
                ..default()
            },
            text_layout: TextLayout::new_with_justify(Justify::Center),
            position: Transform::from_xyz(0., window_height, 1.),
        }
    }
}

pub fn increment_score(mut query: Query<(&mut Score, &mut Text2d), With<Scoreboard>>, points: u64) {
    let (mut score, mut text) = query.single_mut().unwrap();
    if points > 0 {
        score.0 += 100 * 2_u64.pow((points - 1_u64).try_into().unwrap());

        *text = Text2d::new(score.0.to_string());
    }
}
