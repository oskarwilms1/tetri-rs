use bevy::prelude::Vec3;

pub const TETRIMINO_O_CELLS: [[Vec3; 4]; 4] = [
    // rotation 0
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
    // rotation 1
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
    // rotation 2
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
    // rotation 3
    [
        Vec3::new(1., -1., 0.),
        Vec3::new(2., -1., 0.),
        Vec3::new(1., -2., 0.),
        Vec3::new(2., -2., 0.),
    ],
];
