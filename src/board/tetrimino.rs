use crate::board::tetrimino_square::spawn_tetrinimo_children;
use crate::board::tetrimino_square::TetriminoVariant;
use bevy::prelude::*;

#[derive(Component)]
pub struct Tetrimino;
#[derive(Component)]
pub enum IsActive {
    Yes,
    //No,
}
#[derive(Bundle)]
pub struct TetriminoBundle {
    tetrimino: Tetrimino,
    is_active: IsActive,
    transform: Transform,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
}

impl TetriminoBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            tetrimino: Tetrimino,
            is_active: IsActive::Yes,
            transform: Transform::from_xyz(x, y, 1.),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
        }
    }
}

pub fn tetrimino(
    variant: &TetriminoVariant,
    cell_mesh: &Handle<Mesh>,
    cell_color: &Handle<ColorMaterial>,
    x: f32,
    y: f32,
) -> impl Bundle {
    (
        TetriminoBundle::new(x, y),
        spawn_tetrinimo_children(variant, cell_mesh, cell_color),
    )
}
