use bevy::prelude::*;

pub fn sprite(image: Handle<Image>, position: (f32, f32), scale_xyz: f32) -> (Sprite, Transform) {
    (
        Sprite::from_image(image),
        Transform::from_xyz(position.0, position.1, 0.).with_scale(Vec3::splat(scale_xyz)),
    )
}
