use crate::bundles::sprite::sprite;
use bevy::prelude::*;

const IMAGE_PATH: &str = "images/tetrinimo_L_image.png";
const SCALE: f32 = 10.;
const CELL_SIZE: f32 = 80.;
#[derive(Component)]
pub struct Tetrimino;

pub fn tetrimino(
    asset_server: &Res<AssetServer>,
    window: &Window,
) -> ((Sprite, Transform), Tetrimino) {
    let image = asset_server.load(IMAGE_PATH);
    let position = (window.width() / 2., window.height());
    println!("{}", window.height());
    (sprite(image, position, SCALE), Tetrimino)
}
pub fn move_tetrimino(
    mut tetrimino: Single<&mut Transform, With<Tetrimino>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    if kb_input.just_pressed(KeyCode::KeyD) {
        tetrimino.translation += Vec3::new(CELL_SIZE, 0., 0.);
    }

    if kb_input.just_pressed(KeyCode::KeyA) {
        tetrimino.translation -= Vec3::new(CELL_SIZE, 0., 0.);
    }

    if kb_input.just_pressed(KeyCode::KeyW) {
        tetrimino.rotate_z(f32::to_radians(90.));
    }
}
