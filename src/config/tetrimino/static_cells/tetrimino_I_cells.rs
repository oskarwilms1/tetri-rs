use bevy::prelude::Vec3;
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub static TETRINIMO_I_CELLS: Lazy<HashMap<i32, [Vec3; 4]>> = Lazy::new(|| {
    HashMap::from([
        (
            0,
            [
                Vec3::new(1., -1., 0.),
                Vec3::new(2., -1., 0.),
                Vec3::new(3., -1., 0.),
                Vec3::new(4., -1., 0.),
            ],
        ),
        (
            1,
            [
                Vec3::new(2., 0., 0.),
                Vec3::new(2., -1., 0.),
                Vec3::new(2., -2., 0.),
                Vec3::new(2., -3., 0.),
            ],
        ),
        (
            2,
            [
                Vec3::new(1., -1., 0.),
                Vec3::new(2., -1., 0.),
                Vec3::new(3., -1., 0.),
                Vec3::new(4., -1., 0.),
            ],
        ),
        (
            3,
            [
                Vec3::new(2., 0., 0.),
                Vec3::new(2., -1., 0.),
                Vec3::new(2., -2., 0.),
                Vec3::new(2., -3., 0.),
            ],
        ),
    ])
});
