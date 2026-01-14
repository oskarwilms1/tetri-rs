use crate::board::tetrimino_square::spawn_tetrinimo_children;
use crate::board::tetrimino_square::TetriminoVariant;
use crate::config::grid::grid_config::CELL_SIZE;
use crate::config::tetrimino::variants::VARIANTS;
use crate::plugins::assets_plugin::TetriminoAssets;
use crate::plugins::startup_plugin::GameEntity;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Tetrimino;
#[derive(Bundle)]
pub struct TetriminoBundle {
    tetrimino: Tetrimino,
    transform: Transform,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
}

impl TetriminoBundle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            tetrimino: Tetrimino,
            transform: Transform::from_xyz(x, y, 1.),
            visibility: Visibility::default(),
            inherited_visibility: InheritedVisibility::default(),
        }
    }
}

pub fn spawn_tetrimino(
    commands: &mut Commands,
    parent: Entity,
    tetrimino_assets: &ResMut<TetriminoAssets>,
) {
    let mut rng = rand::rng();
    let variant: &TetriminoVariant = &VARIANTS[rng.random_range(0..=6)];
    let mesh = &tetrimino_assets.cell_mesh;
    let material = &tetrimino_assets.material(*variant);
    let tetrimino = commands
        .spawn(tetrimino(variant, mesh, material, 3. * CELL_SIZE, 0.))
        .id();
    commands.entity(parent).add_child(tetrimino);
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
        GameEntity,
        spawn_tetrinimo_children(variant, cell_mesh, cell_color),
    )
}
