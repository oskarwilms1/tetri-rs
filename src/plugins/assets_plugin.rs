use bevy::color::palettes::tailwind::{
    BLUE_500, CYAN_500, GREEN_500, ORANGE_500, PURPLE_500, RED_500, YELLOW_500,
};
use bevy::prelude::*;

use crate::board::tetrimino_square::TetriminoVariant;

#[derive(Resource)]
pub struct UiFont {
    pub font: Handle<Font>,
}

#[derive(Resource)]
pub struct BackgroundAssets {
    pub cell_mesh: Handle<Mesh>,
    pub background_material: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct TetriminoAssets {
    pub cell_mesh: Handle<Mesh>,
    pub materials: [Handle<ColorMaterial>; 7],
}

impl TetriminoAssets {
    #[inline]
    pub fn material(&self, variant: TetriminoVariant) -> Handle<ColorMaterial> {
        self.materials[variant as usize].clone()
    }
}

pub struct AssetsPlugin;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct AssetLoading;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_assets.in_set(AssetLoading));
    }
}

fn load_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: ResMut<AssetServer>,
) {
    let cell_mesh = meshes.add(Rectangle::default());

    commands.insert_resource(BackgroundAssets {
        cell_mesh: cell_mesh.clone(),
        background_material: materials.add(ColorMaterial::default()),
    });

    commands.insert_resource(TetriminoAssets {
        cell_mesh,
        materials: [
            materials.add(ColorMaterial::from_color(CYAN_500)),
            materials.add(ColorMaterial::from_color(YELLOW_500)),
            materials.add(ColorMaterial::from_color(PURPLE_500)),
            materials.add(ColorMaterial::from_color(GREEN_500)),
            materials.add(ColorMaterial::from_color(RED_500)),
            materials.add(ColorMaterial::from_color(BLUE_500)),
            materials.add(ColorMaterial::from_color(ORANGE_500)),
        ],
    });
    let font = asset_server.load("fonts/BoldPixels.ttf");

    commands.insert_resource(UiFont { font });
}
